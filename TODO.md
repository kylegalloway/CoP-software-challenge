# TODO — Today in Cinema

Detailed, ordered checklist following `PLAN.md`'s phases. Check items off as
you go by hand. Items originally marked **(ADR)** are now resolved — see
`docs/decisions/` (linked inline below).

## Phase 0 — Project bootstrap

- [x] HTTP layer: axum, per [ADR-0001](./docs/decisions/0001-http-layer.md).
- [x] Write `Cargo.toml`: package metadata, chosen HTTP crate (or none),
      `rusqlite`, `serde`/`serde_json` for JSON.
- [x] `cargo build` sanity check with just a "hello world" `main.rs` before
      writing real logic, to confirm toolchain/deps work.
- [x] Confirm SQLite file location/name (e.g. `data/app.db`, already
      gitignored) and that it's created on first run if missing.

## Phase 1 — Data model & schema

- [x] Write `src/db/schema.sql`:
  - `entries` table: `id`, `month` (1-12), `day` (1-31), `title`, `quote`,
    `source_url`, `secondary_source_url` (nullable), `needs_secondary_source`
    (bool), `image_url` (nullable — see ADR-0006, app-hosted path not an
    external URL), `status` (`pending`/`confirmed`/`rejected`),
    `submitted_by` (nullable — no login, so this might just be a display
    name string), `created_at`.
  - `votes` table: `id`, `entry_id`, `direction` (+1/-1), `voter_fingerprint`
    (see [ADR-0003](./docs/decisions/0003-vote-anti-abuse.md)), `created_at`.
  - Indexes: at least `(month, day)` on `entries` for the hot lookup path.
- [x] Moderation threshold: net +3 votes, DB-only admin override, per
      [ADR-0002](./docs/decisions/0002-moderation-threshold.md). (schema
      supports this; the actual recompute-on-vote logic is Phase 6.)
- [x] Vote anti-abuse: cookie fingerprint, no rate limit, votes mutable, per
      [ADR-0003](./docs/decisions/0003-vote-anti-abuse.md) — remember the
      `(entry_id, voter_fingerprint)` unique constraint this implies.
      (constraint is in schema; cookie-fingerprint mechanism itself is
      Phase 6.)
- [x] Write `data/seed/dates.schema.json` describing the shape of a seed
      entry (mirrors the `entries` columns minus DB-only fields like `id`).
- [ ] Hand-author a small, *verified* seed list in
      `data/seed/dates.seed.json` — quality over volume here. Suggested
      starting set: Mean Girls (Oct 3), Back to the Future Part II (Oct 21),
      Independence Day (Jul 3–4), Die Hard (Dec 24), Ferris Bueller's Day Off
      (Jun 5), Groundhog Day (Feb 2), Star Wars ("May the 4th"). Run each of
      these through the verification workflow in
      `docs/corpus-verification.md` even though they're well-known — get in
      the habit early.
- [x] Write a small import routine (part of `src/db/mod.rs` or a `--seed`
      CLI flag on the binary) that loads `dates.seed.json` into `entries` on
      first run only (don't duplicate on every boot).

## Phase 1.5 — Retrofit: specific-year dates

Some entries are only true for one specific year, not "this date, every
year" (e.g. *Back to the Future Part II*'s Oct 21, **2015**; many
real-world news/history dates once corpus mining starts in Phase 7). The
current schema/model has no way to express that — needs a retrofit before
more entries (especially mined ones) make the gap harder to fix.

- [ ] Add `needs_specific_year` (bool) and `specific_year` (nullable
      integer) columns to `entries` in `src/db/schema.sql`, mirroring the
      `needs_secondary_source` / (missing) `secondary_source_url` pairing
      style already used for the source-verification fields.
- [ ] Decide: does `specific_year` require `needs_specific_year = 1`, or
      can a year be attached even when not strictly "needed" (e.g. flavor
      info like "this line is from the 1985 original" on a date that's
      otherwise evergreen)? Consider a `CHECK` constraint once decided.
- [ ] Update `data/seed/dates.schema.json` with the new optional fields.
- [ ] Update `src/models.rs`: add the fields to `Entry` and `NewEntry`.
- [ ] Update `src/db/mod.rs`'s seed insert (column list + `params!`) to
      include the new columns.
- [ ] Retrofit existing seed entries: is *Back to the Future Part II*
      (if/when added) `specific_year: 2015`? Any other seeded/mined entries
      that are actually year-specific rather than annually-recurring?
- [ ] Decide how `GET /api/date/:mm-dd` (Phase 2) should surface this to
      the frontend — e.g. does the response need a `specific_year` field
      per entry, and should skins render "October 21, 2015" differently
      than a recurring "October 21" when it's set?

## Phase 2 — Minimal API (seed data only, no submit/vote yet)

- [ ] `GET /api/date/:mm-dd` in `src/routes/dates.rs` — returns confirmed
      entries for that date (empty array is a valid, honest response — "not
      a notable date" matters as much as a hit).
- [ ] Decide exact JSON response shape and document it inline as a comment
      or in `PLAN.md` (e.g. `{ date, entries: [{title, quote, source_url}],
      stats: { total_confirmed_entries_sitewide } }`).
- [ ] Wire `src/routes/mod.rs` to route requests to the right handler.
- [ ] `src/main.rs`: start the server, open the DB, serve `web/` as static
      files, mount the API routes.
- [ ] Manual test: hit a known-good date and an empty date with `curl`,
      confirm both look right.

## Phase 3 — Base HTML/CSS/JS shell (no visual identity yet)

- [ ] `web/index.html` — semantic structure only: a place for the date
      picker/today display, a list of entries, a skin `<select>` or link
      picker, a `<link id="skin-css">` placeholder, an empty-state message
      for "no notable date."
- [ ] `web/core.css` — layout/reset only, deliberately ugly/unstyled. If it
      looks like a skin, it's doing too much.
- [ ] `web/core.js`:
  - Fetch `GET /api/date/:mm-dd` for today on load; render into the DOM.
  - Read/write the active skin choice (query param and/or `localStorage`).
  - Swap `<link id="skin-css">`'s `href` (and load/unload a skin's `skin.js`
    if present) based on the active skin — this is the mechanism the whole
    "13 skins, 1 dataset" claim depends on, so keep it simple and correct.
- [ ] Confirm: viewing with **no** skin selected still renders usable,
      correct data (core shell should never be broken on its own).

## Phase 4 — Skin mechanism proof + first skin

- [ ] `web/skins/registry.json` — list of `{slug, label}` for every skin,
      consumed by the picker UI in `core.js`.
- [ ] Build `modern` skin first (it's the "normal" baseline, easiest to get
      right, good smoke test for the swap mechanism).
- [ ] Confirm switching skins via the picker requires zero changes to
      `index.html` or `core.js` — if it does, fix the mechanism before
      building more skins.

## Phase 5 — Remaining 12 skins

Suggested build order (cheapest/most fun first, save the fussiest for last):

- [ ] `terminal` — monospace, green-on-black, minimal layout work.
- [ ] `craigslist` — plain blue-link brutalism, minimal layout work.
- [ ] `mantine` — CSS-only homage to Mantine's (mantinedev/mantine) visual
      language: its color palette, rounded card components, subtle
      shadows, pill-shaped badges/buttons, and typography scale — reference
      their docs site or Storybook for the exact tokens. No React, no
      Mantine package — reproduce the *look* in `skin.css` only. Worth
      calling out on the page itself somewhere (footer credit) since it's a
      deliberate homage, not an accident.
- [ ] `wikipedia` — infobox + citations; consider surfacing
      `secondary_source_url` here specifically, it fits the theme.
- [ ] `google-serp` — mock search result card.
- [ ] `newspaper` — serif headline, columns.
- [ ] `blogspot` — sidebar widgets, "mood" indicator.
- [ ] `teletext` — blocky broadcast-TV grid, teletext color palette.
- [ ] `aim` — buddy list / away-message framing.
- [ ] `win95` — desktop + dialog window; nice nod back to the original
      `challenge.png` mockup framing.
- [ ] `myspace` — Top 8 + glitter cursor + profile-era layout.
- [ ] `geocities` — table layout, marquee, hit counter, "under
      construction" — probably the most fun, save for when you want a break
      from the others.
- [ ] After all 13 exist: spot check every skin against both a "hit" date
      and a "no notable date" date — the empty state is easy to forget to
      style per-skin.

## Phase 6 — Crowdsourcing (submit + vote)

- [ ] `POST /api/submit` in `src/routes/submit.rs` — validate required
      fields (month/day/title/quote/source_url), insert as `status: pending`.
- [ ] Image upload handling for submissions, per **ADR-0006**: accept an
      optional image on `POST /api/submit`, resize to a max dimension (e.g.
      800px wide) and re-encode to a single consistent format (e.g.
      JPEG/WebP) server-side regardless of input, store under a gitignored
      `data/uploads/` dir, save the resulting path/URL to `entries.image_url`.
      Serve uploads via a static route, separate from the base API response.
- [ ] Wire the frontend to request images lazily (`loading="lazy"` and/or
      fetch-on-reveal) — the base `GET /api/date/:mm-dd` response carries
      only `image_url` (or `null`), never image bytes, so a page load's
      guaranteed footprint is unaffected by whether an entry has an image.
- [ ] Basic input sanitization/length limits (this is public-facing).
- [ ] `POST /api/vote` in `src/routes/vote.rs` — record a vote, recompute
      whether the entry crosses the moderation threshold from Phase 1, flip
      `pending` → `confirmed` when it does (only if it also has a
      `secondary_source_url` — see `docs/corpus-verification.md`).
- [ ] Add a submission form to `web/index.html`/`core.js` (kept
      skin-agnostic; each skin can restyle it but shouldn't need its own
      copy of the logic).
- [ ] Decide how/whether `pending` entries needing a secondary source are
      surfaced to encourage crowd help finding one (per
      `docs/corpus-verification.md` step 4).
- [ ] Manual abuse test: try to spam votes/submissions, confirm your
      chosen anti-abuse approach actually does something.

## Phase 7 — Corpus growth: mining + verification

- [ ] Corpus source: OPUS/OpenSubtitles dump, single-line excerpts only, per
      [ADR-0005](./docs/decisions/0005-corpus-mining-source.md).
- [ ] Write the mining script: regex/NLP pass for absolute date mentions,
      output candidate JSON matching `data/seed/dates.schema.json`, status
      `pending`.
- [ ] Read `docs/corpus-verification.md` in full before running any mined
      output into the live DB.
- [ ] Implement (or manually perform, at least at first) the corroboration
      pass: check Checkiday's API, National Day Calendar, Days Of The Year,
      Wikipedia's per-date "Holidays and observances" sections.
- [ ] Attach `secondary_source_url` where found; set
      `needs_secondary_source: true` where not.
- [ ] Import the corroborated batch into `entries`.
- [ ] Update the site somewhere (maybe the `modern` or `wikipedia` skin, or
      a shared stat in the API response) to display total confirmed corpus
      size — this is your "largest corpus" flex, make it visible.

## Phase 8 — Footprint budget

- [ ] Byte budget: sub-20KB uncompressed is the aim, but stays a soft,
      non-gating target for now — see `PLAN.md`'s footprint section.
      Revisit with a hard number once the app is running and there's real
      data (especially entry images, per
      [ADR-0006](./docs/decisions/0006-entry-images.md)) to measure.
- [ ] Write `scripts/measure_footprint.sh`: curl the page + core assets +
      one skin's assets + one API response, sum bytes, compare to budget.
- [ ] Run it against every skin, not just the default — some (geocities,
      myspace) are more asset-heavy by nature; decide if they get a
      separate, explicitly-called-out budget or must still comply.
- [ ] If over budget anywhere: trim before moving on. Don't let this slide
      to "later."

## Phase 9 — Polish / submission-readiness

- [ ] Fill in `README.md`'s "Running" section with real, clone-and-run
      instructions once the binary actually runs — cover both first-run
      auto-seeding and how to manually re-seed/extend the corpus later, per
      [ADR-0007](./docs/decisions/0007-deploy-target.md) (no hosted deploy;
      DB and uploaded images stay gitignored, never committed).
- [ ] Final pass on all 13 skins for the "no notable date today" empty
      state — this is an easy thing to forget and it's the more *honest*
      state, worth getting right.
- [ ] Write up the final footprint numbers and corpus size somewhere
      visible for whoever's judging the challenge — these are the two
      numbers that make the "off-the-wall" angle land as more than a joke.

## Ongoing

- [ ] Log every agent interaction in `docs/agent-log.md` per `AGENTS.md`'s
      logging rule — don't let this fall behind.
