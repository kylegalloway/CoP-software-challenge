# Testing plan

Layered so the cheap/fast checks run often and the expensive/manual ones run
before milestones, not every change.

## 1. Unit tests (Rust, `cargo test`)

Target the logic that's easy to get subtly wrong, not the glue code:

- **Date handling**: `mm-dd` parsing/validation (reject `13-40`, handle
  leap-day `02-29` sanely, zero-padding either way).
- **Moderation threshold logic**: given a set of votes, does an entry
  correctly flip `pending` → `confirmed` only when (a) net votes clear the
  threshold from `docs/decisions/` **and** (b) `secondary_source_url` is
  present? Test both conditions independently — an entry with enough votes
  but no secondary source must stay `pending`.
- **Vote recording**: same fingerprint voting again on an entry updates the
  existing vote (change direction, or clear it) rather than creating a
  second row or being rejected — this is the actual functional
  requirement, not an anti-abuse rule.
- **Submission validation**: required fields, length limits, obviously
  malformed input (empty title, missing date, garbage URL).

Put these alongside the code they test (`#[cfg(test)]` modules in the
relevant `src/` files) rather than a separate test tree — keeps them honest
about which module they're covering.

## 2. Integration tests (API level)

Spin up the app against an in-memory or temp-file SQLite DB (not
`data/app.db`) and hit the real HTTP routes:

- `GET /api/date/:mm-dd` for a date seeded with confirmed entries → correct
  JSON shape and content.
- `GET /api/date/:mm-dd` for a date with none → empty array, not an error.
- `GET /api/date/:mm-dd` for a date with only `pending`/unconfirmed entries
  → excluded from the default response (confirm the filter is actually
  applied, not just "happens to work" in the happy path).
- `POST /api/submit` with valid payload → row created as `pending`.
- `POST /api/submit` with invalid/missing fields → rejected with a sane
  status code, no partial row created.
- `POST /api/vote` sequence → confirm the threshold-crossing behavior from
  the unit tests actually fires end-to-end through the real route.
- Repeat-vote from the same fingerprint → updates the existing vote
  in place (upsert), doesn't create a duplicate row.

## 3. Manual skin/UI testing (do this before considering a skin "done")

For **each of the 13 skins**, check both states:

- [ ] Renders correctly on a date with a confirmed entry.
- [ ] Renders correctly on a date with none (the empty state is the one
      that gets forgotten — check it explicitly, don't assume it inherits
      styling correctly).

That's 26 checks total; track them as a simple table if useful (skin name x
hit/empty columns) rather than trusting memory across a long session.

Also manually confirm, once per skin-mechanism change (not per skin):

- [ ] Switching skins via the picker requires no page reload issues, no
      flash of unstyled content beyond what's acceptable, and correctly
      persists across a refresh.
- [ ] Viewing with no skin selected (bare `core.css` only) still shows
      correct, readable data.

## 4. Corpus quality spot-checks

Not automatable in a meaningful way — this is a sampling exercise:

- Before importing any batch from `scripts/seed/mine_corpus.py`, hand-verify
  a random sample (suggest 10%, minimum 10 entries) against
  `docs/corpus-verification.md`'s workflow. If the sample's error rate is
  high, fix the mining script rather than hand-correcting every entry.
- Periodically (e.g. weekly during active use by the judges/team) spot-check
  a sample of `confirmed` entries to make sure the vote+secondary-source
  gate is actually holding the line, not just at launch.

## 5. Footprint testing (gate, not a suggestion)

- `scripts/measure_footprint.sh` should be run before merging any change
  that touches `web/core.*` or adds a new skin, and its output compared
  against the budget set in `docs/decisions/`.
- Treat a budget failure as a blocker for that change, not a "note for
  later" — footprint creep is exactly the kind of thing that's cheap to fix
  now and annoying to fix after 13 skins exist.

## 6. Basic-functionality smoke tests (before treating submit/vote as done)

Not abuse-hardening — this is shown to a small known audience (judges/team),
not the open internet — just confirming the mundane cases work correctly:

- Submit with script tags / obviously malicious `quote` or `title` content
  → confirm it's stored/rendered safely (no injection into any skin's
  markup — remember skins share the same rendering path, so this only
  needs testing once, not per-skin). Good hygiene regardless of audience
  size, not audience-specific.
- Click vote repeatedly on the same entry → confirm it doesn't stack up
  (still just one vote per fingerprint per entry), then change the vote
  direction → confirm it updates rather than adding a second row.
- Long-input test (very long quote/title) → confirm it's rejected or
  truncated, not silently breaking layout in some skin.

## What "done" looks like for a phase

A phase from `TODO.md`/`PLAN.md` isn't complete until:

- Relevant unit + integration tests pass.
- Manual checklist items for anything user-facing in that phase are
  checked off.
- Footprint script has been run if the change touched shared assets.
- The interaction is logged in `docs/agent-log.md` if an agent was involved
  in getting there.
