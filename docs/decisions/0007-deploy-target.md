---
status: accepted
---

# Deploy target: clone-and-run only, auto-seeded from committed JSON on first boot

Context: `TODO.md` Phase 9 flagged "decide deploy target (if any)" as open. The challenge submission format doesn't require a hosted, reviewer-facing URL.

Decision:

- **No hosted deploy.** Submission is clone-and-run: a reviewer clones the repo, runs the binary locally, and README instructions cover it. No hosting infrastructure to build, pay for, or maintain.
- **Known trade-off, accepted**: this weakens the "crowdsourced growth" pitch as a live demo, since each reviewer gets their own isolated local DB rather than a shared one accumulating real votes/submissions across reviewers. The crowdsourcing mechanism is still demonstrable locally (submit + vote against your own instance), just not as a multi-reviewer shared corpus.
- **Seeding stays exactly as originally planned in `TODO.md` Phase 0/1**: the SQLite DB file itself remains gitignored, never committed, and is created fresh (auto-seeded from `data/seed/dates.seed.json`) on first run if missing. No binary DB file or uploaded image files get committed to the repo — only the plain-text/JSON seed source does. README documents both the automatic first-run seeding and how to re-run/extend it (e.g. re-seed with a larger corpus batch from Phase 7's mining output) as a manual follow-up step.

Rejected: shipping a pre-populated binary `.db` file (or a separate committed seed-DB artifact) directly in the repo. Reversed after considering the consequences — a committed binary file accumulates diff noise across regenerations, and the same reasoning extends to never committing crowdsourced image uploads (ADR-0006) into version control at all.
