// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{parent::Parents, Error, MessageId};
use bee_test::rand::message::{rand_message_id, rand_message_ids};

use packable::{bounded::TryIntoBoundedU8Error, error::UnpackError, prefix::VecPrefix, PackableExt};

use core::ops::Deref;

#[test]
fn new_valid_iter() {
    let inner = rand_message_ids(8);
    let parents = Parents::new(inner.clone()).unwrap();

    let parents_vec = parents.iter().copied().collect::<Vec<MessageId>>();

    assert_eq!(inner, parents_vec[0..].to_vec());
}

#[test]
fn new_valid_deref() {
    let inner = rand_message_ids(8);
    let parents = Parents::new(inner.clone()).unwrap();

    assert_eq!(parents.deref(), &inner.into_boxed_slice());
}

#[test]
fn new_invalid_more_than_max() {
    let mut inner = vec![rand_message_id()];

    for _ in 0..8 {
        Parents::new(inner.clone()).unwrap();
        inner.push(rand_message_id());
        inner.sort();
    }

    assert!(matches!(
        Parents::new(inner),
        Err(Error::InvalidParentCount(TryIntoBoundedU8Error::Invalid(9)))
    ));
}

#[test]
fn new_invalid_not_sorted() {
    let mut inner = rand_message_ids(8);
    inner.reverse();

    assert!(matches!(Parents::new(inner), Err(Error::ParentsNotUniqueSorted)));
}

#[test]
fn new_invalid_not_unique() {
    let mut inner = rand_message_ids(7);
    inner.push(*inner.last().unwrap());

    assert!(matches!(Parents::new(inner), Err(Error::ParentsNotUniqueSorted)));
}

#[test]
fn packed_len() {
    let parents = Parents::new(rand_message_ids(5)).unwrap();

    assert_eq!(parents.packed_len(), 1 + 5 * 32);
    assert_eq!(parents.pack_to_vec().len(), 1 + 5 * 32);
}

#[test]
fn pack_unpack_valid() {
    let parents_1 = Parents::new(rand_message_ids(8)).unwrap();
    let parents_2 = Parents::unpack_verified(&mut parents_1.pack_to_vec().as_slice()).unwrap();

    assert_eq!(parents_1, parents_2);
}

#[test]
fn pack_unpack_invalid_less_than_min() {
    let bytes = vec![
        0, 227, 127, 245, 158, 220, 152, 191, 107, 27, 218, 187, 247, 227, 25, 215, 141, 92, 95, 138, 21, 98, 20, 83,
        206, 92, 26, 62, 9, 221, 81, 191, 4, 96, 54, 232, 50, 83, 49, 236, 80, 189, 251, 191, 192, 122, 206, 202, 209,
        145, 50, 168, 233, 176, 12, 164, 138, 207, 22, 96, 82, 189, 64, 188, 130,
    ];

    assert!(matches!(
        Parents::unpack_verified(&mut bytes.as_slice()),
        Err(UnpackError::Packable(Error::InvalidParentCount(
            TryIntoBoundedU8Error::Invalid(0)
        )))
    ));
}

#[test]
fn pack_unpack_invalid_more_than_max() {
    let bytes = vec![
        9, 227, 127, 245, 158, 220, 152, 191, 107, 27, 218, 187, 247, 227, 25, 215, 141, 92, 95, 138, 21, 98, 20, 83,
        206, 92, 26, 62, 9, 221, 81, 191, 4, 96, 54, 232, 50, 83, 49, 236, 80, 189, 251, 191, 192, 122, 206, 202, 209,
        145, 50, 168, 233, 176, 12, 164, 138, 207, 22, 96, 82, 189, 64, 188, 130,
    ];

    assert!(matches!(
        Parents::unpack_verified(&mut bytes.as_slice()),
        Err(UnpackError::Packable(Error::InvalidParentCount(
            TryIntoBoundedU8Error::Invalid(9)
        )))
    ));
}

#[test]
fn unpack_invalid_not_sorted() {
    let mut inner = rand_message_ids(8);
    inner.reverse();
    let inner = VecPrefix::<_, u8>::try_from(inner).unwrap();

    let packed = inner.pack_to_vec();
    let parents = Parents::unpack_verified(&mut packed.as_slice());

    assert!(matches!(
        parents,
        Err(UnpackError::Packable(Error::ParentsNotUniqueSorted))
    ),);
}

#[test]
fn unpack_invalid_not_unique() {
    let mut inner = rand_message_ids(7);
    inner.push(*inner.last().unwrap());
    let inner = VecPrefix::<_, u8>::try_from(inner).unwrap();

    let packed = inner.pack_to_vec();
    let parents = Parents::unpack_verified(&mut packed.as_slice());

    assert!(matches!(
        parents,
        Err(UnpackError::Packable(Error::ParentsNotUniqueSorted))
    ),);
}
