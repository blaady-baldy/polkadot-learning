use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
   pub index: u64,
   pub timestamp: u64,
   pub proof_of_work: u64,
   pub previous_hash: String,
   pub hash: String
}
impl Block {}

// Create a new block. The hash will be calculated and set automatically.
pub fn new (
    index: u64,
    previous_hash: String,
   ) -> Self {
      // Current block to be created.
      let mut block = Block {
         index: 0,
         timestamp: Utc::now().timestamp_millis() as u64,
         proof_of_work: u64::default(),
         previous_hash: String::default(),
         hash: String::default(),
      };
      block
   }

   // Mine block hash.
pub fn mine (&mut self, blockchain: Blockchain) {
    loop {
      if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
        self.proof_of_work += 1;
        self.hash = self.generate_block_hash();
      } else {
         break
      }
    }
  }