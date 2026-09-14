# 🚚️ Move

Everything that relocates text: moving and copying files and folders together with the `AGENTS.md`
headings that name them, integrating a source file into a target file's section, extracting a section
back out into its own file, moving a section within a file, and the repository-wide token rename
across all three case foldings.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-move`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/move`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; there is no third party that shares this repository's section grammar, casing
folding or documentation-index convention, so every case rests on a recorded no-oracle decision in
`🔮️oracle/🔣️.json` backed by specification vectors and round-trip metamorphic laws.

`🚚️file-folder-move`, `📥️file-integrate`, `🧲️section-extract`, `📑️section-move`, `🔤️rename-casings`.
