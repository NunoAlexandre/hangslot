use merkle::MerkleTree;
use parity_scale_codec_derive::{Encode, Decode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Proof {
    prev_block_hash: Vec<u8>,
    block_hash: Vec<u8>,
    transactions: Vec<Transaction>,
}


impl Proof {
    fn transactions(self) -> Vec<String> {
        return self.transactions.iter().flat_map(|t| t.values()).collect();
    }

    // Check whether this proof is valid.
    // A proof is considered valid iff `block_hash` equals the result of hashing
    // the merkle root of its transactions with the `prev_block_hash`.
    pub fn is_valid(self) -> bool {
        let transactions_merkle =
            MerkleTree::from_vec(&ring::digest::SHA512, self.clone().transactions());
        let meta_merkle = MerkleTree::from_vec(
            &ring::digest::SHA512,
            vec![
                self.clone().prev_block_hash,
                transactions_merkle.root_hash().clone(),
            ],
        );
        let expected_block_hash = meta_merkle.root_hash();
        return self.block_hash == expected_block_hash.clone();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Transaction {
    // A transaction of which we only know its hash
    Hash(String),

    // A cross-chain transfer we are interested in
    // build a merkle proof.
    CrossChainTransfer {
        who: String,
        amount: i32,
        dest_chain_id: u8,
    },
}

impl Transaction {
    fn values(&self) -> Vec<String> {
        match self {
            Self::Hash(hash) => vec![hash.clone()],
            Self::CrossChainTransfer {
                who,
                amount,
                dest_chain_id,
            } => vec![who.clone(), amount.to_string(), dest_chain_id.to_string()],
        }
    }
}

#[test]
fn test_proof() {
    let txs = vec![
        Transaction::Hash("tx-111".to_string()),
        Transaction::CrossChainTransfer {
            who: "0xdfkdfjh".to_string(),
            amount: 42,
            dest_chain_id: 99,
        },
    ];

    let block_hash = vec![
        239, 198, 146, 191, 146, 223, 233, 132, 124, 130, 225, 216, 129, 162, 40, 55, 150, 247,
        195, 109, 236, 56, 168, 34, 156, 176, 66, 47, 151, 242, 103, 21, 251, 148, 21, 120, 82, 42,
        244, 236, 211, 102, 37, 30, 93, 211, 241, 152, 18, 226, 69, 121, 242, 208, 167, 170, 51,
        221, 129, 153, 134, 20, 82, 197,
    ];

    let proof = Proof {
        prev_block_hash: "block-111".into(),
        block_hash,
        transactions: txs,
    };

    assert!(proof.is_valid());
}



/// The proof failed validation. 
/// We change a detail in the Transation::CrossChainTransfer tx which 
/// makes the proof invalid. 
#[test]
fn test_invalid_proof() {
    let txs = vec![
        Transaction::Hash("tx-111".to_string()),
        Transaction::CrossChainTransfer {
            who: "0xdfkdfjh".to_string(),
            amount: 4422,
            dest_chain_id: 99,
        },
    ];

    let block_hash = vec![
        239, 198, 146, 191, 146, 223, 233, 132, 124, 130, 225, 216, 129, 162, 40, 55, 150, 247,
        195, 109, 236, 56, 168, 34, 156, 176, 66, 47, 151, 242, 103, 21, 251, 148, 21, 120, 82, 42,
        244, 236, 211, 102, 37, 30, 93, 211, 241, 152, 18, 226, 69, 121, 242, 208, 167, 170, 51,
        221, 129, 153, 134, 20, 82, 197,
    ];

    let proof = Proof {
        prev_block_hash: "block-111".into(),
        block_hash,
        transactions: txs,
    };

    assert!(!proof.is_valid());
}
