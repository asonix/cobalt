// Taken from https://briansmith.org/rustdoc/ring/pbkdf2/index.html

use ring::{digest, pbkdf2};
use database;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const PBKDF2_ITERATIONS: u32 = 100_100;
const DATABASE_SALT_COMPONENT: [u8; 16] = [
    0xd6,
    0x26,
    0x98,
    0xda,
    0xf4,
    0xdc,
    0x50,
    0x52,
    0x24,
    0xf2,
    0x27,
    0xd1,
    0xfe,
    0x39,
    0x01,
    0x8a,
];

pub type EncryptedPassword = [u8; CREDENTIAL_LEN];

pub fn encypt_password(username: &str, password: &str) -> EncryptedPassword {
    let salt = salt(username);
    let mut encrypted_password: EncryptedPassword = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        DIGEST_ALG,
        PBKDF2_ITERATIONS,
        &salt,
        password.as_bytes(),
        &mut encrypted_password,
    );

    encrypted_password
}

pub fn verify_password(
    username: &str,
    attempted_password: &str,
    password: &EncryptedPassword,
) -> bool {
    let salt = salt(username);

    match pbkdf2::verify(
        DIGEST_ALG,
        PBKDF2_ITERATIONS,
        &salt,
        attempted_password.as_bytes(),
        password,
    ) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn salt(username: &str) -> Vec<u8> {
    let mut salt = Vec::with_capacity(DATABASE_SALT_COMPONENT.len() + username.as_bytes().len());

    salt.extend(DATABASE_SALT_COMPONENT.as_ref());
    salt.extend(username.as_bytes());

    salt
}
