use ic_agent::{Agent, ic_types::Principal};
use ic_agent::identity::BasicIdentity;
use ring;
use futures::executor;

pub async fn query_call() -> Vec<u8> {
    let url = format!("https://ic0.app");
    let rng = ring::rand::SystemRandom::new();
    let key_pair = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng).expect("Could not generate a key pair.");
    let identity = BasicIdentity::from_key_pair(ring::signature::Ed25519KeyPair::from_pkcs8(key_pair.as_ref()).expect("Could not read the key pair."));
    let agent = Agent::builder()
                .with_url(url)
                .with_identity(identity)
                .build().unwrap();
    let canister_id:Principal = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").expect("Could not decode the principal.");
    let method_name: String = "get_value".to_owned();
    let query_builder = agent.query(&canister_id, method_name);
    let response = query_builder.call().await;
    let wrong_vec = vec![0,0,0];
    match response {
        Ok(r) => return r,
        _ => return wrong_vec,
    }
}


fn main() {
    let res = executor::block_on(query_call());
    // let res = query_call().await;
    println!("{:#?}", res);
}
