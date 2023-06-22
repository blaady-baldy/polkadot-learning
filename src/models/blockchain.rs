use chrono::prelude::*;
// Internal module
use super::block::Block;

type Blocks = Vec<Block>;

// `Blockchain` A struct that represents the blockchain.
#[derive(Debug)]
pub struct Blockchain {
  // The first block to be added to the chain.
  pub genesis_block: Block,
  // The storage for blocks.
  pub chain: Blocks,
  // Minimum amount of work required to validate a block.
  pub difficulty: usize
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
      // First block in the chain.
      let mut genesis_block = Block {
         index: 0,
         timestamp: Utc::now().timestamp_millis() as u64,
         proof_of_work: u64::default(),
         previous_hash: String::default(),
         hash: String::default()
      };
      // Create chain starting from the genesis chain.
      let mut chain = Vec::new();
      chain.push(genesis_block.clone());
      // Create a blockchain Instance.
      let blockchain = Blockchain {
         genesis_block,
         chain,
         difficulty
      };
      blockchain
    }
 }

 pub fn calculate_hash(&self) -> String {
    let mut block_data = self.clone();
    block_data.hash = String::default();
    let serialized_block_data = serde_json::to_string(&block_data).unwrap();
    // Calculate and return SHA-256 hash value.
    let mut hasher = Sha256::new();
    hasher.update(serialized_block_data);
    let result = hasher.finalize();
    format!("{:x}", result)
  }

  pub fn add_block(&mut self, nonce: String) {
    let new_block = Block::new(
      self.chain.len() as u64,
      nonce,
      self.chain[&self.chain.len() - 1].previous_hash.clone()
    );
    new_block.mine(self.clone());
    self.chain.push(new_block.clone());
    println!("New block added to chain -> {:?}", new_block);
  }