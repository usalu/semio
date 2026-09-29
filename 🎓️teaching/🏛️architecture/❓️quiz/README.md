# ❓️ Quizze · Architektur und Technologie

The quiz website of **quizze.architektur-und-technologie.de**: the architecture catalog (`🔣️.json`, id `architecture`) with
the quizzes it names, rendered by `@semio-tech/quiz-react` and proctored by `proctor` (`🎓️teaching/🛂️proctor`), which serves
the built site, the catalog and the gateway routes from one process over one SQLite file.

| Path | What |
|---|---|
| `🔣️.json` | the catalog: introduction, quiz paths (relative to this file), badges |
| `🟦️.ts`, `🌐️.html` | the site entry: `mountQuiz(root, { proctor: "", tenant: "architecture" })` |
| `🎨️.css` | the brand layer: the semio palette compiled to custom properties, its fonts shipped with the build |
| `🏗️builder/🌐️vite/🟦️.ts` | Vite configuration: host HTML, `CNAME`, aliases, dev proxy of the gateway routes to the dev proctor |
| `📦️packages/🟦️typescript` | `@teaching/architecture-quiz`: `package.json`, `📋️project.json`, `📜️script.ts` |
| `🧪️tests/🧪️catalog/🟦️.ts` | the catalog and every quiz validate in the TS core and against the draft-07 contract (ajv) |
| `🚀️deploy/` | `Dockerfile`, `compose.yaml`, `Caddyfile` and the image build/check verbs (`🟦️.ts`) |

## Develop

Run everything from `.vscode/launch.json` (group `3_dev`):

| Launch row | Command | Port |
|---|---|---|
| `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` | both rows below | |
| `🛠️dev🎓️teaching🛂️proctor` | `bun nx run @teaching/proctor:dev` | `8791` (`PROCTOR_PORT`) |
| `🛠️dev🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:dev` | `6061` (`TEACHING_ARCHITECTURE_QUIZ_PORT`) |
| `🧪️test🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:test` | |
| `✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog` | `bun nx run @teaching/architecture-quiz:check` (`proctor check` on the catalog) | |
| `📦️build🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:build` → `📦️packages/🟦️typescript/dist` | |
| `🔁️rebuild🎓️teaching🛂️proctor` | `bun nx run @teaching/proctor:rebuild` (refold the dev proctor's read models) | |

The dev site proxies `/instance`, `/commands`, `/queries`, `/actors` (WebSocket event streams included) and `/scopes` to
`http://127.0.0.1:${PROCTOR_PORT:-8791}`, so the browser talks to one origin exactly as in production.

## Deploy

One host with Docker Engine (with the compose plugin) and Caddy runs the site. The proctor container publishes plain HTTP on
the host's loopback only (`127.0.0.1:8791`); Caddy terminates TLS in front of it and obtains the certificate itself.

| File | Role |
|---|---|
| `🚀️deploy/Dockerfile` | two stages: bun builds the site and cargo builds `proctor` from the repository; the runtime is `debian:bookworm-slim` + `tini`, user `quiz`, one volume `/srv/quiz/data`, `HEALTHCHECK` on `/instance` |
| `🚀️deploy/compose.yaml` | service `proctor` on `127.0.0.1:8791`, `PROCTOR_MODE=production`, `PROCTOR_ALLOWED_ORIGINS=https://quizze.architektur-und-technologie.de`, `PROCTOR_TRUSTED_FORWARDING=proxy`, volume `architecture-quiz_proctor-data` |
| `🚀️deploy/Caddyfile` | `quizze.architektur-und-technologie.de` → `reverse_proxy 127.0.0.1:8791` with automatic TLS |

The production posture refuses to start unless all three statements are present (mode, origin allowlist, trusted
forwarding), and with `PROCTOR_TRUSTED_FORWARDING=proxy` every request that does not arrive through the proxy
(`X-Forwarded-Proto: https`) is refused. The image verbs prove this on a fresh volume:
`📦️build🎓️teaching🏛️architecture❓️quiz🐳️docker-image` (`docker-image-build [--tag <tag>] [--jobs <n>]`) and
`⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-image` (`docker-image-check [--tag <tag>] [--port <n>] [--keep]`: ready behind
the proxy, site and SPA fallback served, cleartext refused, `docker stop` drains with exit code 0, `proctor.sqlite` on the
volume).

### DNS

Point the name at the host before the first start, so Caddy can answer the ACME challenge:

```
quizze.architektur-und-technologie.de.  A     <IPv4 of the host>
quizze.architektur-und-technologie.de.  AAAA  <IPv6 of the host>
```

Ports 80 and 443 must reach Caddy; port 8791 stays closed to the outside (compose publishes it on loopback only).

### First start

```sh
git clone https://github.com/usalu/semio.git && cd semio/🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy
docker compose up --build --detach
docker compose ps
curl --fail --header "X-Forwarded-Proto: https" http://127.0.0.1:8791/instance
sudo cp Caddyfile /etc/caddy/Caddyfile && sudo systemctl reload caddy
curl --fail https://quizze.architektur-und-technologie.de/instance
```

The build compiles the site and the proctor inside the builder stage (the build context is the repository root; the root
`.dockerignore` keeps host-local state out). Cap the compiler jobs on a small host with
`docker compose build --build-arg CARGO_BUILD_JOBS=2`.

### Backup

The whole state is `proctor.sqlite` with its WAL and SHM siblings on the volume. Copy it only while the proctor is stopped,
so the WAL is checkpointed and the copy is consistent:

```sh
docker compose stop proctor
docker run --rm --volume architecture-quiz_proctor-data:/data:ro --volume "$PWD/backup":/backup debian:bookworm-slim \
  sh -c 'stamp="/backup/$(date +%Y-%m-%dT%H%M)" && mkdir -p "$stamp" && cp -a /data/. "$stamp/"'
docker compose start proctor
```

Restore is the reverse copy into the stopped volume, then a start:

```sh
docker compose stop proctor
docker run --rm --volume architecture-quiz_proctor-data:/data --volume "$PWD/backup":/backup:ro debian:bookworm-slim \
  sh -c 'rm -f /data/proctor.sqlite* && cp -a /backup/<stamp>/. /data/'
docker compose start proctor
```

### Update

```sh
git pull
docker compose up --build --detach
```

The volume survives the new container. A quiz whose file changed gets a new revision: an open run on the old revision can no
longer record answers (`quiz-revised`) and is voided when it is submitted or started again; submitted results, badges and
the leaderboard stay.
`docker compose logs --follow proctor` shows the proctor's log.
