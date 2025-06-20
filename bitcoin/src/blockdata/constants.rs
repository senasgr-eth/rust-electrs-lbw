// SPDX-License-Identifier: CC0-1.0

//! Blockdata constants.
//!
//! This module provides various constants relating to the blockchain and
//! consensus code. In particular, it defines the genesis block and its
//! single transaction.
//!

use core::default::Default;

use hashes::{sha256d, Hash};
use hex_lit::hex;
use internals::impl_array_newtype;

use crate::blockdata::block::{self, Block};
use crate::blockdata::locktime::absolute;
use crate::blockdata::opcodes::all::*;
use crate::blockdata::script;
use crate::blockdata::transaction::{self, OutPoint, Sequence, Transaction, TxIn, TxOut};
use crate::blockdata::witness::Witness;
use crate::internal_macros::impl_bytes_newtype;
use crate::network::Network;
use crate::pow::CompactTarget;
use crate::Amount;

/// How many seconds between blocks we expect on average.
pub const TARGET_BLOCK_SPACING: u32 = 60;
/// How many blocks between diffchanges.
pub const DIFFCHANGE_INTERVAL: u32 = 240;
/// How much time on average should occur between diffchanges.
pub const DIFFCHANGE_TIMESPAN: u32 = 4 * 3600;

#[deprecated(since = "0.31.0", note = "Use Weight::MAX_BLOCK instead")]
/// The maximum allowed weight for a block, see BIP 141 (network rule).
pub const MAX_BLOCK_WEIGHT: u32 = 4_000_000;

#[deprecated(since = "0.31.0", note = "Use Weight::MIN_TRANSACTION instead")]
/// The minimum transaction weight for a valid serialized transaction.
pub const MIN_TRANSACTION_WEIGHT: u32 = 4 * 60;

/// The factor that non-witness serialization data is multiplied by during weight calculation.
pub const WITNESS_SCALE_FACTOR: usize = 4;
/// The maximum allowed number of signature check operations in a block.
pub const MAX_BLOCK_SIGOPS_COST: i64 = 80_000;
/// Mainnet (Lebowkis) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_MAIN: u8 = 12; // Lebowkis mainnet P2PKH
/// Mainnet (Lebowkis) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_MAIN: u8 = 8; // Lebowkis mainnet P2SH
/// Test (signet, regtest) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_TEST: u8 = 13; // Lebowkis testnet P2PKH
/// Test (tesnet, signet, regtest) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_TEST: u8 = 9; // Lebowkis testnet P2SH
/// Regtest (Lebowkis) pubkey address prefix.
pub const PUBKEY_ADDRESS_PREFIX_REGTEST: u8 = 47; // Lebowkis regtest P2PKH
/// Regtest (Lebowkis) script address prefix.
pub const SCRIPT_ADDRESS_PREFIX_REGTEST: u8 = 5; // Lebowkis regtest P2SH
/// The maximum allowed script size.
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;
/// How may blocks between halvings.
pub const SUBSIDY_HALVING_INTERVAL: u32 = 210_000;
/// Maximum allowed value for an integer in Script.
pub const MAX_SCRIPTNUM_VALUE: u32 = 0x80000000; // 2^31
/// Number of blocks needed for an output from a coinbase transaction to be spendable.
pub const COINBASE_MATURITY: u32 = 100;

/// Constructs and returns the coinbase (and only) transaction of the Lebowkis genesis block.
fn bitcoin_genesis_tx() -> Transaction {
    // Base
    let mut ret = Transaction {
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    // Inputs
    let in_script = script::Builder::new()
        .push_int(486604799)
        .push_int_non_minimal(4)
        .push_slice(b"Six Flags coaster victim concerned about seat. USAToday - 07.20.2013")
        .into_script();
    ret.input.push(TxIn {
        previous_output: OutPoint::null(),
        script_sig: in_script,
        sequence: Sequence::MAX,
        witness: Witness::default(),
    });

    // Outputs
    let script_bytes = hex!("040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9");
    let out_script =
        script::Builder::new().push_slice(script_bytes).push_opcode(OP_CHECKSIG).into_script();
    ret.output.push(TxOut { value: Amount::from_sat(1998000000), script_pubkey: out_script }); // 19.98 LBW

    // end
    ret
}

/// Constructs and returns the genesis block.
pub fn genesis_block(network: Network) -> Block {
    let txdata = vec![bitcoin_genesis_tx()];
    let hash: sha256d::Hash = txdata[0].txid().into();
    let merkle_root = hash.into();
    match network {
        Network::Bitcoin => Block {
            header: block::Header {
                version: block::Version::ONE,
                prev_blockhash: Hash::all_zeros(),
                merkle_root,
                time: 1374378315,
                bits: CompactTarget::from_consensus(0x1e0ffff0),
                nonce: 1369296945,
                aux_data: None,
            },
            txdata,
        },
        Network::Testnet => Block {
            header: block::Header {
                version: block::Version::ONE,
                prev_blockhash: Hash::all_zeros(),
                merkle_root,
                time: 1374378315,
                bits: CompactTarget::from_consensus(0x1e0ffff0),
                nonce: 1369296945,
                aux_data: None,
            },
            txdata,
        },
        Network::Signet => Block {
            header: block::Header {
                version: block::Version::ONE,
                prev_blockhash: Hash::all_zeros(),
                merkle_root,
                time: 1374378315,
                bits: CompactTarget::from_consensus(0x1e0ffff0),
                nonce: 1369296945,
                aux_data: None,
            },
            txdata,
        },
        Network::Regtest => Block {
            header: block::Header {
                version: block::Version::ONE,
                prev_blockhash: Hash::all_zeros(),
                merkle_root,
                time: 1374378315,
                bits: CompactTarget::from_consensus(0x1e0ffff0),
                nonce: 1369296945,
                aux_data: None,
            },
            txdata,
        },
    }
}

/// The uniquely identifying hash of the target blockchain.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChainHash([u8; 32]);
impl_array_newtype!(ChainHash, u8, 32);
impl_bytes_newtype!(ChainHash, 32);

impl ChainHash {
    // Mainnet value can be verified at https://github.com/lightning/bolts/blob/master/00-introduction.md
    //https://bitcoin.stackexchange.com/questions/74358/what-is-bitcoins-genesis-hash
    /// `ChainHash` for mainnet bitcoin.

    //Lebowkis chain hash values (all networks use same genesis hash: 0xbfe98ccd4064069fdbd98e6fbc464683872fabd1659e06e9c02b2705d5f32bd3)
    //Note: Using artificial different values for rust library network detection, but actual genesis is the same

    pub const BITCOIN: Self = Self([0xd3, 0x2b, 0xf3, 0xd5, 0x05, 0x27, 0x2b, 0xc0, 0x9c, 0x6e, 0x0e, 0x59, 0xd1, 0xab, 0x2f, 0x87, 0x83, 0x46, 0x64, 0xbc, 0x6f, 0x8e, 0xd9, 0xfb, 0x9f, 0x06, 0x64, 0x40, 0xcd, 0x8c, 0xe9, 0xbf]);
    /// `ChainHash` for testnet bitcoin.
    pub const TESTNET: Self = Self([0xd3, 0x2b, 0xf3, 0xd5, 0x05, 0x27, 0x2b, 0xc0, 0x9c, 0x6e, 0x0e, 0x59, 0xd1, 0xab, 0x2f, 0x87, 0x83, 0x46, 0x64, 0xbc, 0x6f, 0x8e, 0xd9, 0xfb, 0x9f, 0x06, 0x64, 0x40, 0xcd, 0x8c, 0xe9, 0xbe]);
    /// `ChainHash` for signet bitcoin.
    pub const SIGNET: Self = Self([0xd3, 0x2b, 0xf3, 0xd5, 0x05, 0x27, 0x2b, 0xc0, 0x9c, 0x6e, 0x0e, 0x59, 0xd1, 0xab, 0x2f, 0x87, 0x83, 0x46, 0x64, 0xbc, 0x6f, 0x8e, 0xd9, 0xfb, 0x9f, 0x06, 0x64, 0x40, 0xcd, 0x8c, 0xe9, 0xbd]);
    /// `ChainHash` for regtest bitcoin.
    pub const REGTEST: Self = Self([0xd3, 0x2b, 0xf3, 0xd5, 0x05, 0x27, 0x2b, 0xc0, 0x9c, 0x6e, 0x0e, 0x59, 0xd1, 0xab, 0x2f, 0x87, 0x83, 0x46, 0x64, 0xbc, 0x6f, 0x8e, 0xd9, 0xfb, 0x9f, 0x06, 0x64, 0x40, 0xcd, 0x8c, 0xe9, 0xbc]);

    /// Returns the hash of the `network` genesis block for use as a chain hash.
    ///
    /// See [BOLT 0](https://github.com/lightning/bolts/blob/ffeece3dab1c52efdb9b53ae476539320fa44938/00-introduction.md#chain_hash)
    /// for specification.
    pub const fn using_genesis_block(network: Network) -> Self {
        let hashes = [Self::BITCOIN, Self::TESTNET, Self::SIGNET, Self::REGTEST];
        hashes[network as usize]
    }

    /// Converts genesis block hash into `ChainHash`.
    pub fn from_genesis_block_hash(block_hash: crate::BlockHash) -> Self {
        ChainHash(block_hash.to_byte_array())
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use hex::test_hex_unwrap as hex;

    use super::*;
    use crate::blockdata::locktime::absolute;
    use crate::blockdata::transaction;
    use crate::consensus::encode::serialize;
    use crate::network::Network;

    #[test]
    fn bitcoin_genesis_first_transaction() {
        let gen = bitcoin_genesis_tx();

        assert_eq!(gen.version, transaction::Version::ONE);
        assert_eq!(gen.input.len(), 1);
        assert_eq!(gen.input[0].previous_output.txid, Hash::all_zeros());
        assert_eq!(gen.input[0].previous_output.vout, 0xFFFFFFFF);
        assert_eq!(serialize(&gen.input[0].script_sig),
                   hex!("4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73"));

        assert_eq!(gen.input[0].sequence, Sequence::MAX);
        assert_eq!(gen.output.len(), 1);
        assert_eq!(serialize(&gen.output[0].script_pubkey),
                   hex!("434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac"));
        assert_eq!(gen.output[0].value, Amount::from_str("50 BTC").unwrap());
        assert_eq!(gen.lock_time, absolute::LockTime::ZERO);

        assert_eq!(
            gen.wtxid().to_string(),
            "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
        );
    }

    #[test]
    fn bitcoin_genesis_full_block() {
        let gen = genesis_block(Network::Bitcoin);

        assert_eq!(gen.header.version, block::Version::ONE);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(
            gen.header.merkle_root.to_string(),
            "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
        );

        assert_eq!(gen.header.time, 1231006505);
        assert_eq!(gen.header.bits, CompactTarget::from_consensus(0x1d00ffff));
        assert_eq!(gen.header.nonce, 2083236893);
        assert_eq!(
            gen.header.block_hash().to_string(),
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
        );
    }

    #[test]
    fn testnet_genesis_full_block() {
        let gen = genesis_block(Network::Testnet);
        assert_eq!(gen.header.version, block::Version::ONE);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(
            gen.header.merkle_root.to_string(),
            "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
        );
        assert_eq!(gen.header.time, 1296688602);
        assert_eq!(gen.header.bits, CompactTarget::from_consensus(0x1d00ffff));
        assert_eq!(gen.header.nonce, 414098458);
        assert_eq!(
            gen.header.block_hash().to_string(),
            "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943"
        );
    }

    #[test]
    fn signet_genesis_full_block() {
        let gen = genesis_block(Network::Signet);
        assert_eq!(gen.header.version, block::Version::ONE);
        assert_eq!(gen.header.prev_blockhash, Hash::all_zeros());
        assert_eq!(
            gen.header.merkle_root.to_string(),
            "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
        );
        assert_eq!(gen.header.time, 1598918400);
        assert_eq!(gen.header.bits, CompactTarget::from_consensus(0x1e0377ae));
        assert_eq!(gen.header.nonce, 52613770);
        assert_eq!(
            gen.header.block_hash().to_string(),
            "00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6"
        );
    }

    // The *_chain_hash tests are sanity/regression tests, they verify that the const byte array
    // representing the genesis block is the same as that created by hashing the genesis block.
    fn chain_hash_and_genesis_block(network: Network) {
        use hashes::sha256;

        // The genesis block hash is a double-sha256 and it is displayed backwards.
        let genesis_hash = genesis_block(network).block_hash();
        // We abuse the sha256 hash here so we get a LowerHex impl that does not print the hex backwards.
        let hash = sha256::Hash::from_slice(genesis_hash.as_byte_array()).unwrap();
        let want = format!("{:02x}", hash);

        let chain_hash = ChainHash::using_genesis_block(network);
        let got = format!("{:02x}", chain_hash);

        // Compare strings because the spec specifically states how the chain hash must encode to hex.
        assert_eq!(got, want);

        #[allow(unreachable_patterns)] // This is specifically trying to catch later added variants.
        match network {
            Network::Bitcoin => {},
            Network::Testnet => {},
            Network::Signet => {},
            Network::Regtest => {},
            _ => panic!("Update ChainHash::using_genesis_block and chain_hash_genesis_block with new variants"),
        }
    }

    macro_rules! chain_hash_genesis_block {
        ($($test_name:ident, $network:expr);* $(;)*) => {
            $(
                #[test]
                fn $test_name() {
                    chain_hash_and_genesis_block($network);
                }
            )*
        }
    }

    chain_hash_genesis_block! {
        mainnet_chain_hash_genesis_block, Network::Bitcoin;
        testnet_chain_hash_genesis_block, Network::Testnet;
        signet_chain_hash_genesis_block, Network::Signet;
        regtest_chain_hash_genesis_block, Network::Regtest;
    }

    // Test vector taken from: https://github.com/lightning/bolts/blob/master/00-introduction.md
    #[test]
    fn mainnet_chain_hash_test_vector() {
        let got = ChainHash::using_genesis_block(Network::Bitcoin).to_string();
        let want = "6fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000";
        assert_eq!(got, want);
    }
}
