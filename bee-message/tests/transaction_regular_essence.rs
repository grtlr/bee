// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    address::{Address, Ed25519Address},
    constant::IOTA_SUPPLY,
    input::{Input, TreasuryInput, UtxoInput},
    output::{unlock_condition::AddressUnlockCondition, BasicOutput, Output, TreasuryOutput},
    payload::{
        milestone::MilestoneId,
        transaction::{RegularTransactionEssence, TransactionId},
        Payload,
    },
    Error,
};
use bee_test::rand::{
    bytes::rand_bytes_array,
    payload::{rand_tagged_data_payload, rand_treasury_transaction_payload},
};

use packable::bounded::TryIntoBoundedU16Error;

const TRANSACTION_ID: &str = "52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const ED25519_ADDRESS_1: &str = "d56da1eb7726ed482dfe9d457cf548c2ae2a6ce3e053dbf82f11223be476adb9";
const ED25519_ADDRESS_2: &str = "efda4275375ac3675abff85235fd25a1522a2044cc6027a31b310857246f18c0";

#[test]
fn kind() {
    assert_eq!(RegularTransactionEssence::KIND, 0);
}

#[test]
fn build_valid() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input1 = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let input2 = Input::Utxo(UtxoInput::new(txid, 1).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input1, input2])
        .add_output(output)
        .finish();

    assert!(essence.is_ok());
}

#[test]
fn build_valid_with_payload() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input1 = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let input2 = Input::Utxo(UtxoInput::new(txid, 1).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );
    let payload = Payload::from(rand_tagged_data_payload());

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input1, input2])
        .add_output(output)
        .with_payload(payload)
        .finish();

    assert!(essence.is_ok());
}

#[test]
fn build_valid_add_inputs_outputs() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input1 = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let input2 = Input::Utxo(UtxoInput::new(txid, 1).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input1, input2])
        .add_output(output)
        .finish();

    assert!(essence.is_ok());
}

#[test]
fn build_invalid_payload_kind() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input1 = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let input2 = Input::Utxo(UtxoInput::new(txid, 1).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );
    let payload = rand_treasury_transaction_payload();

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input1, input2])
        .add_output(output)
        .with_payload(payload.into())
        .finish();

    assert!(matches!(essence, Err(Error::InvalidPayloadKind(4))));
}

#[test]
fn build_invalid_input_count_low() {
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder().add_output(output).finish();

    assert!(matches!(
        essence,
        Err(Error::InvalidInputCount(TryIntoBoundedU16Error::Invalid(0)))
    ));
}

#[test]
fn build_invalid_input_count_high() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input; 129])
        .add_output(output)
        .finish();

    assert!(matches!(
        essence,
        Err(Error::InvalidInputCount(TryIntoBoundedU16Error::Invalid(129)))
    ));
}

#[test]
fn build_invalid_output_count_low() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input = Input::Utxo(UtxoInput::new(txid, 0).unwrap());

    let essence = RegularTransactionEssence::builder().add_input(input).finish();

    assert!(matches!(
        essence,
        Err(Error::InvalidOutputCount(TryIntoBoundedU16Error::Invalid(0)))
    ));
}

#[test]
fn build_invalid_output_count_high() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .add_input(input)
        .with_outputs(vec![output; 129])
        .finish();

    assert!(matches!(
        essence,
        Err(Error::InvalidOutputCount(TryIntoBoundedU16Error::Invalid(129)))
    ));
}

#[test]
fn build_invalid_duplicate_utxo() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input; 2])
        .add_output(output)
        .finish();

    assert!(matches!(essence, Err(Error::DuplicateUtxo(_))));
}

#[test]
fn build_invalid_input_kind() {
    let input = Input::Treasury(TreasuryInput::new(MilestoneId::new(rand_bytes_array())));
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .add_input(input)
        .add_output(output)
        .finish();

    assert!(matches!(essence, Err(Error::InvalidInputKind(1))));
}

#[test]
fn build_invalid_output_kind() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let amount = 1_000_000;
    let output = Output::Treasury(TreasuryOutput::new(amount).unwrap());

    let essence = RegularTransactionEssence::builder()
        .add_input(input)
        .add_output(output)
        .finish();

    assert!(matches!(essence, Err(Error::InvalidOutputKind(2))));
}

#[test]
fn build_invalid_accumulated_output() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input = Input::Utxo(UtxoInput::new(txid, 0).unwrap());

    let bytes1: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address1 = Address::from(Ed25519Address::new(bytes1));
    let amount1 = IOTA_SUPPLY - 1_000_000;
    let output1 = Output::Basic(
        BasicOutput::build(amount1)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address1).into())
            .finish()
            .unwrap(),
    );

    let bytes2: [u8; 32] = hex::decode(ED25519_ADDRESS_2).unwrap().try_into().unwrap();
    let address2 = Address::from(Ed25519Address::new(bytes2));
    let amount2 = 2_000_000;
    let output2 = Output::Basic(
        BasicOutput::build(amount2)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address2).into())
            .finish()
            .unwrap(),
    );

    let essence = RegularTransactionEssence::builder()
        .add_input(input)
        .with_outputs(vec![output1, output2])
        .finish();

    assert!(matches!(essence, Err(Error::InvalidAccumulatedOutput(_))));
}

#[test]
fn getters() {
    let txid = TransactionId::new(hex::decode(TRANSACTION_ID).unwrap().try_into().unwrap());
    let input1 = Input::Utxo(UtxoInput::new(txid, 0).unwrap());
    let input2 = Input::Utxo(UtxoInput::new(txid, 1).unwrap());
    let bytes: [u8; 32] = hex::decode(ED25519_ADDRESS_1).unwrap().try_into().unwrap();
    let address = Address::from(Ed25519Address::new(bytes));
    let amount = 1_000_000;
    let outputs = vec![Output::Basic(
        BasicOutput::build(amount)
            .unwrap()
            .add_unlock_condition(AddressUnlockCondition::new(address).into())
            .finish()
            .unwrap(),
    )];
    let payload = Payload::from(rand_tagged_data_payload());

    let essence = RegularTransactionEssence::builder()
        .with_inputs(vec![input1, input2])
        .with_outputs(outputs.clone())
        .with_payload(payload.clone())
        .finish()
        .unwrap();

    assert_eq!(essence.outputs(), outputs.as_slice());
    assert_eq!(essence.payload().unwrap(), &payload);
}
