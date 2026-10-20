# 🎡️ semio-tech Play

The play site of semio-tech (`@semio-tech/semio-tech-play`): every semio app live in one grid. The authored catalog `🔨️modules/🧩️runtime/🔣️.json` lists 18 groups with 149 panes (counted from the file); each pane boots one app of the framework OS in a React renderer, with a curated example where the app publishes one. The dev server listens on port `6033`; a release build assembles static pages (`build`). A German version of the developer chapter follows the last section.

| Path | What |
|---|---|
| `📋️project.json` | the neutral manifest: the targets below, their dependencies, ports, cache flags and dashboard metadata (`build` carries the release job graph) |
| `📜️script.ts` | the router of `test` (vitest, `🧪️tests/🎚️config/🟦️.ts`) |
| `🔨️modules/🧩️runtime/` | the pane catalog (`🔣️.json`), `prepare`, `activate`, `serve`, `prepare-test` and `serve-test` (`📜️script.ts`) and the activation reader (`♻️activation`) |
| `🔨️modules/📦️site/` | the release: `build`, `map-tiles`, `bundle`, `pages`, `verify` (`📜️script.ts`) and the page publication (`📄pages`) |
| `🔨️modules/🧪️e2e/` | the Playwright runner and its configuration (`🎚️config/🟦️.ts`), the release check (`📦️release`) |
| `🏗️builder/🌐️vite/`, `🟦️.tsx`, `🌐️.html` | the Vite configuration and the browser entry |
| `🧪️tests/` | unit and law tests, and `🎭️acceptance/🟦️.ts`, the one spec Playwright runs |

## Develop

Every command is a target of the project `@semio-tech/semio-tech-play` and runs through the tool-neutral build entry: `bun repo [--tool=nx|bazel] run @semio-tech/semio-tech-play:<target>[:<configuration>] [-- <arguments>]`. Nx with Bun is the default; Bazel executes the same graph with `--tool=bazel` (or `SEMIO_BUILD_TOOL=bazel`). Bazel on macOS needs an ASCII checkout path. The [build chapter](../../🧰️framework/🛍️products/🦑️repo/README.md#-build-and-run) documents both implementations, the cache rules and the recovery steps; never call `nx`, `bazel`, raw `cargo` or `rustc` yourself. In the dashboard (`bun run dashboard`, **New Task**, `Ctrl+B n`) search for `semio-tech-play` plus the operation; the dashboard command id is the `<project>:<target>` id used here, the global `build-tool` axis of its form selects Nx or Bazel, and **Open ready address** (`Ctrl+B u`) opens a ready server.

### Prerequisites

| Need | How | Needed by |
|---|---|---|
| Pinned build tools | `bun repo tools setup nx` (Bazel: `bun repo tools setup bazel`, both: `all`; `bun repo run workspace:setup` includes it) | every target |
| Dependencies | `bun repo dependencies sync --all`; after any `package.json` edit run `bun repo dependencies lock --all` first, and never a plain `bun install` | every target |
| Host check | `bun repo preflight` | before the cache partitions are created |
| Rust and wasm | no unmanaged installation: the toolchain and the wasm targets of `rust-toolchain.toml`, reached through `bun repo cargo --store=shared` by the lane tasks; a cold run compiles the components of all 35 lanes (not timed) | `prepare-prototype`, `activate-prototype` and everything after them |
| `wasm-opt` | `bun repo run workspace:deps-wasm-opt`, a prerequisite of the release job graph | `build` |
| Chromium for Playwright | `bun repo run workspace:deps-browsers` runs `playwright install chromium` into `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, which is a download; the revision is the one the installed `playwright-core` names | `prepare-e2e` and `test-release` depend on it |
| A GPU adapter | the map and wgpu panes need a real adapter: ANGLE Metal on macOS, ANGLE GL elsewhere; `PLAY_E2E_GPU=swiftshader` forces the software stack for hosts without one, and the map pane cannot be graded there | `test-e2e` |

Play needs neither Docker nor a Linux host.

### Commands

| Command (`bun repo run …`) | What it does | Cache | Port |
|---|---|---|---|
| `@semio-tech/semio-tech-play:prepare-prototype` | verifies that the 35 `@semio-tech/framework-os-dev:prepare-<lane>-react-prototype` tasks and the wasm tasks staged every runtime component into one plugin-modules root (`Incomplete play runtime component` otherwise) | yes (output-free) | |
| `@semio-tech/semio-tech-play:activate-prototype` | after `prepare-prototype` and the 35 `…:activate-<lane>-react-prototype` tasks: reads the lane receipts and writes the union receipt `dist/♻️activation/prototype` | yes | |
| `@semio-tech/semio-tech-play:dev`, `serve` | the Vite server over the activated prototype; both need `activate-prototype`, `dev` also sets `SEMIO_TECH_PLAY_PORT=6033` | never (continuous) | `6033` (`SEMIO_TECH_PLAY_PORT`) on `127.0.0.1`, or `0.0.0.0` when `DEVCONTAINER=true` |
| `@semio-tech/semio-tech-play:build` | the release as one driver: 13 jobs declared in the target (describe tool, fonts, browser support, canvas assets, five wasm engines, map tiles, the Vite bundle, the pages) published through `publishPlayPageCohortV1`; parameter `generation=fresh` | never; declares its store receipt `.🧾️store/build.json` | |
| `@semio-tech/semio-tech-play:test` | the unit and law tests; the level is an argument (`-- long`, `exhaustive`; the default is `fundamental`) | never | |
| `@semio-tech/semio-tech-play:prepare-e2e` | after `activate-prototype` and `workspace:deps-browsers`: opens an isolated service session | never | |
| `@semio-tech/semio-tech-play:serve-e2e` | after `prepare-e2e`: serves the activated prototype on a free OS port, frozen (`SEMIO_TECH_PLAY_FROZEN=true`, no file watching) and announces its address through the session | never (continuous) | `0` (OS-selected) |
| `@semio-tech/semio-tech-play:test-e2e` | after `serve-e2e`: waits for the announced address and runs Playwright (`🧪️tests/🎭️acceptance/🟦️.ts`, one chromium project, one worker, two retries when `CI` is set) against it | never | |
| `@semio-tech/semio-tech-play:test-e2e-enduser` | the same service, narrowed to the 3d panes and the language choice (`--grep "3d (en-US|de-DE)|language.*choose"`) | never | |
| `@semio-tech/semio-tech-play:test-release` | Playwright against the published release pages served statically, with fresh browser contexts and no dev server | never | |
| `@semio-tech/semio-tech-play:test-release-routing`, `release-verify` | the language-neutral release path and origin routing contracts; the release gate over the published pages | never | |

The 35 lane tasks are inferred from the playground registry: `prepare-<variant>-react-<profile>` is driver-owned and never cached (the native build driver decides freshness itself), `activate-<variant>-react-<profile>` is cached (`deterministic`) with the outputs `.cache/b/p/m/browser/<profile>/runtime/react/<variant>`. Only `prepare-prototype` and `activate-prototype` of play are cached; `activate-prototype` hashes the sources of the project and of its workspace dependencies without tests (`activationSources`) together with the outputs of its dependencies. Under Bazel a cached target is an action with its declared inputs and outputs and uses Bazel's action cache; a target whose manifest does not allow replay is executed every time.

### The chain

```bash
bun repo tools setup all                                                  # once per host
bun repo dependencies sync --all                                          # once per checkout and after every lockfile change
bun repo run @semio-tech/semio-tech-play:activate-prototype               # prepare, then activate (cached)
bun repo run @semio-tech/semio-tech-play:serve                            # http://127.0.0.1:6033 until stopped
bun repo run workspace:deps-browsers                                      # Chromium; test-e2e also depends on it
bun repo run @semio-tech/semio-tech-play:test-e2e                         # serve-e2e, then Playwright; independent of the dev server on 6033
bun repo --tool=bazel run @semio-tech/semio-tech-play:serve               # the same chain under Bazel
bun repo run @semio-tech/semio-tech-play:build --configuration=release    # the release pages
```

`test-e2e` runs its own server on a free port; the dev server on `6033` does not have to run. Reports land in `dist/reports/e2e/<session id>`, or in `🗑️generated/play-e2e/<session id>` of the ticket named by `SEMIO_TICKET_DIR`.

### Dashboard registry

The registry is data beside the manifest (`metadata.semio.dashboard` of `📋️project.json` and of the root `📋️project.json`, validated against `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json`):

| Entry | Content |
|---|---|
| readiness | `dev` and `serve` report ready on port `6033` (`SEMIO_TECH_PLAY_PORT`) |
| parameter `generation` of `build` | `fresh` sets `SEMIO_NATIVE_BUILD_GENERATION=fresh`; together with the global `build-mode=ship` (release build, both caches skipped) it reproduces a cold release build |
| global axes | `build-tool` (Nx or Bazel), `cache` (use, skip local, skip all) and `build-mode`, offered on every verb that runs build tasks ([Command registry](../../🧰️framework/🛍️products/🦑️repo/README.md#-command-registry)) |

Not yet verified: no run of this chain under either tool is recorded for this documentation. Target names, dependencies, ports, cache flags and registry entries are read from the manifest, the scripts and the Playwright configuration on 2026-10-20; the pane and group counts are counted from the catalog. Browsers of the revision `playwright-core` names were not installed on 2026-10-19, so `test-e2e` first downloads them. `test-release` and `release-verify` have no dependency on `build`: they read the pages a previous `build` published. Test counts are not stated because the spec generates its cases from the catalog. Static checks of 2026-10-20: the Vite configuration could not load under Vite's default `runner` config loader (the inventory import of the build tool opens a blob URL), so `serve`, `serve-test` and the site `build` pass `configLoader: "native"`, and it resolves with it; `playwright test --list`, started with Bun (`node` fails with `ERR_UNSUPPORTED_ESM_URL_SCHEME`), enumerates 157 tests in one file; the Playwright browsers are downloaded by `bun repo run workspace:deps-browsers`.

## 🇩🇪️ Deutsch

### Entwickeln

Jeder Befehl ist ein Target des Projekts `@semio-tech/semio-tech-play` und läuft über den werkzeugneutralen Build-Einstieg: `bun repo [--tool=nx|bazel] run @semio-tech/semio-tech-play:<target>[:<configuration>] [-- <Argumente>]`. Nx mit Bun ist der Standard; Bazel führt denselben Graphen mit `--tool=bazel` (oder `SEMIO_BUILD_TOOL=bazel`) aus. Bazel braucht unter macOS einen ASCII-Checkout-Pfad. Das [Build-Kapitel](../../🧰️framework/🛍️products/🦑️repo/README.md#-deutsch) beschreibt beide Implementierungen, die Cache-Regeln und die Wiederherstellung; rufe weder `nx`, `bazel`, rohes `cargo` noch `rustc` selbst auf. Im Dashboard (`bun run dashboard`, **New Task**, `Ctrl+B n`) suchst du nach `semio-tech-play` und der Operation; die Dashboard-Befehls-ID ist die hier verwendete ID `<Projekt>:<Target>`, die globale Achse `build-tool` im Formular wählt Nx oder Bazel, und **Open ready address** (`Ctrl+B u`) öffnet einen bereiten Server.

### Voraussetzungen

| Bedarf | Wie | Benötigt von |
|---|---|---|
| Gepinnte Build-Werkzeuge | `bun repo tools setup nx` (Bazel: `bun repo tools setup bazel`, beide: `all`; `bun repo run workspace:setup` enthält es) | jedes Target |
| Abhängigkeiten | `bun repo dependencies sync --all`; nach jeder Änderung einer `package.json` zuerst `bun repo dependencies lock --all`, nie ein einfaches `bun install` | jedes Target |
| Host-Prüfung | `bun repo preflight` | vor dem Anlegen der Cache-Partitionen |
| Rust und Wasm | keine selbst verwaltete Installation: Toolchain und Wasm-Ziele aus `rust-toolchain.toml`, von den Lane-Tasks über `bun repo cargo --store=shared` erreicht; ein kalter Lauf kompiliert die Komponenten aller 35 Lanes (nicht gemessen) | `prepare-prototype`, `activate-prototype` und alles danach |
| `wasm-opt` | `bun repo run workspace:deps-wasm-opt`, Voraussetzung des Release-Jobgraphen | `build` |
| Chromium für Playwright | `bun repo run workspace:deps-browsers` führt `playwright install chromium` nach `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` aus, ein Download; die Revision nennt das installierte `playwright-core` | `prepare-e2e` und `test-release` hängen davon ab |
| Ein GPU-Adapter | Karten- und wgpu-Panes brauchen einen echten Adapter: ANGLE Metal auf macOS, sonst ANGLE GL; `PLAY_E2E_GPU=swiftshader` erzwingt den Software-Stack für Hosts ohne GPU, die Karten-Pane lässt sich dort nicht bewerten | `test-e2e` |

Play braucht weder Docker noch einen Linux-Host.

### Befehle

| Befehl (`bun repo run …`) | Was er tut | Cache | Port |
|---|---|---|---|
| `@semio-tech/semio-tech-play:prepare-prototype` | prüft, dass die 35 Tasks `@semio-tech/framework-os-dev:prepare-<Lane>-react-prototype` und die Wasm-Tasks jede Laufzeitkomponente in ein Plugin-Module-Verzeichnis gestellt haben | ja (ohne Ausgabedateien) | |
| `@semio-tech/semio-tech-play:activate-prototype` | nach `prepare-prototype` und den 35 Tasks `…:activate-<Lane>-react-prototype`: liest die Lane-Quittungen und schreibt die Vereinigungsquittung `dist/♻️activation/prototype` | ja | |
| `@semio-tech/semio-tech-play:dev`, `serve` | der Vite-Server über dem aktivierten Prototyp; beide brauchen `activate-prototype`, `dev` setzt zusätzlich `SEMIO_TECH_PLAY_PORT=6033` | nie (dauerhaft) | `6033` (`SEMIO_TECH_PLAY_PORT`) auf `127.0.0.1`, bei `DEVCONTAINER=true` auf `0.0.0.0` |
| `@semio-tech/semio-tech-play:build` | das Release als ein Treiber: 13 im Target deklarierte Jobs (Describe-Werkzeug, Schriften, Browser-Unterstützung, Canvas-Assets, fünf Wasm-Engines, Kartenkacheln, das Vite-Bundle, die Seiten), veröffentlicht über `publishPlayPageCohortV1`; Parameter `generation=fresh` | nie; deklariert seine Store-Quittung `.🧾️store/build.json` | |
| `@semio-tech/semio-tech-play:test` | die Unit- und Gesetzestests; die Stufe ist ein Argument (`-- long`, `exhaustive`; Standard ist `fundamental`) | nie | |
| `@semio-tech/semio-tech-play:prepare-e2e` | nach `activate-prototype` und `workspace:deps-browsers`: öffnet eine isolierte Dienst-Sitzung | nie | |
| `@semio-tech/semio-tech-play:serve-e2e` | nach `prepare-e2e`: bedient den aktivierten Prototyp auf einem freien Port des Betriebssystems, eingefroren (`SEMIO_TECH_PLAY_FROZEN=true`, keine Dateibeobachtung) und meldet seine Adresse über die Sitzung | nie (dauerhaft) | `0` (vom Betriebssystem gewählt) |
| `@semio-tech/semio-tech-play:test-e2e` | nach `serve-e2e`: wartet auf die gemeldete Adresse und führt Playwright dagegen aus (`🧪️tests/🎭️acceptance/🟦️.ts`, ein Chromium-Projekt, ein Worker, zwei Wiederholungen bei gesetztem `CI`) | nie | |
| `@semio-tech/semio-tech-play:test-e2e-enduser` | derselbe Dienst, eingeschränkt auf die 3D-Panes und die Sprachwahl (`--grep "3d (en-US|de-DE)|language.*choose"`) | nie | |
| `@semio-tech/semio-tech-play:test-release` | Playwright gegen die veröffentlichten Release-Seiten, statisch bedient, mit frischen Browser-Kontexten und ohne Dev-Server | nie | |
| `@semio-tech/semio-tech-play:test-release-routing`, `release-verify` | die sprachneutralen Verträge für Release-Pfad und Origin-Routing; das Release-Tor über den veröffentlichten Seiten | nie | |

Die 35 Lane-Tasks werden aus der Playground-Registry abgeleitet: `prepare-<Variante>-react-<Profil>` gehört dem Treiber und ist nie gecacht (der native Build-Treiber entscheidet selbst über Frische), `activate-<Variante>-react-<Profil>` ist gecacht (`deterministic`) mit den Ausgaben `.cache/b/p/m/browser/<Profil>/runtime/react/<Variante>`. Bei Play sind nur `prepare-prototype` und `activate-prototype` gecacht; `activate-prototype` hasht die Quellen des Projekts und seiner Workspace-Abhängigkeiten ohne Tests (`activationSources`) zusammen mit den Ausgaben seiner Abhängigkeiten. Unter Bazel ist ein gecachtes Target eine Aktion mit ihren deklarierten Ein- und Ausgaben und nutzt Bazels Aktions-Cache; ein Target, dessen Manifest keine Wiedergabe erlaubt, wird jedes Mal ausgeführt.

### Die Kette

```bash
bun repo tools setup all                                                  # einmal je Host
bun repo dependencies sync --all                                          # einmal je Checkout und nach jeder Lockfile-Änderung
bun repo run @semio-tech/semio-tech-play:activate-prototype               # prepare, dann activate (gecacht)
bun repo run @semio-tech/semio-tech-play:serve                            # http://127.0.0.1:6033 bis zum Stoppen
bun repo run workspace:deps-browsers                                      # Chromium; test-e2e hängt ebenfalls davon ab
bun repo run @semio-tech/semio-tech-play:test-e2e                         # serve-e2e, dann Playwright; unabhängig vom Dev-Server auf 6033
bun repo --tool=bazel run @semio-tech/semio-tech-play:serve               # dieselbe Kette unter Bazel
bun repo run @semio-tech/semio-tech-play:build --configuration=release    # die Release-Seiten
```

`test-e2e` startet einen eigenen Server auf einem freien Port; der Dev-Server auf `6033` muss nicht laufen. Berichte liegen in `dist/reports/e2e/<Sitzungs-ID>` oder in `🗑️generated/play-e2e/<Sitzungs-ID>` des Tickets, das `SEMIO_TICKET_DIR` nennt.

### Dashboard-Register

Das Register sind Daten neben dem Manifest (`metadata.semio.dashboard` in `📋️project.json` und im `📋️project.json` der Wurzel, gegen `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json` geprüft):

| Eintrag | Inhalt |
|---|---|
| Bereitschaft | `dev` und `serve` melden sich auf Port `6033` (`SEMIO_TECH_PLAY_PORT`) bereit |
| Parameter `generation` von `build` | `fresh` setzt `SEMIO_NATIVE_BUILD_GENERATION=fresh`; zusammen mit der globalen Achse `build-mode=ship` (Release-Build, beide Caches übersprungen) ergibt das einen kalten Release-Build |
| globale Achsen | `build-tool` (Nx oder Bazel), `cache` (nutzen, lokal überspringen, alles überspringen) und `build-mode`, angeboten bei jedem Verb, das Build-Tasks ausführt (siehe [Befehlsregister](../../🧰️framework/🛍️products/🦑️repo/README.md#-deutsch)) |

Nicht verifiziert: Für diese Dokumentation ist kein Lauf dieser Kette unter einem der beiden Werkzeuge aufgezeichnet. Targetnamen, Abhängigkeiten, Ports, Cache-Flags und Registereinträge sind am 2026-10-20 aus Manifest, Skripten und Playwright-Konfiguration gelesen; die Pane- und Gruppenzahlen sind aus dem Katalog gezählt. Browser in der Revision, die `playwright-core` nennt, waren am 2026-10-19 nicht installiert, `test-e2e` lädt sie daher zuerst herunter. `test-release` und `release-verify` hängen nicht von `build` ab: Sie lesen die Seiten, die ein früheres `build` veröffentlicht hat. Testzahlen stehen nicht hier, weil die Spec ihre Fälle aus dem Katalog erzeugt. Statische Prüfungen vom 2026-10-20: Die Vite-Konfiguration ließ sich unter Vites Standard-Konfigurationslader `runner` nicht laden (der Inventar-Import des Build-Werkzeugs öffnet eine Blob-URL), deshalb übergeben `serve`, `serve-test` und der Site-`build` `configLoader: "native"`, und sie löst damit auf; `playwright test --list`, mit Bun gestartet (`node` scheitert mit `ERR_UNSUPPORTED_ESM_URL_SCHEME`), zählt 157 Tests in einer Datei; die Playwright-Browser lädt `bun repo run workspace:deps-browsers`.
