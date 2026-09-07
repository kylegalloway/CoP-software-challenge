# modern

**Concept:** the "normal best-practice SaaS site" baseline — the skin every other skin is a deliberate departure *from*. Also the default (`registry.json`'s fallback slug, per `core.js`), so it doubles as the no-skin-selected experience.

## Palette

- Background: `#FAFAFA` (page) / `#FFFFFF` (cards/content area).
- Body text: `#1A1A1A` — near-black, not pure `#000000` (too harsh at this scale).
- Secondary/muted text (dates, labels): `#6B7280`.
- Accent (links, active states, year badge): `#2563EB` (a mid-saturation "trustworthy SaaS" blue), hover/active at `#1D4ED8`.
- Borders/dividers: `#E5E7EB`.
- Card elevation (if used at all): `box-shadow: 0 1px 3px rgba(0,0,0,0.08)` — no gradients, nothing heavier.

## Typography

- Font stack: `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif`.
- Body size: `16px`, `line-height: 1.5`.
- Small/secondary text (dates, badges): `14px`.
- Headings: `24px`/`600` weight for the page title, `18px`/`600` for section labels — differentiate by weight, not color.
- Letter-spacing: default (`0`) everywhere — no tracking tricks.

## Layout

- Content max-width: `640px`, centered (`margin: 0 auto`).
- Page padding: `24px` on mobile, `48px`+ on wider viewports.
- Vertical rhythm: `16px` between list items, `32px` between major sections.
- Border-radius: `8px` on any card/container — enough to read as "designed," not `win95`'s beveled `0` or `mantine`'s pill-shaped `9999px`.
- Dividers between entries: `1px solid #E5E7EB`, not full card borders.

## Entry card (title + image)

Per [ADR-0006](../../../docs/decisions/0006-entry-images.md), `image_url` is nullable and lazy-loaded — every card must look intentional with or without one.

- Layout: image left, text right on wide viewports (`display: flex`, image `flex: 0 0 96px`, text `flex: 1`); stack image-on-top-of-text below `~480px`.
- Image box: `96px × 96px`, `object-fit: cover`, `border-radius: 8px` (matches the card radius).
- No-image fallback: a flat `#EAECF0` placeholder box at the same `96px × 96px` size — never collapse the layout when `image_url` is `null`, so hit/no-image entries don't visually jump.
- Title: `16px`/`600` weight, sits above the quote text with `4px` gap.
- Quote/detail text: `14px`, `#6B7280`, `line-height: 1.5`, max `3` lines with `text-overflow: ellipsis` if needed.
- Gap between image and text block: `16px`.

## Empty state

Plain and friendly: something like "No notable dates today — check back tomorrow." No irony, no bit — this skin's whole point is to be the sincere, un-gimmicked baseline that makes the other twelve skins' choices legible as choices.

## Why this reinforces the identity

This is the control group. Every decision here should be justifiable by "what would a competent product designer actually ship," with zero references to a specific era or platform — if this skin starts feeling *nostalgic* or *ironic*, it's drifted out of its lane. It also needs to look genuinely fine on its own — since it's both the fallback and a real menu option, it can't read as a placeholder.
