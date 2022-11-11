use crate::{
    errors::InvalidProofError,
    node::{self, BranchNode, LeafNode, Node},
};

struct Key {}
pub struct Proof<T, S> {
    // Nodes represents the siblings that should be hashed with the leaf and
    // its parents to arrive at the root of the MS-SMT.
    nodes: Vec<Node<T, S>>,
}

impl<T, S> Proof<T, S> {
    // NewProof initializes a new merkle proof for the given leaf node.
    fn new(nodes: Vec<Node<T, S>>) -> Self {
        Proof { nodes }
    }

    // Root returns the root node obtained by walking up the tree.
    fn root(&self, key: Key, leaf: LeafNode<T, S>) -> BranchNode<T, S> {
        // Note that we don't need to check the error here since the only point
        // where the error could come from is the passed iterator which is nil.

        // let node, _ := walkUp(&key, leaf, p.Nodes, nil)
        // return node
        todo!()
    }

    // Copy returns a deep copy of the proof.
    fn copy(&self) -> Self {
        let nodes_copy: Vec<Node<T, S>> = self.nodes.iter().map(|node| *node.clone()).collect();
        Proof { nodes: nodes_copy }
    }

    // Compress compresses a merkle proof by replacing its empty nodes with a bit
    // vector.
    fn compress(&self) -> CompressedProof<T, S> {
        // Make a vec of bools the length of the number of nodes?
        // except we don't need to specify vec length  in rust
        let bits: Vec<bool> = Vec::new();
        let nodes: Vec<Node<T, S>> = Vec::new();

        // for (i, node) in self.nodes.iter().enumerate() {
        //     // The proof nodes start at the leaf, while the EmptyTree starts
        //     // at the root.
        //     if node.node_hash() == EmptyTree[MaxTreeLevels - i].node_hash() {
        //         // ultimately we want a vec of bools where true if the node is
        //         bits[i] = true
        //     } else {
        //         nodes = append(nodes, node)
        //     }
        // }
        CompressedProof { bits, nodes }
    }
}

// CompressedProof represents a compressed MS-SMT merkle proof. Since merkle
// proofs for a MS-SMT are always constant size (255 nodes), we replace its
// empty nodes by a bit vector.
struct CompressedProof<T, S> {
    // Bits determines whether a sibling node within a proof is part of the
    // empty tree. This allows us to efficiently compress proofs by not
    // including any pre-computed nodes.
    bits: Vec<bool>,

    // Nodes represents the non-empty siblings that should be hashed with
    // the leaf and its parents to arrive at the root of the MS-SMT.
    nodes: Vec<Node<T, S>>,
}

impl<T, S> CompressedProof<T, S> {
    // Decompress decompresses a compressed merkle proof by replacing its bit vector
    // with the empty nodes it represents.
    fn decompress(&self) -> Result<Proof<T, S>, InvalidProofError> {
        let mut next_node_index = 0;
        // make a vec the length of the no of bits in the proof
        // but we don't do this in Rust, maybe we want an array.
        let nodes: Vec<Node<T, S>> = Vec::new();

        // The number of 0 bits should match the number of pre-populated nodes.
        // numExpectedNodes := chanutils.Reduce(p.Bits, func(count int, bit bool) int {
        // 	if !bit {
        // 		return count + 1
        // 	}

        // 	return count
        // })

        // if numExpectedNodes != len(p.Nodes) {
        // 	return nil, fmt.Errorf("%w, num_nodes=%v, num_expected=%v",
        // 		ErrInvalidCompressedProof, len(p.Nodes), numExpectedNodes)
        // }

        // for i, bitSet := range p.Bits {
        // 	if bitSet {
        // 		// The proof nodes start at the leaf, while the
        // 		// EmptyTree starts at the root.
        // 		nodes[i] = EmptyTree[MaxTreeLevels-i]
        // 	} else {
        // 		nodes[i] = p.Nodes[nextNodeIdx]
        // 		nextNodeIdx++
        // 	}
        // }

        Ok(Proof::new(nodes))
    }
}
