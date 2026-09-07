# teletext

**Concept:** Ceefax/teletext — the blocky, low-resolution broadcast-TV information service aesthetic, page-numbered and grid-locked.

## Palette

The classic teletext 8-color CLUT, used exactly, no in-between shades:

- Black: `#000000` (background)
- White: `#FFFFFF`
- Red: `#FF0000` (alerts, empty-state)
- Green: `#00FF00` (body text)
- Yellow: `#FFFF00` (headers)
- Blue: `#0000FF`
- Magenta: `#FF00FF`
- Cyan: `#00FFFF` (secondary body text)

No gradients, no opacity/blend tricks — every fill is a flat, full-opacity block from this list.

## Typography

- Font: `"Courier New", "Lucida Console", monospace`.
- Base size: `16px`, `letter-spacing: 0.05em` (generous, uniform — approximates a fixed character grid).
- Line-height: `1.4`.
- No italics, no font-weight variation — use color changes for emphasis instead.

## Layout

- Header/footer color bands: full-width blocks, `padding: 8px 16px`, background yellow (`#FFFF00`) with black text for headers.
- Page-number indicator: top-right corner, `12px`, e.g. `P104`, in cyan or white.
- Section dividers: `4px` solid color bars (not whitespace) — alternate colors per section (e.g. yellow header band, green content band, red footer band).

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)). Real teletext couldn't show photos at all (block-graphics only) — lean into that tension rather than hiding it.

- Image: `80px × 80px`, `image-rendering: pixelated` and `filter: saturate(1.4) contrast(1.1)` to fake a chunky, low-res broadcast feed rather than a clean photo. `border: 4px solid #FFFFFF` (a thick white "teletext block" frame).
- No-image fallback: a solid color-block grid pattern (a few `16px` squares alternating 2–3 palette colors, CSS `background: repeating-conic-gradient(...)` or a few nested divs) at the same `80px × 80px` — reads as "this page has no picture data," in-universe.
- Title: `16px`, bold, yellow (`#FFFF00`) on a black band directly beneath the image, `padding: 4px 8px`.
- Quote/detail text: `14px`, green (`#00FF00`) or cyan (`#00FFFF`), left-aligned beside or beneath the image+title block, same monospace treatment as body text elsewhere.
- Card border: `2px solid #FFFFFF` around the whole entry, full-width color band separating entries (per Layout above) rather than a soft gap.

## Empty state

Should read like an actual broadcast error: "NO DATA FOUND — PAGE 404" (as already suggested in [ADR-0004](../../../docs/decisions/0004-empty-state-per-skin.md)) rendered in the alert color (red or yellow-on-red) with the same blocky monospace treatment as everything else, not a softened "sorry" message.

## Why this reinforces the identity

The temptation is to make this skin "just monospace and dark" — the actual signature move is the *fixed, saturated palette used with strict semantic consistency* plus full-width color bands instead of margins/padding for separation. If it looks like a terminal, it's drifted into the `terminal` skin's territory instead — teletext is broadcast-television-flavored, brighter and more colorful, not a shell prompt.
