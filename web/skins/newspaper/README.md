# newspaper

**Concept:** broadsheet serif, multi-column layout, "Extra! Extra!" energy — print journalism rendered on the web.

## Palette

Newsprint-adjacent: off-white/cream background (never pure white — real newsprint has warmth and slight texture), pure black text, no color at all in the primary palette — this is a black-and-white-with-cream skin by design, evoking ink on paper. If any color appears, it should be a single restrained red (as in a "breaking news" masthead flourish or the year badge), used the way old papers used a spot color, never a full palette.

## Typography

A strong serif for headlines (a slab or traditional serif, bold, large, tight letter-spacing) contrasted with a smaller, denser serif or even a different serif for body copy — real newspapers pair a display serif with a distinct body serif. Justified text alignment in the body column is a strong signal of the genre (careful with `hyphens` for ragged justification at narrow widths). Small caps or all-caps treatment for section labels/bylines.

## Layout

Multi-column text flow wherever the content allows it (CSS `column-count`/`column-width` on the entries list, if there's enough content to fill more than one column meaningfully — collapse to single-column gracefully on narrow viewports since CSS multicol handles that natively). A masthead-style header treatment for the site title (large, centered, with rule lines above/below, like a paper's nameplate). The date picker can be framed as the "edition date" line papers print near the masthead; the skin picker can sit unobtrusively, maybe styled like small print/classifieds.

## Empty state

Framed as a wire-service non-story: something dry and matter-of-fact like "No news on this date." — newspapers don't apologize for a quiet day, they just report it.

## Why this reinforces the identity

The signature move is the *masthead + multi-column + justified serif body* combination — any one of those alone reads as "generic old-timey," but together they specifically read as "newspaper." Avoid warm decorative flourishes (those belong to `blogspot`) — this skin's personality is authority and formality, not personal voice.
