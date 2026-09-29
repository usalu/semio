# ❓️ Quizze · Architektur und Technologie

The quiz website of **quizzes.architektur-und-technologie.de** and its proctor at
**proctor.quizzes.architektur-und-technologie.de**: the architecture catalog (`🔣️.json`, id `architecture`) with the quizzes
it names, rendered by `@semio-tech/quiz-react` in a static site on a CDN and proctored by `proctor`
(`🎓️teaching/🛂️proctor`), an API-only server over one SQLite file that runs as a zero-touch Docker stack. Both hosts, the
image and the port live in one place, `🚀️deploy/🔣️.json`; the Vite config, the operator verbs and the tests read them there.

| Path | What |
|---|---|
| `🔣️.json` | the catalog: introduction, quiz paths (relative to this file), badges |
| `🟦️.ts`, `🌐️.html` | the site entry: `mountQuiz(root, { proctor: <baked proctor origin or "">, tenant: "architecture" })` |
| `🎨️.css` | the brand layer: the semio palette compiled to custom properties, its fonts shipped with the build |
| `🏗️builder/🌐️vite/🟦️.ts` | Vite configuration: host HTML, `CNAME`, aliases; `build` bakes `VITE_PROCTOR_URL`, `serve` proxies the gateway routes to the dev proctor |
| `📦️packages/🟦️typescript` | `@teaching/architecture-quiz`: `package.json`, `📋️project.json`, `📜️script.ts` |
| `🧪️tests/🧪️catalog/🟦️.ts` | the catalog and every quiz validate in the TS core and against the draft-07 contract (ajv) |
| `🧪️tests/🧪️deploy/🟦️.ts` | Dockerfile, `compose.yaml` (parsed with `yaml`), Caddyfile and the publish check agree with `🚀️deploy/🔣️.json` |
| `🚀️deploy/` | `🔣️.json` (hosts), `Dockerfile`, `compose.yaml`, `Caddyfile` and the operator verbs (`🟦️.ts`) |

## Develop

Run everything from `.vscode/launch.json` (group `3_dev`):

| Launch row | Command | Port |
|---|---|---|
| `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` | both rows below | |
| `🛠️dev🎓️teaching🛂️proctor` | `bun nx run @teaching/proctor:dev` | `8791` (`PROCTOR_PORT`) |
| `🛠️dev🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:dev` | `6061` (`TEACHING_ARCHITECTURE_QUIZ_PORT`) |
| `🧪️test🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:test` | |
| `✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog` | `bun nx run @teaching/architecture-quiz:check` (`proctor check` on the catalog) | |
| `🔁️rebuild🎓️teaching🛂️proctor` | `bun nx run @teaching/proctor:rebuild` (refold the dev proctor's read models) | |

The dev site bakes no proctor origin and proxies `/instance`, `/commands`, `/queries`, `/actors` (WebSocket event streams
included) and `/scopes` to `http://127.0.0.1:${PROCTOR_PORT:-8791}`, so the browser talks to one origin in dev.

## Deploy

Two artifacts, released independently:

| Artifact | Where | Built and proven by |
|---|---|---|
| the site: static files with `index.html`, `404.html` (path routes), `CNAME`, `.nojekyll`, `_headers` | any static CDN at `https://quizzes.architektur-und-technologie.de` (GitHub Pages by default) | `🚚️publish🎓️teaching🏛️architecture❓️quiz` (`publish`) |
| the proctor image `ghcr.io/usalu/architecture-quiz-proctor:<version>`/`latest` | one Docker host at `https://proctor.quizzes.architektur-und-technologie.de` | `📦️build…🐳️docker-image`, `⚖️gate…🐳️docker-image`, `⚖️gate…🐳️docker-stack`, `🚚️publish…🐳️docker-image` |

### CORS contract

The site calls the proctor cross-origin, without credentials. The proctor grants exactly `PROCTOR_ALLOWED_ORIGINS`
(default `https://quizzes.architektur-und-technologie.de`, baked into the image): preflights of `POST /commands` and
`POST /queries` answer `204` with the echoed origin, `content-type` allowed and a cache lifetime, every other origin gets no
grant. The site bakes its proctor origin at build time: `PROCTOR_URL`, else `https://proctor.quizzes.architektur-und-technologie.de`.
A staging site on another origin needs both: `PROCTOR_URL` when building it, and its origin added (comma-separated) to
`PROCTOR_ALLOWED_ORIGINS` on the proctor host.

### DNS

```
quizzes.architektur-und-technologie.de.          CNAME  usalu.github.io.        ; the CDN (GitHub Pages)
proctor.quizzes.architektur-und-technologie.de.  A      <IPv4 of the Docker host>
proctor.quizzes.architektur-und-technologie.de.  AAAA   <IPv6 of the Docker host>
```

### Site on the CDN

`bun nx run @teaching/architecture-quiz:publish` builds the site with the production proctor origin, refuses an artifact
without `index.html`, `404.html`, `.nojekyll`, a `CNAME` naming the site host, the baked proctor origin, or with any
loopback address, and stages it in `📦️packages/🟦️typescript/dist/pages/quizzes`. `_headers` there marks the hashed
`/assets/*` immutable and the documents `no-cache` for CDNs that honour it (Cloudflare Pages, Netlify; GitHub Pages ignores
it). Upload that folder to any static host, or run the manual workflow `.github/workflows/architecture-quiz.yml` (job
`site`), which deploys it to GitHub Pages. One-time GitHub settings: Settings → Pages → Source "GitHub Actions", custom
domain `quizzes.architektur-und-technologie.de`, "Enforce HTTPS".

### Proctor on Docker (zero-touch)

The host needs Docker Engine with the compose plugin, the DNS records above, and ports 80, 443 and 443/udp open. Nothing
else: the image carries the production configuration (mode, origin allowlist, trusted forwarding, catalog and data
paths, bind and port), `compose.yaml` adds Caddy, which obtains and renews the certificate for the proctor host itself, and
keeps the proctor port on the compose network only.

```sh
git clone https://github.com/usalu/semio.git && cd semio/🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy
docker compose up -d
curl --fail https://proctor.quizzes.architektur-und-technologie.de/instance
```

`docker compose up -d` pulls `ghcr.io/usalu/architecture-quiz-proctor:latest` when the host does not have it
(`pull_policy: missing`); `docker compose up -d --build` compiles it from the clone instead (for an architecture the registry
does not carry; cap the compiler with `docker compose build --build-arg CARGO_BUILD_JOBS=2` on a small host). Caddy starts
once the proctor's health check passes. Overrides, from the environment or an `.env` beside `compose.yaml`: `PROCTOR_HOST`,
`PROCTOR_TAG`, `PROCTOR_ALLOWED_ORIGINS`, `QUIZ_HTTP_PORT`, `QUIZ_HTTPS_PORT`.

Publishing the image: `bun nx run @teaching/architecture-quiz:docker-image-publish` builds it and pushes
`<workspace version>` and `latest` with your own `docker login ghcr.io`, or run the workflow's `proctor` job
(`GITHUB_TOKEN`). After the first push, set the package's visibility to public on GitHub so hosts pull without login.

### Backup

The whole state is `proctor.sqlite` with its WAL and SHM siblings on the volume `architecture-quiz_proctor-data`. Copy it only
while the proctor is stopped, so the WAL is checkpointed and the copy is consistent:

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

`caddy-data` holds the ACME account and certificates; keep it to avoid re-issuing them.

### Update

```sh
git pull
docker compose pull proctor && docker compose up -d
```

Publish the site again for a new client. The volume survives the new container. A quiz whose file changed gets a new
revision: an open run on the old revision can no longer record answers (`quiz-revised`) and is voided when it is submitted
or started again; submitted results, badges and the leaderboard stay. `docker compose logs --follow proctor caddy` shows
both logs.
