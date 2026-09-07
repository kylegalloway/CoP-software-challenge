# Corpus verification: cross-checking beyond subtitles

Subtitle mining (`scripts/seed/mine_corpus.py`) is good for *finding*
candidate date references cheaply and at volume, but it's noisy — OCR/subtitle
errors, ambiguous phrasing ("the third" without a month), lines that aren't
actually about a calendar date, misattributed films, etc. Treat it as a lead
generator, not a source of truth.

## The rule

**An entry only reaches `confirmed` status if it has both:**

1. **A primary source** — the exact in-film reference (quote + which
   film/show + ideally a timestamp or scene description), traceable back to
   the actual media, not just the mined subtitle line.
2. **At least one independent secondary source** — something that
   corroborates the date/film pairing *without* having come from the same
   subtitle-mining pipeline.

This is what makes "largest corpus" a credible claim rather than "largest
pile of unverified regex matches." It also gives every entry a citable
`source_url` that isn't just "trust me."

## Secondary sources worth checking (non-subtitle)

- **Checkiday** (`checkiday.com`, has a free public API at
  `api.checkiday.com`) — tracks unofficial/fan holidays by date, and several
  movie-inspired ones (e.g. "Mean Girls Day") are already listed there. Since
  it's an actual API, not just a page to scrape, this is the best candidate
  for a semi-automated corroboration step in the mining/import pipeline —
  worth a real look when building step 7.
- **National Day Calendar** (`nationaldaycalendar.com`) — similar concept,
  searchable by name; good for the same category of fan-holiday corroboration.
- **Days Of The Year** (`daysoftheyear.com`) — same category, third
  independent tracker; useful when Checkiday/National Day Calendar disagree
  or one lacks an entry.
- **Wikipedia's per-date pages** (e.g. `en.wikipedia.org/wiki/October_3`) —
  many have a "Holidays and observances" section that lists fan-recognized
  days like this. Free, citable, and often has its own citation trail to
  follow further.
- **IMDb / TMDb "Quotes" and "Trivia" sections for the specific film** —
  corroborates the *exact* wording and which character/scene it's from,
  which subtitle mining alone can get slightly wrong (paraphrase, OCR typos,
  dubbed vs. original line).
- **Fan wikis (Fandom) for the property** — often have full transcripts,
  useful to confirm exact scene/line and sometimes an in-universe date vs.
  real-world release date (important distinction to keep straight).
- **Published entertainment journalism** — Mental Floss, Entertainment
  Weekly, Time, USA Today, ScreenRant, etc. regularly publish "movie-inspired
  holidays" roundups. These make great secondary-source citations because
  they're independently written, dated, and attributable.
- **TV Tropes** — not authoritative on its own (community-editable, no
  strong citation discipline) but useful for *finding* candidates worth
  chasing down elsewhere.
- **Reddit (r/todayilearned, r/movies, franchise subreddits)** — leads only,
  same caveat as TV Tropes. Never treat a Reddit thread as the secondary
  source itself; use it to find the real one.

## Suggested workflow per candidate entry

1. Mining pipeline produces a candidate: `{month, day, title, quote,
   source_url: <subtitle file ref>}`, status `pending`.
2. Before it's eligible for the submission/voting flow at all, run it
   through a corroboration pass:
   - Query Checkiday's API for that month/day; look for a matching holiday
     name.
   - Spot-check National Day Calendar / Days Of The Year if Checkiday has
     nothing.
   - Check the Wikipedia date page's "Holidays and observances" section.
   - If none of the above surface anything, fall back to a manual search for
     journalism coverage of "[Film] day [date]".
3. If a secondary source is found: attach it as `secondary_source_url`,
   entry becomes eligible for voting.
4. If no secondary source is found after a real attempt: still keep it as
   `pending`, but flag it (e.g. `needs_secondary_source: true`) so the
   submission UI can specifically ask the judges/team to help find one —
   this doubles as a way to route the gap to the small group who'll see
   it, not just a rejection.
5. Community voting (see `PLAN.md` phase 6) is *additional* signal on top of
   this, not a replacement for it — votes shouldn't be able to promote an
   entry to `confirmed` without at least one secondary source attached,
   otherwise the corpus quality bar quietly erodes over time.

## Data model implication

`entries` (per `PLAN.md`) should carry both `source_url` (primary) and
`secondary_source_url` (nullable) fields, plus the `needs_secondary_source`
flag described above. Finalize this in `src/db/schema.sql` and
`data/seed/dates.schema.json` when you get to Phase 1/7.
