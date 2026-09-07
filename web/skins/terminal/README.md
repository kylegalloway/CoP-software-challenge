# terminal

**Concept:** green-on-black monospace, fake typing animation — the CRT terminal / hacker-movie shell aesthetic.

## Palette

Pure black background, phosphor green foreground (`#33FF33`/`#00FF41`-family) as the single dominant color — this skin should feel almost monochrome, unlike `teletext`'s multi-color broadcast palette. A very subtle CRT glow can be simulated with `text-shadow` (soft green blur behind the same green text) rather than introducing a second hue. If any accent color is needed (e.g. the year badge), stay within green's range (a slightly desaturated or amber-tinted variant) rather than reaching for red/blue — breaking the monochrome rule undercuts the whole bit.

## Typography

Monospace throughout, no exceptions — this is the one skin where that's load-bearing rather than a nice-to-have. Consider a scanline or slight text "flicker" via CSS animation (very subtle, low-opacity, so it reads as atmosphere rather than becoming distracting or a11y-hostile). Command-prompt framing: entries can be prefixed with a `>` or `$` glyph as if each were a line of output from a query the user just "ran."

## Layout

Minimal chrome — no boxes, borders, or cards; the illusion is a raw terminal buffer, so content should flow top-to-bottom as plain lines of monospace text with generous line-height, not grouped into visually separated widgets. The date picker and skin `<select>` should be restyled to look like inline prompt input rather than GUI form controls (remove native chrome via `appearance: none` and re-skin minimally, keeping them functionally identical form elements).

## Typing animation gimmick

The other strong candidate (alongside `aim`/`myspace`) for a `skin.js` gimmick: animate the entry text appearing character-by-character as if being typed/output live, using a `setInterval`/`requestAnimationFrame` reveal over the *already-fetched* data — never delaying or blocking the real fetch, since `skin.js` must stay presentation-only per `AGENTS.md`. The page should be fully readable and functional immediately if `skin.js` fails to load; the typing effect is a bonus animation over already-rendered content, not a gate on it.

## Empty state

Terse and system-message-like: something like `no results for query.` in lowercase, matching a shell's typical error tone — no punctuation flourish, no apology.

## Why this reinforces the identity

Discipline is the design principle here: one color, one font, no decorative boxes. Every other skin in the set leans into visual noise or branding; this one earns its identity by being the starkest, quietest option in the picker.
