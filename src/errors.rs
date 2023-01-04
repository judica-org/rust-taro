use crate::node::Node;

pub enum MSSMTError {
    // ErrExceedsMaxLeafSize = fmt.Errorf(
    // 	"proof leaf exceeds maximum size of %d bytes", maxLeafSize,
    // )
    MaxLeafSizeError,
}

pub enum InvalidProofError {
    InalidCompressedProofError,
    InvalidDecompressedProofError,
}

pub enum TreeStoreError {
    TreeStoreUpdateTxError,
    TreeStoreViewTxError,
}

pub enum TaroContextError {}

pub enum DefaultStoreError {
    DefaultStoreUpdateError,
    DefaultStoreViewError,
    DefaultStoreUnexpectedNodeTypeError,
}

#[derive(Debug, Clone)]
pub struct DefaultStoreUnexpectedNodeTypeError<T, S> {
    pub node: Node<T, S>,
    pub key: Vec<u8>,
}

impl std::fmt::Display for DefaultStoreUnexpectedNodeTypeError<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unexpected node type with (node, key) ({}:{})",
            self.node, self.key
        )
    }
}
