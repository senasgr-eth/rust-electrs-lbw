use bitcoin::{Address, Network, PublicKey};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::blockdata::constants::genesis_block;
use std::str::FromStr;

fn main() {
    println!("🔧 Testing Lebowkis Network Parameters...\n");

    // Test 1: Address Encoding
    test_address_encoding();

    // Test 2: Network Magic Bytes
    test_network_magic();

    // Test 3: Genesis Block Parameters
    test_genesis_block();
}

fn test_address_encoding() {
    println!("=== 1. Address Encoding Test ===");

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&[1u8; 32]).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_private_key(&secp, &bitcoin::PrivateKey::new(secret_key, Network::Bitcoin));

    // Generate P2PKH address
    let p2pkh_address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("Mainnet P2PKH: {}", p2pkh_address);

    // Generate P2SH address
    let script = bitcoin::ScriptBuf::from_hex("76a914389ffce9cd9ae88dcc0631e88a821ffdbe9bfe2615").unwrap();
    let p2sh_address = Address::p2sh(&script, Network::Bitcoin).unwrap();
    println!("Mainnet P2SH:  {}", p2sh_address);

    // Test testnet addresses
    let p2pkh_testnet = Address::p2pkh(&public_key, Network::Testnet);
    println!("Testnet P2PKH: {}", p2pkh_testnet);

    let p2sh_testnet = Address::p2sh(&script, Network::Testnet).unwrap();
    println!("Testnet P2SH:  {}", p2sh_testnet);

    // Test regtest addresses
    let p2pkh_regtest = Address::p2pkh(&public_key, Network::Regtest);
    println!("Regtest P2PKH: {}", p2pkh_regtest);

    // Test address parsing
    match Address::from_str("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa") {
        Ok(addr) => println!("❌ Bitcoin address incorrectly parsed: {}", addr.assume_checked()),
        Err(e) => println!("✅ Bitcoin address correctly rejected: {}", e),
    }

    let lebowkis_addr_str = p2pkh_address.to_string();
    match Address::from_str(&lebowkis_addr_str) {
        Ok(addr) => println!("✅ Lebowkis address correctly parsed: {}", addr.assume_checked()),
        Err(e) => println!("❌ Lebowkis address incorrectly rejected: {}", e),
    }
    println!();
}

fn test_network_magic() {
    println!("=== 2. Network Magic Bytes Test ===");

    println!("Mainnet magic: {:02x?}", Network::Bitcoin.magic().to_bytes());
    println!("Testnet magic: {:02x?}", Network::Testnet.magic().to_bytes());
    println!("Signet magic:  {:02x?}", Network::Signet.magic().to_bytes());
    println!("Regtest magic: {:02x?}", Network::Regtest.magic().to_bytes());

    // Verify against expected Lebowkis values
    assert_eq!(Network::Bitcoin.magic().to_bytes(), [0xcc, 0xf1, 0xc0, 0xee]);
    assert_eq!(Network::Testnet.magic().to_bytes(), [0xfc, 0xc1, 0xb7, 0xdc]);
    assert_eq!(Network::Signet.magic().to_bytes(), [0xfc, 0xc1, 0xb7, 0xdc]);
    assert_eq!(Network::Regtest.magic().to_bytes(), [0xc0, 0xc0, 0xc0, 0xc0]);

    println!("✅ All magic bytes match Lebowkis core wallet values");
    println!();
}

fn test_genesis_block() {
    println!("=== 3. Genesis Block Test ===");

    let mainnet_genesis = genesis_block(Network::Bitcoin);
    let testnet_genesis = genesis_block(Network::Testnet);
    let regtest_genesis = genesis_block(Network::Regtest);

    println!("Mainnet genesis hash: {}", mainnet_genesis.block_hash());
    println!("Testnet genesis hash: {}", testnet_genesis.block_hash());
    println!("Regtest genesis hash: {}", regtest_genesis.block_hash());

    // Verify genesis block parameters
    println!("Genesis block time: {}", mainnet_genesis.header.time);
    println!("Genesis block nonce: {}", mainnet_genesis.header.nonce);
    println!("Genesis block bits: 0x{:08x}", mainnet_genesis.header.bits.to_consensus());

    // Check that all networks use the same genesis (as per Lebowkis core)
    assert_eq!(mainnet_genesis.block_hash(), testnet_genesis.block_hash());
    assert_eq!(mainnet_genesis.block_hash(), regtest_genesis.block_hash());

    println!("✅ Genesis block parameters match Lebowkis core wallet");
    println!();
}
