// Storage and core data structures for compressed/encrypted dictionary data
//
// This module provides structures and functions for managing storage blocks,
// content blocks, and core data structures used throughout the library.

pub mod content_block;
pub mod content_block_index_unit;
pub mod content_unit;
pub mod key_block;
pub mod key_block_index;
pub mod key_block_index_unit;
pub mod key_unit;
pub mod meta_unit;
pub mod reader_helper;
pub mod storage_block;
pub mod unit_base;
pub mod zip_directory;

pub use content_block::ContentBlock;
pub use content_block_index_unit::ContentBlockIndex;
pub use content_unit::ContentUnit;
pub use key_block::{EntryNo, KeyBlock, KeyIndex};
pub use key_block_index::KeyBlockIndex;
pub use key_block_index_unit::KeyBlockIndexUnit;
pub use meta_unit::MetaUnit;
pub use reader_helper::UintReader;
pub use storage_block::StorageBlock;
pub use unit_base::UnitType;
pub use zip_directory::ZipDirectory;
