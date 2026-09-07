---
status: accepted
---

# Voting mechanism: first-party cookie fingerprint identifies a voter, votes are mutable

Context: `PLAN.md` calls out "no login" for voting as a deliberate choice needing a documented rationale and a `voter_fingerprint` column. This site is shown to a small known audience (judges/team), not deployed to the open internet, so the goal here is **correct basic functionality** — a person can't rack up unlimited votes by clicking repeatedly, and changing their mind (up → down → none) works — not resistance to a determined, scripted, public abuser. Needs: identify "the same voter" well enough for that, and decide what a repeat vote from the same voter does.

Decision:

- **Fingerprint = a random ID in a first-party cookie**, set on first visit. Chosen over IP-hashing because it doesn't penalize shared networks (offices, NAT, campus wifi) — the trade-off is it's defeated by clearing cookies/private browsing, which is a non-issue for this audience and explicitly not a threat model being defended against.
- **No rate limiting, and no anti-abuse hardening beyond the fingerprint.** This isn't a gap being tolerated — it's simply out of scope, because there's no adversarial public audience to defend against. The fingerprint's actual job is just to stop the mundane case (a UI button that fires "vote" without checking whether you already voted) from letting one person's repeat clicks count as many votes.
- **Votes are mutable, not append-only.** A voter can change entry_id + voter_fingerprint's vote (upvote ↔ downvote ↔ none) rather than having repeats rejected. This is the actual functional requirement — "vote again" should mean "update my vote," not "add another vote" or "get rejected." This means `votes` needs a **unique constraint on `(entry_id, voter_fingerprint)`**, and `/api/vote` does an upsert (update-in-place or delete+reinsert) rather than a pure insert — a schema/logic difference from a simple append-only vote log, worth keeping in mind when writing `src/db/schema.sql` and `src/routes/vote.rs`.
