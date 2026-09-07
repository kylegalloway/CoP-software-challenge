# myspace

**Concept:** Top 8, glitter cursor, profile-song era — maximalist personal-branding web, every profile a competing aesthetic statement.

## Palette

- Background: `#000000`, or a busy tiled pattern image over black.
- Primary accent: `#FF1493` (hot pink) — or swap for electric blue `#00BFFF`, pick one and commit.
- Glitter-text heading treatment: base color `#FFFFFF` or the accent, layered `text-shadow: 0 0 4px #FF1493, 0 0 8px #FF69B4, 0 0 12px #FFFFFF` to fake sparkle without an image asset.
- Card borders (Top 8 grid): `4px solid` in the accent color, or alternate accent/black per card.

## Typography

- Heading font: a rounded/script system font if available (`"Comic Sans MS", cursive` is period-accurate and zero-cost — no external font loading), `20px`–`28px`.
- Body text: `12px`–`13px`, dense line-height (`1.2`–`1.3`).
- Header treatment: slight rotation `transform: rotate(-2deg)` to `rotate(3deg)` per heading, plus `text-shadow: 2px 2px 0 #000000` for a drop-shadow-on-pink-background look.

## Layout

- Top-8 grid: `grid-template-columns: repeat(4, 1fr)` (or `repeat(2, 1fr)` on narrow viewports), `gap: 8px`, fixed card aspect (roughly `1:1` to `4:3`).
- Card border: `4px solid` accent, `border-radius: 0` (sharp corners — MySpace didn't round things).
- Profile-info box (pickers): `border: 3px solid #FFFFFF`, slight `transform: rotate(-1deg)`, `box-shadow: 4px 4px 0 #000000`.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — the Top-8 grid conceit depends on every card looking full either way.

- Each grid cell is one entry: image fills the cell as a "profile picture," `aspect-ratio: 1 / 1`, `object-fit: cover`, full-bleed to the card's `4px solid` accent border (no internal padding around the image).
- No-image fallback: a solid-color tile in the accent color (`#FF1493` or the chosen alternate) with a large centered `🎬` emoji or a bold `?`-style glyph — reads as "customized placeholder," not a broken-image icon.
- Title overlays the bottom of the image: `position: absolute`, `background: rgba(0,0,0,0.6)`, white text, `12px` bold, `padding: 4px 6px`, full width of the card — the classic "caption overlaid on a Top-8 photo" look.
- Quote/detail text does not appear on the grid card itself (keep the card visual-first); if shown at all, put it in a tooltip/expand-on-click, not inline — the grid's job is images, not paragraphs.

## Glitter cursor gimmick

A candidate for the one allowed `skin.js` gimmick: a CSS-cursor swap or a lightweight trailing-sparkle effect on mousemove, purely decorative and disabled without breaking any real functionality if JS fails to load (progressive enhancement — the page must still work with `skin.js` absent, per the skins-are-presentation-only rule in `AGENTS.md`).

## Empty state

Voice-y and a little dramatic: something like "Nothing today... my profile is SO boring right now 😩" (era-appropriate emoticon/expressiveness over a flat message) — matching MySpace's confessional-teenager tone.

## Why this reinforces the identity

MySpace skins were defined by *inconsistency* — that's the joke to lean into: intentionally slightly-off spacing, one too many drop shadows, a font pairing nobody would recommend. Resist the urge to make it tasteful; tasteful is the opposite of MySpace.
