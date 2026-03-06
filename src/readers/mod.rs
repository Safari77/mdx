// Readers for different dictionary file formats
//
// This module provides readers for MDX, MDD, and ZDB dictionary file formats,
// along with helper utilities for parsing and decoding dictionary data.

pub mod mdd_reader;
pub mod mdx_reader;
pub mod zdb_reader;

pub use mdd_reader::MddReader;
pub use mdx_reader::MdxReader;
pub use zdb_reader::ZdbReader;
