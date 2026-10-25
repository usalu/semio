# 🦀️ Print (native)

The native package of the [print product](../README.md) (`@semio-tech/print-rs`, crate `semio-framework-print`, library `../🦀️.rs`): event-sourced print chart mutations and native LaTeX inference. It registers executable mutation and inference services in the OS registry; its TikZ inference delegates the numerical grammar operations to the LaTeX implementation of the product.

## Commands

Every command is a target of the project `@semio-tech/print-rs` and runs through the tool-neutral build entry
(`repo run <project>:<target>`; Bazel with `repo --tool=bazel run …`). In the dashboard (`repo dashboard`,
**New Task**, `Ctrl+B n`) search `print-rs` plus the action. Never call `cargo` yourself.

| Command | What it does | Cache |
|---|---|---|
| `repo run @semio-tech/print-rs:build` | builds the native package | yes |
| `repo run @semio-tech/print-rs:test` | the package tests and the `semio-framework-print-tests` suite | yes |
| `repo run @semio-tech/print-rs:test-snapshot-sqlite` | chart snapshots through SQLite, native and shared source (`-native` and `-source` run one side) | yes |
| `repo run @semio-tech/print-rs:test-logical-owner` | checks the canonical chart logical-owner assets of the SQLite snapshot | yes |
| `repo run @semio-tech/print-rs:produce-logical-owner` | regenerates the logical-owner chart fixtures in the workspace | never |
| `repo run @semio-tech/print-rs:test-native-emission` | native chart source compiled by the print toolchain and the canonical mutation vectors | yes |

## Chain

The TypeScript side of the product (`repo run @semio-tech/print:test -- quick`) executes the schema, replay, D3
arithmetic and rendering checks of the authored chart inference; this package proves the native services. Run `produce-logical-owner` only after a deliberate change of the logical-owner contract and review
the diff.

## 🇩🇪️ Deutsch

### Befehle

Jeder Befehl ist ein Target des Projekts `@semio-tech/print-rs` und läuft über den werkzeugneutralen Build-Einstieg (`repo run <Projekt>:<Target>`; Bazel mit `repo --tool=bazel run …`). Im Dashboard (`repo dashboard`, **New Task**, `Ctrl+B n`) `print-rs` plus die Aktion suchen. `cargo` nie selbst aufrufen.

| Befehl | Wirkung | Cache |
|---|---|---|
| `repo run @semio-tech/print-rs:build` | baut das native Paket | ja |
| `repo run @semio-tech/print-rs:test` | Paket-Tests und die Suite `semio-framework-print-tests` | ja |
| `repo run @semio-tech/print-rs:test-snapshot-sqlite` | Diagramm-Snapshots über SQLite, nativ und gemeinsame Quelle (`-native` und `-source` führen je eine Seite aus) | ja |
| `repo run @semio-tech/print-rs:test-logical-owner` | prüft die kanonischen Chart-Assets des logischen Owners im SQLite-Snapshot | ja |
| `repo run @semio-tech/print-rs:produce-logical-owner` | erzeugt die Chart-Fixtures des logischen Owners im Workspace neu | nie |
| `repo run @semio-tech/print-rs:test-native-emission` | native Diagrammquelle, von der Print-Toolchain kompiliert, und die kanonischen Mutationsvektoren | ja |

`produce-logical-owner` nur nach einer bewussten Änderung des Vertrags des logischen Owners ausführen und die Differenz prüfen. Die TypeScript-Seite (`repo run @semio-tech/print:test -- quick`) führt die Schema-, Replay-, D3-Arithmetik- und Rendering-Prüfungen der Diagramm-Inferenz aus.
