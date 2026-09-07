# modern

**Concept:** the "normal best-practice SaaS site" baseline — the skin every other skin is a deliberate departure *from*. Also the default (`registry.json`'s fallback slug, per `core.js`), so it doubles as the no-skin-selected experience.

## Palette

Restrained and neutral: a near-white or very light grey background, dark neutral grey/near-black for body text (not pure black — pure black-on-white is slightly harsh at this scale), and a single accent color (a mid-saturation blue or similar "trustworthy SaaS" hue) used sparingly for interactive elements and the year badge. No gradients, no drop shadows beyond a very subtle card elevation. This is the palette that should look like it came from a design system, because in-universe, it did.

## Typography

A clean system sans-serif stack (the kind of `-apple-system, "Segoe UI", sans-serif` stack real products ship), moderate line-height, comfortable body text size (not cramped like `aim`, not oversized like `geocities`). Headings differentiate by weight and size, not by color — restraint is the entire visual argument this skin is making.

## Layout

Generous whitespace, a single-column centered content area with a comfortable max-width (so lines of text don't stretch edge-to-edge on wide viewports), light card/section separation (subtle border or very soft shadow, not the beveled 3D look of `win95`/`aim`). The date picker and skin picker sit in a clean horizontal control row; entries render as a simple vertical list with light dividers, not boxed cards — approachable rather than dense.

## Empty state

Plain and friendly: something like "No notable dates today — check back tomorrow." No irony, no bit — this skin's whole point is to be the sincere, un-gimmicked baseline that makes the other twelve skins' choices legible as choices.

## Why this reinforces the identity

This is the control group. Every decision here should be justifiable by "what would a competent product designer actually ship," with zero references to a specific era or platform — if this skin starts feeling *nostalgic* or *ironic*, it's drifted out of its lane. It also needs to look genuinely fine on its own — since it's both the fallback and a real menu option, it can't read as a placeholder.
