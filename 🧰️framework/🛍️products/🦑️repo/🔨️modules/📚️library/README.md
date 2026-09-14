# 📚️ Library

The TypeScript tooling every repo package script builds on: the `BundleScript` / `ScriptRouter`
foundation, the workspace discovery and normalisation, the build, lint and test runners, the budgeted
process helpers, the repo implementation switch (`SEMIO_REPO_IMPLEMENTATION`) and the nx plugin.

## 📦️ Packages

- `📦️packages/🟦️typescript` — `🟦️.ts`, imported by every `📜️script.ts` in the product

## 🧪️ Tests

The cases under `🧪️tests/` specify the tooling contracts themselves — discovery, mutation inventory,
reference coordinates, cargo and nextest projection, taxonomy compilation, transaction recovery and
the rest — each as one `🥒️.feature` with a TypeScript adapter, run through the `🧪️test` harness.
