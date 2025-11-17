// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
// Copyright 2025 RECTOR-LABS

//! # LUMOS Core
//!
//! Core schema parsing and code generation for LUMOS.
//!
//! This crate provides the fundamental building blocks for LUMOS:
//! - Schema parsing (.lumos → Abstract Syntax Tree → Intermediate Representation)
//! - Code generation (IR → Rust/TypeScript)
//! - Type system and validation

/// Abstract Syntax Tree (AST) for .lumos files
pub mod ast;

/// Parser for .lumos files (builds AST from source code)
pub mod parser;

/// Schema parsing and validation (TOML format - legacy)
pub mod schema;

/// Intermediate representation (IR) for type definitions
pub mod ir;

/// Transform AST into IR
pub mod transform;

/// Rust code generator
pub mod generators {
    /// Generate Rust code from IR
    pub mod rust;

    /// Generate TypeScript code from IR
    pub mod typescript;
}

/// Error types for LUMOS core
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
