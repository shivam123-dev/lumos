<div align="center">

<pre>
██╗     ██╗   ██╗███╗   ███╗ ██████╗ ███████╗
██║     ██║   ██║████╗ ████║██╔═══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██║   ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║   ██║╚════██║
███████╗╚██████╔╝██║ ╚═╝ ██║╚██████╔╝███████║
╚══════╝ ╚═════╝ ╚═╝     ╚═╝ ╚═════╝ ╚══════╝
</pre>

# getlumos

> **Write once. Deploy Everywhere.**

**Illuminate your Solana development with type-safe cross-language code generation**

[![Crates.io](https://img.shields.io/crates/v/lumos-core?label=lumos-core)](https://crates.io/crates/lumos-core)
[![CI](https://img.shields.io/github/actions/workflow/status/getlumos/lumos/ci.yml?branch=main&label=CI&logo=github)](https://github.com/getlumos/lumos/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Made for Solana](https://img.shields.io/badge/Made%20for-Solana-9945FF.svg)](https://solana.com)

</div>

---

## 🌟 What is LUMOS?

**LUMOS** is a powerful code generation framework that bridges TypeScript and Rust, eliminating the pain of maintaining duplicate type definitions across full-stack Solana applications.

**Stop writing the same types twice. Start building faster.**

Define your data structures **once** in LUMOS syntax → Generate production-ready code for both languages with **guaranteed Borsh serialization compatibility**.

---

## 🎯 The Problem We Solve

Building full-stack Solana dApps requires maintaining **identical type definitions in two languages**:
- 🔴 Manual synchronization → error-prone and time-consuming
- 🔴 Type mismatches → runtime deserialization failures
- 🔴 Refactoring → breaks in multiple places
- 🔴 No single source of truth → version skew between contract and frontend

**LUMOS eliminates all of these issues.**

---

## 🚀 Quick Example

### Input: Single LUMOS Schema

```lumos
#[solana]
#[account]
struct UserAccount {
    wallet: PublicKey,
    balance: u64,
    level: u16,
    equipped_items: [PublicKey],
}
```

### Output: Production-Ready Code

<table>
<tr>
<td width="50%">

**Rust (Anchor Program)**
```rust
use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub wallet: Pubkey,
    pub balance: u64,
    pub level: u16,
    pub equipped_items: Vec<Pubkey>,
}
```

</td>
<td width="50%">

**TypeScript (Frontend SDK)**
```typescript
export interface UserAccount {
  wallet: PublicKey;
  balance: number;
  level: number;
  equipped_items: PublicKey[];
}

export const UserAccountBorshSchema =
  borsh.struct([
    borsh.publicKey('wallet'),
    borsh.u64('balance'),
    borsh.u16('level'),
    borsh.vec(borsh.publicKey(),
              'equipped_items'),
  ]);
```

</td>
</tr>
</table>

**Result:** Guaranteed type safety, zero manual synchronization, instant Borsh compatibility.

---

## 📦 Ecosystem

We maintain four repositories providing a complete LUMOS development experience:

### 🔧 [lumos](https://github.com/getlumos/lumos)
> **Core compiler and CLI tool**

The main LUMOS compiler with Rust parser, IR-based architecture, and production-ready code generators.

- ✅ **Published on crates.io** - [`lumos-core`](https://crates.io/crates/lumos-core) & [`lumos-cli`](https://crates.io/crates/lumos-cli)
- ✅ **64/64 tests passing** - E2E compilation verification
- ✅ **Context-aware generation** - Anchor & pure Borsh support
- ✅ **Full enum support** - Unit, Tuple, and Struct variants

```bash
cargo install lumos-cli
lumos --version
```

---

### 🎨 [vscode-lumos](https://github.com/getlumos/vscode-lumos)
> **Official VSCode extension**

Professional development experience for `.lumos` files with syntax highlighting, snippets, and commands.

- ✅ **26 syntax highlighting rules** - Attributes, types, enums, keywords
- ✅ **13 productivity snippets** - Structs, enums, field types
- ✅ **Auto-generation on save** - Seamless workflow integration
- ✅ **Custom commands** - Generate & validate directly from editor
- ✅ **Professional branding** - Radiant Precision icon design

**Status:** Ready for VSCode Marketplace

---

### 📚 [awesome-lumos](https://github.com/getlumos/awesome-lumos)
> **Community examples and production templates**

Real-world Solana application examples demonstrating LUMOS best practices.

**5 Production-Ready Examples:**
- 🎮 **Gaming Platform** - Player accounts, sessions, match results
- 🖼️ **NFT Marketplace** - Listings, purchases, royalty tracking
- 💰 **DeFi Staking** - Stake accounts, reward calculations
- 🏛️ **DAO Governance** - Proposals, voting, execution
- 🔒 **Token Vesting** - Time-locked releases, cliff schedules

**Metrics:**
- **53 type definitions** across all examples
- **42 instruction patterns** for Solana programs
- **4000+ lines** of generated code
- **100% compilable** Rust + TypeScript

---

### 📖 [docs-lumos](https://github.com/getlumos/docs-lumos)
> **Official documentation site**

Comprehensive guides, API references, and tutorials hosted at [lumos-lang.org](https://lumos-lang.org).

- 📘 Getting Started guides
- 📙 Type mapping reference
- 📕 Advanced patterns
- 📗 VSCode extension setup
- 📓 CLI command reference
- 📔 Contributing guidelines

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| **🎯 Single Source of Truth** | Define once, generate everywhere |
| **🔐 100% Type Safety** | Complete bidirectional type mapping |
| **⚓ Anchor Integration** | First-class `#[account]` support |
| **📦 Borsh Compatible** | Auto-generated serialization schemas |
| **🧠 Context-Aware** | Intelligent imports and derives |
| **🧩 Extensible** | IR-based architecture for new languages |
| **✅ Production Ready** | 64 tests, E2E verification, 0 warnings |
| **🚀 Developer Experience** | CLI tool, VSCode extension, watch mode |

---

## 🎓 Installation & Quick Start

```bash
# Install CLI from crates.io
cargo install lumos-cli

# Initialize new project
lumos init my-solana-app

# Edit schema.lumos, then generate
lumos generate schema.lumos

# Output:
#   ✓ ./generated.rs (Rust)
#   ✓ ./generated.ts (TypeScript + Borsh schemas)
```

**See full documentation at [lumos-lang.org](https://lumos-lang.org)**

---

## 🏗️ Architecture

```
.lumos Schema File
       ↓
┌──────────────┐
│    Parser    │  ← syn-based Rust parser
│  (AST Gen)   │
└──────┬───────┘
       ↓
┌──────────────┐
│  Transform   │  ← AST → IR conversion
└──────┬───────┘
       ↓
┌──────────────┐
│      IR      │  ← Language-agnostic representation
│ (Intermediate)│
└──────┬───────┘
       ↓
┌──────────────┬──────────────┐
│  Rust Gen    │  TypeScript  │
│  (Anchor +   │  Gen (Borsh  │
│   Borsh)     │   Schemas)   │
└──────────────┴──────────────┘
```

---

## 🤝 Contributing

We welcome contributions from the Solana community! Areas we need help:

- 🐛 **Bug Reports** - Found an issue? Open a GitHub issue
- 📝 **Documentation** - Improve guides and examples
- ✨ **Features** - Implement roadmap items
- 🧪 **Testing** - Add edge case coverage
- 🎨 **Examples** - Create real-world schemas
- 🌍 **Community** - Share LUMOS with developers

**See individual repos for contribution guidelines.**

---

## 🗺️ Roadmap

- ✅ **Phase 1** - Core TypeScript ↔ Rust codegen
- ✅ **Phase 2** - CLI & developer tools
- ✅ **Phase 3.1** - Enum support (Unit, Tuple, Struct variants)
- ✅ **Phase 3.2** - VSCode extension
- 📋 **Phase 3.3** - PDA helpers, instruction generation, validation
- 🌍 **Phase 4** - Multi-language support (C++, Python, Go)

**See [ROADMAP.md](https://github.com/getlumos/lumos/blob/main/ROADMAP.md) for detailed plans.**

---

## 📊 Project Status

| Metric | Value |
|--------|-------|
| **Tests Passing** | 64/64 (100%) |
| **Published Crates** | 2 (lumos-core, lumos-cli) |
| **Latest Version** | 0.1.0 |
| **Rust Version** | 1.70+ |
| **Example Schemas** | 5 production-ready |
| **Community Examples** | 53 types, 42 instructions |
| **CI Status** | ✅ All checks passing |

---

## 🌐 Resources

- **Website:** [lumos-lang.org](https://lumos-lang.org)
- **Documentation:** [docs-lumos](https://github.com/getlumos/docs-lumos)
- **Examples:** [awesome-lumos](https://github.com/getlumos/awesome-lumos)
- **VSCode Extension:** [vscode-lumos](https://github.com/getlumos/vscode-lumos)
- **Crates.io:** [lumos-core](https://crates.io/crates/lumos-core) | [lumos-cli](https://crates.io/crates/lumos-cli)

---

## 📄 License

All repositories are dual-licensed under **Apache 2.0** or **MIT** (your choice).

---

<div align="center">

**Built with dedication for the Solana developer community**

⭐ **Star our repos** if you find LUMOS useful!

**[Get Started](https://github.com/getlumos/lumos)** • **[Read Docs](https://lumos-lang.org)** • **[Join Discussions](https://github.com/getlumos/lumos/discussions)**

---

*Empowering developers to build faster, safer, and smarter Solana applications*

</div>
