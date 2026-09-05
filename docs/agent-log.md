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
