---
status: accepted
---

# Moderation threshold: net +3 votes, no app-level admin path

Context: per `docs/corpus-verification.md`, an entry needs a secondary source before it's even eligible for voting; on top of that, `PLAN.md`/`TODO.md` call for a net-vote threshold to flip `pending` → `confirmed`. Separately, the operator (solo, no login system) wants a way to insta-approve an entry without waiting on votes.

Decision:

- **Threshold: net +3 votes** (upvotes − downvotes), on entries that already carry a `secondary_source_url`. Low enough to be reachable at the vote volume a challenge submission will realistically see, high enough not to be a rubber stamp.
- **Admin override: direct DB edit only.** No admin auth, no admin routes/UI. The operator flips `status` to `confirmed` by hand via sqlite3/DB Browser when they want to insta-approve something. This is a deliberate scope cut, not an oversight — building and securing even a minimal shared-secret admin route is surface area this challenge entry doesn't need, and there's exactly one operator with direct DB access anyway.
