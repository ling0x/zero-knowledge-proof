use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::{Rng, thread_rng};

// This is a simplified Schnorr zero-knowledge proof demonstration in Rust.
pub fn interactive_schnorr() {
    // Public parameters: large prime p = 204859, generator g = 5, secret x = 6, h = 5^6 mod 204859 = 8
    let p: BigInt = BigInt::from(204859u64);
    let g: BigInt = BigInt::from(5u32);
    let x: BigInt = BigInt::from(6u32); // Prover's secret
    let h = g.modpow(&x, &p); // h = g^x mod p

    // Simulate several rounds of the protocol
    for _ in 0..20 {
        // Prover: generate random commitment k and t = g^k mod p
        let mut rng = thread_rng(); // CORRECT - get a random generator
        let k = rng.gen_bigint_range(&BigInt::one(), &(&p - BigInt::one())); // CORRECT - uses RandBigInt trait
        let t = g.modpow(&k, &p);
        println!("Prover sends commitment t: {}", t);

        // Verifier: generate random challenge c (here simplified to range 0..10)
        let c: BigInt = BigInt::from(rng.gen_range(0..10)); // CORRECT - gen_range not random_range
        println!("Verifier sends challenge c: {}", c);

        // Prover: computes response r = k - c * x mod (p-1)
        let order = &p - BigInt::one(); // group order
        let r = (&k - &c * &x).modpow(&BigInt::one(), &order); // Ensures non-negative
        println!("Prover responds with r: {}", r);

        // Verifier: checks if g^r * h^c ≡ t (mod p)
        let left = (g.modpow(&r, &p) * h.modpow(&c, &p)) % &p;
        if left == t {
            println!("✅ Verification successful!");
        } else {
            println!("❌ Verification failed!");
        }
    }
}
