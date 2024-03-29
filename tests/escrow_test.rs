use io::{InitEscrow, EscrowAction, EscrowEvent};
use gtest::{Log, Program, System};

const BUYER: u64 = 100;
const SELLER: u64 = 101;
const PRICE: u128 = 100_000;
const ESCROW_ID: u64 = 1;

#[test]
fn deposit() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);
    sys.mint_to(BUYER, PRICE);

    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );
    let log = Log::builder().dest(BUYER).payload(EscrowEvent::FundsDeposited);
    assert!(res.contains_log(&log));

    let escrow_balance = sys.balance_of(ESCROW_ID);
    assert_eq!(escrow_balance, PRICE);
}

fn init_escrow(sys: &System) {
    sys.init_logger();
    let escrow = Program::current(sys);

    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            amount: PRICE,
        },
    );
    assert!(!res.main_failed());
}