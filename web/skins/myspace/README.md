# myspace

**Concept:** Top 8, glitter cursor, profile-song era — maximalist personal-branding web, every profile a competing aesthetic statement.

## Palette

Deliberately clashing and personal rather than systematic: a hot pink / black combo is the obvious default, but the point is it should feel like *a specific person's* profile, not a neutral brand palette — pick one saturated primary (pink, or an alternate like electric blue) against black or a busy background image, with glitter-text treatment on headings (CSS gradient + text-shadow tricks simulating sparkle, no actual animated GIF asset needed to keep footprint down).

## Typography

Ornate/script display font for headings if a suitable web-safe or system font exists (fall back gracefully — no external font loading, which would blow the footprint budget), body text small and dense. Text can be presented at a slight rotation or with drop shadows on headers — the "I customized my CSS myself, badly, with pride" look, not clean alignment.

## Layout

A "Top 8"-style grid should stand in for something the app actually has data for — e.g. render the date's entries in a small fixed-size grid of cards rather than a plain list, each with a thick colored border, echoing the Top-8 friend grid without inventing fake friends. The date picker and skin picker can sit inside a "profile info" styled box (bordered, slightly tilted, drop-shadowed) rather than a plain form.

## Glitter cursor gimmick

A candidate for the one allowed `skin.js` gimmick: a CSS-cursor swap or a lightweight trailing-sparkle effect on mousemove, purely decorative and disabled without breaking any real functionality if JS fails to load (progressive enhancement — the page must still work with `skin.js` absent, per the skins-are-presentation-only rule in `AGENTS.md`).

## Empty state

Voice-y and a little dramatic: something like "Nothing today... my profile is SO boring right now 😩" (era-appropriate emoticon/expressiveness over a flat message) — matching MySpace's confessional-teenager tone.

## Why this reinforces the identity

MySpace skins were defined by *inconsistency* — that's the joke to lean into: intentionally slightly-off spacing, one too many drop shadows, a font pairing nobody would recommend. Resist the urge to make it tasteful; tasteful is the opposite of MySpace.
