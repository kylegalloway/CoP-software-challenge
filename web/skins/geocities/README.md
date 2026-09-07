# geocities

**Concept:** table-layout web, marquee text, hit counter, "under construction" gifs — the free-hosting personal-page era before CSS layout was trusted for anything.

## Palette

- Background: hot magenta `#FF00FF` or teal `#00FFCC` — pick one, then tile a small repeating background image over it (stars/flames pattern, `background-repeat: repeat`, a `40px`–`80px` tile size).
- Links: unvisited `#0000FF`, visited `#800080` — literal browser defaults, never overridden.
- Body text: whatever survives contrast against the tile — `#000000` on the magenta option, `#FFFFFF` on a darker tile.
- Heading colors: cycle 3–4 saturated hues per heading, word by word — `#FF0000`, `#FFFF00`, `#00FF00`, `#00FFFF` is a period-accurate rotation.
- Borders (table-cell look): `2px outset #C0C0C0` / `2px inset #C0C0C0`.

## Typography

- Font: `"Times New Roman", serif` for body copy.
- Body size: `16px`–`18px`, default `line-height` (don't set one — unstyled is the point).
- Headings: `28px`–`36px`, bold, centered (`text-align: center` everywhere, including body copy).
- Blink-approximation: `animation: blink 1s steps(1) infinite` toggling `opacity: 0`/`1` — use on no more than one element, sparingly.

## Layout

- Section "cells": `border: 2px outset #C0C0C0`, `padding: 8px`, around each of date picker / skin picker / entries list.
- Hit counter: static text like `000482`, monospace/digital-looking font (`"Courier New", monospace`), small `12px`–`14px`, bottom of page.
- Marquee substitute: `white-space: nowrap; overflow: hidden;` container with a child `animation: marquee 12s linear infinite` sliding `translateX(100%)` → `translateX(-100%)`, text like "WELCOME TO MY PAGE — SIGN MY GUESTBOOK".

## Entry card (title + image)

`image_url` is nullable per [ADR-0006](../../../docs/decisions/0006-entry-images.md) — never assume every entry has one.

- Each entry is its own table-cell box: `border: 2px inset #C0C0C0`, `padding: 6px`, `background: #FFFFFF` (a plain rectangle interrupting the tiled page background, like an old "table with a white cell" trick).
- Image: capped at `150px` wide, `border: 2px solid #000000` and a thick drop shadow substitute (`box-shadow: 3px 3px 0 #000000`, hard-edged, no blur — blur reads too modern). Center it above the title, never beside it.
- No-image fallback: a static "under construction" style placeholder — a bordered box with centered `12px` text reading something like "IMAGE COMING SOON", same `150px` footprint so layout doesn't shift entry to entry.
- Title: `20px`, bold, one of the rotating heading colors from the palette above, centered beneath the image.
- Quote text: `14px`–`16px` body serif, centered, `margin-top: 4px`.
- Gap between stacked entry cells: `12px`, each cell width capped around `200px`–`240px` if laid out in a row (mimicking an old table `<td>` width).

## Empty state

Per [ADR-0004](../../../docs/decisions/0004-empty-state-per-skin.md): "Nothing here yet! Sign our guestbook" — in keeping with the theme, this should look like a plea rather than a neutral system message, maybe with a little construction-worker icon substitute (CSS-drawn or an inline SVG, not an external image, to keep the footprint promise).

## Why this reinforces the identity

The joke only lands if the visual noise feels *earnest* rather than ironic-minimalist — every choice above should read as "somebody was proud of this in 1998," not "somebody making fun of 1998." Constraint reminder: no actual `<table>`/`<marquee>`/`<font>` tags exist in the shared markup (`core.js`'s DOM is fixed and semantic) — every effect here is CSS-only, and any per-skin motion gimmick belongs in `skin.js`, never in something the app depends on for correctness.
