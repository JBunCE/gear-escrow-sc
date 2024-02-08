use gstd::msg;
use io::{EscrowEvent, EscrowState};
use scale_info::build::state;

use crate::scrow_mut;

#[derive(Default)]
pub struct EscrowContract { }

impl EscrowContract {

    pub fn deposit(&self) {
        let state = scrow_mut();
        let sender_account = msg::source();

        assert_eq!(state.state, EscrowState::AwaitingPayment, "Invalid state");
        assert_eq!(sender_account, state.buyer, "Invalid sender");
        assert_eq!(msg::value(), state.price, "Invalid value");

        state.state = EscrowState::AwaitingDelivery;
        let _ = msg::reply(EscrowEvent::FundsDeposited, 0);
    }

    pub fn confirm_delivery(&self) {
        
    }

}