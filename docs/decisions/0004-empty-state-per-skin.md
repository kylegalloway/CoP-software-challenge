---
status: accepted
---

# Empty-state copy: per-skin flavor text via CSS generated content

Context: "no notable date today" needs to render somehow in every skin. `AGENTS.md` reserves `skin.js` for gimmicks only, never core behavior — so whatever mechanism supplies the empty-state text can't require a skin to ship JS just to have its own copy.

Decision: `core.js` always renders a fixed, empty container for the empty state (e.g. `<p data-empty-state></p>`) whenever the API returns `entries: []`, with no text content of its own — `core.css` supplies a neutral fallback string so the shell is never broken skin-less. Each `skin.css` may override the copy via generated content keyed off the data attribute (e.g. `[data-empty-state]::before { content: "..."; }`).

This keeps empty-state personality (`geocities`: "Nothing here yet! Sign our guestbook"; `teletext`: "NO DATA FOUND — PAGE 404"; etc.) entirely within `skin.css`, with zero `skin.js` involvement — consistent with skins being pure CSS deltas and the API remaining the single source of truth for *what* happened (empty vs. not), while skins only control *how it's said*.

Rejected: a skin-slug → string lookup table in `core.js`/`registry.json`. Keeps copy out of CSS, but makes a skin's personality partially depend on editing a shared core file — breaks self-containment more than the CSS approach does.
