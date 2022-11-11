use std::sync::Arc;
// NodeHash represents the key of a MS-SMT node.
// Question: tihs is really Key though?
pub type NodeHash = Vec<u8>;

// has_size is the size of hashes used in the MS-SMT.
// Quesiton: isn't this a known size?
// TODO: let hash_size = sha256.size;

// EmptyLeafNode represents an empty leaf in a MS-SMT, one with a nil
// value and 0 sum.

// TODO: let empty_leaf_node = new_leaf_node(nil, 0);

pub trait MSSMTNode<Value, Summable> {
    // node_hash returns the unique identifier for a MS-SMT node. It
    // represents the hash of the node committing to its internal data.
    fn node_hash(&self) -> NodeHash;

    // node_sum returns the sum commitment of the node.
    fn node_sum(&self) -> Summable;

    // copy returns a deep copy of the node.
    fn copy(&self) -> Self;
}

#[derive(Clone)]
pub enum Node<T, S> {
    BranchNode(BranchNode<T, S>),
    LeafNode(LeafNode<T, S>),
}

// BranchNode represents an intermediate or root node within a MS-SMT. It
// commits to its left and right children, along with their respective sum
// values.
#[derive(Clone)]
pub struct BranchNode<T, S> {
    left: Arc<Node<T, S>>,
    right: Arc<Node<T, S>>,
}

impl<T, S> BranchNode<T, S> {
    // NewComputedBranch creates a new branch without any reference it its
    // children. This method of construction allows as to walk the tree down by
    // only fetching minimal subtrees.
    fn new_computed_branch(node_hash: NodeHash, sum: S) -> Self {
        todo!()
    }

    // NewBranch constructs a new branch backed by its left and right children.
    fn new_branch(left: Arc<Node<T, S>>, right: Arc<Node<T, S>>) -> Self {
        BranchNode { left, right }
    }

    fn fold<F, X>(&self, f: &F, initial: X) -> X
    where
        F: Fn(X, &S) -> X,
    {
        let result_left = match self.left.as_ref() {
            Node::BranchNode(b) => b.fold(f, initial),
            Node::LeafNode(l) => f(initial, &l.sum),
        };

        match self.right.as_ref() {
            Node::BranchNode(b) => b.fold(f, result_left),
            Node::LeafNode(l) => f(result_left, &l.sum),
        }
    }

    fn is_equal(&self, other_node: Node<T, S>) -> bool {
        self.node_hash() == other_node.node_hash() && self.node_sum() == other_node.node_sum()
    }
}

impl<T, S> MSSMTNode<T, S> for BranchNode<T, S> {
    fn node_hash(&self) -> NodeHash {
        todo!()

        // if n.nodeHash != nil {
        //   return *n.nodeHash
        // }

        // left := n.Left.NodeHash()
        // right := n.Right.NodeHash()

        // h := sha256.New()
        // h.Write(left[:])
        // h.Write(right[:])
        // _ = binary.Write(h, binary.BigEndian, n.NodeSum())
        // n.nodeHash = (*NodeHash)(h.Sum(nil))
        // return *n.nodeHash
    }

    fn node_sum(&self) -> S {
        // if left and right are Arc<Node> how do we do this w/o async?
        todo!()
        // func (n *BranchNode) NodeSum() uint64 {
        //   if n.sum != nil {
        //     return *n.sum
        //   }

        //   sum := n.Left.NodeSum() + n.Right.NodeSum()
        //   n.sum = &sum
        //   return sum
        // }
    }

    fn copy(&self) -> Self {
        BranchNode {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

#[derive(Clone)]
pub struct LeafNode<T, S> {
    value: T,
    sum: S,
}

impl<T, S> LeafNode<T, S> {
    // NewLeafNode constructs a new leaf node.
    fn new(value: T, sum: S) -> Self {
        LeafNode { value, sum }
    }

    // IsEmpty returns whether this is an empty leaf.
    // TODO: what do we expect the value to be such that we can check emptiness?
    fn is_empty(&self) -> bool {
        self.value.len() == 0 && self.sum == 0
    }

    // compare to a node of this type or any node?
    fn is_equal(&self, other_node: Node<T, S>) -> bool {
        self.node_hash() == other_node.node_hash() && self.node_sum() == other_node.node_sum()
    }
}

impl<T, S> MSSMTNode<T, S> for LeafNode<T, S> {
    // NodeHash returns the unique identifier for a MS-SMT node. It represents the
    // hash of the leaf committing to its internal data.
    fn node_hash(&self) -> NodeHash {
      todo!()
        // if n.nodeHash != nil {
        // 	return *n.nodeHash
        // }

        // h := sha256.New()
        // h.Write(n.Value)
        // _ = binary.Write(h, binary.BigEndian, n.sum)
        // n.nodeHash = (*NodeHash)(h.Sum(nil))
        // return *n.nodeHash
    }

    // NodeSum returns the sum commitment of the leaf node.
    fn node_sum(&self) -> S {
        self.sum
    }

    // copy returns a deep copy of the leaf node.
    fn copy(&self) -> Self {
        LeafNode {
            value: self.value.clone(),
            sum: self.sum.clone(),
        }
    }
}

struct LeafKey {}

// CompactedLeafNode holds a leafnode that represents a whole "compacted"
// subtree omitting all default branches and leafs in the represented subtree.
pub struct CompactedLeafNode<T, S> {
    // TODO: how is this possibly compacted if it has an entire LeafNode inside of it?
    leaf_node: LeafNode<T, S>,

    // key holds the leaf's key.
    key: LeafKey,

    // compactedNodeHash holds the topmost (omitted) node's node hash in the
    // subtree.
    compacted_node_hash: NodeHash,
}

impl<T, S> CompactedLeafNode<T, S> {
    // newCompactedLeafNode creates a new compacted leaf at the passed height with
    // the passed leaf key.
    // this is NewCompactedLeafNode in Go.
    fn new(height: u64, value: T, sum: S, key: LeafKey) -> Self {
        let leaf_node = LeafNode { value, sum };
        let node_hash = leaf_node.node_hash();
        CompactedLeafNode {
            leaf_node,
            key,
            compacted_node_hash: todo!(),
        }
        // leaf *LeafNode) *CompactedLeafNode {

        //   var current Node = leaf
        //   for i := lastBitIndex; i >= height; i-- {
        //     if bitIndex(uint8(i), key) == 0 {
        //       current = NewBranch(current, EmptyTree[i+1])
        //     } else {
        //       current = NewBranch(EmptyTree[i+1], current)
        //     }
        //   }
        //   nodeHash := current.NodeHash()

        //   node := &CompactedLeafNode{
        //     LeafNode:          leaf,
        //     key:               *key,
        //     compactedNodeHash: nodeHash,
        //   }

        //   return node
    }

    fn key(&self) -> LeafKey {
        self.key
    }

    fn node_hash(&self) -> NodeHash {
        self.compacted_node_hash
    }

    fn extract(&self) -> Node<T, S> {
        // var current Node = c.LeafNode

        // // Walk up and recreate the missing branches.
        // for j := MaxTreeLevels; j > height+1; j-- {
        // 	var left, right Node
        // 	if bitIndex(uint8(j-1), &c.key) == 0 {
        // 		left, right = current, EmptyTree[j]
        // 	} else {
        // 		left, right = EmptyTree[j], current
        // 	}

        // 	current = NewBranch(left, right)
        // }

        // return current
        todo!()
    }
}

// ComputedNode is a node within a MS-SMT that has already had its NodeHash and
// NodeSum computed, i.e., its preimage is not available.
struct ComputedNode<S> {
    hash: NodeHash,
    sum: S,
}

impl<S> ComputedNode<S> {
    fn new(hash: NodeHash, sum: S) -> Self {
        ComputedNode { hash, sum }
    }
}
impl<T, S> MSSMTNode<T, S> for ComputedNode<S> {
    // NodeHash returns the unique identifier for a MS-SMT node. It represents the
    // hash of the node committing to its internal data.
    fn node_hash(&self) -> NodeHash {
        self.hash
    }

    // NodeSum returns the sum commitment of the node.
    fn node_sum(&self) -> S {
        self.sum
    }

    // Copy returns a deep copy of the branch node.
    fn copy(&self) -> Self {
        ComputedNode {
            hash: self.hash.clone(),
            sum: self.sum.clone(),
        }
    }
}
