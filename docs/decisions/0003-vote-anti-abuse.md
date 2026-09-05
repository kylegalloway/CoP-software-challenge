---
status: accepted
---

# Vote anti-abuse: first-party cookie fingerprint, no rate limiting, votes are mutable

Context: `PLAN.md` calls out "no login" for voting as a deliberate choice needing a documented rationale and a `voter_fingerprint` column. Needs: prevent trivial double-voting, decide whether to defend against determined scripted abuse, and decide what a repeat vote from the same voter does.

Decision:

- **Fingerprint = a random ID in a first-party cookie**, set on first visit. Chosen over IP-hashing because it doesn't penalize shared networks (offices, NAT, campus wifi) — the trade-off is it's trivially defeated by clearing cookies/private browsing, which is accepted (see below).
- **No rate limiting.** A deliberate script that rotates its own cookie each request is not defended against. Accepted as a known, explicit limitation: stakes are low (net +3 threshold per `ADR-0002`, and the operator can hand-fix `status` directly in the DB regardless of vote count), and building real defense against a determined scripted abuser would mean real auth — out of scope for a no-login challenge entry.
- **Votes are mutable, not append-only.** A voter can change entry_id + voter_fingerprint's vote (upvote ↔ downvote ↔ none) rather than having repeats rejected. This means `votes` needs a **unique constraint on `(entry_id, voter_fingerprint)`**, and `/api/vote` does an upsert (update-in-place or delete+reinsert) rather than a pure insert — a schema/logic difference from a simple append-only vote log, worth keeping in mind when writing `src/db/schema.sql` and `src/routes/vote.rs`.
