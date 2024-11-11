#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![doc = include_str!("../README.md")]

/// `1` Ether = `1e18` Wei = `1000000000000000000` Wei
///
/// /// # Example
///
/// ```rust
/// use alloy_unit_macros::ether;
/// use alloy_primitives::{I256, U256};
///
/// assert_eq!(ether!(1), U256::from(10).pow(U256::from(18)));
/// assert_eq!(ether!(-1), -I256::try_from(10).unwrap().pow(U256::from(18)));
#[macro_export]
macro_rules! ether {
    ($($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [18] $($t)*)
    }
}

/// `1` GWei = `1e9` Wei = `1000000000` Wei
///
/// # Example
///
/// ```rust
/// use alloy_unit_macros::gwei;
/// use alloy_primitives::{I256, U256};
///
/// assert_eq!(gwei!(1), U256::from(10).pow(U256::from(9)));
/// assert_eq!(gwei!(-1), -I256::try_from(10).unwrap().pow(U256::from(9)));
/// ```
#[macro_export]
macro_rules! gwei {
    ($($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [9] $($t)*)
    }
}

/// `1` BTC = `1e8` Wei = `100000000` Wei
///
/// # Example
///
/// ```rust
/// use alloy_unit_macros::btc;
/// use alloy_primitives::{I256, U256};
///
/// assert_eq!(btc!(1), U256::from(10).pow(U256::from(8)));
/// assert_eq!(btc!(-1), -I256::try_from(10).unwrap().pow(U256::from(8)));
/// ```
#[macro_export]
macro_rules! btc {
    ($($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [8] $($t)*)
    }
}

/// `1` MWei = `1e6` Wei = `1000000` Wei
///
/// Unit used by USDC and USDT.
///
/// # Example
///
/// ```rust
/// use alloy_unit_macros::mwei;
/// use alloy_primitives::{I256, U256};
///
/// assert_eq!(mwei!(1), U256::from(10).pow(U256::from(6)));
/// assert_eq!(mwei!(-1), -I256::try_from(10).unwrap().pow(U256::from(6)));
/// ```
#[macro_export]
macro_rules! mwei {
    ($($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [6] $($t)*)
    }
}

/// # Example
///
/// ```rust
/// use alloy_unit_macros::wei;
/// use alloy_primitives::{I256, U256};
///
/// assert_eq!(wei!(1), U256::from(1));
/// assert_eq!(wei!(-1), -I256::try_from(1).unwrap());
/// ```
#[macro_export]
macro_rules! wei {
    ($($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [0] $($t)*)
    }
}

/// # Example
///
/// ```rust
/// use alloy_unit_macros::u256;
/// use alloy_primitives::{I256, U256};
///
/// assert_eq!(u256!(1), U256::from(1));
/// assert_eq!(u256!(-1), -I256::try_from(1).unwrap());
/// ```
#[macro_export]
macro_rules! u256 {
    ($($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [0] $($t)*)
    }
}

/// # Usage
///
/// ```rust
/// use alloy_unit_macros::*;
///
/// assert_eq!(amount!(0, 100), wei!(100));
/// assert_eq!(amount!(6, 100), mwei!(100));
/// assert_eq!(amount!(8, 100), btc!(100));
/// assert_eq!(amount!(9, 100), gwei!(100));
/// assert_eq!(amount!(18, 100), ether!(100));
/// ```
#[macro_export]
macro_rules! amount {
    ($d:literal, $($t:tt)*) => {
        $crate::__private::alloy_unit_macros_impl::amount_impl!([$crate] [$d] $($t)*)
    }
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use alloy_primitives;
    #[doc(hidden)]
    pub use alloy_unit_macros_impl;
}
