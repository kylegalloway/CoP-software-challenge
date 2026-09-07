# teletext

**Concept:** Ceefax/teletext — the blocky, low-resolution broadcast-TV information service aesthetic, page-numbered and grid-locked.

## Palette

Teletext's fixed 8-color CLUT is the whole design system here: black background, and text/blocks in saturated primary red, green, yellow, blue, magenta, cyan, and white — no gradients, no in-between shades, no anti-aliasing feel (crisp edges, flat fills only). Color should be used *semantically* the way real teletext did: e.g. yellow for headers, cyan or green for body text, red for an alert/highlight (maybe the "year badge" for `specific_year` entries), never mixed arbitrarily.

## Typography

Monospace, blocky, and grid-aligned — teletext text rendered in a fixed character grid (traditionally 40x25), so even though this is a fluid webpage, the type should feel locked to a monospace rhythm (a monospace font stack, consistent character width implied by generous, uniform letter-spacing). No italics, no varied weights — teletext had one "font," essentially.

## Layout

Strong grid-line discipline: sections separated by full-width solid color bars rather than whitespace or soft dividers, mimicking teletext's page-header/page-footer color bands. A fake "page number" indicator (e.g. "P104") in a corner, since teletext services were always navigated by page number — this can be a static decorative string keyed to nothing real, purely atmospheric. The date picker and skin picker can be presented like teletext's own navigation row of colored keys ("red: index, green: next," etc. — reinterpreted here as color-coded buttons/labels).

## Empty state

Should read like an actual broadcast error: "NO DATA FOUND — PAGE 404" (as already suggested in [ADR-0004](../../../docs/decisions/0004-empty-state-per-skin.md)) rendered in the alert color (red or yellow-on-red) with the same blocky monospace treatment as everything else, not a softened "sorry" message.

## Why this reinforces the identity

The temptation is to make this skin "just monospace and dark" — the actual signature move is the *fixed, saturated palette used with strict semantic consistency* plus full-width color bands instead of margins/padding for separation. If it looks like a terminal, it's drifted into the `terminal` skin's territory instead — teletext is broadcast-television-flavored, brighter and more colorful, not a shell prompt.
