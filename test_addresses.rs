use bitcoin::{Address, Network, PublicKey};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use std::str::FromStr;

fn main() {
    let secp = Secp256k1::new();
    
    // Test with a known private key to get consistent results
    let secret_key = SecretKey::from_slice(&[1u8; 32]).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_private_key(&secp, &bitcoin::PrivateKey::new(secret_key, Network::Bitcoin));
    
    // Generate P2PKH address (should start with 'L' for Lebowkis mainnet)
    let p2pkh_address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("P2PKH Address (mainnet): {}", p2pkh_address);
    
    // Generate P2SH address (should start with '3' for Lebowkis mainnet)
    let script = bitcoin::ScriptBuf::from_hex("76a914389ffce9cd9ae88dcc0631e88a821ffdbe9bfe2615").unwrap();
    let p2sh_address = Address::p2sh(&script, Network::Bitcoin).unwrap();
    println!("P2SH Address (mainnet): {}", p2sh_address);
    
    // Test testnet addresses (should start with 'l' for P2PKH)
    let p2pkh_testnet = Address::p2pkh(&public_key, Network::Testnet);
    println!("P2PKH Address (testnet): {}", p2pkh_testnet);
    
    let p2sh_testnet = Address::p2sh(&script, Network::Testnet).unwrap();
    println!("P2SH Address (testnet): {}", p2sh_testnet);
    
    // Test regtest addresses
    let p2pkh_regtest = Address::p2pkh(&public_key, Network::Regtest);
    println!("P2PKH Address (regtest): {}", p2pkh_regtest);
    
    // Test address parsing - try to parse a Bitcoin address and see if it fails
    match Address::from_str("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa") {
        Ok(addr) => println!("Parsed Bitcoin address: {}", addr),
        Err(e) => println!("Failed to parse Bitcoin address (expected): {}", e),
    }
    
    // Test parsing our new Lebowkis addresses
    let lebowkis_addr_str = p2pkh_address.to_string();
    match Address::from_str(&lebowkis_addr_str) {
        Ok(addr) => println!("Successfully parsed Lebowkis address: {}", addr.assume_checked()),
        Err(e) => println!("Failed to parse Lebowkis address: {}", e),
    }
}
