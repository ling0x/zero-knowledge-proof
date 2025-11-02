use rand::{Rng, thread_rng};

pub fn simple_proof() {
    // The Prover knows the secret
    let knows_secret = true;
    let num_rounds = 40;
    let mut rng = thread_rng();

    for round in 1..=num_rounds {
        // The Prover chooses a path at random
        let initial_path = if rng.gen_bool(0.5) { "A" } else { "B" };

        // The Verifier asks for a random path to return
        let requested_path = if rng.gen_bool(0.5) { "A" } else { "B" };

        // The Prover must return via requested path
        let success = if knows_secret {
            // The prover can always comply
            true
        } else {
            // The Prover can comply only if The Verifier guesses its path
            initial_path == requested_path
        };

        println!(
            "Round {}: The Prover starts at {}, The Verifier requests {} - The Prover {}",
            round,
            initial_path,
            requested_path,
            if success { "succeeds!" } else { "fails." }
        );
    }
}
