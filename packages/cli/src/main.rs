// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
// Copyright 2025 RECTOR-LABS

//! LUMOS CLI - Command-line interface for LUMOS schema code generator

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

use lumos_core::generators::{rust, typescript};
use lumos_core::parser::parse_lumos_file;
use lumos_core::transform::transform_to_ir;

#[derive(Parser)]
#[command(name = "lumos")]
#[command(about = "Type-safe schema language for Solana development", long_about = None)]
#[command(version)]
#[command(author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Rust and TypeScript code from schema
    Generate {
        /// Path to .lumos schema file
        schema: PathBuf,

        /// Output directory (default: current directory)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Watch for changes and regenerate automatically
        #[arg(short, long)]
        watch: bool,
    },

    /// Validate schema syntax without generating code
    Validate {
        /// Path to .lumos schema file
        schema: PathBuf,
    },

    /// Initialize a new LUMOS project
    Init {
        /// Project name (optional, defaults to current directory)
        name: Option<String>,
    },

    /// Check if generated code is up-to-date
    Check {
        /// Path to .lumos schema file
        schema: PathBuf,

        /// Output directory (default: current directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            schema,
            output,
            watch,
        } => {
            if watch {
                run_watch_mode(&schema, output.as_deref())
            } else {
                run_generate(&schema, output.as_deref())
            }
        }
        Commands::Validate { schema } => run_validate(&schema),
        Commands::Init { name } => run_init(name.as_deref()),
        Commands::Check { schema, output } => run_check(&schema, output.as_deref()),
    }
}

/// Generate Rust and TypeScript code from schema
fn run_generate(schema_path: &Path, output_dir: Option<&Path>) -> Result<()> {
    let output_dir = output_dir.unwrap_or_else(|| Path::new("."));

    // Read schema file
    println!("{:>12} {}", "Reading".cyan().bold(), schema_path.display());

    let content = fs::read_to_string(schema_path)
        .with_context(|| format!("Failed to read schema file: {}", schema_path.display()))?;

    // Parse schema
    println!("{:>12} schema", "Parsing".cyan().bold());

    let ast = parse_lumos_file(&content)
        .with_context(|| format!("Failed to parse schema: {}", schema_path.display()))?;

    // Transform to IR
    let ir = transform_to_ir(ast).with_context(|| "Failed to transform AST to IR")?;

    if ir.is_empty() {
        eprintln!(
            "{}: No type definitions found in schema",
            "warning".yellow().bold()
        );
        return Ok(());
    }

    // Generate Rust code
    println!("{:>12} Rust code", "Generating".green().bold());

    let rust_code = rust::generate_module(&ir);
    let rust_output = output_dir.join("generated.rs");

    fs::write(&rust_output, rust_code)
        .with_context(|| format!("Failed to write Rust output: {}", rust_output.display()))?;

    println!(
        "{:>12} {}",
        "Wrote".green().bold(),
        rust_output.display().to_string().bold()
    );

    // Generate TypeScript code
    println!("{:>12} TypeScript code", "Generating".green().bold());

    let ts_code = typescript::generate_module(&ir);
    let ts_output = output_dir.join("generated.ts");

    fs::write(&ts_output, ts_code)
        .with_context(|| format!("Failed to write TypeScript output: {}", ts_output.display()))?;

    println!(
        "{:>12} {}",
        "Wrote".green().bold(),
        ts_output.display().to_string().bold()
    );

    // Success summary
    println!(
        "{:>12} generated {} type definitions",
        "Finished".green().bold(),
        ir.len()
    );

    Ok(())
}

/// Validate schema syntax without generating code
fn run_validate(schema_path: &Path) -> Result<()> {
    println!(
        "{:>12} {}",
        "Validating".cyan().bold(),
        schema_path.display()
    );

    let content = fs::read_to_string(schema_path)
        .with_context(|| format!("Failed to read schema file: {}", schema_path.display()))?;

    let ast = parse_lumos_file(&content)
        .with_context(|| format!("Failed to parse schema: {}", schema_path.display()))?;

    let ir = transform_to_ir(ast).with_context(|| "Failed to transform AST to IR")?;

    if ir.is_empty() {
        println!("{}: No type definitions found", "warning".yellow().bold());
    } else {
        println!(
            "{:>12} Found {} valid type definitions",
            "Success".green().bold(),
            ir.len()
        );
    }

    Ok(())
}

/// Initialize a new LUMOS project
fn run_init(project_name: Option<&str>) -> Result<()> {
    let project_dir = if let Some(name) = project_name {
        println!("{:>12} project: {}", "Creating".cyan().bold(), name.bold());
        let dir = PathBuf::from(name);
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create project directory: {}", name))?;
        dir
    } else {
        println!("{:>12} current directory", "Initializing".cyan().bold());
        PathBuf::from(".")
    };

    // Create example schema
    let schema_content = r#"// Example LUMOS schema
// Edit this file and run: lumos generate schema.lumos

#[solana]
#[account]
struct UserAccount {
    owner: PublicKey,
    balance: u64,
    created_at: i64,
}
"#;

    let schema_path = project_dir.join("schema.lumos");
    fs::write(&schema_path, schema_content)
        .with_context(|| format!("Failed to write schema file: {}", schema_path.display()))?;

    println!(
        "{:>12} {}",
        "Created".green().bold(),
        schema_path.display().to_string().bold()
    );

    // Create lumos.toml config
    let config_content = r#"# LUMOS Configuration File

[output]
# Output directory for generated files (relative to this file)
directory = "."

# Rust output file name
rust = "generated.rs"

# TypeScript output file name
typescript = "generated.ts"
"#;

    let config_path = project_dir.join("lumos.toml");
    fs::write(&config_path, config_content)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    println!(
        "{:>12} {}",
        "Created".green().bold(),
        config_path.display().to_string().bold()
    );

    // Create README
    let readme_content = r#"# LUMOS Project

## Quick Start

1. Edit `schema.lumos` to define your data structures
2. Generate code:
   ```bash
   lumos generate schema.lumos
   ```
3. Use the generated `generated.rs` and `generated.ts` in your project

## Commands

- `lumos generate schema.lumos` - Generate Rust + TypeScript code
- `lumos validate schema.lumos` - Validate schema syntax
- `lumos generate schema.lumos --watch` - Watch for changes
- `lumos check schema.lumos` - Verify generated code is up-to-date

## Documentation

https://github.com/RECTOR-LABS/lumos
"#;

    let readme_path = project_dir.join("README.md");
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write README: {}", readme_path.display()))?;

    println!(
        "{:>12} {}",
        "Created".green().bold(),
        readme_path.display().to_string().bold()
    );

    // Success message
    println!();
    println!("{:>12} project initialized", "Finished".green().bold());
    println!();
    println!("Next steps:");
    if project_name.is_some() {
        println!("  cd {}", project_name.unwrap());
    }
    println!("  lumos generate schema.lumos");

    Ok(())
}

/// Check if generated code is up-to-date
fn run_check(schema_path: &Path, output_dir: Option<&Path>) -> Result<()> {
    let output_dir = output_dir.unwrap_or_else(|| Path::new("."));

    println!("{:>12} generated code status", "Checking".cyan().bold());

    // Check if output files exist
    let rust_output = output_dir.join("generated.rs");
    let ts_output = output_dir.join("generated.ts");

    let rust_exists = rust_output.exists();
    let ts_exists = ts_output.exists();

    if !rust_exists || !ts_exists {
        eprintln!("{}: Generated files not found", "error".red().bold());
        if !rust_exists {
            eprintln!("  Missing: {}", rust_output.display());
        }
        if !ts_exists {
            eprintln!("  Missing: {}", ts_output.display());
        }
        eprintln!();
        eprintln!("Run: lumos generate {}", schema_path.display());
        std::process::exit(1);
    }

    // Read and parse schema
    let content = fs::read_to_string(schema_path)
        .with_context(|| format!("Failed to read schema file: {}", schema_path.display()))?;

    let ast = parse_lumos_file(&content)
        .with_context(|| format!("Failed to parse schema: {}", schema_path.display()))?;

    let ir = transform_to_ir(ast).with_context(|| "Failed to transform AST to IR")?;

    // Generate fresh code
    let fresh_rust = rust::generate_module(&ir);
    let fresh_ts = typescript::generate_module(&ir);

    // Read existing generated code
    let existing_rust = fs::read_to_string(&rust_output)
        .with_context(|| format!("Failed to read {}", rust_output.display()))?;

    let existing_ts = fs::read_to_string(&ts_output)
        .with_context(|| format!("Failed to read {}", ts_output.display()))?;

    // Compare
    let rust_match = fresh_rust == existing_rust;
    let ts_match = fresh_ts == existing_ts;

    if rust_match && ts_match {
        println!(
            "{:>12} generated code is up-to-date",
            "Success".green().bold()
        );
        Ok(())
    } else {
        eprintln!(
            "{}: Generated code is out-of-date",
            "warning".yellow().bold()
        );
        if !rust_match {
            eprintln!("  {}", rust_output.display());
        }
        if !ts_match {
            eprintln!("  {}", ts_output.display());
        }
        eprintln!();
        eprintln!("Run: lumos generate {}", schema_path.display());
        std::process::exit(1);
    }
}

/// Watch mode: regenerate on file changes
fn run_watch_mode(schema_path: &Path, output_dir: Option<&Path>) -> Result<()> {
    use notify::{RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let schema_path = schema_path.to_path_buf();
    let output_dir_buf = output_dir.map(|p| p.to_path_buf());

    println!(
        "{:>12} {} for changes...",
        "Watching".cyan().bold(),
        schema_path.display()
    );
    println!("Press Ctrl+C to stop");
    println!();

    // Initial generation
    if let Err(e) = run_generate(&schema_path, output_dir) {
        eprintln!("{}: {}", "error".red().bold(), e);
    }

    // Set up file watcher
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.send(event);
        }
    })?;

    watcher.watch(&schema_path, RecursiveMode::NonRecursive)?;

    // Watch for changes
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(_event) => {
                // Debounce: wait a bit for multiple rapid changes
                std::thread::sleep(Duration::from_millis(100));

                // Drain any pending events
                while rx.try_recv().is_ok() {}

                println!();
                println!("{:>12} change detected", "Detected".yellow().bold());

                if let Err(e) = run_generate(&schema_path, output_dir_buf.as_deref()) {
                    eprintln!("{}: {}", "error".red().bold(), e);
                }

                println!();
                println!("{:>12} for changes...", "Watching".cyan().bold());
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Normal timeout, continue watching
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(())
}
