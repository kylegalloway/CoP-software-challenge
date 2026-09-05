---
status: accepted
---

# HTTP layer: axum vs. std-only TcpListener

Context: the backend needs routing (`GET /api/date/:mm-dd`, `POST /api/submit`, `POST /api/vote`), JSON request/response handling, and static-file serving for `web/`. "Footprint," as a judged axis, is a browser-side/load-time concept (see `CONTEXT.md`), not a constraint on backend dependencies — so a backend crate never touches the budget being competed on.

Considered options:

- **axum** — routing, extractors, and JSON handling out of the box; pairs with `tower-http`'s `ServeDir` for static files. Small, widely-used dependency; still compiles to one static binary.
- **std-only (`TcpListener` + hand-rolled parsing)** — zero HTTP dependencies, but requires hand-rolling request parsing, routing, static file serving with correct content-types, and JSON content negotiation.

Decision: **axum.** Since footprint is scoped to what ships to the browser, axum costs nothing against the axis being competed on, and buys back time for corpus/skins/crowdsourcing work instead of hand-rolling HTTP parsing.
