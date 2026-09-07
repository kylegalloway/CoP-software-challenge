# google-serp

**Concept:** the fact rendered as a mock Google search-engine-results page — the entry(s) presented the way a search snippet or featured-answer box would show them.

## Palette

- Background: `#FFFFFF`.
- Title link: `#1A0DAB`.
- URL breadcrumb: `#006621` (older style) or `#4D5156`/`#202124` (URL text, modern style) — pick one and use it consistently.
- Snippet/body text: `#4D5156`.
- Result-count/timing line: `#70757A`.

## Typography

- Font: `Arial, sans-serif` (or `-apple-system, sans-serif` as a close modern fallback).
- Title line: `20px`, regular weight, blue.
- URL breadcrumb line: `14px`, green or grey per above.
- Snippet line: `14px`, `line-height: 1.4`, body-grey.
- Result-count line: `13px`, muted grey, sits above the results.

## Layout

Render the date query itself as if it were the search box content (e.g. a disabled/read-only-styled input showing something like `"[date]" movie reference` as the "searched" term, purely decorative framing around the real date-picker control) with a fake results-count/timing line ("About 3 results (0.24 seconds)" — decorative text, not a real timer) above the results. Each entry renders as one search result block: bolded/blue title (the movie), green fake URL breadcrumb (can be synthesized from the real `source_url`'s domain, which is honest data rather than invented), and a snippet line built from the entry's `quote`. If a date has exactly one strong match, consider a "featured snippet" box treatment (bordered card at the top) for it — echoing Google's answer-box pattern.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — a real SERP entry without an image thumbnail is the common case, so no image is the default look, not a broken one.

- Image: only when present, small thumbnail to the right of the text block (like a knowledge-panel/rich-result thumbnail) — `92px × 92px`, `object-fit: cover`, `border-radius: 8px`, `margin-left: 16px`.
- No-image fallback: the result block simply uses the full row width for title/URL/snippet text (a plain three-line text result) — this is the more common Google result shape, so it should feel like the default, not a degraded state.
- Title/URL/snippet stack stays exactly as in Typography/Layout above, `flex: 1` against the optional thumbnail so the text column reflows to full width when there's no image.
- Never crop or distort the thumbnail to force a "with image" look on entries that don't have one — the honesty of the plain-text result is part of what sells the reference.

## Empty state

The most literal to write: mimic Google's actual "did not match any documents" messaging — something like "Your search - [date] - did not match any documents." in the same grey, slightly apologetic tone, possibly with a fake "Suggestions:" list styled but non-functional.

## Why this reinforces the identity

This skin's entire joke depends on precision, not vibes — anyone who's used Google will immediately clock small deviations from the real title/URL/snippet color and size relationships. Resist adding decoration Google itself doesn't have (no drop shadows, no rounded cards beyond what a real SERP uses) — starkness and restraint sell the reference.
