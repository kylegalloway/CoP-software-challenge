# Local development guide

## Prerequisites

One-time machine setup, macOS/Homebrew-first:

1. **Rust, via `rustup`** — *not* `brew install rust`. Install from
   [rustup.rs](https://rustup.rs) (`curl --proto '=https' --tlsv1.2 -sSf
   https://sh.rustup.rs | sh`). This is the one exception to "prefer
   Homebrew": rustup manages toolchains and components (`clippy`,
   `rustfmt`) directly, which the Homebrew `rust` formula doesn't do well.
   After installing, run:

   ```sh
   rustup component add clippy rustfmt
   ```

   Check with `cargo --version`, `cargo clippy --version`, `cargo fmt
   --version`.
2. **Everything else, via Homebrew** — run `brew bundle` from the repo
   root. This installs, per the [`Brewfile`](../Brewfile):
   - `sqlite` — `sqlite3` CLI, for inspecting `data/app.db` directly.
   - `python@3.14` — runs `scripts/seed/mine_corpus.py` (Phase 7 corpus
     mining). Not needed to build or run the site itself.
   - `gh` — GitHub CLI.
   - `watchexec` — optional generic file-watcher for auto-rebuild/reload
     (e.g. `watchexec -e rs -- cargo run`); `cargo install cargo-watch` is
     an alternative if you'd rather stay inside `cargo`.
   - `visual-studio-code` cask, if not already installed.
3. **VS Code** — open the repo folder and accept the recommended
   extensions prompt (from `.vscode/extensions.json`), or install manually:
   `rust-analyzer` (language server + inline errors), `vscode-lldb`
   (debugger, pairs with rust-analyzer's "Debug" code lens), `Even Better
   TOML` (`Cargo.toml`), `SQLite Viewer` (browse `data/app.db` without
   leaving the editor), `Live Server` (quick static preview of `web/`
   skins independent of the Rust server), plus `EditorConfig`,
   `markdownlint`, `Markdown All in One`, `Code Spell Checker`, and
   `GitLens` for the doc-heavy parts of this repo. `.vscode/settings.json`
   wires rust-analyzer to run clippy on save and format-on-save via
   rustfmt; `.vscode/tasks.json` has build/run/test/clippy/footprint tasks
   available from the Command Palette ("Tasks: Run Task").
4. A recent browser for manual skin testing. No build step for the
   frontend — it's plain HTML/CSS/JS, edit and refresh.

No Node runtime is required anywhere in this project.

## First run

1. `cargo build` — confirms the toolchain and `Cargo.toml` deps are sane.
2. Run the binary (exact invocation depends on how you wire up `main.rs`;
   suggest supporting a `--seed` flag or env var to trigger the one-time
   import of `data/seed/dates.seed.json` into `data/app.db` on first boot).
3. Confirm `data/app.db` gets created (it's gitignored — never commit it).
4. Visit `http://localhost:<port>/` (pick and document the port once
   decided — suggest something memorable, e.g. `4038` for no particular
   reason, or just `8080`).

## Day-to-day workflow

- **Backend changes** (`src/`): `cargo run` picks up changes on rebuild;
  there's no hot-reload by default. If you want it, `cargo install
  cargo-watch` and run `cargo watch -x run` — optional, not required.
- **Frontend/skin changes** (`web/`): since these are static files served
  as-is, a browser refresh is enough — no rebuild needed. If the server
  caches static files aggressively, disable caching in dev or hard-refresh.
- **Schema changes** (`src/db/schema.sql`): during early development,
  simplest path is deleting `data/app.db` and letting it recreate + reseed
  on next run. Once there's real crowdsourced data worth keeping locally,
  switch to writing an actual migration instead of nuking the file.
- **Adding a skin**: create `web/skins/<slug>/skin.css` (+ optional
  `skin.js`), add `{slug, label}` to `web/skins/registry.json`. Should
  require touching nothing else — if it does, something's wrong with the
  skin mechanism (see `PLAN.md`).

## Inspecting the database directly

```sh
sqlite3 data/app.db
.tables
.schema entries
select * from entries where status = 'pending';
```

Useful for checking moderation state and vote counts without going through
the API.

## Common local scenarios to poke at manually

- Today's date has a confirmed entry → renders correctly.
- Today's date has zero entries → honest empty state renders (in every
  skin you've built so far, not just the default).
- A `pending` entry with no `secondary_source_url` → shouldn't show up in
  the default confirmed view, per `docs/corpus-verification.md`.
- Switching skins via the picker → URL/localStorage state updates, correct
  `skin.css`/`skin.js` loads, nothing else on the page changes.

## Environment/config

Document actual env vars/flags here once they exist (e.g. port, DB path,
moderation threshold if made configurable rather than hardcoded). Keep this
section current — it's the first place you'll look after not touching the
project for a week.
