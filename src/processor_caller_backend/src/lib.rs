use std::str::FromStr;

use ic_cdk::println;
use candid::Principal;
use types::PriceResponse;

pub mod types;

#[ic_cdk::update]
async fn submit_price_request(currency_pairs: String) -> String {
    let processor_canister_principal = Principal::from_str("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

    let (request_id,): (String,) = ic_cdk::call(
        processor_canister_principal,
        "request_prices",
        (currency_pairs,),
    )
    .await
    .unwrap();

    // println!("{:?}", request_id)
    return request_id;
}

#[ic_cdk::update]
fn receive_price_response(response: PriceResponse) {
    println!("receive_price_response: {:?}", response);
}
