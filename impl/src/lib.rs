#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use alloy_primitives::{I256, U256};
use quote::{quote, ToTokens};
use syn::bracketed;

#[proc_macro]
pub fn amount_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match try_expand(item.into()) {
        Ok(res) => res.into(),
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #error
            }
            .into()
        }
    }
}

fn try_expand(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut iter = input.clone().into_iter();

    let path = iter
        .next()
        .ok_or_else(|| syn::Error::new_spanned(&input, "path expected"))?;
    let proc_macro2::TokenTree::Group(group) = path else {
        return Err(syn::Error::new_spanned(&input, "path expected"));
    };
    let path = group.stream();

    let decimals = match iter
        .next()
        .ok_or_else(|| syn::Error::new_spanned(&input, "expected decimal number"))?
    {
        proc_macro2::TokenTree::Group(group) => syn::parse::Parser::parse2(
            |group: syn::parse::ParseStream<'_>| {
                let inner;
                bracketed!(inner in group);
                let lit = inner.parse::<proc_macro2::Literal>()?;
                lit.to_string().parse::<usize>().map_err(|err| {
                    syn::Error::new_spanned(&input, format!("parsing failed: {err}"))
                })
            },
            group.to_token_stream(),
        )?,
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "expected decimal number in group",
            ));
        }
    };

    let mut item = iter
        .next()
        .ok_or_else(|| syn::Error::new_spanned(&input, "content is empty"))?;

    let sign = if let proc_macro2::TokenTree::Punct(punct) = item {
        item = iter
            .next()
            .ok_or_else(|| syn::Error::new_spanned(&punct, "missing literal"))?;
        if punct.as_char() == '+' {
            Some(true)
        } else if punct.as_char() == '-' {
            Some(false)
        } else {
            return Err(syn::Error::new_spanned(&punct, "expected sign or literal"));
        }
    } else {
        None
    };

    let proc_macro2::TokenTree::Literal(literal) = &item else {
        return Err(syn::Error::new_spanned(&item, "expected literal"));
    };

    if let Some(next) = iter.next() {
        return Err(syn::Error::new_spanned(&next, "unexpeted item"));
    }

    let literal_string = literal.to_string();
    let mut literal_iter = literal_string.split(".");

    let upper = U256::from_str_radix(literal_iter.next().unwrap(), 10)
        .map_err(|err| syn::Error::new_spanned(literal, err.to_string()))?;

    let (lower, shift) = if let Some(lower) = literal_iter.next() {
        let shift = lower.chars().filter(|x| x.is_numeric()).count();
        let lower = U256::from_str_radix(lower, 10)
            .map_err(|err| syn::Error::new_spanned(literal, err.to_string()))?;
        if let Some(next) = literal_iter.next() {
            return Err(syn::Error::new_spanned(next, "expected literal"));
        }
        (lower, shift)
    } else {
        (U256::ZERO, 0)
    };

    let upper = upper
        .checked_mul(U256::from(10).pow(U256::from(decimals)))
        .ok_or_else(|| syn::Error::new_spanned(literal, "overflow"))?;
    let shift_rev = decimals
        .checked_sub(shift)
        .ok_or_else(|| syn::Error::new_spanned(literal, "too many decimals"))?;
    let lower = lower
        .checked_mul(U256::from(10).pow(U256::from(shift_rev)))
        .ok_or_else(|| syn::Error::new_spanned(literal, "overflow"))?;

    let res = upper
        .checked_add(lower)
        .ok_or_else(|| syn::Error::new_spanned(literal, "overflow"))?;

    let result = if let Some(sign) = sign {
        let mut res =
            I256::try_from(res).map_err(|_| syn::Error::new_spanned(literal, "overflow"))?;
        if !sign {
            res = I256::ZERO
                .checked_sub(res)
                .ok_or_else(|| syn::Error::new_spanned(literal, "overflow"))?;
        }

        let [a, b, c, d] = res.as_limbs();

        quote! {
            #path ::__private::alloy_primitives::I256::from_limbs([#a, #b, #c, #d])
        }
    } else {
        let [a, b, c, d] = res.as_limbs();

        quote! {
            #path ::__private::alloy_primitives::U256::from_limbs([#a, #b, #c, #d])
        }
    };

    Ok(result)
}
