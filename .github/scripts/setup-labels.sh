#!/bin/bash
# LUMOS Issue Label Setup Script
# Run this script to create all labels for the LUMOS ecosystem
# Usage: bash .github/scripts/setup-labels.sh

set -e

# Check GitHub CLI authentication
if ! gh auth status &>/dev/null; then
  echo "❌ Error: GitHub CLI not authenticated"
  echo ""
  echo "Please authenticate with GitHub CLI first:"
  echo "  gh auth login"
  echo ""
  echo "Then run this script again."
  exit 1
fi

echo "🏷️  Setting up LUMOS issue labels..."
echo ""

# Color palette
BLUE="0052CC"
PURPLE="5319E7"
GREEN="0E8A16"
CYAN="1D76DB"
GRAY="6E7781"
RED="D73A4A"
ORANGE="D93F0B"
YELLOW="FBCA04"
PINK="E99695"
LIGHT_GREEN="BFD4F2"

echo "📍 Creating AREA labels (component-specific)..."

# Area labels - Which component is affected?
gh label create "area:core" \
  --description "Core compiler (parser, generator, IR, transform)" \
  --color "$BLUE" \
  --force

gh label create "area:cli" \
  --description "CLI tool (lumos-cli commands)" \
  --color "$BLUE" \
  --force

gh label create "area:vscode" \
  --description "VSCode extension (syntax, commands, snippets)" \
  --color "$PURPLE" \
  --force

gh label create "area:examples" \
  --description "awesome-lumos repository (production examples)" \
  --color "$GREEN" \
  --force

gh label create "area:docs" \
  --description "Documentation (lumos-lang.org, guides)" \
  --color "$CYAN" \
  --force

gh label create "area:ecosystem" \
  --description "Cross-repo initiatives and ecosystem-wide features" \
  --color "$GRAY" \
  --force

gh label create "area:wasm" \
  --description "WASM/browser playground support" \
  --color "$PINK" \
  --force

gh label create "area:ci" \
  --description "CI/CD, GitHub Actions, build infrastructure" \
  --color "$GRAY" \
  --force

echo "✅ Area labels created"
echo ""

echo "📝 Creating TYPE labels (issue classification)..."

# Type labels - What kind of issue?
gh label create "type:bug" \
  --description "Something isn't working correctly" \
  --color "$RED" \
  --force

gh label create "type:feature" \
  --description "New feature or functionality" \
  --color "$GREEN" \
  --force

gh label create "type:enhancement" \
  --description "Improvement to existing feature" \
  --color "$LIGHT_GREEN" \
  --force

gh label create "type:refactor" \
  --description "Code refactoring without behavior change" \
  --color "$ORANGE" \
  --force

gh label create "type:documentation" \
  --description "Documentation improvements or additions" \
  --color "$CYAN" \
  --force

gh label create "type:question" \
  --description "Question or request for help" \
  --color "$PURPLE" \
  --force

gh label create "type:security" \
  --description "Security vulnerability or concern" \
  --color "$RED" \
  --force

gh label create "type:performance" \
  --description "Performance optimization or issue" \
  --color "$YELLOW" \
  --force

echo "✅ Type labels created"
echo ""

echo "🔄 Creating STATUS labels (workflow tracking)..."

# Status labels - What's happening with this issue?
gh label create "status:triage" \
  --description "Needs initial review and prioritization" \
  --color "$GRAY" \
  --force

gh label create "status:confirmed" \
  --description "Issue verified and ready to work on" \
  --color "$GREEN" \
  --force

gh label create "status:in-progress" \
  --description "Currently being worked on" \
  --color "$YELLOW" \
  --force

gh label create "status:blocked" \
  --description "Blocked by another issue or dependency" \
  --color "$RED" \
  --force

gh label create "status:need-info" \
  --description "Waiting for more information from reporter" \
  --color "$YELLOW" \
  --force

gh label create "status:duplicate" \
  --description "Duplicate of another issue" \
  --color "$GRAY" \
  --force

gh label create "status:wontfix" \
  --description "Will not be fixed or implemented" \
  --color "$GRAY" \
  --force

gh label create "status:stale" \
  --description "Inactive for extended period" \
  --color "$GRAY" \
  --force

echo "✅ Status labels created"
echo ""

echo "⚡ Creating PRIORITY labels (urgency)..."

# Priority labels - How urgent is this?
gh label create "priority:critical" \
  --description "Critical issue breaking core functionality" \
  --color "$RED" \
  --force

gh label create "priority:high" \
  --description "High priority, should be addressed soon" \
  --color "$ORANGE" \
  --force

gh label create "priority:medium" \
  --description "Medium priority, normal timeline" \
  --color "$YELLOW" \
  --force

gh label create "priority:low" \
  --description "Low priority, nice to have" \
  --color "$GREEN" \
  --force

echo "✅ Priority labels created"
echo ""

echo "🎯 Creating SPECIAL labels (community/workflow)..."

# Special labels
gh label create "good-first-issue" \
  --description "Good for newcomers to the project" \
  --color "$GREEN" \
  --force

gh label create "help-wanted" \
  --description "Community help and contributions welcome" \
  --color "$CYAN" \
  --force

gh label create "breaking-change" \
  --description "Introduces breaking API changes" \
  --color "$RED" \
  --force

gh label create "needs-discussion" \
  --description "Requires community discussion before implementation" \
  --color "$PURPLE" \
  --force

gh label create "needs-tests" \
  --description "Requires additional test coverage" \
  --color "$YELLOW" \
  --force

gh label create "needs-docs" \
  --description "Requires documentation updates" \
  --color "$CYAN" \
  --force

gh label create "upstream" \
  --description "Depends on upstream dependency changes" \
  --color "$GRAY" \
  --force

gh label create "research" \
  --description "Requires research and investigation" \
  --color "$PURPLE" \
  --force

echo "✅ Special labels created"
echo ""

echo "🗑️  Removing default GitHub labels that conflict..."

# Remove default labels that we're replacing
gh label delete "bug" --yes 2>/dev/null || true
gh label delete "enhancement" --yes 2>/dev/null || true
gh label delete "documentation" --yes 2>/dev/null || true
gh label delete "question" --yes 2>/dev/null || true
gh label delete "invalid" --yes 2>/dev/null || true
gh label delete "wontfix" --yes 2>/dev/null || true
gh label delete "duplicate" --yes 2>/dev/null || true

echo "✅ Cleanup complete"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All labels created successfully!"
echo ""
echo "📊 Label Summary:"
echo "   • Area labels: 8 (component organization)"
echo "   • Type labels: 8 (issue classification)"
echo "   • Status labels: 8 (workflow tracking)"
echo "   • Priority labels: 4 (urgency levels)"
echo "   • Special labels: 8 (community/workflow)"
echo "   • Total: 36 labels"
echo ""
echo "🔗 View all labels:"
echo "   https://github.com/getlumos/lumos/labels"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
