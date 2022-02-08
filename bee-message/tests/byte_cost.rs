// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    byte_cost::{min_deposit, ByteCost, ByteCostConfig},
    milestone::MilestoneIndex,
    output::{
        unlock_condition::{AddressUnlockCondition, DustDepositReturnUnlockCondition},
        BasicOutput, BasicOutputBuilder, Output, OutputId,
    },
    MessageId,
};
use bee_test::rand::{
    address::rand_alias_address,
    output::{rand_alias_output, rand_basic_output, rand_foundry_output, rand_nft_output},
};

use std::mem::size_of;

const CONFIG: ByteCostConfig = ByteCostConfig {
    byte_cost: 1.0,
    weight_for_data: 10,
    weight_for_key: 1,
};

type ConfirmationUnixTimestamp = f32;

const OFFSET: u64 = (size_of::<OutputId>()
    + size_of::<MessageId>()
    + size_of::<MilestoneIndex>()
    + size_of::<ConfirmationUnixTimestamp>()) as u64;

fn output_in_range(output: Output, range: std::ops::RangeInclusive<u64>) {
    let v_bytes = &output.weighted_bytes(&CONFIG);
    assert!(range.contains(v_bytes), "{:#?} has byte cost {}", output, v_bytes);
}

#[test]
fn valid_byte_cost_range() {
    output_in_range(Output::Alias(rand_alias_output()), (445 - OFFSET)..=(29_620 - OFFSET));
    output_in_range(Output::Basic(rand_basic_output()), (414 - OFFSET)..=(13_485 - OFFSET));
    output_in_range(
        Output::Foundry(rand_foundry_output()),
        (496 - OFFSET)..=(21_365 - OFFSET),
    );
    output_in_range(Output::Nft(rand_nft_output()), (436 - OFFSET)..=(21_734 - OFFSET));
}

#[test]
fn basic_output() {
    let output = BasicOutput::build(2_000_000)
        .unwrap()
        .add_unlock_condition(AddressUnlockCondition::new(rand_alias_address().into()).into())
        .add_unlock_condition(
            DustDepositReturnUnlockCondition::new(rand_alias_address().into(), 414)
                .unwrap()
                .into(),
        )
        .finish()
        .unwrap();
    assert_eq!(min_deposit(&CONFIG, &Output::Basic(output),), 414);
}
