# PLAN.md — Today in Cinema

## The pitch

Given a date, answer honestly: is this a notable date *within* pop culture
(a date a movie/show explicitly references — "It's October 3rd," "Today we
celebrate our Independence Day" — not just a release date)? Say so, or say
plainly that nothing notable landed today.

Instead of competing on "nicest single site," this entry competes on four
axes at once:

| Axis | How |
|---|---|
| Smallest footprint | One base payload (HTML+JSON), skins are pure CSS/JS deltas, no framework |
| Largest corpus | Seed from mining real dialogue/metadata, not hand-typed trivia |
| Collaborative corpus | Submission + voting/moderation grows the corpus over time, for the small known audience (judges/team) this gets shown to — not the open internet |
| Skins | 12 total visual identities over one dataset (CSS Zen Garden pattern) |

## Architecture

```text
Browser
  -> GET /                  (index.html + core.css + core.js, tiny)
  -> GET /skins/registry.json
  -> GET /skins/<name>/skin.css [+ skin.js]   (only the active skin loads)
  -> GET /api/date/:mm-dd    -> { date, entries: [...], stats }
  -> POST /api/submit        { date, title, quote, source_url }
  -> POST /api/vote          { entry_id, direction }

Rust binary
  -> serves static web/ assets
  -> owns SQLite (rusqlite), single file, no external DB dependency
  -> moderation: an entry needs N net upvotes (or an admin flag) before it's
     "confirmed" and shown by default
```

### Why this satisfies "smallest footprint"

The thing that grows over time (corpus size, number of skins) never touches
the bytes shipped to a given visitor. A visitor always downloads: one HTML
shell, one core CSS, one core JS, one skin CSS(+js), one JSON response for
the day. Adding skin #13 or corpus entry #10,000 changes none of that.
`scripts/measure_footprint.sh` should assert a byte-budget per page load in
CI/manually (target: TBD, aim sub-20KB uncompressed for the default skin).
This stays a **soft, non-gating target for now** — revisit once the app is
actually running and there's real data to measure against, and especially
once the per-entry image question (see below) is resolved, since images
will dominate the byte count far more than markup/CSS/JS ever would.

### Why Rust

Single static binary, no runtime/interpreter to ship, small dependency
surface if kept to axum (or std-only) + rusqlite. Fits the minimalism theme
on the backend too, not just the frontend.

## Data model (sketch — human to finalize in `src/db/schema.sql`)

- `entries`: id, month, day, title (film/show), quote, source_url,
  status (pending/confirmed/rejected), created_at
- `votes`: id, entry_id, direction (+1/-1), voter_fingerprint, created_at

Seed schema for the corpus-mining output lives in
`data/seed/dates.schema.json`; actual seed rows go in
`data/seed/dates.seed.json` and get imported into `entries` at first boot
or via a one-off import script.

## Corpus seeding strategy

1. Start from a small hand-verified list (accuracy over volume) covering the
   obvious cases (Mean Girls, Back to the Future, Independence Day, Die
   Hard, Ferris Bueller, etc).
2. Mine a subtitle corpus (e.g. OpenSubtitles dumps) for absolute date
   mentions ("October 3rd", "July 4th") via regex/NLP, cross-reference
   against the film/show it came from, and queue matches as `pending`
   entries for human/community review rather than auto-publishing — mined
   data needs a confirmation step before it's trustworthy.
3. Submissions from the judges/team extend the corpus after launch — this
   is a closed group, not the open internet, so growth here is steady
   rather than "indefinite at scale."

`scripts/seed/mine_corpus.py` is the offline tool for step 2. It is *not*
part of the running app — it produces a JSON file that gets imported.

## Skins (13)

CSS Zen Garden pattern: identical markup, wildly different presentation.
Each skin is `web/skins/<slug>/skin.css` (+ optional `skin.js` for a
gimmick only — never for core behavior).

| Slug | Reference |
|---|---|
| `geocities` | table layout, marquee, hit counter, "under construction" |
| `aim` | AOL Instant Messenger buddy list / away message |
| `myspace` | Top 8, glitter cursor, profile-song-era vibe |
| `win95` | desktop + dialog window (nod to the original challenge mockup) |
| `teletext` | Ceefax/teletext blocky broadcast-TV info page |
| `terminal` | green-on-black monospace, fake typing animation |
| `modern` | the "normal best-practice SaaS site" baseline |
| `newspaper` | broadsheet serif, columns, "Extra! Extra!" |
| `blogspot` | early-2000s blog, sidebar widgets, mood indicator |
| `wikipedia` | infobox, citations, talk-page framing (pairs well with voting) |
| `google-serp` | fact rendered as a mock search result |
| `craigslist` | plain blue-link brutalism |
| `mantine` | CSS-only homage to the [Mantine](https://github.com/mantinedev/mantine) React component library's visual language (palette, rounded cards, subtle shadows, pill badges) — no React, no Mantine dependency, just its look reproduced in vanilla CSS |

`web/skins/registry.json` lists these for the picker UI — keep it in sync
when a skin is added/removed.

## Phased build order

1. **Data model + schema** — `src/db/schema.sql`, `data/seed/dates.schema.json`.
2. **Minimal API** — `GET /api/date/:mm-dd` serving the hand-verified seed
   list only (skip mining/crowdsourcing at first). Response shape (decided,
   implemented in `src/routes/dates.rs`):

   ```json
   {
     "date": "10-21",
     "entries": [ /* full Entry rows, minus submitted_by */ ],
     "stats": { "global_total_confirmed_entries": 3 }
   }
   ```

   `entries` only ever contains `status: "confirmed"` rows for the given
   `month`/`day`; an empty array is a valid, honest response for "not a
   notable date." `specific_year` is present and `null` on every entry
   (never omitted) so frontends can check for non-null to render a
   year-specific date. `stats.global_total_confirmed_entries` is a sitewide
   count, independent of the date filter. Malformed or out-of-range
   `mm-dd` (month outside 1-12, day outside 1-31, unparseable) is a `400`,
   distinct from a valid-but-empty date.
3. **Base HTML/CSS/JS shell** — `web/index.html`, `core.css`, `core.js`,
   fetches from the API and renders with zero visual identity of its own.
4. **Skin mechanism** — registry + picker + first skin (`modern`) to prove
   the swap mechanism end to end.
5. **Remaining 11 skins** — one at a time, cheapest/most fun first.
6. **Collaborative corpus** — `POST /api/submit`, moderation status, `POST
   /api/vote`, surfacing pending vs confirmed appropriately per skin.
7. **Corpus mining** — `scripts/seed/mine_corpus.py`, import pipeline,
   re-seed `entries` with a much larger pending queue.
8. **Footprint pass** — `scripts/measure_footprint.sh`, trim until under
   budget, document the number somewhere visible on the site itself (nice
   flex if true).

## Decisions

All originally-open questions are now resolved — see `docs/decisions/`:

- HTTP layer: axum ([ADR-0001](./docs/decisions/0001-http-layer.md)).
- Moderation threshold: net +3 votes, DB-only admin override
  ([ADR-0002](./docs/decisions/0002-moderation-threshold.md)).
- Voting mechanism: cookie fingerprint identifies a voter so their vote is
  changeable rather than duplicable; not an anti-abuse measure — this is
  shown to a small known audience (judges/team), not the open internet
  ([ADR-0003](./docs/decisions/0003-vote-anti-abuse.md)).
- Empty-state flavor text: per-skin via CSS generated content
  ([ADR-0004](./docs/decisions/0004-empty-state-per-skin.md)).
- Corpus mining source: OPUS/OpenSubtitles, single-line excerpts only
  ([ADR-0005](./docs/decisions/0005-corpus-mining-source.md)).
- Per-entry scene images: app-hosted, capped, lazy-loaded
  ([ADR-0006](./docs/decisions/0006-entry-images.md)).
- Deploy target: clone-and-run only, auto-seeded on first boot
  ([ADR-0007](./docs/decisions/0007-deploy-target.md)).
