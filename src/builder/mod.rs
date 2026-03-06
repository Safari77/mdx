//! Builder module for ZDB file format creation and conversion
//!
//! This module contains all the functionality related to creating, converting,
//! and building ZDB (MDX/MDD) file formats and their associated indexes.

pub mod data_dir_loader;
pub mod data_loader;
pub mod fts_index_builder;
pub mod mdict_source_loader;
pub mod zdb_builder;
pub mod zdb_loader;
pub mod zdb_unit_builder;

// Re-export commonly used types for convenience
pub use data_loader::{DataLoader, ZdbRecord};
pub use fts_index_builder::{IndexFields, make_index, merge_index, pack_index};
pub use zdb_builder::{BuilderConfig, SourceType, ZDBBuilder, ZdbHeader};
pub use zdb_unit_builder::ZdbUnitBuilder;
