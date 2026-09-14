# Summary

The coordinator: an append-only event log, its crash-recovery protocol, the projections replayed from it and the HTTP surface the repo CLI talks to. Shipped twice — `📦️packages/🐹️go` and `📦️packages/🦀️rust` — plus the Next.js dashboard and API facade in `📦️packages/🟦️typescript`.

# Docs

The CLI sends unified diffs or file snapshots; the coordinator parses them, reindexes affected files, updates claims, and emits conflict warnings and precommit blockers.
HTTP endpoints cover ticket lifecycle commands, diff ingestion, indexing, event ingestion (`/events`, `/api/v1/events`), the GitHub webhook (`/webhooks/github`) and read-only queries for warnings, breachs and scopes.
Every append is staged (`.next`), published (`.stage`), backed up (`.backup`) and only then swapped in, each step fsynced together with its parent directory, so an interruption at any phase leaves exactly the previously committed log behind.
Both implementations are held to one contract by the language-agnostic cases under `🧪️tests/`; deployment files live in `🚀️deploy/`.

# 💯️Requirements
