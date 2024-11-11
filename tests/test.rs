use alloy_primitives::U256;
use alloy_unit_macros::*;

#[test]
fn test_macros() {
    assert_eq!(
        ether!(150_000),
        U256::from(150000) * U256::from(10).pow(U256::from(18))
    );

    let a = ether!(1.678);
    println!("{}", a);

    let a = gwei!(1.678);
    println!("{}", a);

    let a = mwei!(-1.678);
    println!("{}", a);

    let a = wei!(+1);
    println!("{}", a);
}
