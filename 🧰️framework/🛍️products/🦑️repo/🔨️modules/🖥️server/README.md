# 🖥️ Server

The server side of the repository: currently the `🎛️coordinator`, which owns the canonical
append-only JSONL event store and its SHA-256 checksums, the durability targets per operating system,
the HTTP API, the projection and the webhooks. Deployment files live under its `🚀️deploy/`.

## 📦️ Packages

- `🎛️coordinator/📦️packages/🦀️rust` — `semio-framework-repo-coordinator`
- `🎛️coordinator/📦️packages/🐹️go` — `github.com/usalu/semio/repo/coordinator`, entry point in `📦️main`
- `🎛️coordinator/📦️packages/🟦️typescript` — the Next.js application

## 🧪️ Tests

The cases live with their owner in `🎛️coordinator/🧪️tests/`, one `🥒️.feature` per case with an
adapter per implementation, run through the `🧪️test` harness.
