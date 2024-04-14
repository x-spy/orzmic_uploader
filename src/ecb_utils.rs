use openssl::symm::{Cipher, Crypter, Mode};
use openssl::error::ErrorStack;

pub fn decrypt_ecb(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_ecb();

    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, None)?;

    let block_size = cipher.block_size();
    let mut decrypted = vec![0; data.len() + block_size];
    let mut count = crypter.update(data, &mut decrypted)?;

    let rest = crypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest);

    Ok(decrypted)
}

pub fn encrypt_ecb(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_ecb();

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, None)?;

    let block_size = cipher.block_size();
    let mut encrypted = vec![0; data.len() + block_size];
    let mut count = crypter.update(data, &mut encrypted)?;

    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest); // 移除多余的部分

    Ok(encrypted)
}