# blogspot

**Concept:** early-2000s personal blog — sidebar widgets, mood indicators, "Powered by Blogger"-era warmth and clutter.

## Palette

- Background: `#FFF8F0` (warm cream) — or swap for pale blue `#EAF2FB` / pale pink `#FDEEF4`, pick one.
- Accent (links, headers): `#C1440E` (dusty orange) — or `#7B5EA7` (muted purple) as an alternate.
- Sidebar widget background: a slightly darker/complementary tint of the main background, e.g. `#F5E9DC` against the `#FFF8F0` main area.
- Body text: `#3A3A3A` (soft dark grey, not full black — keeps the warmth).

## Typography

- Body font: `Verdana, Geneva, sans-serif` or `Georgia, serif` — either reads period-appropriate; pick one and use it everywhere (don't mix families).
- Body size: `14px`, `line-height: 1.6`.
- Post title: `22px`–`26px`, colored in the accent, `text-decoration: underline` optional.
- Sidebar widget headers: `13px`, bold, uppercase, `letter-spacing: 0.03em`.

## Layout

- Two-column grid: main content `~65%` width, sidebar `~35%`, `gap: 24px` (`grid-template-columns: 2fr 1fr` is a close approximation).
- Sidebar widget box: `padding: 12px`, `border: 1px solid` a slightly darker shade of the sidebar tint, `border-radius: 4px`, `margin-bottom: 16px` between widgets.
- Main content padding: `16px`–`24px`.

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)).

- Image: sits above the post title (blogs of this era put the photo at the top of the post body), `max-width: 100%`, capped height `~240px`, `object-fit: cover`, `border: 4px solid #FFFFFF` plus a thin outer `1px solid` the accent color — mimicking an inserted-into-a-post-editor photo frame.
- No-image fallback: skip straight to the title — no placeholder box needed, since plenty of real blog posts of this era were text-only.
- Title: sits directly beneath the image (or at the top if no image), `22px`–`26px` in the accent color, `margin-top: 8px`.
- Quote/detail text: `14px`, body grey, `margin-top: 4px`, reads like the post's opening paragraph.
- Entry spacing: `24px` between entries in the main column — enough to read as separate "posts," each with its own image/title/text block.

## Empty state

Should read like a personal, slightly apologetic blog post: something like "Nothing to post about today, sorry! Check back soon 💭" — first-person, informal, a little emotionally present, unlike `modern`'s neutral tone or `newspaper`'s dry one.

## Why this reinforces the identity

The two-column-with-decorative-sidebar structure is what separates this from generic "old web" pastiche — the sidebar widgets should feel like the specific, slightly-too-personal clutter a real blogger would have added (mood rings, guestbooks-once-removed, visitor counters) without becoming functional UI the app depends on. Warmth and first-person voice are the throughline; keep it sincere rather than ironic.
