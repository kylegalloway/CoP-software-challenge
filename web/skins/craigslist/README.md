# craigslist

**Concept:** plain blue-link brutalism — Craigslist's famously unchanged, function-over-form listings page.

## Palette

As minimal as this whole set gets: white background, default/system blue links (`#0000EE`-ish, unvisited), visited-link purple left mostly to browser default rather than heavily overridden, plain black body text, and a single grey hairline (`#CCCCCC`-ish) for dividers. No accent color at all — if anything needs to stand out (the year badge, say), a bold weight or bracket notation (`[2015]`) does the job instead of introducing color.

## Typography

Default system/serif-or-sans stack at a small size, left-aligned, single column, dense line-height — Craigslist has never invested in typographic hierarchy and this skin shouldn't either. Headings differentiate from body text mainly through bold weight, not size or color.

## Layout

A plain top navigation strip listing categories-as-metaphor (could reuse the skin names or date ranges as fake "category" links, purely decorative, non-functional, so it doesn't compete with the real skin-picker `<select>`) exactly like Craigslist's actual site-wide top bar. Entries render as a flat list of plain-text lines, each looking like a classifieds listing: title as a bare blue link, quote/detail as plain black text directly after it on the same or next line, no card, no border, no padding beyond what's needed for line separation. The date picker should look like an unstyled native `<input>` — actively resist customizing it, since an unstyled native control *is* the Craigslist aesthetic.

## Empty state

As terse as Craigslist itself: literally something like "no results" — no punctuation, no personality, lowercase. This is the one skin where under-designing the empty state is the correct design decision.

## Why this reinforces the identity

Every other skin in this set adds decoration; this one earns its identity entirely by *withholding* it — the challenge is resisting the urge to add a single polish touch. If a reviewer says "this looks unfinished," that's this skin working as intended, provided the "finished" skins (`modern`, `wikipedia`) exist right next to it in the picker for contrast.
