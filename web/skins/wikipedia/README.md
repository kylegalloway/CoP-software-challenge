# wikipedia

**Concept:** infobox, citations, talk-page framing — encyclopedic neutrality, and a natural pairing with the site's real voting/moderation mechanic (per `PLAN.md`).

## Palette

Wikipedia's actual restrained palette: white/very-light-grey background, the specific muted blue for links (`#0645AD`-family, visited state slightly desaturated), light grey for infobox/table borders and header shading (`#EAECF0`/`#F8F9FA`-family) — nothing saturated or decorative. This is a palette in service of legibility and citation-density, not branding.

## Typography

A standard serif or Wikipedia's own sans-serif system stack for body text, small and information-dense, with a very clear heading hierarchy (larger, bottom-bordered section headings, echoing Wikipedia's classic underlined `<h2>` styling). Superscript citation markers (`[1]`, `[2]`-style, CSS-generated or literal characters) next to entries with a `source_url`/`secondary_source_url` are a strong, cheap signature move.

## Layout

An infobox-style sidebar or top box summarizing the current date at a glance (styled like Wikipedia's right-aligned infobox: bordered table look, label/value rows) — this is a great home for the sitewide `stats.global_total_confirmed_entries` count, framed like an infobox stat. The entries list itself can read like an article body, each entry a short "paragraph" with its citation marker, and a footer area styled like a References section listing the source URLs as a numbered list (real `<a>` links to `source_url`/`secondary_source_url`, not decorative).

## Talk-page framing (pairs with voting)

Since Wikipedia's actual differentiator is *editorial process*, this skin is the natural place to make the crowdsourcing/voting mechanic (once built) visually legible — e.g. framing vote counts or moderation status the way a talk-page discussion or an article's "citation needed" banner would. Even before that's wired up, leave visual room (a muted banner-style element) for it.

## Empty state

Should read like an actual "no article" state: something like "No Wikipedia article exists for this date range." with the calm, slightly bureaucratic tone Wikipedia uses for its own missing-page and disambiguation messaging.

## Why this reinforces the identity

Citation markers and the infobox pattern are the two moves that make this unmistakably "Wikipedia" rather than generic "clean informational site" (which would just be `modern` again) — lean hard into both, since together they also happen to reinforce the app's real "these are sourced facts" premise better than any other skin can.
