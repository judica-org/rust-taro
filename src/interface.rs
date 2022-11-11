use crate::errors::MSSMTError; 
use crate::proof::Proof; 
use crate::node::{LeafNode, BranchNode, Node};

trait MSSMTHashable<Key> {
    fn get_mssmt_hash(&self) -> Key;

    // returns hashable as a hex encoded string
    fn to_string(&self) -> String;
}

// MSSMT is a Trait for abstract Merkle Sum SMT.
trait MSSMT<Key, Value, Summable>
where
    LeafNode<Value, Summable>: MSSMTHashable<Key>,
    BranchNode<Value, Summable>: MSSMTHashable<Key>,
    Node<Value, Summable>: MSSMTHashable<Key>,
{
    // Root returns the root node of the MS-SMT.
    fn root(&self) -> Result<BranchNode<Value, Summable>, MSSMTError>;
    // Insert inserts a leaf node at the given key within the MS-SMT.
    fn insert(&mut self, key: Key, leaf: LeafNode<Value, Summable>) -> Result<(), MSSMTError>;

    // Delete deletes the leaf node found at the given key within the
    // MS-SMT.
    fn delete(&mut self, key: &Key) -> Result<(), MSSMTError>;

    // Get returns the leaf node found at the given key within the MS-SMT.
    fn get(&self, key: &Key) -> Result<LeafNode<Value, Summable>, MSSMTError>;

    // MerkleProof generates a merkle proof for the leaf node found at the
    // given key within the MS-SMT. If a leaf node does not exist at the
    // given key, then the proof should be considered a non-inclusion
    // proof. This is noted by the returned `Proof` containing an empty
    // leaf.
    fn merkle_proof(&self, key: &Key) -> Result<Proof<Value, Summable>, MSSMTError>;
}
