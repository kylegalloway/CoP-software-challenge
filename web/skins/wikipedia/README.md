# wikipedia

**Concept:** infobox, citations, talk-page framing — encyclopedic neutrality, and a natural pairing with the site's real voting/moderation mechanic (per `PLAN.md`).

## Palette

- Background: `#FFFFFF`.
- Links: `#0645AD` (unvisited), `#0B0080` desaturated or `#795CB2` (visited).
- Borders/table lines: `#A2A9B1`.
- Header/infobox shading: `#EAECF0` (section headers), `#F8F9FA` (infobox background).
- Body text: `#202122`.

## Typography

- Font: `sans-serif` system stack (Wikipedia's own is close to `-apple-system, "Helvetica Neue", Arial, sans-serif`).
- Body size: `14px`, `line-height: 1.6`.
- Section headings: `20px`, `border-bottom: 1px solid #A2A9B1`, `padding-bottom: 4px`.
- Citation markers: `font-size: 11px`, `vertical-align: super`, in the link-blue color, wrapped in literal `[1]`-style brackets.

## Layout

- Infobox: `width: 280px` (or `~30%` on wide viewports), floated or grid-placed to one side, `border: 1px solid #A2A9B1`, `background: #F8F9FA`, `padding: 8px`, label/value rows each `padding: 4px 0` with a `1px solid #EAECF0` row divider.
- Article body max-width: `640px`–`720px`.
- References footer: numbered list, `12px`, `padding-top: 16px`, `border-top: 1px solid #A2A9B1`.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — Wikipedia articles without a lead image are completely normal, so no fallback placeholder is needed.

- Image lives inside the infobox as the "lead image": `width: 100%` of the infobox's `280px`, `object-fit: cover`, capped height `~200px`, `border: 1px solid #A2A9B1`.
- Caption beneath the image, inside the infobox: `11px`, `#54595D`, centered, `padding: 4px`.
- Title (the movie) sits as the infobox's top row, styled like an article's infobox title bar: `16px` bold, centered, `background: #EAECF0`, `padding: 6px`.
- No-image fallback: infobox simply starts with the title bar and skips straight to the label/value rows — exactly how a real stub article's infobox looks.
- Quote/detail text stays in the main article-body column (not the infobox), formatted as regular article prose with its citation marker per the Typography section above.

## Talk-page framing (pairs with voting)

Since Wikipedia's actual differentiator is *editorial process*, this skin is the natural place to make the collaborative-corpus/voting mechanic (once built) visually legible — e.g. framing vote counts or moderation status the way a talk-page discussion or an article's "citation needed" banner would. Even before that's wired up, leave visual room (a muted banner-style element) for it.

## Empty state

Should read like an actual "no article" state: something like "No Wikipedia article exists for this date range." with the calm, slightly bureaucratic tone Wikipedia uses for its own missing-page and disambiguation messaging.

## Why this reinforces the identity

Citation markers and the infobox pattern are the two moves that make this unmistakably "Wikipedia" rather than generic "clean informational site" (which would just be `modern` again) — lean hard into both, since together they also happen to reinforce the app's real "these are sourced facts" premise better than any other skin can.
