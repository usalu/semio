# 🌎️ `os-hub` — operating the collaboration hub

The hub is the server half of semio: identity and tenancy (the *directory*), the durable
event-sourced document store, presence and the collaboration sockets two browsers meet on, plus the
artifact-authority and hub-backed inference lanes. It is a single native binary, `os-hub`, with no
runtime service dependencies beyond a data directory on disk.

This page is for whoever runs it. For how it is built and what lives where, read the module sources;
for the end-user AI integration read
[`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md`](../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md).

## Read this first — what `os-hub` can and cannot do today

`os-hub` has **two** topologies, and they want completely different things from you.
`validate_auth_startup` (`🏗️bootstrap/🦀️.rs`) is the gate that decides which one you are in:

| you run | mode chosen | result |
|---|---|---|
| `OS_HUB_MODE=production`, `OS_HUB_CREDENTIAL_SIGN_IN=true`, `OS_HUB_ADMIN_SUBJECTS=…`, loopback bind | `production` | **boots as a plain process** — no launcher, no fd 3 |
| the same plus `OS_HUB_BIND=10.0.0.4`, `OS_HUB_ALLOWED_ORIGINS=…`, `OS_HUB_TRUSTED_FORWARDING=proxy` | `production` | **boots and answers on the network**, behind the proxy those two variables describe |
| `OS_HUB_MODE=production` with none of the above | `production` | refuses, naming the one thing that is missing |
| `os-hub` with no `OS_HUB_MODE` at all, default `OS_HUB_BIND=0.0.0.0` | `production` (bind is not loopback) | refuses — a network bind needs the allowlist and the proxy statement |
| `OS_HUB_BIND=127.0.0.1`, no `OS_HUB_MODE` | `development` | boots **only if it inherits the local-bootstrap socket on fd 3** |
| `OS_HUB_MODE=development`, non-loopback bind | `development` | refuses — `development mode must bind to loopback` |

**Production mode used to be unreachable**, and older notes in this repository still say so. Its only
identity requirement was an external `IdentityAssertionVerifier` adapter — of which this repository
ships none, and `main` passed a hardcoded `None` — so every `OS_HUB_MODE=production` died on the
first check. That was never the real rule. What production needs is *an identity authority*, and the
hub grew its own: `os-hub credential set` seeds the first user with no server running, and
`POST /auth/sessions` verifies the password and mints the session. `OS_HUB_CREDENTIAL_SIGN_IN=true`
is therefore enough, and the `IdentityAssertionVerifier` stays what it always was — the seam for an
*external* IdP, optional and unimplemented.

A **network bind** is admitted in production, and only under three statements at once, because each
one is something the hub cannot work out for itself:

1. `OS_HUB_MODE=production` — you are not asking for the launcher-supervised dev topology;
2. `OS_HUB_ALLOWED_ORIGINS=…` — which browser origins may hold a credentialed session
   ([Cross-origin access](#cross-origin-access));
3. `OS_HUB_TRUSTED_FORWARDING=proxy` — a TLS-terminating reverse proxy is the **only** thing that
   can reach this socket. The hub speaks plain HTTP and always will; this is you taking
   responsibility for the transport, and it switches on the `X-Forwarded-Proto` enforcement in
   [TLS-terminating reverse proxy](#tls-terminating-reverse-proxy).

Any one of them missing is a refusal that names it. Get the third one wrong — publish the port
without a proxy in front — and you have published bearer tokens in cleartext; the hub cannot detect
that for you.

**Development mode** is unchanged and is still the dev loop's own topology: it requires a
`LocalBootstrapTransport`, which `🚀️local-bootstrap/🦀️.rs` obtains from **inherited file descriptor 3**
(`INHERITED_BOOTSTRAP_DESCRIPTOR: i32 = 3`) and then runs a keyed hello exchange over. A bare
`os-hub` started by `systemd`, `docker run` or a shell gets no fd 3 and exits, so development mode
runs as a supervised child of `🚀️local-bootstrap/🏃️execution/🟦️.ts`'s `startLocalHub`, on loopback
only. Use it when you want the launcher's local credentials and the `s`/native/MCP children; use
production mode when you want a server.

## Build

```bash
bun nx run os-hub:build        # cargo build --release --bin os-hub → dist/build
bun nx run os-hub:build-dev    # dev profile, what the dev launcher execs
```

`build` is the release target (`📦️packages/🦀️rust/📜️script.ts`, `BuildScript` →
`["--release", "--bin", "os-hub"]`). Optional cargo features select the swappable backends:
`sqlite` (on by default for the directory), `postgres`, `neo4j`, `native-artifact-execution`. A
backend named by an env var whose feature was not compiled in is a boot error naming the backend.

## Run it

### As a server (production mode)

No launcher, no fd 3 — just the binary. Seed the first user first; the verb needs the data root, not
a running hub.

```bash
export OS_HUB_DATA=/srv/semio-hub/data
os-hub credential set --email ada@example.com --display-name "Ada"   # password on stdin

# loopback: reach it through a reverse proxy on the same host
OS_HUB_MODE=production \
OS_HUB_BIND=127.0.0.1 \
OS_HUB_CREDENTIAL_SIGN_IN=true \
OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com \
os-hub

# on a network interface: the allowlist and the proxy statement are both mandatory
OS_HUB_MODE=production \
OS_HUB_BIND=10.0.0.4 \
OS_HUB_CREDENTIAL_SIGN_IN=true \
OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com \
OS_HUB_ALLOWED_ORIGINS=https://s.example.com \
OS_HUB_TRUSTED_FORWARDING=proxy \
os-hub
```

The startup line reports the three postures it resolved:
`[INFO] bind scope network (10.0.0.4:8787), cross-origin policy allowlist, trusted forwarding proxy`.

> Production mode has been run, on a network bind, against a release binary. 2026-09-21: the
> `dist/build/os-hub` release binary, started as a plain process on `192.168.178.70:7661` with
> exactly the variables above, answered `/healthz` and `/readyz` (`status: ready`, every gate open),
> refused the same requests `403 x-semio-refusal: insecure-transport` without `X-Forwarded-Proto`,
> admitted a credential sign-in and refused a wrong password, closed a held socket cleanly on
> `SIGTERM` and exited `0`, restarted onto the same sqlite root as the same user, and adopted a
> trusted catalog a different binary had published (9/9 resolved). Transcript: ticket 26/09/18,
> `📓️rb1-release-builds-and-production-posture.md`. What is still unobserved is everything *behind*
> a real reverse proxy — the proxy itself is simulated here by sending the headers it would send.

### As the dev loop (development mode)

```bash
bun nx run os-hub:dev                 # loopback hub, fd-3 supervised, data root ./.🧬semio/🌐hub
bun nx run os-hub:dev-secure-suite    # hub + React `s` + native + MCP children, one credential each
```

`dev` publishes a trusted catalog into the data root on first run if none is there yet (it
materializes the stdio+GIS bundle and validates it) — that step costs a native compile, not a
database migration, and until it completes `/readyz` reports `artifactAuthority` closed.

## Environment variables

Every variable below is read directly with `std::env::var` — there is no config file, no `.env`
loading and no schema/validation layer. Defaults are the literal fallbacks in the source.

### Process and mode

| variable | default | meaning |
|---|---|---|
| `OS_HUB_PORT` | `8787` | TCP port. An unparseable value silently falls back to the default. |
| `OS_HUB_BIND` | `0.0.0.0` | Bind address. Must parse as an IP or boot fails. Development mode refuses anything but loopback; production admits a network interface together with `OS_HUB_ALLOWED_ORIGINS` and `OS_HUB_TRUSTED_FORWARDING=proxy` (see above). The bind also decides the default cross-origin posture. |
| `OS_HUB_ALLOWED_ORIGINS` | unset → loopback-development when the bind is loopback, closed otherwise | Comma-separated `scheme://host[:port]` origins admitted for *credentialed* cross-origin browser requests; max 32, each entry must be a serialized origin (no path, no wildcard) or boot fails. See [Cross-origin access](#cross-origin-access). |
| `OS_HUB_MODE` | inferred: `development` if the bind is loopback, else `production` | `development` \| `production`. Any other value fails boot. `production` is a plain process; `development` needs the launcher's fd 3. |
| `OS_HUB_TRUSTED_FORWARDING` | `none` | `none` \| `proxy`. `proxy` states that a TLS-terminating reverse proxy is the only thing that can reach this socket, which makes `X-Forwarded-Proto`/`X-Forwarded-Host` trustworthy and **enforced** — a request the proxy reports as cleartext is refused `403 x-semio-refusal: insecure-transport`. Required for a non-loopback production bind. Any other value fails boot. |
| `OS_HUB_DATA` | `./.🧬semio/🌐hub/` (relative to cwd) | The server-owned data root. Give it an absolute path. `trusted-catalog publish` *requires* an absolute one. |
| `OS_HUB_ADMIN_SUBJECTS` | empty | Comma-separated `provider:subject` identities granted the admin surface; max 64, duplicates rejected. Required in production mode. For a password credential the provider is literally `credential.password.v1` (`🔐️auth/🦀️.rs`, `CREDENTIAL_IDENTITY_PROVIDER`) and the subject is the email `credential set` was given, e.g. `credential.password.v1:ada@example.com`. A mismatched provider string still boots — the admin routes simply answer `401` for everyone. |
| `OS_HUB_ADMIN_DIR` | the admin SPA's built `📤️dist` next to the crate | Static asset root for the admin SPA. Set it when the binary is not co-located with its source tree. |
| `OS_HUB_EXTENSIONS_DIR` | `{OS_HUB_DATA}/extension-modules` | Extension module root; created at boot. |
| `OS_HUB_MERGE_POLICY` | `normal` | `laissez-faire` \| `normal` \| `vigilant`, read once at startup. An unknown value warns and falls back — it does not fail boot. |
| `OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE` | `false` | `true`/`1` lets the artifact-CAS maintenance supervisor actually delete swept chunks; anything but `true`/`false`/`1`/`0`/empty fails boot. |
| `OS_HUB_TEST_INFERENCE_CHECKPOINT_FD` | unset | Test-only: an inherited fd the harness steps inference checkpoints over. Never set it in a deployment. |

### Document store (`OS_HUB_STORAGE_BACKEND`)

Read twice — once for the document store, once for the artifact chunk CAS — so both always agree.

| variable | default | meaning |
|---|---|---|
| `OS_HUB_STORAGE_BACKEND` | `fs` | `fs` \| `sqlite` \| `postgres` \| `neo4j`. `fs` is zero-touch, rooted at `{OS_HUB_DATA}/db`. |
| `OS_HUB_DB_SQLITE` | `{OS_HUB_DATA}/db.sqlite3` | SQLite file path; read only when the backend is `sqlite`. |
| `OS_HUB_DATABASE_URL` | — | **Required** when the backend is `postgres`. |
| `OS_HUB_NEO4J_URI` | — | **Required** when the backend is `neo4j`. |
| `OS_HUB_NEO4J_USER` | `neo4j` | Neo4j user. |
| `OS_HUB_NEO4J_PASSWORD` | empty string | Neo4j password. |

#### How a `postgres`/`neo4j` document store is actually driven

`sqlx` and `neo4rs` are bound to a Tokio runtime and demand its thread-local context **every time one
of their futures is polled**, not merely when a pool is constructed. The hub's document store does not
run on the server's `#[tokio::main]` runtime: every call becomes a typed task on `Lane::Io` of the one
process `WorkerPool`, whose workers are deliberately plain `std::thread`s with no runtime context at
all (`🧰️framework/🔨️modules/⏳️async` — "No `tokio` in this crate"). Polling a driver future there
aborted the whole process on first use with `this functionality requires a Tokio context`.

So the storage layer owns **one bounded runtime of its own**, and that is the only place such a driver
is ever polled: `db_storage_driver_runtime` (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧵️driver-runtime/`),
reached through the `db_storage::DbIoAsyncDriverRuntime` interface. A driver future is handed to it and
what `Lane::Io` polls instead is a repo-owned `semio_framework_async::oneshot` rendezvous, which is safe
on any thread. Two consequences for an operator:

| variable | default | meaning |
|---|---|---|
| `SEMIO_DB_IO_DRIVER_THREADS` | `2` | Worker threads in that runtime, clamped to `1..=8`. It drives database sockets and timers only — CPU work stays on the `WorkerPool`. Raise it only if a profile shows the driver threads saturated. |

The runtime is created on first use and is **never shut down**: a detached driver turn holds the
storage backend's executor for its whole duration, so tearing the runtime down under it would destroy
state the backend registry owns. Its threads are named `semio-db-io-driver-N`, so a crash report or a
`sample` names them. The **directory** halves (`OS_HUB_DIRECTORY_BACKEND=postgres|neo4j`) are different:
they are awaited directly on the server's own `#[tokio::main]` runtime and never touch this seam.

### Directory / identity store (`OS_HUB_DIRECTORY_BACKEND`)

Chosen independently of the document store.

| variable | default | meaning |
|---|---|---|
| `OS_HUB_DIRECTORY_BACKEND` | `sqlite` | `sqlite` \| `postgres` \| `neo4j`. SQLite is zero-touch and self-seeding. |
| `OS_HUB_DIRECTORY_DATABASE_URL` | — | **Required** when the directory backend is `postgres`. |
| `OS_HUB_DIRECTORY_NEO4J_URI` | — | **Required** when the directory backend is `neo4j`. |
| `OS_HUB_DIRECTORY_NEO4J_USER` | `neo4j` | Neo4j user. |
| `OS_HUB_DIRECTORY_NEO4J_PASSWORD` | empty string | Neo4j password. |

Launching both halves on PostgreSQL in development: `bun nx run os-hub:dev-postgres` (launch rows
`🛠️dev🐘️os-hub🗄️postgres` / `📦️build-dev🐘️os-hub🗄️postgres`). It stages its own binary with the
`postgres,neo4j` features into `dist/build-dev-postgres`, requires `OS_HUB_DATABASE_URL`, and points the
directory at the same database unless `OS_HUB_DIRECTORY_BACKEND`/`OS_HUB_DIRECTORY_DATABASE_URL` say
otherwise. The live lanes behind `bun nx run os-hub:directory-live-lanes` exercise both databases for
real (they start their own containers and need a running Docker daemon).

The SQLite directory path is **not** configurable: it is always `{OS_HUB_DATA}/directory.db`, and
its parent is created at boot.

### Password sign-in (`🔐️auth/🦀️.rs`, `CredentialSignInPolicyV1::from_env`)

| variable | default | meaning |
|---|---|---|
| `OS_HUB_CREDENTIAL_SIGN_IN` | `false` | `true`/`1` enables the public `POST /auth/sessions` route, and is what makes **production mode** reachable without an external IdP. Absent means disabled; an unreadable value fails boot. While disabled, `/readyz` reports `publicSessionIssuance: false`. |
| `OS_HUB_SESSION_TTL_SECONDS` | `43200` (12 h) | Session lifetime, `60..=31536000`. Read only when sign-in is enabled. |
| `OS_HUB_PASSWORD_ITERATIONS` | `210000` | PBKDF2 iterations, `1000..=999999999`. Read independently by `credential set`, so an operator can mint at a different cost than the server verifies at. |

> `OS_HUB_TRUSTED_CATALOG_BUNDLE`, `OS_HUB_TRUSTED_CATALOG_PROFILE` and `OS_HUB_ADMIN_TOKEN` appear
> in the launcher's env-scrubbing list but are read by no Rust code in `🌎️hub`. They are vestigial;
> setting them does nothing.

## Data-root layout

Everything durable lives under `OS_HUB_DATA`. With the defaults (`fs` store, `sqlite` directory):

```
$OS_HUB_DATA/
├── db/                          # fs document store: WAL + snapshots + compaction
├── directory.db                 # sqlite identity/tenancy: principals, spaces, sessions, invites
├── extension-modules/           # OS_HUB_EXTENSIONS_DIR, created at boot
└── trusted-catalog/
    ├── current.json             # the pointer /readyz's artifactAuthority gate reads
    └── generations/<id>/        # published generations
```

With `OS_HUB_STORAGE_BACKEND=sqlite`, `db/` is replaced by `db.sqlite3` (or `OS_HUB_DB_SQLITE`).
With a `postgres`/`neo4j` backend the corresponding tree lives in that database instead and is
**not** covered by the file-copy backup below.

What a `postgres` document store moves into the database, measured on a live boot: `db_wal_segment`,
`db_snapshot_generation`, `db_payload`, `db_catalog_root`, `db_index_run`, `db_lease`, and the whole
artifact chunk CAS (`hub_artifact_cas_*`, `hub_artifact_checkpoint*`, `hub_artifact_retention`). What
stays on the filesystem under `OS_HUB_DATA` **on every backend** is the four server-product stores the
hub instance owns — `instance/{authority,projections,blobs,sessions}` — plus `trusted-catalog/` and
`extension-modules/`. Those four have no database lane at all: they are opened from
`StorageProfile::Embedded { data_dir }` and stamped with `format.json` (`🗄️stores/🦀️.rs`), which is why
a `postgres` hub still needs a durable `OS_HUB_DATA` directory and why a data root and its database must
be backed up and restored **together**.

## First user

There is no sign-up. The first principal of a fresh data root is created by an operator verb on the
binary, dispatched before any service opens:

```bash
OS_HUB_DATA=/srv/semio-hub/data os-hub credential set --email ada@example.com --display-name "Ada"
# password is read from stdin
```

Deliberately not an HTTP route: it requires read/write on the server-owned data root, which is
strictly stronger than any capability reachable over the network, so no bearer token and no "first
request wins" race can stand in for physical control of the deployment. The password is read from
stdin — never `argv` (every process on the machine can read that) and never an environment variable
(children inherit those). The verb is idempotent: an absent account is created, an existing one keeps
its identity and gets the new credential plus a `credential-changed` fact, and in both cases **every
live session of that principal is revoked** before it returns. Only `--email` and `--display-name`
are accepted.

For that account to be able to sign in over HTTP, the server must also run with
`OS_HUB_CREDENTIAL_SIGN_IN=true` — which is the same variable that makes **production mode**
reachable at all, because the hub's credentials *are* its identity authority. The verb is dispatched
at the very top of `main`, before the mode, the bind, the local-bootstrap transport or any store is
touched, so it works identically in both topologies and needs no fd 3 and no running server.

Self-service password change (old password required) is `POST /auth/credentials` and does not need
this verb.

## Publishing a trusted catalog

Until a trusted catalog generation exists in the data root, the artifact-authority gate stays closed
and `/readyz` reports `trusted-catalog-never-published-in-this-data-root`. Publication is the
binary's second operator verb, fed one bounded command on stdin:

```bash
OS_HUB_DATA=/srv/semio-hub/data os-hub trusted-catalog publish < command.json
```

`OS_HUB_DATA` must be absolute here or the verb refuses. `os-hub:dev` runs this for you on a fresh
data root.

## Cross-origin access

A browser that loads `s` from one origin and talks to the hub on another needs a real cross-origin
grant, and the grant must be *credentialed* — the hub's session bearer travels on it. The CORS
specification forbids `access-control-allow-origin: *` together with
`access-control-allow-credentials: true`, so a wildcard was never an option; an allowlist is.

`CrossOriginPolicyV1::from_environment` (`🏗️bootstrap/🦀️.rs`) resolves one policy at startup and
`/readyz`'s neighbouring startup line reports which one:

| `OS_HUB_ALLOWED_ORIGINS` | `OS_HUB_BIND` | policy | what a browser gets |
|---|---|---|---|
| set | any | `allowlist` | `access-control-allow-origin: <the request's own origin>` + credentials, for exactly the listed origins |
| unset | loopback (`127.0.0.1`, `::1`) | `loopback-development` | any `http`/`https` origin whose host is `localhost` or a loopback address, on any port |
| unset | a network interface | `closed` | no cross-origin grant at all |

Entries are whole serialized origins compared ASCII-case-insensitively — `https://s.example.com`,
`https://admin.example.com:8443`, `http://[::1]:3000`. A path, a query, a trailing slash, a
wildcard or a non-`http(s)` scheme is a boot error naming the entry, never a pattern the hub
interprets. `https://example.com` and `https://example.com:8443` are different origins, as are
`http://` and `https://` of the same host.

A refused origin is not an error response: the hub answers the request normally and simply omits
the grant, which is what makes the *browser* refuse to hand the answer to the page. `Vary: Origin`
is emitted for every request that carried an `Origin`, admitted or not, so a shared cache can never
serve one origin's grant to another.

```bash
OS_HUB_ALLOWED_ORIGINS=https://s.example.com,https://admin.example.com os-hub
```

Note the interaction with the bind gate above: because `validate_auth_startup` refuses a
non-loopback bind in *both* modes today, the `closed` default is a guard that takes effect the day
a network bind becomes bootable rather than a posture anyone can reach right now. What changes
today is that a loopback hub no longer hands a credentialed grant to `https://evil.test`.

## Health endpoints

| route | meaning |
|---|---|
| `GET /healthz` | Liveness. Always `200` while the process serves: `{schema: "semio.hub.liveness/v1", status: "live", runId, uptimeMs}`. It says nothing about readiness. |
| `GET /readyz` | Readiness. `200` when every gate is open, **`503` otherwise**, with the same body either way. |
| `GET /admin/api/observability` | Per-event counters and latency percentiles, behind the same admin capability as every other `/admin/api` route. Body: `{schema: "semio.hub.observability/v1", level, droppedEvents, declaredEvents, rows: [{event, started, ok, refused, failed, cancelled, total, samples, p50Us, p95Us, p99Us}]}`. |

`/readyz`'s body names what is closed and why rather than claiming a bare status: `blocked_by` is a
list of `{gate, reason}` pairs (e.g. `artifactCasSweeper` /
`artifact-cas-maintenance-supervisor-failed-closed`), and the authentication block carries
`publicSessionIssuance`. Point a load balancer at `/readyz` and a process supervisor at `/healthz`.

The hub carries **no `tracing`/`prometheus`/`opentelemetry` dependency** — the observer is
first-party (`semio_framework_trace`). Every HTTP route, the document WebSocket handler, the
directory backend faults and the boot itself open a span on it, configured from `SEMIO_TRACE_LEVEL`
and `SEMIO_TRACE_SINK`; the records go to the configured sink, and the bounded counter table behind
them is what `/admin/api/observability` returns. `declaredEvents` ships the vocabulary so a
dashboard shows a never-fired event as a zero rather than a missing series, and a non-zero
`droppedEvents` means some event name is being built from request data. Startup still prints a
`[WARN] … closed gates: …` line when it comes up not-ready. There is no Prometheus exposition
format and no scrape endpoint: point a dashboard at the admin route, a load balancer at `/readyz`
and a process supervisor at `/healthz`.

## Backup and restore

The backup unit is **the whole `OS_HUB_DATA` tree, with the hub stopped.** There is no `os-hub
backup`/`restore` verb and no online-snapshot endpoint.

Stop first, and stop cleanly. The document store is an event-sourced WAL with snapshots and
background compaction and the directory is a live SQLite database; copying either while the process
writes can capture a torn state.

`SIGTERM` and `SIGINT` both drain: the hub stops accepting, lets in-flight work finish, and exits.
Give that drain room — `systemctl stop` or `docker stop` with a stop timeout of at least 30 s, never
`SIGKILL`. Take the copy only after the process is actually gone, not after the stop command returns.

```bash
# back up
systemctl stop semio-hub
tar -C /srv/semio-hub -czf "hub-$(date -u +%Y%m%dT%H%M%SZ).tar.gz" data
systemctl start semio-hub

# restore — onto a hub that is stopped, into an empty path
systemctl stop semio-hub
rm -rf /srv/semio-hub/data
tar -C /srv/semio-hub -xzf hub-20260920T010000Z.tar.gz
chown -R semio:semio /srv/semio-hub/data
systemctl start semio-hub
curl -fsS http://127.0.0.1:8787/readyz | jq .
```

Restore into an **empty** path, not over a populated one: mixing generations of an event-sourced
store is not a supported state.

With `postgres` or `neo4j` selected for either store, that database is outside the tarball — back it
up with its own native tooling, at the same stopped moment, and restore both halves together.

**There is no cross-version upgrade path.** The codebase says so out loud
(`📇️directory/🐘️postgres/🦀️.rs`: *"no migration framework (greenfield: there are no users yet, so
schema changes are edited in place, not migrated)"*). Treat a hub upgrade as: back up, stop, and be
prepared to start from an empty data root if the new binary cannot read the old one. Do not put data
you cannot lose into a hub yet.

## TLS-terminating reverse proxy

The hub speaks **plain HTTP only** — there is no `rustls`/`native-tls` anywhere in it. A reverse
proxy that terminates TLS is therefore mandatory for any deployment a browser on another machine
reaches, whether the hub itself binds loopback (proxy on the same host) or a network interface
(proxy the only route to the port).

Set `OS_HUB_TRUSTED_FORWARDING=proxy` when you do this, and understand what it buys and costs. The
hub then **trusts and enforces** `X-Forwarded-Proto`: a request the proxy reports as having reached
the client over cleartext is refused `403` with `x-semio-refusal: insecure-transport`, before CORS,
rate limiting or any handler — a session bearer is never minted onto a connection that crossed the
network in the clear. `X-Forwarded-Host` becomes the host the hub believes the client used. Both are
ignored entirely when the variable is unset, because a header anyone who reaches the socket can set
is worth less than no header at all. **So the setting is only true if it is true**: nothing but the
proxy may be able to reach the port. A proxy that strips inbound `X-Forwarded-*` from clients and
sets its own is the configuration both examples below show.

Both examples below forward to `127.0.0.1:8787` and carry the WebSocket upgrade, which is not
optional: collaboration, presence and the directory all run over
`GET …/socket/v1` routes (`/directory/socket/v1`, `/directory/spaces/{space}/documents/{doc}/socket/v1`,
`/spaces/{space}/documents/{id}/socket/v1`). A proxy that drops `Upgrade`/`Connection` gives you a
hub that signs in and then never syncs.

### Caddy

```caddyfile
hub.example.com {
	encode zstd gzip

	# WebSocket upgrades: Caddy v2 reverse_proxy passes them through unchanged,
	# but the read timeout must not cut an idle collaboration socket.
	reverse_proxy 127.0.0.1:8787 {
		transport http {
			read_timeout 0
			write_timeout 0
		}
		header_up X-Forwarded-Proto {scheme}
		header_up X-Forwarded-For {remote_host}
	}

	@health path /healthz /readyz
	handle @health {
		reverse_proxy 127.0.0.1:8787
	}
}
```

Caddy obtains and renews the certificate itself; nothing else is required for TLS.

### nginx

```nginx
map $http_upgrade $connection_upgrade {
    default upgrade;
    ''      close;
}

server {
    listen 443 ssl;
    http2 on;
    server_name hub.example.com;

    ssl_certificate     /etc/letsencrypt/live/hub.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/hub.example.com/privkey.pem;

    # Artifact chunk uploads are large; the hub enforces its own bounds.
    client_max_body_size 128m;

    location / {
        proxy_pass http://127.0.0.1:8787;
        proxy_http_version 1.1;

        # The WebSocket upgrade. Without these two lines every socket route 400s.
        proxy_set_header Upgrade    $http_upgrade;
        proxy_set_header Connection $connection_upgrade;

        proxy_set_header Host              $host;
        proxy_set_header X-Real-IP         $remote_addr;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Collaboration sockets idle between edits; do not let nginx cut them.
        proxy_read_timeout  1h;
        proxy_send_timeout  1h;
        proxy_buffering     off;
    }
}
```

### What the proxy does *not* fix

The proxy does not decide who may call the hub from a browser — [Cross-origin
access](#cross-origin-access) does, and its default is deliberately narrow. Set
`OS_HUB_ALLOWED_ORIGINS` to the origins your `s` bundle is actually served from before putting a
proxy in front of this hub.

The hub does not read `X-Forwarded-For`: its per-remote-address rate limiting sees the proxy's
address, so every client shares one bucket. Rate-limit at the proxy as well.

## Container image

`🌎️hub/Dockerfile` and `🌎️hub/compose.yaml` build the **production** topology: a binary-only runtime
image with no repository, no Rust toolchain, no `bun` and no Nx in it. `docker build --check` passes
with no warnings and `docker compose config` resolves (Docker 29.5, buildx 0.34), but the image has
**not been built**: the builder stage compiles the hub's full release dependency graph. The binary
it would produce has been run outside a container in exactly this posture — release profile, plain
process, network bind, allowlist + proxy trust, credential sign-in, SIGTERM drain.

```bash
docker build -f 🌎️hub/Dockerfile -t semio/os-hub .              # context = repository root

# seed the first user into the volume — no server needed, and there is no sign-up
docker run --rm -it -v semio-hub-data:/srv/semio-hub/data semio/os-hub \
  credential set --email ada@example.com --display-name Ada

docker run --rm -p 127.0.0.1:8787:8787 -v semio-hub-data:/srv/semio-hub/data \
  -e OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com \
  -e OS_HUB_ALLOWED_ORIGINS=https://s.example.com \
  -e OS_HUB_TRUSTED_FORWARDING=proxy \
  --stop-timeout 30 semio/os-hub

docker compose -f 🌎️hub/compose.yaml up --build
docker compose -f 🌎️hub/compose.yaml --profile postgres up --build
```

The entrypoint is `os-hub` itself under `tini`. Inside the container the bind is `0.0.0.0` — that is
the container's own interface, and a loopback bind inside a container makes `-p` unreachable — so
the production network-bind rules apply and the container **refuses to start** until you state the
allowlist and the proxy. That refusal is the design, not a bug: publish the port on the host's
loopback and put the TLS terminator in front of it.

Two things the image needs that a source checkout does not: `OS_HUB_ADMIN_DIR`, because the admin
SPA's compile-time default path points into the crate's source tree, which this image does not
carry; and a `HEALTHCHECK` that sends `X-Forwarded-Proto: https`, because with
`OS_HUB_TRUSTED_FORWARDING=proxy` the transport-security layer is outermost on the whole router and
refuses `/healthz` and `/readyz` too. `OS_HUB_DATA` is the single writable volume and the whole
backup unit.

An earlier revision of this file carried the whole repository and ran `bun nx run os-hub:dev`,
because production mode was then unreachable and development mode needs the launcher's inherited
fd 3. That is still the right shape for a *development* container and the wrong one for a server.

## For the people who will use this hub

An operator's last job is telling users how to reach the hub they just started. There are two
clients, and each needs one thing from you: the origin `s` is served from (which must also be in
`OS_HUB_ALLOWED_ORIGINS`), and, for an AI client, a credential file.

### A browser

Give them the URL of the `s` bundle, not of the hub. `s` asks for the hub URL on first run and signs
in against `POST /auth/sessions` with the email and password `os-hub credential set` minted. Serve
the release bundle (`bun nx run @semio-tech/framework-os-dev:build-s-react-release`, output
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/build-s-react-release`)
from any static web server, put its origin in `OS_HUB_ALLOWED_ORIGINS`, and put both behind the same
TLS-terminating proxy.

### An AI client (Claude Desktop, Claude Code, or any MCP client)

semio ships an MCP server, `semio-os-mcp`, that hands a user's own AI client semio's capabilities —
open and create artifacts, prepare and invoke actions, snapshot, undo/redo, transactions,
inference — either against a folder on their disk (`--folder`) or against a space on this hub
(`--hub`). No model and no model provider is part of this repository; the client the user already
pays for is the model.

Build the binary once:

```bash
bun nx run @semio-tech/framework-os-mcp-rs:build-release
# → 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp
```

**Claude Code** — `.mcp.json` in the project the user wants the server available from:

```json
{
  "mcpServers": {
    "semio": {
      "type": "stdio",
      "command": "/Users/you/src/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp",
      "args": [
        "stdio",
        "--folder", "/Users/you/Documents/my-semio-space",
        "--scopes", "workspace.read,artifact.read,artifact.write,inference.execute,ui.observe,ui.control"
      ]
    }
  }
}
```

**Claude Desktop** — `claude_desktop_config.json` (macOS
`~/Library/Application Support/Claude/claude_desktop_config.json`, Windows
`%APPDATA%\Claude\claude_desktop_config.json`). Desktop passes no `cwd`, so the absolute binary
path is the only form that works — the emoji directory names are literal and must be copied exactly:

```json
{
  "mcpServers": {
    "semio": {
      "command": "/Users/you/src/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp",
      "args": [
        "stdio",
        "--folder", "/Users/you/Documents/my-semio-space",
        "--scopes", "workspace.read,artifact.read,artifact.write,inference.execute,ui.observe,ui.control"
      ]
    }
  }
}
```

To point that same server at a space on **this** hub instead of a local folder, replace
`"--folder", "<dir>"` with `"--hub", "https://hub.example.com", "--space", "<space id>",
"--credential-file", "/Users/you/.config/semio/agent.json"`. The credential file is not something an
operator hands out: a signed-in user creates it themselves in the *Agent delegations* panel of their
`s` window, which `POST /auth/agent-delegations` answers with a downloadable file. It carries a
scoped, expiring delegation of that user's own authority, so an agent can never exceed the person who
delegated to it.

Every tool, the approval model for destructive capabilities and configs for the other clients are in
[the MCP README](../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md).

## Distribution tarball

```bash
bun ./📜️script.ts publish os-hub     # → 🌎️hub/📦️packages/🦀️rust/dist/publish/…tar.gz + .sha256
bun ./📜️script.ts publish os-mcp     # the end-user MCP binary, same shape
```

Each slice runs its project's `publish` target, which depends on that project's release build and
writes `<name>-<version>-<platform>-<arch>.tar.gz` with a sibling `.sha256`. Both have been produced
for real (2026-09-21, darwin-arm64): `os-hub-0.1.0-darwin-arm64.tar.gz` (23 182 368 bytes) and
`semio-os-mcp-0.1.0-darwin-arm64.tar.gz` (11 682 122 bytes), each with its checksum file. The version is the
workspace's `[workspace.package] version`. **Nothing is uploaded or pushed anywhere** — where the
tarball goes next is a deployment decision this repository does not take, and `.github/workflows/`
is still empty.

## systemd

The unit runs the **release binary directly** in production mode. Development mode cannot be a
systemd unit — it needs the launcher's inherited fd 3 — so this is the production topology from the
top of this page, and the three network statements are all here. `Type=simple` with a readiness poll
rather than `Type=notify`: the hub implements no sd_notify protocol.

```ini
# /etc/systemd/system/semio-hub.service
[Unit]
Description=semio collaboration hub
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=semio
Group=semio
Environment=OS_HUB_DATA=/srv/semio-hub/data
Environment=OS_HUB_MODE=production
Environment=OS_HUB_CREDENTIAL_SIGN_IN=true
Environment=OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com
Environment=OS_HUB_SESSION_TTL_SECONDS=43200

# Loopback, with the reverse proxy on this same host. To answer on a network interface
# instead, set OS_HUB_BIND to that address and keep BOTH statements below — the hub
# refuses a network bind without them.
Environment=OS_HUB_BIND=127.0.0.1
Environment=OS_HUB_PORT=8787
# Environment=OS_HUB_ALLOWED_ORIGINS=https://s.example.com
# Environment=OS_HUB_TRUSTED_FORWARDING=proxy

ExecStart=/srv/semio-hub/bin/os-hub

Restart=on-failure
RestartSec=5s

# The hub stops on SIGTERM: it stops accepting, lets in-flight requests finish, then
# drains the admin-operation and artifact-creation task owners and closes the inference
# runtime. Give that room before systemd escalates to SIGKILL.
KillMode=mixed
KillSignal=SIGTERM
TimeoutStopSec=30s

# The data root is the only path it must write.
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
NoNewPrivileges=true
ReadWritePaths=/srv/semio-hub/data

[Install]
WantedBy=multi-user.target
```

Seed the first account before the first start — `os-hub credential set` needs no server running, and
`OS_HUB_ADMIN_SUBJECTS` must name an identity that exists:

```bash
install -o semio -g semio -d /srv/semio-hub/data
sudo -u semio OS_HUB_DATA=/srv/semio-hub/data /srv/semio-hub/bin/os-hub credential set --email ada@example.com
```

```bash
systemctl daemon-reload
systemctl enable --now semio-hub
curl -fsS http://127.0.0.1:8787/readyz | jq .
journalctl -u semio-hub -f
```

## Known gaps, stated plainly

An operator should know these before putting anything real into a hub:

- **No TLS in-process, ever.** A non-loopback production hub must sit behind a TLS-terminating
  reverse proxy and must be told so (`OS_HUB_TRUSTED_FORWARDING=proxy`). The hub enforces what the
  proxy reports; it cannot detect a missing proxy.
- **No external IdP.** `IdentityAssertionVerifier` has no implementation in this repository, so
  production identity means the hub's own password credentials and nothing else.
- **No real reverse proxy has ever fronted this hub.** Production mode itself has now been run on a
  network bind (see "Run it"), but the TLS terminator was simulated by sending the headers Caddy or
  nginx would send. The Caddy/nginx/systemd examples below remain correct-by-construction against
  the router's real socket routes, not configs anyone has loaded.
- **No Prometheus/OpenTelemetry exposition.** Structured tracing and per-event counters *are*
  shipped (`SEMIO_TRACE_LEVEL`/`SEMIO_TRACE_SINK`, `GET /admin/api/observability`, see
  "Health and readiness"); what does not exist is a scrape endpoint in anyone else's format, so a
  Prometheus-based stack needs an exporter in front of the admin route.
- **No cross-version *migration*.** Each durable store now stamps its format version on creation and
  refuses a data root written by a different one with a named error, so an upgrade cannot corrupt
  history silently — but there is still nothing that converts an old root into a new one.
- **No CI.** `.github/workflows/` is empty; `os-hub:publish` produces a local tarball and nothing
  uploads it anywhere.
- **The container image is unbuilt.** `Dockerfile`/`compose.yaml` now pass `docker build --check`
  (no warnings) and `docker compose config`, and they target the production topology, but no image
  has been built: the builder stage compiles the hub's full release dependency graph, ~40 min on a
  warm cache and considerably more in a cold container.
- **A published trusted catalog does not survive a codegen-policy change.** A generation is stamped
  with the jco version of the binary that materialized it, and a binary carrying a different policy
  refuses the whole root at boot —
  `ArtifactAuthority(Catalog("trusted browser actor identity differs from its package or renderer"))`,
  observed 2026-09-21 with a jco-1.27 root and a jco-1.34 binary. There is no migration and no
  partial republish: a policy bump costs a full materialize-and-publish. Reuse across restarts of
  the *same* build is instant and is proven.
