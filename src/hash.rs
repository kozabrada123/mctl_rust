use argonautica::{Hasher, Verifier};

/// Hashes something using the preferred secret hashing algorithm
///
/// Currnetly uses argon 2
pub fn hash(input: String) -> String {
    let mut hasher = Hasher::default();
    let hash = hasher
        .opt_out_of_secret_key(true) // FIXME: Use a secret key for hashes
        .with_password(input)
        .hash()
        .unwrap();

    return hash;
}

/// Verifies an input against a hash using the preferred hashing algorithm
///
/// Currnetly uses argon 2
pub fn verify(input: String, hash: String) -> bool {
    let mut verifier = Verifier::default();
    let validity_result = verifier.with_password(input).with_hash(hash).verify();

    if validity_result.is_err() {
        return false;
    }

    validity_result.unwrap()
}
