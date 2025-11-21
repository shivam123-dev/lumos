# LUMOS Issue Templates

This directory contains GitHub issue templates for the LUMOS ecosystem.

## Templates Overview

### 🐛 Bug Report (`bug_report.yml`)
For reporting bugs in any LUMOS component (core, CLI, VSCode, docs, examples).

**Auto-applies labels:** `type:bug`, `status:triage`

### ✨ Feature Request (`feature_request.yml`)
For suggesting new features across the LUMOS ecosystem.

**Auto-applies labels:** `type:feature`, `status:triage`

### 🔌 VSCode Extension Issue (`vscode_extension.yml`)
For VSCode extension-specific issues (syntax highlighting, commands, snippets).

**Auto-applies labels:** `area:vscode`, `status:triage`

**Note:** Issues created here are tracked centrally but reference the [vscode-lumos](https://github.com/getlumos/vscode-lumos) repository.

### 📖 Documentation Improvement (`documentation.yml`)
For documentation improvements, fixes, or additions.

**Auto-applies labels:** `area:docs`, `type:documentation`, `status:triage`

**Note:** Tracks work for [docs-lumos](https://github.com/getlumos/docs-lumos) and lumos-lang.org.

### 💡 Example Request (`example_request.yml`)
For requesting new production examples for awesome-lumos.

**Auto-applies labels:** `area:examples`, `type:feature`, `status:triage`

**Note:** Tracks work for [awesome-lumos](https://github.com/getlumos/awesome-lumos).

## Configuration

### `config.yml`
Configures the issue template menu and contact links:
- Disables blank issues (forces template use)
- Links to Discussions, Documentation, and related repositories

## Label System

LUMOS uses a comprehensive label system for issue management:

### Area Labels (Component)
- `area:core` - Core compiler (parser, generator, IR)
- `area:cli` - CLI tool
- `area:vscode` - VSCode extension
- `area:examples` - awesome-lumos
- `area:docs` - Documentation
- `area:ecosystem` - Cross-repo initiatives
- `area:wasm` - WASM support
- `area:ci` - CI/CD infrastructure

### Type Labels (Classification)
- `type:bug` - Something broken
- `type:feature` - New functionality
- `type:enhancement` - Improvement
- `type:refactor` - Code refactoring
- `type:documentation` - Docs
- `type:question` - Help needed
- `type:security` - Security issue
- `type:performance` - Performance

### Status Labels (Workflow)
- `status:triage` - Needs review
- `status:confirmed` - Verified
- `status:in-progress` - Being worked on
- `status:blocked` - Blocked
- `status:need-info` - Waiting for info
- `status:duplicate` - Duplicate
- `status:wontfix` - Won't fix
- `status:stale` - Inactive

### Priority Labels (Urgency)
- `priority:critical` - Breaking functionality
- `priority:high` - Important soon
- `priority:medium` - Normal timeline
- `priority:low` - Nice to have

### Special Labels
- `good-first-issue` - For newcomers
- `help-wanted` - Community welcome
- `breaking-change` - API breaking
- `needs-discussion` - Needs discussion
- `needs-tests` - Needs tests
- `needs-docs` - Needs docs
- `upstream` - Upstream dependency
- `research` - Needs investigation

## Setting Up Labels

To create all labels in your repository:

```bash
# Make sure you're authenticated with gh CLI
gh auth status

# Run the label setup script
bash .github/scripts/setup-labels.sh
```

This will:
1. Create all 36 labels with proper colors and descriptions
2. Remove conflicting default GitHub labels
3. Display a summary of created labels

## Usage for Contributors

When creating an issue:

1. Click "New Issue"
2. Select the appropriate template from the menu
3. Fill out all required fields (marked with *)
4. The template will auto-apply initial labels
5. Maintainers will add priority/status labels during triage

## Usage for Maintainers

### Triage Process
1. New issue arrives with `status:triage`
2. Review and verify the issue
3. Add appropriate `area:*` labels if missing
4. Add `priority:*` label based on impact
5. Change status:
   - `status:confirmed` if valid
   - `status:need-info` if more info needed
   - `status:duplicate` if duplicate
   - `status:wontfix` if won't be addressed

### Workflow Tracking
- Mark `status:in-progress` when work begins
- Mark `status:blocked` if blocked
- Close issue when resolved (status auto-becomes closed)

### Cross-Repo Coordination
For issues affecting multiple repositories:
- Create issue in `getlumos/lumos` (central hub)
- Add `area:*` label for affected component
- Reference in specific repo if needed: "Tracking getlumos/lumos#123"

## Customization

To modify templates:
1. Edit the `.yml` files in this directory
2. Test by creating a draft issue
3. Commit changes to `dev` branch
4. Merge to `main` when ready

Templates use GitHub's [issue forms syntax](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms).

## Ecosystem Structure

```
getlumos/lumos (main repo)
├── Issue tracking (centralized here)
├── Labels (comprehensive system)
└── Templates (all ecosystems)

getlumos/vscode-lumos
└── Technical PRs → Reference lumos#123 for tracking

getlumos/awesome-lumos
└── Example additions → Reference lumos#123 for tracking

getlumos/docs-lumos
└── Content updates → Reference lumos#123 for tracking
```

## Questions?

- 💬 [GitHub Discussions](https://github.com/getlumos/lumos/discussions)
- 📚 [Documentation](https://lumos-lang.org)

---

**Last Updated:** 2025-11-21
**Maintained by:** LUMOS Core Team
