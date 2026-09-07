# aim

**Concept:** AOL Instant Messenger — buddy list window chrome, away-message culture, the "door open/close" sound-era of being online.

## Palette

- Title bar gradient: `#003366` → `#3366CC` (`linear-gradient(to right, ...)`, top to bottom or left to right).
- Accent (badges, "online" dot): `#FFCC00`.
- Window chrome/background: `#ECE9D8`.
- Content area (chat-log body): `#FFFFFF`.
- Presence colors: online/confirmed `#2FA84F`, idle/empty `#999999`.
- Border: `2px outset #ECE9D8` for the window frame.

## Typography

- Font: `Tahoma, Geneva, sans-serif`.
- Base size: `12px`–`13px` — noticeably smaller than every other skin.
- Line-height: `1.3` (compact, chat-log density).
- Screen-name treatment (movie title per entry): bold, one consistent accent color, e.g. `#003366`.

## Layout

- Window frame border: `2px outset #ECE9D8`.
- Title bar height: `24px`–`28px`, white text, `13px` bold.
- Content padding: `8px`–`12px`.
- Entry spacing: `4px`–`6px` vertical gap between chat-log lines, left-indent `12px` for each "message."

## Entry card (title + image)

`image_url` is nullable ([ADR-0006](../../../docs/decisions/0006-entry-images.md)) — treat the image as a buddy icon, not a photo.

- Layout: image acts as a "buddy icon" to the left of each chat line — `32px × 32px`, `flex: 0 0 32px`, `object-fit: cover`, `border: 1px solid #999999` (buddy icons were small and square, not rounded).
- No-image fallback: a generic grey silhouette square, same `32px × 32px`, `background: #CCCCCC` — AIM's own default buddy icon was a plain grey outline, so this reads as in-universe rather than "broken image."
- Title (movie name, in the screen-name color `#003366`) sits on the same line as the icon, `13px` bold, `margin-left: 8px`.
- Quote/detail text: `12px`, `#000000`, wraps beneath the title line with the same `8px` left-indent as the icon width plus gap (so text aligns under the title, not the icon).
- Vertical gap between entries: `6px` — keep it tight, like consecutive chat lines.

## Away-message gimmick

The one skin where `skin.js` earns its keep as a *pure cosmetic gimmick* per `AGENTS.md`: on load, briefly show a fake "buddy is typing..." indicator before the entries render (a `setTimeout`-driven placeholder, purely decorative, never gating when real data actually appears — the real fetch/render logic must stay entirely in `core.js`).

## Empty state

Something in the voice of an away message: e.g. "BRB — nothing notable today." Understated, a little wistful, matching AIM's away-message culture rather than a generic "no results."

## Why this reinforces the identity

The nostalgia here is specifically *social* rather than visual-clutter (contrast with `geocities`) — restraint in the palette (three colors, used consistently) and the chat-log framing for the entry list are what sell "AIM" over generic Y2K styling. Skip glitter, marquees, or tiled backgrounds — those belong to other skins.
