# alloy-unit-macros

```rust
use alloy_unit_macros::*;
use alloy_primitives::U256;

const ETHER_AMOUNT: U256 = ether!(1.23);
const GWEI_AMOUNT: U256 = gwei!(1.23);
const BTC_AMOUNT: U256 = btc!(1.23);
const USDC_AMOUNT: U256 = mwei!(1.23);
const WEI_AMOUNT: U256 = wei!(123);

const WAD_AMOUNT: U256 = wad!(123);
const RAY_AMOUNT: U256 = ray!(123);

const AMOUNT: U256 = u256!(123);
const CUSTOM_AMOUNT: U256 = amount!(14, 1.23);
```
