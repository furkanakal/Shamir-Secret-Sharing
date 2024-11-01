use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};

fn main() {
    let secret = BigInt::from(1234);
    let n = 5;  // Number of shares
    let k = 3;  // Threshold number of shares required
    let prime = BigInt::from(7919);  // Large prime number for modulo
    println!("Secret: {}", secret);

    // Generate shares
    let shares = generate_shares(&secret, n, k, &prime);
    println!("Shares: {:?}", shares);

    // Reconstruct secret using first k shares
    let selected_shares = shares[..k].to_vec();
    let reconstructed_secret = lagrange_interpolation(&selected_shares, &prime);
    println!("Reconstructed secret: {}", reconstructed_secret);
}

/// Generates random polynomial coefficients for Shamir's Secret Sharing
fn generate_random_coefficients(secret: &BigInt, k: usize, prime: &BigInt) -> Vec<BigInt> {
    let mut rng = rand::thread_rng();
    let mut coefficients = vec![secret.clone()];  // a0 = secret

    for _ in 1..k {
        let coef = rng.gen_bigint_range(&BigInt::one(), prime);
        coefficients.push(coef);
    }

    coefficients
}

/// Evaluate polynomial at a given x
fn evaluate_polynomial(coefficients: &[BigInt], x: &BigInt, prime: &BigInt) -> BigInt {
    let mut result = BigInt::zero();
    let mut x_pow = BigInt::one();

    for coef in coefficients {
        result = (result + coef * &x_pow) % prime;
        x_pow = (x_pow * x) % prime;
    }

    result
}

/// Generates n shares for the secret
fn generate_shares(secret: &BigInt, n: usize, k: usize, prime: &BigInt) -> Vec<(BigInt, BigInt)> {
    let coefficients = generate_random_coefficients(secret, k, prime);
    let mut shares = vec![];

    for i in 1..=n {
        let x = i.to_bigint().unwrap();
        let y = evaluate_polynomial(&coefficients, &x, prime);
        shares.push((x, y));
    }

    shares
}

/// Lagrange interpolation to reconstruct the secret
fn lagrange_interpolation(shares: &[(BigInt, BigInt)], prime: &BigInt) -> BigInt {
    let mut secret = BigInt::zero();

    for (j, &(ref x_j, ref y_j)) in shares.iter().enumerate() {
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for (m, &(ref x_m, _)) in shares.iter().enumerate() {
            if j != m {
                numerator = (numerator * -x_m) % prime;
                denominator = (denominator * (x_j - x_m)) % prime;
            }
        }

        let lagrange_coefficient = numerator * denominator.modpow(&(prime - BigInt::from(2)), prime);
        secret = (secret + y_j * lagrange_coefficient) % prime;
    }

    (secret + prime) % prime  // Ensuring positive result
}
