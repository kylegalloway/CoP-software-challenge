# craigslist

**Concept:** plain blue-link brutalism — Craigslist's famously unchanged, function-over-form listings page.

## Palette

- Background: `#FFFFFF`.
- Links (unvisited): `#0000EE` — plain browser default, don't override.
- Links (visited): leave as browser default (`#551A8B`-ish) rather than setting explicitly.
- Body text: `#000000`.
- Divider hairline: `#CCCCCC`, `1px`.
- No accent color — a stand-out value like the year badge uses `font-weight: bold` or bracket notation (`[2015]`) instead of color.

## Typography

- Font: system default (`Arial, Helvetica, sans-serif` or simply unset/inherit).
- Base size: `13px`–`14px`, left-aligned, single column.
- Line-height: `1.3`–`1.4` (dense, no generous spacing).
- Headings: same size as body text, `font-weight: bold` only — no size or color jump.

## Layout

A plain top navigation strip listing categories-as-metaphor (could reuse the skin names or date ranges as fake "category" links, purely decorative, non-functional, so it doesn't compete with the real skin-picker `<select>`) exactly like Craigslist's actual site-wide top bar. Entries render as a flat list of plain-text lines, each looking like a classifieds listing: title as a bare blue link, quote/detail as plain black text directly after it on the same or next line, no card, no border, no padding beyond what's needed for line separation. The date picker should look like an unstyled native `<input>` — actively resist customizing it, since an unstyled native control *is* the Craigslist aesthetic.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — most real Craigslist listings have no thumbnail at all, so this is the expected default, not a gap to fill.

- Image, when present: tiny, unstyled thumbnail before the listing line — `48px × 48px`, `object-fit: cover`, `border: 1px solid #CCCCCC` (the only border in this skin, matching real Craigslist's minimal thumbnail treatment), `margin-right: 6px`, inline with the text (`vertical-align: middle`), not a separate block.
- No-image fallback: nothing rendered at all — no placeholder box, no icon — the text listing simply starts flush left. An empty `image_url` should look identical to a listing that was never given one.
- Title: stays the plain blue link exactly as described in Typography above, immediately following the thumbnail (or, if no thumbnail, starting the line).
- Quote/detail text: plain black text directly after the title on the same or next line — no visual distinction introduced by the presence of an image.

## Empty state

As terse as Craigslist itself: literally something like "no results" — no punctuation, no personality, lowercase. This is the one skin where under-designing the empty state is the correct design decision.

## Why this reinforces the identity

Every other skin in this set adds decoration; this one earns its identity entirely by *withholding* it — the challenge is resisting the urge to add a single polish touch. If a reviewer says "this looks unfinished," that's this skin working as intended, provided the "finished" skins (`modern`, `wikipedia`) exist right next to it in the picker for contrast.
