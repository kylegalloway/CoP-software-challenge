# mantine

**Concept:** a CSS-only homage to the [Mantine](https://github.com/mantinedev/mantine) React component library's visual language — palette, rounded cards, subtle shadows, pill badges — with zero actual dependency on Mantine or React (per `PLAN.md`, this is look-only).

## Palette

Mantine's signature indigo/violet-leaning primary color (a mid-saturation blue-violet, roughly Mantine's default `indigo` swatch) against a very light neutral background, with Mantine's characteristic light/dark neutral greys for secondary text and borders. Unlike `modern`'s single restrained accent, this skin can lean into Mantine's fuller swatch system feel — a couple of secondary accent colors (a teal or a soft red) used for state (e.g. the year badge in one accent, an interactive hover state in another) similar to how Mantine's component demos often show multiple color variants side by side.

## Typography

A clean geometric sans-serif (Mantine's default stack leans on system fonts too, so this can overlap with `modern` — the differentiation is mostly structural/component-driven, not typographic). Slightly larger, bolder headings than `modern` uses, with consistent, generous heading-to-body size ratios matching component-library defaults.

## Layout

This is the most "component library demo page" skin in the set: entries render as distinct rounded-corner cards (`border-radius` noticeably larger than `modern`'s, Mantine's default radius scale is quite rounded) with a soft, diffuse box-shadow (not the sharp bevels of `win95`) rather than the flat divided list `modern` uses. Badges (the year badge, a "confirmed" indicator) should use full pill shapes (`border-radius: 9999px`, small padding, colored background with contrasting text) — Mantine's `Badge` component is the direct visual reference. The date and skin pickers can be styled as if they were Mantine's `Select`/`DateInput` components: rounded, bordered, with a visible focus ring in the primary accent color.

## Empty state

Framed like a component-library "empty state" pattern demo: a centered, friendly message with generous whitespace around it, maybe inside its own soft-shadowed card rather than plain text — e.g. "No entries for this date yet." in a muted grey, styled the way a design system's own empty-state example component would look.

## Why this reinforces the identity

The tell that separates this from `modern` is *roundedness + soft shadow + pill badges used consistently everywhere*, echoing a component library's default look rather than a hand-tuned product design. If it starts looking flat or sharp-cornered, it's drifted back into `modern`'s territory.
