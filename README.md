# Today in Cinema

Tells you, honestly, whether today (or any given date) is a date explicitly
referenced in a movie or show — not just a release date. Built for an
internal software challenge, aiming for smallest footprint + largest corpus
+ crowdsourced growth + the same data rendered under 13 different visual
skins.

## Use of AI in this project

AI agents (Claude Code) were used throughout this project's planning, but
**all implementation code — Rust, HTML, CSS, JS, SQL, Python — was written
by hand by the human author.** This was a deliberate, enforced rule (see
[`AGENTS.md`](./AGENTS.md)), not an incidental fact.

What AI was allowed to do:

- Brainstorming and narrowing the initial concept
- Planning and scoping documents (`PLAN.md`, `TODO.md`, ADRs)
- Research (corpus-mining approach, verification methodology, prior art)
- Scaffolding empty directory/file structure
- Documentation, and reviewing hand-written code when asked

What AI was not allowed to do:

- Write, generate, or auto-complete any source code in this repo

Every interaction with an agent in this repo — planning discussions
included, not just code-adjacent ones — is logged in
[`docs/agent-log.md`](./docs/agent-log.md), append-only, so the full extent
of AI involvement is auditable.

See [`PLAN.md`](./PLAN.md) for the full design and build order,
[`TODO.md`](./TODO.md) for the detailed step-by-step checklist, and
[`AGENTS.md`](./AGENTS.md) for the full agent participation rules.

Other docs: [local dev guide](./docs/dev-guide.md),
[testing plan](./docs/testing-plan.md), and
[corpus verification methodology](./docs/corpus-verification.md) (how
mined date references get corroborated against non-subtitle sources before
being trusted).

Original challenge prompt: [`challenge.png`](./challenge.png).

## Status

Scaffold only. Nothing runs yet.

## Stack (planned)

- Backend: Rust, serving a small JSON API + static assets from `web/`.
- Frontend: no framework — one HTML shell, one core CSS/JS, and 12
  swappable skins (CSS Zen Garden pattern) under `web/skins/`.
- Storage: SQLite via `rusqlite`.

## Layout

```text
src/            Rust backend (API, static file serving, DB access)
web/            Static frontend: shell + skins
data/seed/      Corpus seed data and its schema
scripts/seed/   Offline corpus-mining/import tooling (not part of the app)
docs/decisions/ ADRs for non-obvious architectural choices
```

## Running

TBD once the backend exists.

## Changelog

See [`CHANGELOG.md`](./CHANGELOG.md).

## License

Apache License 2.0 — see [`LICENSE`](./LICENSE).
