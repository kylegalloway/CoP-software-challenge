# aim

**Concept:** AOL Instant Messenger — buddy list window chrome, away-message culture, the "door open/close" sound-era of being online.

## Palette

AIM's actual brand blue/yellow: a gradient title bar in that specific saturated blue (`#003366`-to-`#3366CC` range) with a yellow running-man-adjacent accent color used sparingly (badges, the "online" indicator dot). Window chrome is grey (`#ECE9D8`-ish, Windows-XP-era grey) — the *content* inside the "window" (the date/entries) sits on white, like a chat log. Green for "online"/confirmed state, grey for idle/empty state — borrowed directly from buddy-list presence dots.

## Typography

Small system sans-serif (Tahoma or the nearest web-safe equivalent) at a noticeably smaller base size than the other skins — AIM's UI text was always compact. The entry list should visually resemble a chat transcript: each entry prefixed like a buddy's message, maybe with a small colored "screen name" style treatment for the movie title, similar to how AIM colored each participant's name in a conversation.

## Layout

A single bordered "window" frame with a fake title bar (text like "Buddy List" or the site name) and a thin 3D-beveled border (`border-style: outset`), echoing classic OS chrome rather than a flat modern card. The date/skin pickers sit where a buddy-list's top controls would; entries render as a scrollable chat-log area beneath, each one left-aligned like an incoming message bubble (no actual bubble shape needed — just indentation and a light background band per entry).

## Away-message gimmick

The one skin where `skin.js` earns its keep as a *pure cosmetic gimmick* per `AGENTS.md`: on load, briefly show a fake "buddy is typing..." indicator before the entries render (a `setTimeout`-driven placeholder, purely decorative, never gating when real data actually appears — the real fetch/render logic must stay entirely in `core.js`).

## Empty state

Something in the voice of an away message: e.g. "BRB — nothing notable today." Understated, a little wistful, matching AIM's away-message culture rather than a generic "no results."

## Why this reinforces the identity

The nostalgia here is specifically *social* rather than visual-clutter (contrast with `geocities`) — restraint in the palette (three colors, used consistently) and the chat-log framing for the entry list are what sell "AIM" over generic Y2K styling. Skip glitter, marquees, or tiled backgrounds — those belong to other skins.
