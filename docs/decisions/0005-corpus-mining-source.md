---
status: accepted
---

# Corpus mining source: OPUS OpenSubtitles dump, single-line excerpts only

Context: `PLAN.md`/`TODO.md` flag the subtitle corpus source for `scripts/seed/mine_corpus.py` as needing an ADR with licensing notes. Mined output is explicitly a *lead*, not a source of truth (`docs/corpus-verification.md`) — every candidate needs an independent secondary source before it can reach `confirmed`, and the mining step exists to find candidates cheaply at volume, not to be the thing ultimately cited.

Decision:

- **Source: the OPUS project's OpenSubtitles corpus dump** (opus.nlpl.eu), a cleaned, bulk-downloadable aggregation of opensubtitles.org data intended for NLP/research use. Chosen over hand-curation-only because bulk mining is the actual growth engine behind the "largest corpus" competitive axis in `PLAN.md`.
- **Only the single matched line is ever persisted** into `data/seed/dates.seed.json` / `entries.quote` — no surrounding context, no full subtitle file storage or redistribution. The mining script reads the dump to *find* matches; what gets written out and eventually published is a short, attributed excerpt (the line itself, plus title/source), never a scene transcript or the source file. This keeps published output within a quote/citation-shaped boundary rather than bulk redistribution of research-use-licensed data, and matches the existing corroboration workflow where the excerpt is a lead to verify, not the citation itself — the `secondary_source_url` from an independent, non-subtitle source is what actually backs the published claim.
