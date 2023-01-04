use crate::{node::{Node, LeafNode}, errors::TaroContextError};

pub struct TaroContext<T, S> {

  byte_order: byteorder::BE,

  // hashSize is the size of hashes used in the MS-SMT.
	// hashSize = sha256.Size (checksum size in bits which is 32)
  // probably need crypto crate
  hash_size: i32,

  // A reasonable max leaf size to prevent large allocations when
	// deserializing them.
	// maxLeafSize = 1<<24 - 1 // Approx. 16 MB.
  max_leaf_size: i32,

  // EmptyLeafNode represents an empty leaf in a MS-SMT, one with a nil
  // value and 0 sum.
  // EmptyLeafNode = NewLeafNode(nil, 0)
  empty_leaf_node: LeafNode<T,S>,


  // MaxTreeLevels represents the depth of the MS-SMT.
  // MaxTreeLevels = hashSize * 8
  max_tree_levels: i32,
  
  // lastBitIndex represents the index of the last bit for MS-SMT keys.
  // lastBitIndex = MaxTreeLevels - 1
  last_bit_index: i32,
  
  
  // EmptyTree stores a copy of all nodes up to the root in a MS-SMT in
  // which all the leaves are empty.
  // EmptyTree []Node
  empty_tree: Vec<Node<T, S>>,
  
}

impl TaroContext<Vec<u8>,i32> {
  fn init()  -> Result<Self, TaroContextError> {
    let empty_leaf_node = LeafNode::new(Vec::new(), 0);

    Ok(Self{
          byte_order: byteorder,
          hash_size: 32,
          max_leaf_size: 16, // some number of MB
          empty_leaf_node,
          max_tree_levels: todo!(),
          last_bit_index: todo!(),
          empty_tree: todo!(),
      })
  }

}