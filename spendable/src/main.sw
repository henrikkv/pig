predicate;

use std::inputs::*;
use std::outputs::*;
use std::constants::ZERO_B256;
use std::math::*;
use std::auth::*;
use std::primitive_conversions::u32::*;

configurable {
    ASSET_ID: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    OWNER: Address = Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}

fn main() -> bool {
    assert(ASSET_ID != AssetId::default());
    assert(caller_address().unwrap() == OWNER);
    let output_count = output_count();
    let mut i = 0;
    while i < output_count {
        match output_asset_to(i) {
            Some(id) => {
                if (Address::from(id) == OWNER
                    || Address::from(id) == predicate_address())
                {
                    let amount = output_amount(i);
                    if amount <= 9999 {
                        i = i + 1;
                        continue;
                    };
                    let digits = amount.log(10) - 4;
                    let power = 10.pow(<u32 as TryFrom<u64>>::try_from(digits - 4).unwrap());
                    let rounded = (amount / power) * power;
                    assert(amount == rounded);
                }
            },
            _ => {},
        };
        i = i + 1;
    };
    true
}
