use base64::{decode_config, STANDARD};
use openssl::symm::{decrypt, Cipher};
use rusoto_core::Region;
use rusoto_kms::{DecryptRequest, Kms, KmsClient};
use std::{env, str::FromStr};

pub async fn decrypt_data_with_kms(
    encrypted_data_key: &str,
    encrypted_data: String,
    output_as_hex: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let region_name = env::var("AWS_REGION").unwrap_or_else(|_| String::from("eu-central-1"));
    let region = Region::from_str(&region_name)?;

    let client = KmsClient::new(region);

    let encrypted_data_key = decode_config(&encrypted_data_key.replace(" ", ""), STANDARD)
        .map_err(|err| format!("Failed to decode encrypted data key: {}", err))?;

    let encrypted_data = decode_config(&encrypted_data.replace(" ", ""), STANDARD)
        .map_err(|err| format!("Failed to decode encrypted data: {}", err))?;

    let decrypt_request = DecryptRequest {
        ciphertext_blob: encrypted_data_key.into(),
        ..Default::default()
    };

    let decrypt_response = client.decrypt(decrypt_request).await?;
    let decrypted_data_key = decrypt_response
        .plaintext
        .ok_or("Failed to decrypt the data key")?;

    let decrypted_data = decrypt(
        Cipher::aes_256_cbc(),
        &decrypted_data_key,
        Some(&encrypted_data[..16]),
        &encrypted_data[16..],
    )?;

    if output_as_hex {
        Ok(hex::encode(decrypted_data).into())
    } else {
        Ok(String::from_utf8(decrypted_data)?.into())
    }
}
