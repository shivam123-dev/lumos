# LUMOS Official Logo Assets

> **Write once. Deploy Everywhere.**

**Design:** Radiant Precision - Purple gradient circle with gold rays radiating outward
**Concept:** Symbolizes illumination (LUMOS = Latin for "light") and code generation radiating from a single source
**Colors:** Purple (#9333EA → #581C87) + Gold (#FACC15 → #CA8A04)
**Style:** Modern, minimal, geometric, developer-friendly

---

## Files

| File | Size | Dimensions | Use Case |
|------|------|------------|----------|
| `logo.png` | 192 KB | 800×800 | **Primary source** - Main logo, GitHub social preview, README headers |
| `logo-512.png` | 84 KB | 512×512 | Medium resolution - Social media, marketing |
| `logo-256.png` | 28 KB | 256×256 | Documentation sites, blog posts |
| `logo-128.png` | 12 KB | 128×128 | VSCode extension icon, app icons |
| `logo-64.png` | 4 KB | 64×64 | Small icons, toolbars |
| `logo-32.png` | 4 KB | 32×32 | Favicon size, tiny icons |

---

## Usage Guidelines

### GitHub Repository

**Social preview (Settings → Social preview):** Use `logo-512.png`

**README header:**
```markdown
<div align="center">
  <img src="assets/logo/logo.png" alt="LUMOS Logo" width="200"/>
</div>
```

### Documentation Sites

**Favicon:** Use `logo-32.png` or `logo-64.png`

**Header logo:** Use `logo-256.png` or `logo.png`

**Open Graph meta:** Use `logo-512.png`

### VSCode Extension

**Main icon (package.json):** Use `logo-128.png` as `icon.png`

**Size variants:** Provide 32, 64, 128, 512 px versions

### npm/crates.io Package

**Package icon:** Use `logo-256.png` or `logo-512.png`

### Social Media

**Twitter/X avatar:** Use `logo-512.png`

**LinkedIn:** Use `logo-512.png`

**Discord server icon:** Use `logo-256.png` or `logo-512.png`

---

## Design Specifications

### Color Palette

**Primary Purple Gradient:**
- Start: `#9333EA` (vibrant purple)
- End: `#581C87` (deep purple)

**Accent Gold Gradient:**
- Start: `#FACC15` (bright gold)
- End: `#CA8A04` (warm gold)

### Visual Elements

1. **Central circle** - Core identity (16px radius at 64px scale)
2. **8 radiating rays** - Light/code emanating outward (8 cardinal directions)
3. **Inner ring** - Gold highlight (12px radius at 64px scale)
4. **Center dot** - Focal point (4px radius at 64px scale)
5. **Glow effect** - Subtle Gaussian blur for depth

### Design Philosophy

- **Illumination:** Light representing clarity and understanding
- **Precision:** Geometric shapes for type-safety and correctness
- **Duality:** Purple (Rust) + Gold (TypeScript/Solana) bridged together
- **Simplicity:** Works at any size (16px favicon to 1024px poster)

---

## File Formats

### PNG (All Sizes)
- Pre-rendered for optimal display at specific dimensions
- Transparent background
- Use when SVG isn't supported or specific size needed

---

## Updating the Logo

**Source of Truth:** This directory (`lumos/assets/logo/`) in the main repository

**To update logos across all repos:**

1. Update source files here
2. Regenerate sizes if needed:
   ```bash
   cd lumos/assets/logo
   sips -z 512 512 logo.png --out logo-512.png
   sips -z 256 256 logo.png --out logo-256.png
   sips -z 128 128 logo.png --out logo-128.png
   sips -z 64 64 logo.png --out logo-64.png
   sips -z 32 32 logo.png --out logo-32.png
   ```
3. Copy to other repos:
   - `docs-lumos/src/assets/`
   - `vscode-lumos/` (as icon*.png)
   - `awesome-lumos/` (if needed)

---

## Brand Consistency

✅ **Always use the official logo files from this directory**
✅ **Do not modify colors, proportions, or design elements**
✅ **Maintain clear space around logo (minimum 16px padding)**
✅ **Use transparent background (PNG) or original colors**
❌ **Do not stretch, skew, or distort the logo**
❌ **Do not add effects, shadows, or filters**
❌ **Do not use on cluttered or low-contrast backgrounds**

---

## License

Same as LUMOS project: MIT OR Apache-2.0

---

**Last Updated:** 2025-11-21
**Maintained By:** RECTOR (@rz1989s)
