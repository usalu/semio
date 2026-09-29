# 🛂️ Proctor

The proctor is the server of the teaching quizzes. It records who learns, deals every run its randomized sheet, takes
the answers, scores whole runs, awards badges and keeps the leaderboard. It is one native binary, `proctor` (crate
`teaching-proctor`, nx `@teaching/proctor`). Its only state is one SQLite file.

The proctor is **API only**. The quiz site is a static build on a CDN of its own origin
(`https://quizzes.architektur-und-technologie.de`) and calls the proctor cross-origin at
`https://proctor.quizzes.architektur-und-technologie.de`. The proctor serves the gateway routes and nothing else: any
other path answers a JSON `404` (`{"kind":"notFound",…}`).

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
| `PROCTOR_ALLOWED_ORIGINS` | `https://quizzes.architektur-und-technologie.de` | The **site origin**. Only these origins get the CORS grant. Wildcards are refused. |
| `PROCTOR_TRUSTED_FORWARDING` | `proxy` | A TLS-terminating reverse proxy is the only client. Any request whose first `X-Forwarded-Proto` is not `https` gets `403` with `x-semio-refusal: insecure-transport`. |

If any one of the three is missing, the proctor refuses to boot and names the missing variable.

Nothing but the reverse proxy may reach the proctor's port. The deployment in
`🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/` puts Caddy in front: it serves
`proctor.quizzes.architektur-und-technologie.de` with automatic TLS and forwards `X-Forwarded-Proto`. For a health
check, use `GET /instance` with `X-Forwarded-Proto: https`.

### Cross-origin API

The site calls `POST /commands` and `POST /queries` with `content-type: application/json` and no credentials, so every
call is preceded by a CORS preflight. The request gate, the outermost layer, answers every `OPTIONS` itself with `204`:

| Preflight from | Answer |
|---|---|
| An allowlisted origin | `Access-Control-Allow-Origin: <that origin>`, `Access-Control-Allow-Credentials: true`, `Access-Control-Allow-Methods: GET, POST, HEAD, OPTIONS`, `Access-Control-Allow-Headers: content-type`, `Access-Control-Max-Age: 7200` (the browser reuses the grant for two hours instead of asking before every request), `Vary: Origin` |
| Any other origin | `204` with `Vary: Origin` and no `Access-Control-Allow-Origin` or max-age, so the browser blocks the call |

Actual responses carry the same grant (or none). The origin is always echoed, never `*`. The preflight also needs
`X-Forwarded-Proto: https`, which Caddy adds. In development (loopback bind, no allowlist), any loopback origin is
admitted, for example the Vite dev site on `http://localhost:6061`.

## Environment

The proctor is configured by environment variables only:

| Variable | Meaning | Default | Production (`proctor.quizzes.architektur-und-technologie.de`) |
|---|---|---|---|
| `PROCTOR_PORT` | TCP port | `8791` | `8791` |
| `PROCTOR_BIND` | IP address to bind | `127.0.0.1` | `0.0.0.0` (the container's own interface) |
| `PROCTOR_DATA` | Directory holding `proctor.sqlite` (created if missing) | required | the data volume |
| `PROCTOR_CATALOG` | The catalog `🔣️.json`. Its quizzes resolve relative to it. | required | the architecture catalog baked into the image |
| `PROCTOR_MODE` | `development` or `production` | loopback bind → development, otherwise production | `production` |
| `PROCTOR_ALLOWED_ORIGINS` | Comma-separated `scheme://host[:port]` origins (at most 32) | loopback bind → any loopback origin, otherwise none | `https://quizzes.architektur-und-technologie.de` (the site origin) |
| `PROCTOR_TRUSTED_FORWARDING` | `none` or `proxy` | `none` | `proxy` |
| `PROCTOR_PRESENCE_TICK_MS` | How often a presence room sends its coalesced `batch`, 10 to 1000 ms | `100` | `100` |

## Run

| Command | What it does |
|---|---|
| `bun nx run @teaching/proctor:dev` | Serves `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` on `127.0.0.1:8791` over the git-ignored `.🧬semio/🎓️teaching/proctor-dev/`. The dev site (Vite, port 6061) proxies `/instance`, `/commands`, `/queries`, `/actors` and `/scopes` to it, so in development the client stays same-origin. `PROCTOR_*` variables set by the launcher win. |
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
2. Rebuilds the read models if they were built against a different catalog fingerprint or by an earlier projector
   revision. The fingerprint changes whenever a quiz file changes; the revision whenever the proctor starts keeping a
   read model differently (revision 2 added the crowd).
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
- `POST /queries` takes `quiz.catalog`, `quiz.learner`, `quiz.run`, `quiz.leaderboard` or `quiz.crowd`. Its
  `arguments` are the bytes of the quiz `Query` JSON, and it answers a `snapshot` whose `value` is the bytes of the view
  JSON. An unknown learner, run or quiz answers `404`.
- `quiz.crowd` (`{"type":"crowd","quiz":…}`) answers what the learners answered in the submitted runs of one quiz, the
  `CrowdView` (design §17): per task (and matching dimension) and item, the counts per category or value, or the mean
  normalized position of a sorting item. It exists for every quiz of the catalog, with `runs: 0` before the first
  submission. The proctor keeps it incrementally, folding each `run-submitted` result into a stored per-quiz tally,
  and it equals the quiz core's `crowd_view` over every submitted result byte for byte.
- `GET /actors/<catalog>/quiz-learner/<learner>/events[?since=n]` (and `/events/ws`) replays one learner's event
  stream. The roster stream is not readable.
- Any other path answers the gateway's JSON `404`; a known route with another method answers `405` with its `Allow`.

## Presence (design §15–§17)

Every learner sees who else is online, where they are, their cursors and what they drag, and — in a run — what they
currently think. This state is **ephemeral and shared**: the proctor keeps it in memory only, never writes it to
SQLite, and forgets a learner the moment their socket closes.

Clients open `GET /scopes/<scope>/presence/ws?surface=<surface>` as a WebSocket with the subprotocol
`semio.presence.v1` (in TypeScript, `presenceSocketUrl(baseUrl, scope, surface)` from
`@semio-tech/framework-server`). The scope is percent-encoded, so `/` becomes `%2F`. The rooms of a catalog are:

| Scope | State shared there | Who joins |
|---|---|---|
| `<catalog>` | `PresenceState`: public tag, identity, place, whether the tab is visible | every learner past the identity screen |
| `<catalog>/introduction`, `<catalog>/home`, `<catalog>/leaderboard`, `<catalog>/badges` | `CursorState`: public tag, cursor relative to an anchor, keyboard focus; no drag | learners on that page |
| `<catalog>/quiz/<quiz>` | `CursorState`, anchors may be items (`item:<id>`) and categories (`category:<id>`), `drag` names an item of the quiz | learners on the quiz page, in a run or on the results of that quiz |
| `<catalog>/quiz/<quiz>/thinking` | `ThinkingState`: public tag and the draft answers per task — classification and sorting by item and category ids, matching as the values assigned per dimension | learners in a run of that quiz |

The protocol:

1. On join, the server sends `welcome`: the socket's session id (random, generated by the server), its colour slot and
   the room's roster, the new session included.
2. The client sends `{"type":"state","state":…}` whenever its state changes. The server keeps only the latest state of
   each session. A state may be at most 2 KiB, and at most 30 per second are taken; the excess is dropped.
3. Every tick (`PROCTOR_PRESENCE_TICK_MS`, 100 ms by default), each room sends one `batch` with the sessions that
   changed and the sessions that left, so traffic grows with the number of learners who move, not with how often
   they move.
4. A state the proctor does not admit is answered with `{"type":"refused","reason":…}` and not shared. The socket
   stays open.
5. The server pings every 20 s. A socket silent for 60 s is closed, and its session leaves the room.
6. A socket may also **watch** up to 16 other rooms read-only, for example the home page watching every page, quiz
   and thinking room behind its cards: `{"type":"watch","scopes":[…],"intervalMs":250}` replaces the watched set (an
   empty list stops watching). The server answers each newly watched scope with a `watched` frame flagged
   `"snapshot":true` holding its whole roster, then at most one `watched` frame per scope and interval with the
   sessions that changed and those that left. The interval is clamped to at least the tick. Watching never joins: the
   watcher appears in no roster and publishes only to the room it joined. A watcher that falls behind a room gets a
   fresh snapshot of it.

What is admitted:

- **Joining and watching** need the `quiz-presence` policy template (`join`, `publish`, `watch`). Every caller holds
  it inside the catalog's rooms and nowhere else, so a socket to another catalog's room or to an unknown quiz is
  refused with `403`, and a watch naming such a scope is answered `refused` with `forbidden <scope>` and changes
  nothing; more than 16 scopes are refused with `watch-too-many`. A `surface` longer than 64 characters is refused
  with `400`.
- **Origin.** A socket whose `Origin` the cross-origin policy does not admit is refused with `403`: in production only
  the site origin, in development any loopback origin. Behind the proxy, the handshake also needs
  `X-Forwarded-Proto: https`, like every request. Caddy's `reverse_proxy` forwards WebSocket upgrades as they are.
- **States** must be the room's type and pass the quiz core's rules (`presence_problem`, `cursor_problem`,
  `thinking_problem`), and they may name only what the catalog renders: a place names a quiz of this catalog and a task
  of that quiz; a drag names an item of the room's quiz; a draft names tasks of the quiz with answers of their kind,
  their items, categories and dimensions, and for matching only values a card of that dimension shows. The refusal
  reason is the issue code and its JSON pointer (`tag-invalid /tag`, `quiz-unknown /place/quiz`,
  `id-unknown /drag/item`, `kind-mismatch /answers/<task>`, `value-unknown /answers/<task>/values/<dimension>/<item>`),
  `state-invalid` for a state of the wrong shape, `state-too-large`, or `frame-invalid` for anything but a `state` or
  `watch` frame.

The quizzes are for fun: since design §17 learners see what the others think and drag while they play, and what they
answered once submitted (`quiz.crowd`). Presence never carries a learner id, only the public tag.

## Data, backup and restore

All state is `<PROCTOR_DATA>/proctor.sqlite`. It runs in WAL mode with synchronous commits, and holds a format row that
refuses a file of another format. Events, receipts and outbox deliveries are append-only. The read models live in the
same file and can always be rebuilt.

| Task | Steps |
|---|---|
| Backup | Stop the proctor. A clean stop folds the WAL into the file, so `proctor.sqlite` is the whole state. Copy that file, then start the proctor again. |
| Backup on Docker | `docker compose stop proctor`, copy `proctor.sqlite` out of the proctor's data volume (`PROCTOR_DATA`, `/srv/quiz/data` in the image), for example with `docker compose cp proctor:/srv/quiz/data/proctor.sqlite ./proctor-<date>.sqlite`, then `docker compose start proctor`. The Caddy volumes hold only certificates and can be re-issued. |
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
| `🔨️modules/👪️crowd` | The incremental per-quiz crowd tally and its `CrowdView` |
| `🔨️modules/❓️queries` | The five query handlers |
| `🔨️modules/🎚️config` | Environment configuration and production gating |
| `🔨️modules/👥️presence` | The presence rooms of the catalog and the admission of the states shared in them |
| `🔨️modules/🧩️instance` | The `ServerInstance`, module, policy templates, the request gate (transport trust, CORS, preflight max-age), presence wiring, consistency middleware, and serving |
| `🔨️modules/⌨️cli` | `serve`, `check` and `rebuild` |
| `🏗️bootstrap` | Process entry (`🦀️.rs`) and the dev launcher that runs a private copy of the built binary (`🟦️.ts`) |
| `🧫️fixtures` | A two-quiz catalog the tests play |
| `🧪️tests/🔬️conformance` | The framework storage laws, run against these stores |
| `🧪️tests/🌐️end-to-end` | The whole API over HTTP, including a restart, the cross-origin API behind the gate, presence, watching and thinking with real WebSocket clients, and the crowd after submissions |
