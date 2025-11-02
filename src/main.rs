use zero_knowledge_proof::{
    fiat_shamir::fiat_shamir, interactive_schnorr::interactive_schnorr, simple::simple_proof,
};

fn main() {
    simple_proof();
    interactive_schnorr();
    fiat_shamir();
}
