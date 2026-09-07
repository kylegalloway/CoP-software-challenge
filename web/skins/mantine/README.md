# mantine

**Concept:** a CSS-only homage to the [Mantine](https://github.com/mantinedev/mantine) React component library's visual language — palette, rounded cards, subtle shadows, pill badges — with zero actual dependency on Mantine or React (per `PLAN.md`, this is look-only).

## Palette

- Primary (Mantine's default `indigo.6`): `#4C6EF5`.
- Secondary accents: teal `#12B886` (e.g. a "confirmed" indicator), soft red `#FA5252` (e.g. an alert state).
- Background: `#FFFFFF` (cards) on `#F8F9FA` (page).
- Border/dividers: `#DEE2E6`.
- Secondary/muted text: `#868E96`.
- Body text: `#212529`.

## Typography

- Font: `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif`.
- Body size: `16px`, `line-height: 1.55`.
- Headings: `28px`/`700` weight for the page title, `20px`/`600` for card/section titles — bolder and slightly larger than `modern`'s equivalents.

## Layout

- Card border-radius: `16px` (noticeably rounder than `modern`'s `8px`).
- Card shadow: `box-shadow: 0 1px 3px rgba(0,0,0,0.05), 0 8px 24px rgba(0,0,0,0.06)` — soft and diffuse, not a hard drop shadow.
- Badge/pill: `border-radius: 9999px`, `padding: 2px 10px`, `font-size: 12px`, colored background (e.g. teal `#12B886` at low opacity) with matching-hue text.
- Input focus ring: `box-shadow: 0 0 0 2px rgba(76,110,245,0.35)` (primary indigo at reduced opacity).
- Card padding: `20px`; gap between cards: `16px`.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — Mantine's own `Card` component commonly ships with an optional image section, so this maps directly onto the library's real pattern.

- Image sits at the top of the card as a `Card.Section`-style block: full card width, `aspect-ratio: 16 / 9`, `object-fit: cover`, rounded only on the top two corners (`border-radius: 16px 16px 0 0`) to match the card's own `16px` radius.
- No-image fallback: a flat `#F1F3F5` panel at the same `16 / 9` aspect ratio with a centered muted icon/glyph (a simple camera or film-strip glyph, `#ADB5BD`) — mirrors Mantine's own image-placeholder pattern rather than leaving a gap.
- Title: `18px`/`600`, `padding: 16px 20px 0`, sits directly below the image section.
- Quote/detail text: `14px`, `#495057`, `padding: 8px 20px 20px`, `line-height: 1.5`.
- Badge (year, if present): positioned top-right, overlapping the image slightly (`position: absolute`, `top: 12px`, `right: 12px`) using the pill treatment from Layout above — a very characteristic Mantine `Card` + `Badge` combination.

## Empty state

Framed like a component-library "empty state" pattern demo: a centered, friendly message with generous whitespace around it, maybe inside its own soft-shadowed card rather than plain text — e.g. "No entries for this date yet." in a muted grey, styled the way a design system's own empty-state example component would look.

## Why this reinforces the identity

The tell that separates this from `modern` is *roundedness + soft shadow + pill badges used consistently everywhere*, echoing a component library's default look rather than a hand-tuned product design. If it starts looking flat or sharp-cornered, it's drifted back into `modern`'s territory.
