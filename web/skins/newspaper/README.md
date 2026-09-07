# newspaper

**Concept:** broadsheet serif, multi-column layout, "Extra! Extra!" energy — print journalism rendered on the web.

## Palette

- Background: `#F4F1E9` (warm cream, not pure white).
- Body text: `#1A1A1A` (near-black ink, not pure `#000000`).
- Spot color (breaking-news flourish, year badge): `#8B0000` (dark red) — the only color in the palette, used sparingly.
- Rule lines (masthead, dividers): `#1A1A1A`, `1px`–`2px` solid.

## Typography

- Headline font: a slab or traditional serif — `Georgia, "Times New Roman", serif` at `32px`–`44px`, bold, `letter-spacing: -0.01em` (tight).
- Body font: a distinct, denser serif if available (`"Times New Roman", Georgia, serif` at a smaller size than headlines reads fine even without a second family) — `15px`–`16px`, `line-height: 1.5`.
- Body alignment: `text-align: justify; hyphens: auto;`.
- Section labels/bylines: `text-transform: uppercase`, `letter-spacing: 0.08em`, `12px`.

## Layout

- Multi-column body: `column-count: 2` (or `column-width: 300px` for a fluid column count), `column-gap: 24px` — collapses to one column under ~600px automatically via multicol's own behavior.
- Masthead: centered, `36px`–`48px` bold serif, flanked by `2px solid #1A1A1A` rule lines above and below, `16px` vertical spacing from the rules.
- Edition-date line: small, centered, `12px`, uppercase, directly beneath the masthead.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — a photo credit line covers the gap gracefully.

- Image: `filter: grayscale(1) contrast(1.1)` to match newsprint's black-and-white register (a color photo would break the ink-on-paper illusion). Width `100%` of its column, `aspect-ratio: 4 / 3`, `object-fit: cover`.
- Caption: `11px`, italic, `#1A1A1A` at `70%` opacity, centered beneath the image, `margin-top: 4px` — styled like a real photo credit/cutline. Use the entry's title text here rather than duplicating it as a separate heading.
- No-image fallback: skip the image slot for that entry and let the headline run as the top of the column — this is exactly how real papers handle photo-less stories, so no placeholder box is needed here (unlike most other skins).
- Headline (title): sits above or beside the image depending on column width — `20px`–`24px` bold serif, tighter than the masthead but bigger than body text.
- Body/quote text: flows in the multi-column layout as described above, wrapping around the image if the column is wide enough (`float: left` on the image with `margin: 0 16px 8px 0` is a period-accurate "text wraps the photo" treatment).

## Empty state

Framed as a wire-service non-story: something dry and matter-of-fact like "No news on this date." — newspapers don't apologize for a quiet day, they just report it.

## Why this reinforces the identity

The signature move is the *masthead + multi-column + justified serif body* combination — any one of those alone reads as "generic old-timey," but together they specifically read as "newspaper." Avoid warm decorative flourishes (those belong to `blogspot`) — this skin's personality is authority and formality, not personal voice.
