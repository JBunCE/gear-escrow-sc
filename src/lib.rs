#![no_std]
use gstd::{msg, ActorId};
use io::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use contract::escrow::{EscrowContract};

mod contract;

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub amount: u128,
}

static mut ESCROW: Option<Escrow> = None;
static mut ESCROW_CONTRACT: Option<EscrowContract> = None;

pub fn scrow_mut() -> &'static mut Escrow {
    let scrow = unsafe { ESCROW.as_mut() };
    return unsafe { scrow.unwrap_unchecked() }
}

pub fn scrow_contract_mut() -> &'static mut EscrowContract {
    let scrow = unsafe { ESCROW_CONTRACT.as_mut() };
    return unsafe { scrow.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitEscrow = msg::load().expect("Can't load init config");

    unsafe {
        ESCROW_CONTRACT = Some( Default::default() );
        ESCROW = Some( Escrow {
            seller: init_config.seller,
            buyer: init_config.buyer,
            price: init_config.amount,
            state: EscrowState::AwaitingPayment,
        })
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: EscrowAction = msg::load().expect("Can't load action"); 
    let contract = scrow_contract_mut();

    match action {
        EscrowAction::Deposit => {
            contract.deposit();
        }
        EscrowAction::ConfirmDelivery => {
            contract.confirm_delivery()
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let scrow = scrow_mut();
    let _ = msg::reply(scrow, 0);
}