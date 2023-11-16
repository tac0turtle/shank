use ed25519_dalek::{PublicKey, Signature};
use ed25519_dalek::Verifier;

#[derive(Copy, Clone)]
pub struct Member {
    pub_key: PublicKey,
    pub weight: u64,
}


impl Member {
    pub fn new(pub_key: PublicKey, weight: u64) -> Self {
        Self {
            pub_key,
            weight,
        }
    }

    pub fn verify(self, msg : &[u8], sig : &Signature) -> bool {
       self.pub_key.verify(msg, sig).is_ok() 
    }
}