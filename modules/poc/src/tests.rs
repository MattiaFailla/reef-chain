#![cfg(test)]

use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn set_dummy_work() {
	new_test_ext().execute_with(|| {
		// assert_eq!(PoC::dummy(), None);
		// assert_ok!(PoC::set_dummy(Origin::root(), 20));
		// assert_eq!(PoC::dummy(), Some(20));
        //
		// let set_dummy_event = Event::example(crate::Event::Dummy(20));
		// assert!(System::events().iter().any(|record| record.event == set_dummy_event));
	});
}

#[test]
fn do_set_bar_work() {
	new_test_ext().execute_with(|| {
		// assert_eq!(PoC::bar(2), 200);
		// PoC::do_set_bar(&2, 10);
		// assert_eq!(PoC::bar(2), 10);
	});
}
