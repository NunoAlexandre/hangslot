use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn lock_funds_works() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        assert_ok!(TemplateModule::lock(Origin::signed(account_id), 70, 99));
        assert_eq!(TemplateModule::get_locked_amount((99, account_id)), 70);
    });
}

#[test]
fn lock_funds_works_overwrites() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        assert_ok!(TemplateModule::lock(Origin::signed(account_id), 70, 99));
        assert_ok!(TemplateModule::lock(Origin::signed(account_id), 100, 99));
        assert_eq!(TemplateModule::get_locked_amount((99, account_id)), 100);
    });
}

#[test]
fn unlock_funds_with_valid_proof() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::unlock(
            Origin::signed(1),
            vec![1, 2, 3, 4, 5]
        ));
    });
}

#[test]
fn unlock_funds_with_invalid_proof() {
    new_test_ext().execute_with(|| {
        let res = TemplateModule::unlock(Origin::signed(1), vec![6, 7, 8, 9, 10]);
        assert_noop!(res, Error::<Test>::InvalidProof);
    });
}
