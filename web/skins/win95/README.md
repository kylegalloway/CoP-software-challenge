# win95

**Concept:** desktop + dialog window — a nod to the original challenge mockup (`challenge.png`), rendering the whole page as a classic Windows 95 application window sitting on a desktop.

## Palette

- Window chrome/face: `#C0C0C0`.
- Bevel light edge (top/left, raised): `#FFFFFF`; bevel dark edge (bottom/right, raised): `#808080`; outer shadow line: `#000000`. Invert light/dark edges for "pressed"/sunken elements.
- Title bar gradient: `#000080` → `#1084D0` (`linear-gradient(to right, ...)`), title text `#FFFFFF`.
- Content area background: `#FFFFFF` (or `#ECE9D8` for a secondary panel), text `#000000`.
- Desktop background (page margin behind the window): `#008080` (Win95's teal).

## Typography

- Font: `Tahoma, "MS Sans Serif", sans-serif`.
- Base size: `11px`–`12px` — small and dense throughout.
- Title bar text: `12px`, bold, white.
- No other weight/size variation — borders and fill color carry all the visual hierarchy.

## Layout

- Window border: `2px` bevel using the light/dark edge colors above.
- Title bar height: `18px`–`20px`, button glyphs (`_ □ ✕`) at `10px`–`12px`, right-aligned.
- Content padding: `8px`, inset border (`2px`, dark edge on top/left, light on bottom/right) around the date picker and entries list.
- Max window width: `480px`–`640px`, centered on the teal desktop background.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — the fallback should look like a genuine OS placeholder, not a broken link.

- Each entry renders as its own small "dialog pane": `border: 2px` inset bevel (dark edge top/left, light edge bottom/right — sunken, like a content well), `background: #FFFFFF`, `padding: 6px`, `margin-bottom: 8px`.
- Image: `64px × 64px`, `border: 1px solid #000000` plus an outer `1px solid #808080` (double-border, echoing an old thumbnail/icon frame), `object-fit: cover`, positioned left with `margin-right: 8px`.
- No-image fallback: a `64px × 64px` grey box (`#C0C0C0`, same as window chrome) with a small centered CSS-drawn "broken image" icon substitute (a simple mountain/picture glyph via a couple of `div`s or an inline SVG, not an external asset) — should read as "Windows couldn't load this," in-universe rather than as an actual bug.
- Title: `12px`, bold, `#000080` (matches the title-bar blue) — sits top-right of the image.
- Quote/detail text: `11px`, `#000000`, wraps beneath the title within the remaining width next to the image.

## Buttons and inputs

Any interactive element (the date input, the skin `<select>`) should get the beveled 3D button treatment consistent with the OS — flat modern inputs would break the illusion immediately. This is the skin where getting the bevel direction right (raised = interactive/idle, sunken = pressed/active) matters most for selling the reference.

## Empty state

Framed as a system dialog: something like "No data found." with a small CSS-drawn icon substitute (a triangle "!" or "i" in a circle, inline SVG or CSS shapes only — no external icon assets) mimicking a Windows message box.

## Why this reinforces the identity

This skin is the most literal callback in the set — it's referencing `challenge.png` directly — so precision on the bevel/chrome details (not just "grey and blocky") is what makes it land as *specifically* Windows 95 rather than generic retro-OS pastiche.
