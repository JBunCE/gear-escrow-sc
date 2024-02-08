use gmeta::{ In, InOut, Metadata, Out };
use gstd::{prelude::*, ActorId};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug, PartialEq)]
pub enum EscrowState {
    #[default]
    AwaitingPayment,
    AwaitingDelivery,
    Completed,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState
}

pub struct ScrowMetadata;

impl Metadata for ScrowMetadata {
    type Init = In<String>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Escrow>;
}
