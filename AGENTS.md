# AGENTS.md

Guidance for any AI agent (Claude Code or otherwise) working in this repo.

## The one rule that overrides everything else

**The human writes all implementation code by hand.** Do not write, generate,
or auto-complete Rust, HTML, CSS, JS, SQL, or Python source in this repo
unless explicitly asked in the moment. Your default role here is:

- planning and scoping (PLAN.md, ADRs, checklists)
- reviewing hand-written code when asked
- research (data sourcing, prior art, format quirks of a given skin era)
- scaffolding directory structure / empty files when asked
- documentation

If a task seems to require writing code to be useful, say so and ask instead
of doing it.

## Logging rule

**Every single interaction with an agent in this repo must be logged**, in
summary form, in [`docs/agent-log.md`](./docs/agent-log.md). This is not
optional and not just for code changes — planning discussions, brainstorming,
review feedback, and questions all count. Before ending a turn, append an
entry covering what was asked and what the agent did or produced (a few
lines is enough — this is a summary log, not a transcript). Never edit or
delete a prior entry; it's append-only.

## What this project is

A submission for an internal "build a site that tells you if today is a
notable date in pop-culture/cinema" challenge (see `challenge.png` at repo
root for the original prompt). The deliberate angle is to hit *several*
"best of show" axes at once instead of just "nicest site":

1. **Smallest footprint** — page weight stays flat no matter how much content
   or how many skins exist.
2. **Largest corpus** — not a hand-typed list of ~50 dates; seeded from
   mining actual film/TV dialogue and metadata for date references, then
   grown further by...
3. **Crowdsourcing + voting** — anyone can submit a date reference; a simple
   vote/moderation threshold decides what's "confirmed."
4. **13 skins, one dataset** — CSS Zen Garden pattern: one semantic HTML
   document, one JSON API, skins are *purely* swappable CSS (+ a little
   optional per-skin JS for a gimmick). See PLAN.md for the full skin list,
   including a CSS-only homage to the Mantine component library's visual
   style (no React, no Mantine dependency — just its look).

Full rationale and phased plan: see `PLAN.md`.

## Architecture at a glance

- `src/` — Rust backend (axum or std-only, TBD by the human). Serves the API
  and the static `web/` assets. SQLite via `rusqlite`, no ORM.
- `web/` — one `index.html`, one `core.css` (structure/reset only, no visual
  identity), one `core.js` (fetches data, applies the active skin). No skin
  should ever require editing `index.html` or `core.js` — if it does, the
  Zen Garden constraint has been broken.
- `web/skins/<name>/skin.css` (+ optional `skin.js`) — one skin, self
  contained. `web/skins/registry.json` lists the available skins for the
  picker UI.
- `data/seed/` — seed corpus data and its JSON schema.
- `scripts/seed/` — offline mining/import tooling (not served, not part of
  the running app).
- `docs/decisions/` — ADRs for any non-obvious architectural choice.

## Constraints to protect when reviewing or planning

- **No client-side framework.** Vanilla HTML/CSS/JS only, on principle (this
  is the whole footprint bit). A skin may visually *reference* a framework's
  design language (e.g. the `mantine` skin) purely through CSS — it must
  never actually depend on that framework or its JS.
- **A skin must not carry logic the app depends on.** If removing a skin's
  CSS/JS file would break data fetching or voting, that's a bug — skins are
  presentation-only.
- **The API is the single source of truth for content.** Never let a skin
  hardcode or fork the dataset.
- **Footprint has a budget.** See `PLAN.md` for the current target; flag any
  addition that meaningfully grows the base payload (i.e. anything loaded
  regardless of skin).
