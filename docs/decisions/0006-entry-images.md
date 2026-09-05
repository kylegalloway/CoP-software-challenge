---
status: accepted
---

# Per-entry scene images: app-hosted uploads, capped + re-encoded, lazy-loaded

Context: entries currently carry only text (quote + source). A scene image per entry would meaningfully improve the site's appeal ("really make the site useful and pop"), but images are the one asset type that can blow the "smallest footprint" pitch (`CONTEXT.md`) by orders of magnitude if unconstrained, and this wasn't part of the original planned scope (`TODO.md` has no upload infrastructure).

Decision:

- **Source: crowdsourced**, submitted alongside a quote through the same submission flow, riding the same `pending`/`confirmed` moderation pipeline as the quote itself. This is user-attested content, not independently verified — same trust level as `source_url` today, not a stronger claim.
- **Storage: app-hosted upload**, not a hotlinked URL. The app receives and stores the image (e.g. under a gitignored `data/uploads/` dir) rather than trusting a third-party link — avoids depending on external hosts staying up and, critically, lets the app control size/format rather than a submitter dictating it.
- **Constrained on upload**: capped to a max dimension (e.g. 800px wide) and re-encoded to a single consistent format (e.g. JPEG/WebP), regardless of what was submitted. Every stored image is bounded by construction.
- **Lazy/opt-in delivery**: the base API response carries only image metadata (e.g. `entries[].image_url` or `null`); the actual image bytes are fetched separately (`loading="lazy"` and/or fetched only once revealed), never bundled into the guaranteed per-page-load payload. This is what actually protects the footprint pitch — an image existing doesn't change the size of the HTML+CSS+JS+JSON that every page load already costs; it becomes a deferred, optional asset instead of a mandatory one.

Consequences: adds real scope not in the original `TODO.md` — an upload-handling route, on-upload image processing (resize/re-encode), and disk storage/serving, none of which existed in the original axum+SQLite-only plan. `src/db/schema.sql` needs a nullable `image_url` (or similar) on `entries`; `data/seed/dates.schema.json` needs the same if seed entries can carry one. Revisit whether this belongs in an early phase or is deferred to post-MVP polish when re-sequencing `TODO.md`.
