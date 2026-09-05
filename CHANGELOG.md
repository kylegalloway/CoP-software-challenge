# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
once there's a first release.

## [Unreleased]

### Added
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
