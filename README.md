# Shamir's Secret Sharing

Shamir's Secret Sharing (SSS) is a cryptographic technique that enables a **secret** to be split into multiple parts, or **shares**, such that only a subset of them can be used to reconstruct the secret. This is known as a **threshold scheme**. In a `(k, n)` threshold scheme, a secret is divided into `n` shares such that any `k` or more of them can be used to reconstruct the secret, but any fewer than `k` shares provide no information about the secret.

## How It Works

Shamir's Secret Sharing is based on the concept of **polynomial interpolation** over finite fields. The secret is embedded as the constant term in a randomly chosen polynomial of degree `k-1`.

### Mathematical Foundation

1. **Representing the Secret:**  
   Let `S` be the secret you want to share. Choose a large prime number `p` such that `S < p`. All computations are done modulo `p` in a finite field `ℤ_p`.

2. **Random Polynomial Generation:**  
   Define a random polynomial of degree `k - 1` with `S` as the constant term:
   \[
   f(x) = a_0 + a_1 x + a_2 x^2 + \dots + a_{k-1} x^{k-1} \mod p
   \]
   where:
   - `a_0 = S` (the secret),
   - `a_1, a_2, ..., a_{k-1}` are randomly chosen coefficients in `ℤ_p`.

3. **Generating Shares:**  
   Each share corresponds to a point `(x, f(x))` on the polynomial. Choose `n` unique, non-zero values `x_1, x_2, ..., x_n` in `ℤ_p` and compute the shares as:
   \[
   y_i = f(x_i) \mod p
   \]
   Each participant receives a pair `(x_i, y_i)` as their share.

4. **Reconstructing the Secret:**  
   Given any `k` shares, you can use **Lagrange interpolation** to reconstruct the polynomial `f(x)` and, therefore, recover `S = f(0)`. Lagrange interpolation expresses `f(x)` as:
   \[
   f(x) = \sum_{j=1}^{k} y_j \cdot \ell_j(x) \mod p
   \]
   where each `ℓ_j(x)` is the **Lagrange basis polynomial**:
   \[
   \ell_j(x) = \prod_{\substack{1 \le m \le k \\ m \neq j}} \frac{x - x_m}{x_j - x_m} \mod p
   \]
   Finally, evaluating `f(0)` gives `S`.