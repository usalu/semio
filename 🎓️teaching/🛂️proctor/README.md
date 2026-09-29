# 🛂️ Proctor

The proctor is the server of the teaching quizzes. It records who learns, deals every run its randomized sheet, takes
the answers, scores whole runs, awards badges and keeps the leaderboard. It also serves the built quiz site. It is one
native binary, `proctor` (crate `teaching-proctor`, nx `@teaching/proctor`). Its only state is one SQLite file.

Inside, it is an instance of the framework server product (`🧰️framework/🛍️products/🖥️server`), built with CQRS and
event sourcing:

- The **roster** actor identifies learners.
- One **learner** actor per learner decides that learner's runs, using the quiz core's pure `decide`/`evolve`
  (`🧰️framework/🛍️products/❓️quiz`).
- The **enrollment saga** relays each roster fact to the learner it concerns.
- A checkpointed **projector** folds every committed event into the read models: learner views, run views and the
  leaderboard.

Events are the only truth. Every read model can be rebuilt from them.

## Read this first: production

The proctor speaks plain HTTP and never terminates TLS. To bind a network interface, the environment must say three
things at once, exactly like the hub:

| Variable | Value | Why |
|---|---|---|
| `PROCTOR_MODE` | `production` | Development mode binds loopback only. |
| `PROCTOR_ALLOWED_ORIGINS` | `https://quizze.architektur-und-technologie.de` | Only these origins get the CORS grant. Wildcards are refused. |
| `PROCTOR_TRUSTED_FORWARDING` | `proxy` | A TLS-terminating reverse proxy is the only client. Any request whose first `X-Forwarded-Proto` is not `https` gets `403` with `x-semio-refusal: insecure-transport`. |

If any one of the three is missing, the proctor refuses to boot and names the missing variable. Publish the port on
`127.0.0.1` only, and put Caddy (automatic TLS) or nginx in front, forwarding `X-Forwarded-Proto`. For a health check,
use `GET /instance` with `X-Forwarded-Proto: https`.

## Environment

The proctor is configured by environment variables only:

| Variable | Meaning | Default |
|---|---|---|
| `PROCTOR_PORT` | TCP port | `8791` |
| `PROCTOR_BIND` | IP address to bind (`0.0.0.0` in a container) | `127.0.0.1` |
| `PROCTOR_DATA` | Directory holding `proctor.sqlite` (created if missing) | required |
| `PROCTOR_CATALOG` | The catalog `🔣️.json`. Its quizzes resolve relative to it. | required |
| `PROCTOR_SITE` | The built site directory, served for every GET the gateway does not claim. It needs an `index.html`. | none (no site) |
| `PROCTOR_MODE` | `development` or `production` | loopback bind → development, otherwise production |
| `PROCTOR_ALLOWED_ORIGINS` | Comma-separated `scheme://host[:port]` origins | loopback bind → any loopback origin, otherwise none |
| `PROCTOR_TRUSTED_FORWARDING` | `none` or `proxy` | `none` |

## Run

| Command | What it does |
|---|---|
| `bun nx run @teaching/proctor:dev` | Serves `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` on `127.0.0.1:8791` over the git-ignored `.🧬semio/🎓️teaching/proctor-dev/`. The dev site (vite, port 6061) proxies `/instance`, `/commands`, `/queries`, `/actors` and `/scopes` to it. `PROCTOR_*` variables set by the launcher win. |
| `bun nx run @teaching/proctor:check [-- <catalog>]` | `proctor check <catalog>`: validates a catalog and every quiz it lists, and prints each quiz's revision. It exits non-zero and lists `[ERROR] <json-pointer> <code> (<quiz file>)` for every issue. |
| `bun nx run @teaching/proctor:rebuild` | `proctor rebuild`: drops every read model of the dev data directory and refolds the whole event log, with progress. |
| `bun nx run @teaching/proctor:test` | Unit tests, the storage conformance laws and the end-to-end API test. Levels are `test-quick`, `test-long` and `test-exhaustive`. |
| `bun nx run @teaching/proctor:build` | Builds the cargo artifacts. Pass `-- --release` for a deployment. |

The binary itself takes `proctor serve`, `proctor check <catalog>` or `proctor rebuild`.

`dev`, `check` and `rebuild` build the binary and run a private copy of it from the git-ignored
`.🧬semio/🎓️teaching/proctor-bin/` (one copy per run, deleted when it exits; a copy left behind by a killed launcher is
removed by the next run). A running dev proctor therefore never holds Cargo's own `proctor` executable, which Windows
would otherwise refuse to replace, so builds, tests, `check` and `rebuild` never contend with it.

At boot, the proctor:

1. Validates the catalog. An invalid catalog stops the boot and lists every issue.
2. Rebuilds the read models if they were built against a different catalog fingerprint. The fingerprint changes
   whenever a quiz file changes.
3. Relays any missing enrollment.
4. Catches the projections up with progress.
5. Listens.

Ctrl+C, Ctrl+Break, `SIGTERM` (sent by `docker stop` through tini), closing the console or a system shutdown all stop
the proctor gracefully. In-flight requests finish, the sagas and projections settle once more, and SQLite checkpoints
the WAL into `proctor.sqlite`. A catch-up or rebuild interrupted this way stops between batches. The next start resumes
from the last folded batch.

## Wire (design §9a)

Clients use the framework server contract (`@semio-tech/framework-server`), and the gateway routes sit at the origin
root:

- `POST /commands` takes a `CommandEnvelope`:
  - Its `kind` is `quiz.<type>` and its `version` is `1`.
  - `commandId` and `idempotencyKey` both equal the quiz command's `id`.
  - `tenant` and `scope` are the catalog id.
  - `target` is `{kind:"quiz-roster",id:"roster"}` for `identify-learner`, and `{kind:"quiz-learner",id:<learner>}`
    for every other command.
  - `payload` holds the UTF-8 bytes of the quiz `Command` JSON.
- A quiz rejection comes back as `{"status":"rejected","reason":{"kind":"invalid","detail":"<rejection>"}}`.
- Accepted events carry `kind: "quiz.<type>"` and the quiz `Event` JSON as their payload.
- A command id that is resubmitted answers the same receipt and produces no second event.
- `POST /queries` takes `quiz.catalog`, `quiz.learner`, `quiz.run` or `quiz.leaderboard`. Its `arguments` are the
  bytes of the quiz `Query` JSON, and it answers a `snapshot` whose `value` is the bytes of the view JSON. An unknown
  learner or run answers `404`.
- `GET /actors/<catalog>/quiz-learner/<learner>/events[?since=n]` (and `/events/ws`) replays one learner's event
  stream. The roster stream is not readable.

## Static site

Every `GET`/`HEAD` the gateway does not claim is answered from `PROCTOR_SITE`:

| Request | Answer |
|---|---|
| a file under `assets/` (Vite's `build.assetsDir`, content-hashed names) | `Cache-Control: public, max-age=31536000, immutable` |
| a document (`*.html`) or a client-side route (no extension, no file → `index.html`) | `Cache-Control: no-cache` |
| any other file (favicons, copied public files such as `🖼️assets/…`) | `Cache-Control: public, max-age=3600` |
| a missing path with an extension | `404` (a stale asset never receives HTML) |
| a path that could leave the root (`..`, encoded separators, dotfiles, device names) | `400` |
| any other method | `405` with `Allow: GET, HEAD` |

## Data, backup and restore

All state is `<PROCTOR_DATA>/proctor.sqlite`. It runs in WAL mode with synchronous commits, and holds a format row that
refuses a file of another format. Events, receipts and outbox deliveries are append-only. The read models live in the
same file and can always be rebuilt.

| Task | Steps |
|---|---|
| Backup | Stop the proctor. A clean stop folds the WAL into the file, so `proctor.sqlite` is the whole state. Copy that file, then start the proctor again. |
| Backup after a crash | If the proctor died and `proctor.sqlite-wal` still exists, either copy `proctor.sqlite`, `proctor.sqlite-wal` and `proctor.sqlite-shm` together, or start and stop the proctor once first. |
| Restore | Stop the proctor, put the file back as `<PROCTOR_DATA>/proctor.sqlite` (with no stale `-wal`/`-shm` beside it), and start the proctor. |
| Rebuild the read models | Stop the serving proctor first. The directory belongs to one serving process. Then run `proctor rebuild`. |

## Layout

| Path | Content |
|---|---|
| `🔨️modules/🗄️storage` | The four storage roles over one SQLite file: `AuthorityStore`, `ProjectionStore`, `BlobStore` and `SessionStore` |
| `🔨️modules/📚️catalog` | Catalog loading and validation, SHA-256 quiz revisions and the catalog fingerprint |
| `🔨️modules/🎭️actors` | Roster and learner deciders and the enrollment saga |
| `🔨️modules/🔭️projections` | The checkpointed projector |
| `🔨️modules/❓️queries` | The four query handlers |
| `🔨️modules/🌐️site` | Static hosting with single-page fallback, content types, cache headers and a traversal guard |
| `🔨️modules/🎚️config` | Environment configuration and production gating |
| `🔨️modules/🧩️instance` | The `ServerInstance`, module, policy templates, gate and consistency middleware, and serving |
| `🔨️modules/⌨️cli` | `serve`, `check` and `rebuild` |
| `🏗️bootstrap` | Process entry (`🦀️.rs`) and the dev launcher that runs a private copy of the built binary (`🟦️.ts`) |
| `🧫️fixtures` | A two-quiz catalog the tests play |
| `🧪️tests/🔬️conformance` | The framework storage laws, run against these stores |
| `🧪️tests/🌐️end-to-end` | The whole API over HTTP, including a restart |
