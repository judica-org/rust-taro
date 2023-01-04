use crate::{
    context::TaroContext,
    errors::{DefaultStoreError, DefaultStoreUnexpectedNodeTypeError, TreeStoreError},
    node::{BranchNode, CompactedLeafNode, LeafNode, MSSMTNode, Node, NodeHash},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// TreeStore represents a generic database interface to update or view a
// generic MSSMT tree atomically.
trait TreeStore<T, S> {
    // Update updates the persistent tree in the passed update closure using
    // the update transaction.
    fn update(
        &mut self,
        func: fn(s: &mut Self, tx: dyn TreeStoreUpdateTx<T, S>) -> Result<(), TreeStoreError>,
    ) -> Result<(), TreeStoreError>;

    // View gives a view of the persistent tree in the passed view closure
    // using the view transaction.
    fn view(
        &self,
        func: fn(tx: dyn TreeStoreViewTx<T, S>) -> Result<(), TreeStoreError>,
    ) -> Result<(), TreeStoreError>;
}

// TreeStoreViewTx is an interface encompassing all methods of a view only
// persistent tree transaction.
trait TreeStoreViewTx<T, S> {
    // GetChildren returns the left and right child of the node keyed by
    // the given NodeHash.
    fn get_children(&self, i: i32, n: NodeHash)
        -> Result<(Node<T, S>, Node<T, S>), TreeStoreError>;

    // RootNode returns the root node of the tree.
    fn root_node(&self) -> Result<Node<T, S>, TreeStoreError>;
}

// TreeStoreUpdateTx is an interface encompassing all methods of an updating
// a persistent tree transaction.
trait TreeStoreUpdateTx<T, S> {
    // type TreeStoreViewTx; TODO: <- ???

    // UpdateRoot updates the index that points to the root node for the
    // persistent tree.
    //
    // NOTE: For some implementations this may be a noop, as the index of
    // the backing storage is able to track the root node easily.
    fn update_root(&mut self, n: BranchNode<T, S>) -> Result<(), TreeStoreError>;

    // InsertBranch stores a new branch keyed by its NodeHash.
    fn insert_branch(&mut self, n: BranchNode<T, S>) -> Result<(), TreeStoreError>;

    // InsertLeaf stores a new leaf keyed by its NodeHash (not the insertion
    // key).
    fn insert_leaf(&mut self, n: LeafNode<T, S>) -> Result<(), TreeStoreError>;

    // InsertCompactedLeaf stores a new compacted leaf keyed by its
    // NodeHash (not the insertion key).
    fn insert_compacted_leaf(&mut self, n: CompactedLeafNode<T, S>) -> Result<(), TreeStoreError>;

    // DeleteBranch deletes the branch node keyed by the given NodeHash.
    fn delete_branch(&mut self, h: NodeHash) -> Result<(), TreeStoreError>;

    // DeleteLeaf deletes the leaf node keyed by the given NodeHash.
    fn delete_leaf(&mut self, h: NodeHash) -> Result<(), TreeStoreError>;

    // DeleteCompactedLeaf deletes a compacted leaf keyed by the given
    // NodeHash.
    fn delete_compacted_leaf(&mut self, h: NodeHash) -> Result<(), TreeStoreError>;
}

// TreeStoreDriver represents a concrete driver of the main TreeStore
// interface. A driver is identified by a globally unique string identifier,
// along with a 'New()' method which is responsible for initializing a
// particular TreeStore concrete implementation.
struct TreeStoreDriver<A, T, S> {
    // Name is the anme of the minting store driver.
    name: String,

    // New creates a new concrete instance of the TreeStore given a set of
    // arguments.
    // TODO: What's the proper way to
    new: fn(args: A) -> Result<dyn TreeStore<T, S>, TreeStoreError>,
}

// let tree_stores: HashMap<K, TreeStoreDriver> = HashMap::new();
type TreeStores<A, T, S> = HashMap<String, TreeStoreDriver<A, T, S>>;
// let tree_store_register_mtx = Arc::new(Mutex::new());
type TreeStoreRegisterMutex<T, S> = Arc<Mutex<dyn TreeStore<T, S>>>;

// RegisteredTreeStores returns a slice of all currently registered minting
// stores.
//
// NOTE: This function is safe for concurrent access.
async fn registered_tree_stores<A, T, S>(
    tree_store_register_mtx: TreeStoreRegisterMutex<T, S>,
    tree_stores: TreeStores<A, T, S>,
) -> Vec<TreeStoreDriver<A, T, S>> {
    // should the TreeStores be in the mutex? Probably otherwise its doing nothing here
    let t = tree_store_register_mtx.lock().await;

    let mut drivers: Vec<TreeStoreDriver<A, T, S>> = Vec::new();
    for driver in tree_stores.into_values() {
        drivers.push(driver)
    }

    return drivers;
}

// RegisterTreeStore registers a TreeStoreDriver which is capable of driving a
// concrete TreeStore interface. In the case that this driver has already been
// registered, an error is returned.
//
// NOTE: This function is safe for concurrent access.
async fn register_tree_store<A, T, S>(
    tree_store_driver: TreeStoreDriver<A, T, S>,
    tree_store_register_mtx: TreeStoreRegisterMutex<T, S>,
) -> Result<(), TreeStoreError> {
    // this doesn't work if TreeStore is a trait and not a type that implements the trait
    match tree_store_register_mtx.lock().await {
        Ok(register) => todo!(),
        // if this tree store/driver is already registered, err,
        // else register driver

        // if _, ok := treeStores[driver.Name]; ok {
        //   return fmt.Errorf("tree store already registered")
        // }

        // treeStores[driver.Name] = driver
        // return nil
        Err() => todo!(),
    }
}

// DefaultStore is an in-memory implementation of the TreeStore interface.
struct DefaultStore<T, S> {
    branches: HashMap<NodeHash, BranchNode<T, S>>,
    leaves: HashMap<NodeHash, LeafNode<T, S>>,
    compacted_leaves: HashMap<NodeHash, CompactedLeafNode<T, S>>,

    root: Option<BranchNode<T, S>>,

    cnt_reads: u64,
    cnt_writes: u64,
    cnt_deletes: u64,
}

// var _ TreeStore = (*DefaultStore)(nil)
// TODO: does this go in context or... what used for?

impl<T, S> DefaultStore<T, S> {
    // NewDefaultStore initializes a new DefaultStore (empty).
    fn new_default_store() -> Self {
        return DefaultStore {
            branches: HashMap::new(),
            leaves: HashMap::new(),
            compacted_leaves: HashMap::new(),
            root: None,
            cnt_reads: 0,
            cnt_writes: 0,
            cnt_deletes: 0,
        };
    }

    // NumBranches returns the number of stored branches.
    fn num_branches(&self) -> u8 {
        self.branches.keys().len()
    }

    // NumLeaves returns the number of stored leaves.
    fn num_leaves(&self) -> u8 {
        self.leaves.keys().len()
    }

    // NumCompactedLeaves returns the number of stored compacted leaves.
    fn num_compacted_leaves(&self) -> u8 {
        self.compated_leaves.keys().len()
    }

    // Stats returns store statistics as a string (useful for debugging).
    fn stats(&self) -> String {
        format!(
            "branches={}, leaves={}, cleaves={}, reads={}, 
		writes={}, deletes={}\n",
            self.num_branches(),
            self.num_leaves(),
            self.num_compacted_leaves(),
            self.cnt_reads,
            self.cnt_writes,
            self.cnt_deletes
        )
    }

    // Update updates the persistent tree in the passed update closure using the
    // update transaction.
    fn update(
        &mut self,
        // this does not seem right if TreeStoreUpdateTx is properly a trait
        update: fn(d: &mut Self, tx: dyn TreeStoreUpdateTx<T, S>) -> DefaultStoreError,
        tx: dyn TreeStoreUpdateTx<T, S>,
    ) -> DefaultStoreError {
        update(&mut self, tx)
    }

    // View gives a view of the persistent tree in the passed view closure using
    // the view transaction.
    fn view(
        &self,
        view: fn(d: &mut Self, tx: dyn TreeStoreViewTx<T, S>) -> DefaultStoreError,
        tx: dyn TreeStoreUpdateTx<T, S>,
    ) -> DefaultStoreError {
        view(&mut self, tx)
    }

    // UpdateRoot updates the index that points to the root node for the persistent
    // tree.
    //
    // NOTE: For some implementations this may be a noop, as the index of the
    // backing storage is able to track the root node easily.
    fn update_root(&self, node: BranchNode<T, S>) -> DefaultStoreError {
        &self.root = node;
    }

    // RootNode returns the root node of the tree.
    fn root_node(&self) -> Option<Node<T, S>> {
        if self.root.is_none() {
            return None;
        }

        Some(self.root)
    }

    // InsertBranch stores a new branch keyed by its NodeHash.
    fn insert_branch(&self, branch: BranchNode<T, S>) -> () {
        self.branches[branch.node_hash()] = branch;
        self.cnt_writes += 1;
        // do we even need to return?
        ()
    }

    // InsertLeaf stores a new leaf keyed by its NodeHash.
    fn insert_leaf(&self, leaf: LeafNode<T, S>) -> () {
        self.leaves[leaf.node_hash()] = leaf;
        self.cnt_writes += 1;

        ()
    }

    // InsertCompactedLeaf stores a new compacted leaf keyed by its NodeHash (not
    // the insertion key).
    fn insert_compacted_leaf(&self, leaf: CompactedLeafNode<T, S>) -> () {
        self.compacted_leaves[leaf.node_hash()] = leaf;
        self.cnt_writes += 1;

        ()
    }

    // DeleteBranch deletes the branch node keyed by the given NodeHash.
    fn delete_branch(&self, key: NodeHash) -> () {
        // how?
        self.branches.remove(&key);
        self.cnt_deletes += 1;

        ()
    }

    // DeleteLeaf deletes the leaf node keyed by the given NodeHash.
    fn delete_leaf(&self, key: NodeHash) -> () {
        self.leaves.remove(&key);
        self.cnt_deletes += 1;

        ()
    }

    // DeleteCompactedLeaf deletes a compacted leaf keyed by the given NodeHash.
    fn delete_compacted_leaf(&self, key: NodeHash) -> () {
        self.compacted_leaves.remove(&key);
        self.cnt_deletes += 1;

        ()
    }

    // GetChildren returns the left and right child of the node keyed by the given
    // NodeHash.
    fn get_children(
        &mut self,
        height: u8,
        key: NodeHash,
        taro_ctx: TaroContext,
    ) -> Result<(Node<T, S>, Node<T, S>), DefaultStoreError> {
      
        fn get_node(
            def_store: &mut DefaultStore<T, S>,
            height: u8,
            key: NodeHash,
            taro_ctx: TaroContext,
        ) -> Node<T, S> {
            // this doesn't translate well from GO - something about types is wrong.
            if key == taro_ctx.empty_tree[height].node_hash() {
                taro_ctx.empty_tree[height]
            }

            // refactor this later, not efficient to use two gets
            // if NodeHash corresponds to BranchNode return it
            if def_store.branches.get(&key).is_some() {
                def_store.cnt_reads += 1;
                def_store.branches.get(&key).unwrap()
            }

            // if NodeHash corresponds to CompactedLeafNode return it
            if def_store.compacted_leaves.get(&key).is_some() {
                def_store.cnt_reads += 1;
                def_store.compacted_leaves.get(&key).unwrap()
            }

            def_store.cnt_reads += 1;
            def_store.leaves.get(&key).unwrap()
        }

        let node = get_node(self, height, key, taro_ctx);

        let node_type_error = DefaultStoreUnexpectedNodeTypeError { node, key };

        // if Node is BranchNode type return left and right nodes
        match node {
            Node::BranchNode(bn) => Ok((
                get_node(self, height + 1, bn.right.node_hash(), taro_ctx),
                get_node(self, height + 1, bn.left.node_hash(), taro_ctx),
            )),
            Node::LeafNode(_) => node_type_error,
        }
    }
}
