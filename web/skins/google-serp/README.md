# google-serp

**Concept:** the fact rendered as a mock Google search-engine-results page — the entry(s) presented the way a search snippet or featured-answer box would show them.

## Palette

Google's actual restrained palette: white background, the specific link blue (`#1A0DAB`), the muted green/grey URL-breadcrumb color (`#006621`/`#4D5156`-family) beneath each result title, and near-black body text — nothing else. Like `wikipedia`, this palette exists to disappear in service of a very recognizable layout, not to express personality through color.

## Typography

Google's system sans-serif stack (Arial/system-ui-adjacent), specific and consistent sizing: a larger blue "title" line per entry, a small green URL line beneath it, then a smaller grey/black snippet line — this three-line result pattern *is* the design, and getting the relative sizing/color right matters more than any other single decision in this skin.

## Layout

Render the date query itself as if it were the search box content (e.g. a disabled/read-only-styled input showing something like `"[date]" movie reference` as the "searched" term, purely decorative framing around the real date-picker control) with a fake results-count/timing line ("About 3 results (0.24 seconds)" — decorative text, not a real timer) above the results. Each entry renders as one search result block: bolded/blue title (the movie), green fake URL breadcrumb (can be synthesized from the real `source_url`'s domain, which is honest data rather than invented), and a snippet line built from the entry's `quote`. If a date has exactly one strong match, consider a "featured snippet" box treatment (bordered card at the top) for it — echoing Google's answer-box pattern.

## Empty state

The most literal to write: mimic Google's actual "did not match any documents" messaging — something like "Your search - [date] - did not match any documents." in the same grey, slightly apologetic tone, possibly with a fake "Suggestions:" list styled but non-functional.

## Why this reinforces the identity

This skin's entire joke depends on precision, not vibes — anyone who's used Google will immediately clock small deviations from the real title/URL/snippet color and size relationships. Resist adding decoration Google itself doesn't have (no drop shadows, no rounded cards beyond what a real SERP uses) — starkness and restraint sell the reference.
