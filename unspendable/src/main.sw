predicate;

use std::inputs::*;
use std::outputs::*;
use std::constants::ZERO_B256;

configurable {
    ASSET_ID: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}

fn main() -> bool {
    assert(ASSET_ID != AssetId::default());
    let pig = input_asset_id(0).unwrap();
    assert(pig == ASSET_ID);
    let output_count = output_count();
    let mut i = 0;
    while i < output_count {
        match output_asset_id(i) {
            Some(value) => assert(value != ASSET_ID),
            None => {},
        }
        i = i + 1;
    }
    true
}
