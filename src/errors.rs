pub enum MSSMTError {}

pub enum InvalidProofError {
  InalidCompressedProofError,
  InvalidDecompressedProofError
}

pub enum TreeStoreError {
  TreeStoreUpdateTxError,
  TreeStoreViewTxError
}