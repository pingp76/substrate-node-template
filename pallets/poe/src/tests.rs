use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn creat_claim_works() {
	new_test_ext().execute_with(execute: || {
		let claim: Vec<u8> = vec![0,1];
		assert_ok!(PoeModule::create_claim(Oringin::signed(1), claim.clond()));
		assert_eq!(
			Proof::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number())
		));
	})
}


fn creat_claim_failed() {
	new_test_ext().execute_with(execute: || {
		let claim:Vec<u8> = vec![0,1];
		let _=PoeModule::create_claim(Origin::signed(by:1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(by:1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

fn revoke_claim_works() {
	new_test_ext().execute_with(execute: || {
		let claim: Vec<u8> = vec![0,1];
		let _=PoeModule::create_claim(Origin::signed(by:1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Oringin::signed(1), claim.clond()));
		assert_eq!(
			Proof::<Test>::get(&claim),
			None
		)
	})
}

fn revoke_claim_failed() {
	new_test_ext().execute_with(execute: || {
		let claim:Vec<u8> = vec![0,1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(by:1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}