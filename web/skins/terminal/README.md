# terminal

**Concept:** green-on-black monospace, fake typing animation — the CRT terminal / hacker-movie shell aesthetic.

## Palette

- Background: `#0D0D0D` (near-black, slightly softer than pure `#000000`).
- Foreground/dominant color: `#33FF33` (phosphor green).
- Glow: `text-shadow: 0 0 2px #33FF33, 0 0 6px rgba(51,255,51,0.4)` — same hue, no second color.
- Accent (year badge, if needed): `#66FF99` (a lighter/desaturated tint of the same green) — never red/blue.

## Typography

- Font: `"Courier New", "SF Mono", Consolas, monospace` throughout, no exceptions.
- Base size: `15px`–`16px`, `line-height: 1.6` (generous, terminal-buffer spacing).
- Prompt prefix per entry: `> ` or `$ ` glyph before each line.
- Flicker animation (optional, subtle): `animation: flicker 4s infinite`, opacity oscillating `1` → `0.96` — keep the range under 5% to avoid being distracting.

## Layout

- No borders, boxes, or cards anywhere — content is plain stacked lines.
- Vertical spacing between entries: `12px`–`16px`.
- Page padding: `24px`–`32px` on all sides, left-aligned text (no centering).
- Form controls: `appearance: none`, background `transparent`, `border-bottom: 1px solid #33FF33` only — no box chrome.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)). A full-color photo breaks this skin's one-color discipline — force it to obey the same constraint as everything else.

- Image: small, `48px × 48px`, `filter: grayscale(1) brightness(0.9) sepia(1) hue-rotate(70deg) saturate(4)` — this chain pushes any photo toward monochrome-green, keeping the image inside the phosphor-green palette rather than breaking it.
- No image fallback: skip the image slot entirely — just show `[no image]` as literal bracketed text, `#33FF33`, inline after the title, matching the terse system-message voice used elsewhere in this skin.
- Placement: image sits inline to the left of the title line (not stacked, not a separate block) — `display: inline-block`, `vertical-align: middle`, `margin-right: 8px` — so the "no decorative boxes" rule holds; it reads as part of the text line, not a card element.
- Title and quote stay exactly as described in Layout above (plain stacked monospace lines, `> `/`$ ` prefix) — the image never gets its own border, shadow, or background.

## Typing animation gimmick

The other strong candidate (alongside `aim`/`myspace`) for a `skin.js` gimmick: animate the entry text appearing character-by-character as if being typed/output live, using a `setInterval`/`requestAnimationFrame` reveal over the *already-fetched* data — never delaying or blocking the real fetch, since `skin.js` must stay presentation-only per `AGENTS.md`. The page should be fully readable and functional immediately if `skin.js` fails to load; the typing effect is a bonus animation over already-rendered content, not a gate on it.

## Empty state

Terse and system-message-like: something like `no results for query.` in lowercase, matching a shell's typical error tone — no punctuation flourish, no apology.

## Why this reinforces the identity

Discipline is the design principle here: one color, one font, no decorative boxes. Every other skin in the set leans into visual noise or branding; this one earns its identity by being the starkest, quietest option in the picker.
