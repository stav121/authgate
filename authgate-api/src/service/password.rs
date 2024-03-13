use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

/// Hash a password using Argon2 algorithm.
///
/// # Args
///
/// * password - the password to hash
///
/// # Examples
///
/// ```
/// # use tokio_test::assert_ok;
/// # use authgate::service::password::hash_password;
/// # tokio_test::block_on(async {
/// assert_ok!(hash_password("test123").await);
/// # })
/// ```
pub async fn hash_password(password: &str) -> Result<String, ()> {
    // Hash the password
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let hashed_password: password_hash::Result<PasswordHash> =
        Argon2::default().hash_password(password.as_bytes(), &salt);

    match hashed_password {
        Ok(password_hash) => Ok(password_hash.to_string()),
        Err(_) => Err(()),
    }
}

/// Verify a hashed password using the Argon2 algorithm.
///
/// # Args
///
/// * password_hash - the hashed password
///
/// # Examples
///
/// ```
/// # use tokio_test::{assert_ok, assert_err};
/// # use authgate::service::password::{hash_password, verify_password};
/// # tokio_test::block_on(async {
/// # let hashed_password1 = hash_password("test123").await.unwrap();
/// # let hashed_password2 = hash_password("test1234").await.unwrap();
/// assert_ok!(verify_password("test123", &hashed_password1).await);
/// assert_err!(verify_password("test123", &hashed_password2).await);
/// # });
/// ```
pub async fn verify_password(password: &str, password_hash: &str) -> Result<(), ()> {
    let parsed_hash: PasswordHash = PasswordHash::new(&password_hash).unwrap();
    println!("{}", parsed_hash);
    match Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true)
    {
        true => {
            println!("true");
            Ok(())
        },
        false => {
            println!("false");
            Err(())
        },
    }
}
