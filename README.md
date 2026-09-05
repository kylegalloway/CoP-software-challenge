# Today in Cinema

Tells you, honestly, whether today (or any given date) is a date explicitly
referenced in a movie or show — not just a release date. Built for an
internal software challenge, entered sideways: instead of "the nicest
site," this aims for smallest footprint + largest corpus + crowdsourced
growth + the same data rendered under 13 different visual skins.

See [`PLAN.md`](./PLAN.md) for the full design and build order,
[`TODO.md`](./TODO.md) for the detailed step-by-step checklist, and
[`AGENTS.md`](./AGENTS.md) for how this repo expects AI agents to
participate (short version: planning and review only — all code here is
hand-written).

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
