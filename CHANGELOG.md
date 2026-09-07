# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
once there's a first release.

## [Unreleased]

### Fixed

- `web/index.html` was missing its `<script src="/core.js">` tag entirely,
  so none of the frontend's logic (date fetch, skin swap, event listeners)
  ever ran in the browser — this alone explained the date picker showing
  blank, changing the date doing nothing, and no skins applying. Also
  fixed `core.js` setting `date-input`'s initial value in the wrong string
  format (`<input type="date">` requires `YYYY-MM-DD` exactly) and its
  `change` handler's date-string destructuring.

### Documentation

- A design-guide `README.md` in each of the 13 `web/skins/*/` directories:
  palette, typography, layout, and empty-state voice for every planned skin
  (`geocities`, `aim`, `myspace`, `win95`, `teletext`, `terminal`, `modern`,
  `newspaper`, `blogspot`, `wikipedia`, `google-serp`, `craigslist`,
  `mantine`), written as prose design intent rather than actual CSS —
  `skin.css`/`skin.js` for all 13 remain empty placeholders pending Phase
  4/5 hand-implementation.

### Changed

- Reframed the "crowdsourcing"/voting docs to match the real deployment
  target: this is shown to a small known audience (judges/team), not the
  open internet, so voting/submission only needs to be *correct* (no
  double-counting a repeat click, votes are changeable) rather than
  resistant to determined public abuse. Renamed the "Crowdsourced" axis to
  "Collaborative corpus" throughout (`PLAN.md`, `CONTEXT.md`, `README.md`,
  `TODO.md`, `AGENTS.md`); rewrote
  [ADR-0003](./docs/decisions/0003-vote-anti-abuse.md)'s rationale from
  "accepting a known abuse limitation" to "no adversarial audience to
  defend against"; reworded the affected `TODO.md` and
  `docs/testing-plan.md` checklist items (an "abuse test" became a
  "functionality test" for vote-changing) while keeping the still-relevant
  injection/XSS hygiene checks, which aren't audience-specific.

### Added

- Phase 2: `GET /api/date/:mm-dd` in `src/routes/dates.rs`, wired through
  `src/routes/mod.rs` and mounted alongside static file serving in
  `src/main.rs`. Returns confirmed entries for a given month/day plus a
  sitewide `stats.global_total_confirmed_entries` count; an empty
  `entries` array is the honest "not a notable date" response, distinct
  from a `400` on malformed/out-of-range `mm-dd`. `EntryStatus` gained
  `FromSql`/`ToSql` impls in `src/models.rs` so it round-trips through
  SQLite as its lowercase string form. The `submitted_by` field was
  dropped from `entries`/`Entry`/`NewEntry` entirely (no login, no plan to
  store submitter names). Response shape documented in `PLAN.md`'s phased
  build order.
- Phase 1.5 retrofit: `needs_specific_year` (bool) and `specific_year`
  (nullable integer) columns on `entries` (`src/db/schema.sql`), gated by a
  `CHECK` so a year can only be set when the flag is true and vice versa —
  mirrors the existing `needs_secondary_source` pattern. Mirrored in
  `data/seed/dates.schema.json` (new required field, plus an `if`/`then`/
  `else` enforcing the same coupling at the JSON level), `src/models.rs`
  (`Entry`/`NewEntry`), and the seed insert in `src/db/mod.rs`. *Back to the
  Future Part II* added to `data/seed/dates.seed.json` as the first
  year-specific entry (`specific_year: 2015`). `GET /api/date/:mm-dd`
  (Phase 2) will surface this as a flat, always-present, nullable
  `specific_year` field per entry.
- Project scaffold: directory structure for a Rust backend (`src/`), a
  skinnable static frontend (`web/`), corpus seed data (`data/seed/`),
  offline corpus-mining tooling (`scripts/seed/`), and ADRs
  (`docs/decisions/`). All source files created empty — implementation is
  hand-written, not agent-generated.
- `PLAN.md` — full design: smallest-footprint / largest-corpus /
  crowdsourced / 13-skins architecture, CSS Zen Garden skin mechanism,
  data model sketch, phased build order.
- `AGENTS.md` — rules for AI agent participation in this repo, including
  that all implementation code is hand-written, and the logging rule
  requiring every agent interaction to be summarized in
  `docs/agent-log.md`.
- `docs/agent-log.md` — append-only log of agent interactions.
- `TODO.md` — detailed, phase-by-phase implementation checklist.
- `docs/corpus-verification.md` — methodology for corroborating
  subtitle-mined date references against independent secondary sources
  (Checkiday API, National Day Calendar, Days Of The Year, Wikipedia
  per-date pages, IMDb/TMDb, fan wikis, entertainment journalism) before
  an entry is trusted.
- `docs/dev-guide.md` — local development guide.
- `docs/testing-plan.md` — layered testing plan (unit, integration, manual
  skin matrix, corpus sampling, footprint gate, abuse smoke tests).
- `LICENSE` — Apache License 2.0.
- `README.md` and `.gitignore`.
- 13 planned skins over one shared dataset, including a CSS-only homage to
  the [Mantine](https://github.com/mantinedev/mantine) component library's
  visual style (no React or Mantine dependency).
- This changelog.
- `CONTEXT.md` — project glossary, starting with a tight definition of
  "footprint" scoped strictly to the browser-side payload.
- Seven ADRs in `docs/decisions/` resolving every open question from
  `PLAN.md`/`TODO.md`, plus two raised mid-session: HTTP layer (axum),
  moderation threshold (net +3 votes, DB-only admin override), vote
  anti-abuse (cookie fingerprint, mutable votes, no rate limiting),
  per-skin empty-state text (via CSS generated content), corpus mining
  source (OPUS/OpenSubtitles, single-line excerpts only), per-entry scene
  images (crowdsourced, app-hosted, capped, lazy-loaded), and deploy
  target (clone-and-run only, auto-seeded on first boot).

### Changed

- `PLAN.md`, `TODO.md`, `AGENTS.md` updated to reference the new ADRs in
  place of the open-question/TBD markers they previously carried; `TODO.md`
  Phase 1/6 updated for per-entry image support.
- Tagged unlabeled fenced code blocks in `PLAN.md`/`README.md`/
  `docs/dev-guide.md` with a language (`text`/`sh`) and added blank lines
  around headings/lists in `CHANGELOG.md`, to satisfy the new markdownlint
  config below without changing any actual content.
- `README.md` gained a "Use of AI in this project" section summarizing
  what agents were and weren't allowed to do here, and pointing to
  `docs/agent-log.md` as the auditable record; its intro paragraph was
  trimmed of "entered sideways"-style narrative framing, keeping only the
  factual four-axis description.

### Added

- `.markdownlint-cli2.jsonc` — lint config tuned to this repo's existing
  Markdown conventions (dash-style lists, no line-length cap, relaxed table
  pipe spacing) rather than the strict defaults; `docs/agent-log.md` is
  excluded outright since it's an append-only log whose past entries are
  never reformatted.
- `Cargo.toml` dependencies for the axum backend: `axum`, `tokio`
  (`rt-multi-thread`, `macros`), `tower-http` (`fs`, for `ServeDir` static
  file serving), `rusqlite` (`bundled`, so clone-and-run doesn't need a
  system libsqlite3), `serde`/`serde_json`.
- `src/main.rs` hello-world body — Phase 0's toolchain sanity check
  (`cargo build`/`cargo run`) before any real logic goes in.
