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
OS_HUB_ADMIN_SUBJECTS=semio.hub.credential:ada@example.com \
os-hub

# on a network interface: the allowlist and the proxy statement are both mandatory
OS_HUB_MODE=production \
OS_HUB_BIND=10.0.0.4 \
OS_HUB_CREDENTIAL_SIGN_IN=true \
OS_HUB_ADMIN_SUBJECTS=semio.hub.credential:ada@example.com \
OS_HUB_ALLOWED_ORIGINS=https://s.example.com \
OS_HUB_TRUSTED_FORWARDING=proxy \
os-hub
```

The startup line reports the three postures it resolved:
`[INFO] bind scope network (10.0.0.4:8787), cross-origin policy allowlist, trusted forwarding proxy`.

> Production mode has never been run by anyone. The boot rules are unit-tested; the process is not
> yet observed. See "Known gaps".

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
| `OS_HUB_ADMIN_SUBJECTS` | empty | Comma-separated `provider:subject` identities granted the admin surface; max 64, duplicates rejected. Required in production mode. |
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

### Directory / identity store (`OS_HUB_DIRECTORY_BACKEND`)

Chosen independently of the document store.

| variable | default | meaning |
|---|---|---|
| `OS_HUB_DIRECTORY_BACKEND` | `sqlite` | `sqlite` \| `postgres` \| `neo4j`. SQLite is zero-touch and self-seeding. |
| `OS_HUB_DIRECTORY_DATABASE_URL` | — | **Required** when the directory backend is `postgres`. |
| `OS_HUB_DIRECTORY_NEO4J_URI` | — | **Required** when the directory backend is `neo4j`. |
| `OS_HUB_DIRECTORY_NEO4J_USER` | `neo4j` | Neo4j user. |
| `OS_HUB_DIRECTORY_NEO4J_PASSWORD` | empty string | Neo4j password. |

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

`/readyz`'s body names what is closed and why rather than claiming a bare status: `blocked_by` is a
list of `{gate, reason}` pairs (e.g. `artifactCasSweeper` /
`artifact-cas-maintenance-supervisor-failed-closed`), and the authentication block carries
`publicSessionIssuance`. Point a load balancer at `/readyz` and a process supervisor at `/healthz`.

There is **no metrics endpoint and no structured request/WebSocket tracing** — the hub has no
`tracing`/`prometheus`/`opentelemetry` dependency. Startup prints a `[WARN] … closed gates: …` line
when it comes up not-ready. Plan your observability around `/readyz` polling and stdout.

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

`🌎️hub/Dockerfile` and `🌎️hub/compose.yaml` exist and are **unbuilt** — they were authored against
the real build commands on a machine with no Docker, and neither has ever been built or started.
Read them before you trust them.

```bash
docker build -f 🌎️hub/Dockerfile -t semio/os-hub:dev .          # context = repository root
docker run --rm -p 127.0.0.1:8787:8787 -v semio-hub-data:/srv/semio-hub/data semio/os-hub:dev
docker compose -f 🌎️hub/compose.yaml up --build
docker compose -f 🌎️hub/compose.yaml --profile postgres up --build
```

The image carries the repository, not just a binary, for the reason at the top of this page: the
binary needs the launcher's inherited fd 3, so `bun nx run os-hub:dev` is the entrypoint. The
builder stage stages both `dist/build` (release) and `dist/build-dev` (what the launcher execs)
plus the admin SPA, so the runtime stage needs neither `cargo` nor a Rust toolchain. `OS_HUB_DATA`
is the single writable volume and the whole backup unit.

## Distribution tarball

```bash
bun ./📜️script.ts publish os-hub     # → 🌎️hub/📦️packages/🦀️rust/dist/publish/…tar.gz + .sha256
bun ./📜️script.ts publish os-mcp     # the end-user MCP binary, same shape
```

Each slice runs its project's `publish` target, which depends on that project's release build and
writes `<name>-<version>-<platform>-<arch>.tar.gz` with a sibling `.sha256`. The version is the
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
Environment=OS_HUB_ADMIN_SUBJECTS=semio.hub.credential/v1:ada@example.com
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
- **Production mode has never been run.** The gate that used to make it unreachable is gone and the
  new rules are unit-tested (`startup_auth_policy_fails_closed_without_owned_adapters`,
  `a_production_hub_boots_on_its_own_credential_authority_without_any_external_idp`,
  `a_network_bind_is_admitted_only_with_an_allowlist_and_a_declared_tls_terminating_proxy`,
  `a_production_posture_hub_signs_a_browser_in_over_its_declared_proxy`), but no one has yet started
  a real `OS_HUB_MODE=production` process and signed in against it. Expect to be the first.
- **No metrics, no request tracing.** `/readyz` and stdout are the whole observability surface.
- **No cross-version *migration*.** Each durable store now stamps its format version on creation and
  refuses a data root written by a different one with a named error, so an upgrade cannot corrupt
  history silently — but there is still nothing that converts an old root into a new one.
- **No CI.** `.github/workflows/` is empty; `os-hub:publish` produces a local tarball and nothing
  uploads it anywhere.
- **The container image is unbuilt.** `Dockerfile`/`compose.yaml` in this directory were authored
  against the real build commands but have never been built or run — there is no Docker on the
  machine this was written on.
- **Never proven at release profile.** `os-hub:build` (the `--release` target) has compiled in CI
  nowhere and has not been observed producing a running hub; every hub boot recorded so far used
  `build-dev`.
