# geocities

**Concept:** table-layout web, marquee text, hit counter, "under construction" gifs — the free-hosting personal-page era before CSS layout was trusted for anything.

## Palette

Loud and untuned on purpose: a bright non-web-safe-adjacent background (hot magenta or teal, "because I could pick a color") clashing with a tiled background image texture (stars, flames, or a subtle repeating pattern), pure `#0000FF`/`#800080` link blue/purple straight from browser defaults (never overridden — overriding link colors was an advanced move nobody bothered with), and text color chosen for contrast with the tile rather than any palette logic. Nothing here should look chosen by a designer; it should look chosen by someone excited to have *a* color option at all.

## Typography

Times New Roman or the default serif for body copy (browsers didn't ship with better defaults yet), with `<font>`-era oversized, multicolored headings — different words in a heading rendered in different colors, a `<blink>`-adjacent treatment even though `<blink>` itself won't fire in modern browsers (approximate with a CSS animation toggling `visibility` or `opacity`, sparingly, as a nod rather than actual eye strain). Centered alignment everywhere; text-align: left would look "unfinished" in this idiom.

## Layout

Everything nested in table-like boxes — even though the real markup is `core.js`'s fixed semantic shell, the CSS should visually fake table cells with visible borders (`border: 2px outset` / `inset` for that beveled table-cell look) around each section (date picker, skin picker, entries list). A hit counter widget (pure CSS/generated-content, static incrementing-looking number, never actually counting anything real) belongs near the footer. A marquee-style horizontal scroll of text (CSS `overflow` + `animation`, since `<marquee>` won't render as one either) announcing something like "WELCOME TO MY PAGE — SIGN MY GUESTBOOK" reinforces the era without needing `skin.js`.

## Empty state

Per [ADR-0004](../../../docs/decisions/0004-empty-state-per-skin.md): "Nothing here yet! Sign our guestbook" — in keeping with the theme, this should look like a plea rather than a neutral system message, maybe with a little construction-worker icon substitute (CSS-drawn or an inline SVG, not an external image, to keep the footprint promise).

## Why this reinforces the identity

The joke only lands if the visual noise feels *earnest* rather than ironic-minimalist — every choice above should read as "somebody was proud of this in 1998," not "somebody making fun of 1998." Constraint reminder: no actual `<table>`/`<marquee>`/`<font>` tags exist in the shared markup (`core.js`'s DOM is fixed and semantic) — every effect here is CSS-only, and any per-skin motion gimmick belongs in `skin.js`, never in something the app depends on for correctness.
