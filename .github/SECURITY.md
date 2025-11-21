# Security Policy

## Supported Versions

We actively support the following versions of LUMOS with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

We take security seriously and appreciate your efforts to responsibly disclose vulnerabilities. If you discover a security issue in LUMOS, please report it privately.

### How to Report

**Email:** security@lumos-lang.org *(preferred method)*

**GitHub Security Advisories:** [Report a vulnerability](https://github.com/getlumos/lumos/security/advisories/new)

### What to Include

Please provide as much information as possible:

- **Type of vulnerability** (e.g., code injection, buffer overflow, authentication bypass)
- **Component affected** (core compiler, CLI, VSCode extension, etc.)
- **Version affected** (run `lumos --version`)
- **Step-by-step reproduction** (detailed instructions to reproduce)
- **Proof of concept** (code, schema, or commands demonstrating the issue)
- **Impact assessment** (what can an attacker achieve?)
- **Suggested fix** (if you have one)

### Response Timeline

- **Initial response:** Within 48 hours
- **Confirmation/triage:** Within 5 business days
- **Fix timeline:** Depends on severity (see below)
- **Public disclosure:** After fix is released (coordinated disclosure)

### Severity Levels

| Severity | Response Time | Examples |
|----------|---------------|----------|
| **Critical** | 24-48 hours | Remote code execution, arbitrary file write |
| **High** | 7 days | Privilege escalation, authentication bypass |
| **Medium** | 30 days | Information disclosure, DoS |
| **Low** | 90 days | Minor issues with limited impact |

### What to Expect

1. **Acknowledgment** - We'll confirm receipt of your report
2. **Investigation** - We'll verify and assess the vulnerability
3. **Fix Development** - We'll develop and test a patch
4. **Coordinated Disclosure** - We'll work with you on disclosure timing
5. **Credit** - We'll credit you in the security advisory (if you wish)

### Bug Bounty

We currently do not offer a bug bounty program. However, we deeply appreciate security researchers' contributions and will:

- Credit you in the security advisory and changelog
- Acknowledge your contribution publicly (if you wish)
- Fast-track any related feature requests or improvements

## Security Best Practices

When using LUMOS:

### For Developers

1. **Validate Input** - Always validate `.lumos` schema files before processing
2. **Sandbox Execution** - Run `lumos generate` in controlled environments
3. **Review Generated Code** - Audit generated Rust/TypeScript before production use
4. **Keep Updated** - Use the latest LUMOS version for security patches
5. **Dependency Audit** - Regularly audit dependencies (`cargo audit`)

### For Schema Authors

1. **Avoid Sensitive Data** - Don't include secrets in schema files
2. **Namespace Collisions** - Use unique type names to avoid conflicts
3. **Generated Code Review** - Always review generated code before committing
4. **Type Safety** - Leverage LUMOS type system to prevent runtime errors

### For CLI Users

1. **Trust Source** - Only generate code from trusted schema files
2. **Check Permissions** - Ensure generated files have appropriate permissions
3. **Backup Before Generate** - Keep backups before running `lumos generate`
4. **Verify Output** - Inspect generated code for unexpected patterns

## Known Security Considerations

### Code Generation

LUMOS generates Rust and TypeScript code from `.lumos` schemas. This process:

- ✅ **Does not execute arbitrary code** during parsing
- ✅ **Validates syntax** using battle-tested `syn` parser
- ✅ **Sanitizes identifiers** to prevent injection
- ⚠️ **Generates code** that should be reviewed before production use

### File System Operations

The CLI tool:

- Reads `.lumos` schema files
- Writes generated `.rs` and `.ts` files
- Does NOT modify system files outside the target directory
- Respects file permissions and ownership

### Dependencies

We regularly audit dependencies for security vulnerabilities:

```bash
# Check for known vulnerabilities
cargo audit

# Update dependencies
cargo update

# View dependency tree
cargo tree
```

## Security Updates

Security updates are released via:

1. **GitHub Security Advisories** - [View advisories](https://github.com/getlumos/lumos/security/advisories)
2. **Release Notes** - Tagged with `[SECURITY]` prefix
3. **crates.io** - Updated packages (lumos-core, lumos-cli)
4. **RSS Feed** - Subscribe to releases

## Contact

- **Security Email:** security@lumos-lang.org
- **General Issues:** [GitHub Issues](https://github.com/getlumos/lumos/issues)
- **Discussions:** [GitHub Discussions](https://github.com/getlumos/lumos/discussions)

## Acknowledgments

We thank the security research community for helping keep LUMOS and its users safe.

### Hall of Fame

Security researchers who have responsibly disclosed vulnerabilities:

*(No vulnerabilities reported yet)*

---

**Last Updated:** 2025-11-21
**Policy Version:** 1.0
