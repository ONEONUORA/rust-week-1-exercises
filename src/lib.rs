// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {

    let tx_bytes = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    
    if tx_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    let version_bytes: [u8; 4] = tx_bytes[0..4].try_into().unwrap();
    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
