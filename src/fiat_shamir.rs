use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;
use sha2::{Digest, Sha256};
use std::io::Write;

// This function demonstrates a non-interactive Schnorr proof using the Fiat-Shamir heuristic.
pub fn fiat_shamir() {
    // --- Public parameters (in practice, p should be a secure prime, at least 2048 bits) ---
    let p: BigInt = BigInt::from(204859u64);
    let g: BigInt = BigInt::from(2u64);

    // Prover's secret (only known to the prover)
    let secret_x: BigInt = BigInt::from(123456u64);

    // Prover's public key (known to everyone)
    let public_h = g.modpow(&secret_x, &p);

    println!("--- Public parameters ---");
    println!("p = {}", p);
    println!("g = {}", g);
    println!("h = g^x mod p = {}", public_h);
    println!("-------------------------");

    // --- PROVER: Generate the proof ---
    println!("Prover is generating the proof...");
    let mut rng = thread_rng();
    let order = &p - BigInt::one();

    // 1. Commitment: pick random k, compute t = g^k mod p
    let k = rng.gen_bigint_range(&BigInt::one(), &order);
    let t = g.modpow(&k, &p);

    // 2. Challenge (Fiat-Shamir magic here!):
    // Hash public parameters and the commitment t to simulate an unpredictable challenge c
    let mut hasher = Sha256::new();
    hasher.write_all(&g.to_bytes_be().1).unwrap();
    hasher.write_all(&public_h.to_bytes_be().1).unwrap();
    hasher.write_all(&t.to_bytes_be().1).unwrap();
    let hash_bytes = hasher.finalize();
    let c = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash_bytes) % &order;

    // 3. Response: compute r = (k - c*x) mod order
    let cx = (&c * &secret_x) % &order;
    let mut r = (&k - cx) % &order;
    if r < BigInt::zero() {
        r += &order;
    }

    println!("Proof generated: (r = {}, c = {})", r, c);
    println!("-------------------------");

    // --- VERIFIER: Verify the proof ---
    println!("Verifier is verifying the proof...");
    // Verifier recalculates t' = g^r * h^c mod p
    let gr = g.modpow(&r, &p);
    let hc = public_h.modpow(&c, &p);
    let t_prime = (&gr * &hc) % &p;

    // Verifier re-hashes g, h, t' to compute c'
    let mut hasher = Sha256::new();
    hasher.write_all(&g.to_bytes_be().1).unwrap();
    hasher.write_all(&public_h.to_bytes_be().1).unwrap();
    hasher.write_all(&t_prime.to_bytes_be().1).unwrap();
    let hash_bytes = hasher.finalize();
    let c_prime = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash_bytes) % &order;

    if c == c_prime {
        println!("✅ Verification successful!");
    } else {
        println!("❌ Verification failed!");
    }
}
