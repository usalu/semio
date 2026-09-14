# 🗂️ Codebase

The walk that turns a repository root into the artifacts the rest of the product reasons about:
technology and bundle detection from the layout convention, the folder and file records the walk
projects, the three exclusion rules that decide what is considered, and the emoji artifact-id and
`repo://` URI builders for folders, files, sections and definitions.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-codebase`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/codebase`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; every case of this module rests on a recorded no-oracle decision in
`🔮️oracle/🔣️.json`, because the layout convention, the artifact-id grammar and the considered-file
policy are this repository's own — the gitignore semantics the ignore case leans on are judged
against a real engine by `🏠️workspace`.

`📦️technology-bundle-detection`, `🚶️workspace-walk`, `🙈️ignore-integration`, `🪪️artifact-id-builders`.
