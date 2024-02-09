#![no_std]
use io::*;
use gstd::{prelude::*, ActorId};
use gmeta::metawasm;

#[metawasm]
pub mod metafns {
    pub type State = Escrow;

    pub fn seller(state: State) -> ActorId {
        state.seller
    }

    pub fn buyer(state: State) -> ActorId {
        state.buyer
    }

    pub fn scrow_state(state: State) -> EscrowState {
        state.state
    }
}