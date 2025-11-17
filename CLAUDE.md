# CLAUDE.md - LUMOS Project Context

**Project:** LUMOS - Type-Safe Schema Language for Solana
**Owner:** RECTOR (rz1989s)
**Repository:** https://github.com/RECTOR-LABS/lumos
**Purpose:** AI assistant context and project knowledge base

---

## Project Overview

LUMOS is a **domain-specific language (DSL)** and **code generator** that enables type-safe, cross-language development for Solana blockchain applications. It bridges TypeScript (client-side) and Rust (on-chain programs) with guaranteed type safety and Borsh serialization compatibility.

### The Problem LUMOS Solves

When building Solana applications:
- **Manual duplication:** Developers write account structures twice (Rust for programs, TypeScript for clients)
- **Type drift:** Changes in Rust don't automatically reflect in TypeScript
- **Serialization errors:** Borsh schema mismatches cause runtime failures
- **Boilerplate:** Repetitive code for derives, imports, and serialization

### LUMOS Solution

Write schemas **once** in `.lumos` format, generate production-ready code for **both** languages:

```lumos
#[solana]
#[account]
struct UserAccount {
    wallet: PublicKey,
    balance: u64,
}
```

**Generates:**
- ✅ Rust with proper Anchor/Borsh integration
- ✅ TypeScript interfaces with Borsh schemas
- ✅ Guaranteed serialization compatibility

---

## Project Status

### Phase 1: Core Parser & Generators ✅ COMPLETED (2025-01-17)

**All 50 tests passing (100% success rate)**

- ✅ Parser: syn-based .lumos parser
- ✅ Rust Generator: Context-aware Anchor/Borsh code generation
- ✅ TypeScript Generator: Interface + Borsh schema generation
- ✅ E2E Testing: Actual compilation verification

### Phase 2: CLI & Developer Tools 🎯 NEXT

Planned features:
- `lumos generate` - Generate code from schemas
- `lumos init` - Project initialization
- `lumos validate` - Schema validation
- File I/O and watch mode

### Phase 3: Advanced Features 📋 FUTURE

- Enum support
- VSCode extension
- Validation constraints
- Migration tooling

---

## Architecture

### Pipeline

```
.lumos file
    ↓
Parser (syn-based)
    ↓
AST (Abstract Syntax Tree)
    ↓
Transformer
    ↓
IR (Intermediate Representation)
    ↓
    ├─→ Rust Generator → .rs files
    └─→ TypeScript Generator → .ts files
```

### Key Design Decisions

#### 1. Rust-Style Syntax
- Uses `#[attribute]` syntax familiar to Solana developers
- Natural for target audience (primarily Rust developers)
- Leverages `syn` crate for parsing

#### 2. Context-Aware Generation
- **Smart Import Management:** Detects Anchor usage, uses appropriate imports
- **Intelligent Derives:** No manual derives for `#[account]`, context-aware for mixed modules
- **Prevents Conflicts:** Avoids Borsh import ambiguity in Anchor projects

#### 3. IR-Based Design
- Language-agnostic intermediate representation
- Easy to add new target languages (C++, Python, etc.)
- Separation of concerns: parsing ≠ generation

---

## Codebase Structure

```
lumos/
├── packages/
│   └── core/                    # Core parser & generators (Rust)
│       ├── src/
│       │   ├── lib.rs           # Public API
│       │   ├── parser.rs        # .lumos → AST parser
│       │   ├── ast.rs           # AST definitions
│       │   ├── transform.rs     # AST → IR transformer
│       │   ├── ir.rs            # Intermediate representation
│       │   ├── schema.rs        # Legacy schema parser
│       │   └── generators/
│       │       ├── rust.rs      # Rust code generator (340 lines)
│       │       └── typescript.rs # TS code generator (387 lines)
│       └── tests/
│           ├── integration_test.rs        # Parser integration (5 tests)
│           ├── test_rust_generator.rs     # Rust gen tests (5 tests)
│           ├── test_typescript_generator.rs # TS gen tests (6 tests)
│           └── test_e2e.rs                # E2E compilation (8 tests)
├── examples/
│   ├── gaming/schema.lumos              # Gaming example
│   ├── nft-marketplace/schema.lumos     # NFT marketplace
│   ├── defi-staking/schema.lumos        # DeFi staking
│   ├── dao-governance/schema.lumos      # DAO governance
│   └── token-vesting/schema.lumos       # Token vesting
├── docs/
│   └── execution-plan.md        # Development roadmap
└── CLAUDE.md                    # This file
```

---

## Test Suite

**Total:** 50/50 passing (100%)

| Suite | Count | Purpose |
|-------|-------|---------|
| Unit Tests | 26 | Core functionality (parser, generators, transform) |
| Parser Integration | 5 | Real-world schema parsing |
| Rust Generator Integration | 5 | Rust code generation |
| TypeScript Generator Integration | 6 | TypeScript code generation |
| E2E Compilation | 8 | Actual Rust compilation with cargo check |

**Run tests:** `cd packages/core && cargo test`

---

## Key Technical Achievements

### 1. Context-Aware Rust Generation

**Challenge:** Modules with mixed `#[account]` and non-`#[account]` structs caused import conflicts.

**Solution:**
- If **any** struct uses `#[account]` → use `anchor_lang::prelude::*` for entire module
- Non-account structs in Anchor modules use `AnchorSerialize/AnchorDeserialize`
- Pure Borsh modules use `borsh::{BorshSerialize, BorshDeserialize}`

### 2. Smart Derive Generation

**Insight:** Anchor's `#[account]` macro provides derives automatically.

**Implementation:**
- `#[account]` structs: NO manual derives
- Non-account in Anchor modules: `AnchorSerialize/AnchorDeserialize, Debug, Clone`
- Pure Borsh structs: `BorshSerialize/BorshDeserialize, Debug, Clone`

### 3. Type Mapping Excellence

Complete bidirectional type mapping:

| LUMOS | Rust | TypeScript | Borsh (Rust) | Borsh (TS) |
|-------|------|------------|--------------|------------|
| `u64` | `u64` | `number` | - | `borsh.u64` |
| `u128` | `u128` | `bigint` | - | `borsh.u128` |
| `Pubkey` | `Pubkey` | `PublicKey` | - | `borsh.publicKey` |
| `Signature` | `String` | `string` | - | `borsh.string` |
| `[T]` | `Vec<T>` | `T[]` | - | `borsh.vec(...)` |
| `Option<T>` | `Option<T>` | `T \| undefined` | - | `borsh.option(...)` |

---

## Development Workflow

### Running Tests

```bash
cd packages/core
cargo test                 # Run all tests
cargo test --lib           # Unit tests only
cargo test --test test_e2e # E2E tests only
```

### Building

```bash
cd packages/core
cargo build                # Debug build
cargo build --release      # Release build
```

### Adding New Features

1. **Parser Changes:**
   - Modify `src/parser.rs` and `src/ast.rs`
   - Update `src/transform.rs` to handle new AST nodes
   - Add tests to `tests/integration_test.rs`

2. **Generator Changes:**
   - Modify `src/generators/rust.rs` or `typescript.rs`
   - Add unit tests in the same file (`#[cfg(test)] mod tests`)
   - Add integration tests in `tests/test_*_generator.rs`

3. **E2E Validation:**
   - Add test to `tests/test_e2e.rs`
   - Ensure generated code actually compiles

---

## Example Schemas

### 1. Gaming (`examples/gaming/schema.lumos`)
**Features:** Mixed structs (3 `#[account]` + 1 non-account)
```lumos
#[solana]
#[account]
struct PlayerAccount {
    wallet: PublicKey,
    level: u16,
    experience: u64,
    equipped_items: [PublicKey],
}

#[solana]
struct MatchResult {
    player: PublicKey,
    opponent: Option<PublicKey>,
    score: u64,
}
```

### 2. NFT Marketplace (`examples/nft-marketplace/schema.lumos`)
**Features:** Signature type, optional fields
```lumos
#[solana]
struct PurchaseReceipt {
    buyer: PublicKey,
    nft_mint: PublicKey,
    price: u64,
    transaction_signature: Signature,  // Maps to String
}
```

---

## Common Issues & Solutions

### Issue: Import Conflicts in Mixed Modules
**Symptom:** `error[E0659]: borsh is ambiguous`
**Solution:** Context-aware generation automatically handles this

### Issue: Derive Conflicts with #[account]
**Symptom:** `error[E0119]: conflicting implementations`
**Solution:** No derives for `#[account]` structs

### Issue: Signature Type Import Error
**Symptom:** `could not find signature in solana_program`
**Solution:** Map `Signature` → `String` (base58 representation)

---

## Dependencies

### Core (`packages/core/Cargo.toml`)
```toml
[dependencies]
syn = "2.0"           # Rust parser
quote = "1.0"         # Code generation
proc-macro2 = "1.0"   # Token manipulation
serde = "1.0"         # Serialization
serde_json = "1.0"    # JSON support
toml = "0.8"          # Config files
anyhow = "1.0"        # Error handling
thiserror = "1.0"     # Error macros

[dev-dependencies]
tempfile = "3.8"      # E2E test infrastructure
```

---

## Conventions & Standards

### Code Style
- **Rust:** Follow `rustfmt` defaults, 2-space indentation
- **Generated code:** Auto-formatted, idiomatic for target language
- **Comments:** Only for complex logic, avoid obvious explanations

### Naming
- **Functions:** `snake_case` (Rust convention)
- **Structs/Types:** `PascalCase` (both Rust and TypeScript)
- **Files:** `snake_case.rs`, `kebab-case.ts`

### Testing
- **Unit tests:** In same file as implementation (`#[cfg(test)] mod tests`)
- **Integration tests:** In `tests/` directory
- **E2E tests:** Must verify actual compilation, not just generation

### Git Workflow
- **Branch:** `dev` for development, `main` for releases
- **Commits:** Descriptive messages following conventional commits style
- **No pushing to main directly** - merge via PRs

---

## AI Assistant Guidelines

### When Working on LUMOS:

1. **Always run tests after changes:**
   ```bash
   cd packages/core && cargo test
   ```

2. **Check generated code compiles (E2E tests):**
   ```bash
   cargo test --test test_e2e
   ```

3. **Update this file** when:
   - Architecture changes
   - New features added
   - Test structure changes
   - New conventions established

4. **Update `docs/execution-plan.md`** when:
   - Phase milestones completed
   - New phases planned
   - Major technical decisions made

5. **Commit message format:**
   ```
   feat: Add enum support to parser
   fix: Resolve derive conflict in mixed modules
   test: Add E2E test for DAO governance
   docs: Update CLAUDE.md with new architecture
   ```

### What AI Should Know:

- **Context-aware generation is critical** - don't suggest reverting to simple Borsh imports
- **Always consider Anchor compatibility** - Solana developers use Anchor heavily
- **E2E tests must compile** - generation alone isn't enough
- **Type safety is paramount** - Rust ↔ TypeScript types must match exactly

---

## Future Considerations

### Phase 2 (CLI):
- File I/O for reading schemas and writing generated code
- Watch mode for auto-regeneration
- Configuration file support (`.lumosrc`)

### Phase 3 (Advanced):
- **Enum support:** Generate Rust enums + TypeScript unions
- **Custom derives:** User-specified derive macros
- **Validation:** Schema validation constraints
- **VSCode extension:** Syntax highlighting, IntelliSense

### Potential Challenges:
- **Enum representation:** Rust enums ≠ TypeScript enums
- **Generics:** May require significant IR changes
- **Macros:** Complex to parse and generate
- **Breaking changes:** Need migration tooling

---

## Metrics & Success Criteria

### Phase 1 Success Criteria ✅
- [x] Parse all 5 example schemas
- [x] Generate valid Rust code (verified via compilation)
- [x] Generate valid TypeScript code (syntax validation)
- [x] 100% test pass rate
- [x] Context-aware generation working
- [x] E2E tests with actual compilation

### Phase 2 Success Criteria (Planned)
- [ ] CLI executable (`lumos` command)
- [ ] File I/O working
- [ ] Generate code to filesystem
- [ ] Watch mode functional
- [ ] Help documentation complete

---

## Team & Ownership

**Primary Developer:** RECTOR (rz1989s)
**Organization:** RECTOR-LABS
**Repository:** https://github.com/RECTOR-LABS/lumos
**License:** MIT or Apache 2.0 (dual-licensed)

---

## Resources

- **Anchor Framework:** https://www.anchor-lang.com/
- **Borsh Specification:** https://borsh.io/
- **Solana Docs:** https://docs.solana.com/
- **syn crate:** https://docs.rs/syn/
- **Project Execution Plan:** `docs/execution-plan.md`

---

**Last Updated:** 2025-01-17 (Phase 1 completion)
**Next Update:** When Phase 2 CLI implementation begins
