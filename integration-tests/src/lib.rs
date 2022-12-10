use casper_types::U512;
use tests::{auction::AuctionContract, auction_args::AuctionArgsBuilder};

// Trying to bid an amount below the reserve results in User(3) error
#[test]
#[should_panic = "User(19)"]
fn english_auction_bid_too_low_test() {
    let now = AuctionArgsBuilder::get_now_u64();
    let mut auction_args = AuctionArgsBuilder::default();
    //  ?Consider optimizing away the storage of start price key for English auctions
    // auction_args.set_starting_price(Some(U512::from(5000_u16)));
    auction_args.set_reserve_price(U512::from(500_u16));
    dbg!(&auction_args);
    let mut auction_contract = AuctionContract::deploy_contracts(auction_args);
    auction_contract.bid(
        &auction_contract.ali.clone(),
        U512::from(400_u64),
        now + 1000,
    );
}

#[test]
fn english_auction_bid_enough_test() {
    let now = AuctionArgsBuilder::get_now_u64();
    let mut auction_args = AuctionArgsBuilder::default();
    auction_args.set_reserve_price(U512::from(500_u64));
    let mut auction_contract = AuctionContract::deploy_contracts(auction_args);
    auction_contract.bid(
        &auction_contract.bob.clone(),
        U512::from(600_u64),
        now + 1000,
    );
}

#[test]
fn english_auction_bid_finalize_test() {
    let now = AuctionArgsBuilder::get_now_u64();
    let mut auction_contract = AuctionContract::deploy_with_default_args(true, now);
    assert!(now < auction_contract.get_end());
    auction_contract.bid(&auction_contract.ali.clone(), U512::from(30000), now);
    auction_contract.bid(&auction_contract.bob.clone(), U512::from(40000), now);
    auction_contract.finalize(&auction_contract.admin.clone(), now + 3500);
    assert!(auction_contract.is_finalized());
    assert_eq!(auction_contract.bob, auction_contract.get_winner().unwrap());
    assert_eq!(
        U512::from(40000),
        auction_contract.get_winning_bid().unwrap()
    );
}

#[test]
#[should_panic = "User(3)"]
fn dutch_auction_with_reserve_price_bid_too_low_test() {
    let now = AuctionArgsBuilder::get_now_u64();
    let mut auction_args = AuctionArgsBuilder::default();
    auction_args.set_starting_price(Some(U512::from(40000)));
    auction_args.set_reserve_price(U512::from(20000));
    auction_args.set_dutch();
    let mut auction_contract = AuctionContract::deploy_contracts(auction_args);
    auction_contract.bid(&auction_contract.bob.clone(), U512::from(26499), now + 3000);
}

#[test]
fn dutch_auction_with_reserve_price_success_test() {
    let now = AuctionArgsBuilder::get_now_u64();
    let mut auction_args = AuctionArgsBuilder::default();
    dbg!(&auction_args);
    auction_args.set_starting_price(Some(U512::from(40000)));
    auction_args.set_reserve_price(U512::from(20000));
    auction_args.set_dutch();
    let mut auction_contract = AuctionContract::deploy_contracts(auction_args);
    auction_contract.bid(&auction_contract.ali.clone(), U512::from(27500), now + 3000);
    assert!(auction_contract.is_finalized());
    assert_eq!(auction_contract.ali, auction_contract.get_winner().unwrap());
}

#[test]
fn dutch_auction_bid_finalize_test() {
    let now = AuctionArgsBuilder::get_now_u64();
    let mut auction_args = AuctionArgsBuilder::default();
    auction_args.set_starting_price(Some(U512::from(40000)));
    auction_args.set_dutch();
    let mut auction_contract = AuctionContract::deploy_contracts(auction_args);
    auction_contract.bid(&auction_contract.bob.clone(), U512::from(40000), now + 1000);
    assert!(auction_contract.is_finalized());
    assert_eq!(auction_contract.bob, auction_contract.get_winner().unwrap());
    assert_eq!(
        U512::from(40000),
        auction_contract.get_winning_bid().unwrap()
    );
}
