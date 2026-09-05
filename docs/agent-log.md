# Agent interaction log

Append-only. Every interaction with an agent in this repo gets a summary
entry here — see the "Logging rule" in [`AGENTS.md`](../AGENTS.md). Do not
edit or remove prior entries.

Format per entry:

```
## YYYY-MM-DD — short title
Asked: what the human asked for.
Did: what the agent actually did/produced.
```

---

## 2026-09-05 — Brainstorm concept, then scaffold repo
Asked: Brainstorm off-the-wall responses to the "Today in Cinema" challenge
(challenge.png), narrowing through several rounds to a website that
combines smallest footprint, largest corpus, crowdsourcing/voting, and 12
visual skins over one dataset. Then: create a plan doc, AGENTS.md, and an
empty directory/file scaffold, with the human writing all actual code.
Did: Proposed and iterated the concept with the user across several rounds
(format subversions, aesthetic subversions, then the four-axis combo).
Recommended the CSS Zen Garden pattern (one HTML/JSON payload, skins as
pure CSS/JS deltas) to reconcile "smallest footprint" with "12 skins."
Asked and got answers on backend language (Rust) and git init timing.
Created `PLAN.md`, `AGENTS.md`, `README.md`, `.gitignore`, ran `git init`
(no commit), and scaffolded empty files/directories for the Rust backend
(`src/`), the skinnable frontend (`web/`, including all 12 skin folders),
seed data (`data/seed/`), corpus-mining tooling (`scripts/seed/`), and ADRs
(`docs/decisions/`). Added this logging rule and log file per follow-up
request.

## 2026-09-05 — Detailed TODO, corpus verification method, dev/testing docs
Asked: A detailed todo list to follow, a method to verify mined dates using
non-subtitle sources (websites etc.), and a local development guide plus a
testing plan.
Did: Wrote `docs/corpus-verification.md` proposing a two-source rule
(primary in-film reference + independent secondary source) before an entry
can be `confirmed`, with concrete non-subtitle sources to check (Checkiday's
API, National Day Calendar, Days Of The Year, Wikipedia per-date pages,
IMDb/TMDb quotes, fan wikis, entertainment journalism) and a suggested
per-candidate workflow; flagged this as a `secondary_source_url` /
`needs_secondary_source` addition to the data model. Wrote `TODO.md`: a
phase-by-phase checklist (mirroring `PLAN.md`'s 8 phases plus a bootstrap
phase 0 and a submission-readiness phase 9) with concrete sub-tasks and ADR
call-outs. Wrote `docs/dev-guide.md` (prerequisites, first-run steps,
day-to-day workflow, DB inspection, env/config placeholders) and
`docs/testing-plan.md` (unit/integration/manual-skin-matrix/corpus-sampling/
footprint-gate/abuse-smoke-test layers, plus a "done" definition per phase).
Linked all new docs from `README.md`.

## 2026-09-05 — Add LICENSE
Asked: Add a README, gitignore, and Apache 2.0 license if they don't
already exist.
Did: Checked — `README.md` and `.gitignore` already existed; only
`LICENSE` was missing. Added the standard Apache License 2.0 text
(copyright line: 2026, Kyle Galloway) and linked it from `README.md`'s new
License section.

## 2026-09-05 — Add Mantine-inspired skin as #13
Asked: Add a 13th skin modeled on the visual style of the Mantine React
component library (github.com/mantinedev/mantine), which some coworkers
are reportedly using for their own challenge entries.
Did: Scaffolded `web/skins/mantine/` (empty `skin.css`/`skin.js`, same
pattern as the other 12). Updated `PLAN.md`'s skin table/count (12 → 13)
with a description clarifying this is a CSS-only homage to Mantine's
design language (palette, rounded cards, shadows, pill badges) with no
React or Mantine dependency. Updated `AGENTS.md` (skin count, and an
explicit note under the "no framework" rule that a skin may visually
reference a framework purely via CSS but must never depend on it). Updated
`TODO.md` Phase 5 (11 → 12 remaining skins, added a `mantine` task noting
to reference Mantine's docs/Storybook for exact tokens and to credit it
somewhere on the page) and its later "all 12 → 13 skins" empty-state check.
Updated `docs/testing-plan.md`'s skin-matrix counts (12 → 13 skins, 24 → 26
checks). Updated `README.md`'s skin count.

## 2026-09-05 — Add CHANGELOG.md
Asked: Add a simple Keep a Changelog-style changelog and update it.
Did: Created `CHANGELOG.md` (Keep a Changelog format, Semantic Versioning
noted for once a first release exists) with an `[Unreleased]` section
summarizing everything produced so far — scaffold, `PLAN.md`, `AGENTS.md`,
`docs/agent-log.md`, `TODO.md`, `docs/corpus-verification.md`,
`docs/dev-guide.md`, `docs/testing-plan.md`, `LICENSE`, `README.md`,
`.gitignore`, the 13-skin lineup including `mantine`, and the changelog
itself. Linked it from `README.md`.

## 2026-09-05 — Confirmed commit skill provenance; skill inventory; dev env setup
Asked: Confirm whether the global `commit` skill matches the commit style
of github.com/kylegalloway/Forge (checking both `~/.claude` and
`~/src/Forge/.claude`); list available Claude Code skills and recommend
which fit this project; then set up the local dev environment — a
`.vscode` folder with recommended extensions, and a dev-requirements list
aimed at Homebrew-based installs on this MacBook.
Did: Verified via `gh api` that Forge's commit log matches the described
"chaotic/funny + emoji + cultural reference, signed, no AI attribution"
style, then found `~/src/Forge/.claude/skills/commit` and
`~/src/Forge/.claude/rules` are symlinks straight to the global
`~/.claude/skills/commit` and `~/.claude/rules/git-conventions.md` (and
that `.claude/` is gitignored in Forge, explaining an earlier 404 on
GitHub) — confirmed the global skill *is* the source of that style, no gap.
Surveyed installed (`~/.claude/skills`) and bundled skills, recommended a
subset for this project's shape (solo dev, hand-written code per
`AGENTS.md`, Rust + vanilla web, doc-heavy): `grill-with-docs` for the
open `(ADR)` questions in `PLAN.md`, `code-review`/`simplify`/
`security-review` as review-only tools compatible with the no-agent-code
rule, `claude-in-chrome` to automate the 13-skin manual test matrix,
`run` for later app verification, `write-a-skill` for iterating on
`commit` itself; flagged `tdd`/`diagnose`/`prototype` as tension with the
hand-written-code rule since they write/fix code by default.
Created `.vscode/extensions.json`, `.vscode/settings.json` (rust-analyzer
+ clippy on save, format-on-save, file/search excludes for `target`/db
files, spell-check word list), and `.vscode/tasks.json` (cargo
build/run/test/clippy, footprint measurement). Created a `Brewfile`
(sqlite, python@3.14, gh, watchexec, VS Code cask) and expanded
`docs/dev-guide.md`'s Prerequisites section with concrete install steps —
noting Rust itself should come from rustup, not Homebrew, since rustup
manages `clippy`/`rustfmt` components directly. Checked this machine's
existing tools (rustc/cargo already via rustup with clippy+rustfmt
components, sqlite/python/gh already present) and, per the user's
confirmation, ran `brew bundle`, which installed the one missing tool
(`watchexec`) and upgraded `gh`/VS Code as a side effect.

## 2026-09-05 — First commits: split the scaffold into 8 logical commits
Asked: Run the `/commit` skill to commit the accumulated scaffold.
Did: No prior commits existed. Checked for stray TODO comments in code
(none — all source files are empty) and confirmed CHANGELOG.md already
covered the full scaffold, so no changelog edit was needed. Split the
untracked scaffold into 8 small, signed (`-S`) commits by logical
grouping, each with a chaotic/funny title carrying a rotated cultural
reference per the global commit convention: (1) project meta — README,
LICENSE, .gitignore, challenge.png, Cargo.toml, Brewfile; (2) governance
docs — AGENTS.md, PLAN.md, TODO.md; (3) docs/ — agent-log.md,
corpus-verification.md, dev-guide.md, testing-plan.md, decisions/; (4)
src/ Rust scaffold; (5) web/ shell + all 13 skins; (6) data/seed/ +
scripts/; (7) CHANGELOG.md; (8) .vscode/ dev environment config.
Verified each commit's signature with `git log -1 --format="%G? %h %s"`.

## 2026-09-05 — Grill-with-docs session: resolved all open ADR questions
Asked: Run `/grill-with-docs` to work through the ADRs flagged as open in
`PLAN.md`/`TODO.md`, one question at a time with recommended answers.
Did: Created `CONTEXT.md` (glossary; defined "Footprint" as strictly the
browser-side payload, not backend deps/binary size — resolves a term used
loosely elsewhere). Wrote seven ADRs in `docs/decisions/`, each `accepted`:
- **0001-http-layer**: axum (footprint budget doesn't constrain backend
  deps, since it's scoped to the client per `CONTEXT.md`).
- **0002-moderation-threshold**: net +3 votes to confirm; admin override is
  direct DB edit only, no app-level admin auth/routes.
- **0003-vote-anti-abuse**: voter_fingerprint = first-party random-ID
  cookie; no rate limiting (accepted gap vs. a scripted abuser, low stakes);
  votes are mutable (upvote/downvote/retract), which means `votes` needs a
  unique constraint on `(entry_id, voter_fingerprint)` and an upsert, not a
  pure append-only log — flagged for `src/db/schema.sql`/`vote.rs`.
- **0004-empty-state-per-skin**: "no notable date" text is per-skin, but
  mechanically via CSS generated content keyed off a `data-empty-state`
  attribute `core.js` always renders — zero `skin.js` involvement, keeps
  skins pure-CSS per `AGENTS.md`.
- **0005-corpus-mining-source**: mine from the OPUS OpenSubtitles dump;
  only the single matched line is ever persisted (no surrounding context,
  no subtitle file redistribution) to stay within research-use terms and
  the existing "mined data is a lead, not a citation" posture.
- **0006-entry-images** (new scope, raised mid-session by the human, not
  originally in `PLAN.md`): per-entry scene images, crowdsourced alongside
  submissions, app-hosted (not hotlinked), resized/re-encoded on upload to
  a capped size/format, delivered lazily so the guaranteed per-page-load
  footprint is unaffected by whether an entry has an image. Folded into
  Phase 1 (schema: nullable `image_url`) and Phase 6 (upload handling) of
  `TODO.md` per follow-up.
- **0007-deploy-target**: clone-and-run only, no hosted deploy (accepted
  trade-off: weakens the live multi-reviewer crowdsourcing demo). DB and
  any uploaded images stay gitignored, never committed — auto-seeded from
  the existing committed `data/seed/dates.seed.json` on first run, per the
  original Phase 0/1 plan (a pre-seeded binary `.db` in the repo was
  considered and explicitly rejected because of binary-diff churn, same
  reasoning extended to never committing crowdsourced images).

Updated `PLAN.md` (open-questions section replaced with a "Decisions"
list linking all seven ADRs; footprint budget marked explicitly as a soft,
non-gating target to revisit once the app runs and once images are in
play) and `TODO.md` (every `**(ADR)**` marker replaced with a link to its
resolved decision; Phase 1/6 updated for `image_url` and upload handling;
Phase 9 updated for the clone-and-run deploy note). Updated `AGENTS.md`'s
"TBD by the human" HTTP-layer note to point at ADR-0001.

State at end of session: every question originally flagged as open in
`PLAN.md`/`TODO.md` is resolved. No code has been written (per the
no-agent-code rule) — `src/`, `Cargo.toml`, and `web/` remain the empty
scaffold from the first session. Next step per the human is to start
hand-writing code, beginning with Phase 0 (`Cargo.toml`, hello-world
`main.rs`) per `TODO.md`.
