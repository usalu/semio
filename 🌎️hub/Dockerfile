# syntax=docker/dockerfile:1.7
#
# 🐳️ `os-hub` container image — the PRODUCTION topology: one release binary, no repository, no Rust
# toolchain, no `bun`, no Nx in the runtime layer.
#
# ─── why this is a binary-only image ──────────────────────────────────────────────────────────────
#
# An earlier revision of this file carried the whole repository and ran `bun nx run os-hub:dev` as
# its entrypoint, because `validate_auth_startup` (`🏗️bootstrap/🦀️.rs`) then refused `production`
# without an `IdentityAssertionVerifier` adapter — of which this repository still ships none — so
# *development* mode was the only topology that booted, and development mode reads its
# `LocalBootstrapTransport` off an inherited file descriptor 3 that only the Nx launcher creates.
#
# That gate is gone. Production now requires *an identity authority*, and the hub grew its own:
# `os-hub credential set` seeds the first user with no server running and `POST /auth/sessions`
# mints the session, so `OS_HUB_CREDENTIAL_SIGN_IN=true` is enough. A bare `os-hub` process boots.
# The launcher, `bun`, Nx and the repository are all dev-loop concerns and none of them belong in a
# server image; `bun nx run os-hub:dev` is still the right entrypoint for a *development* container,
# and that is not what this file is for.
#
#   docker build -f 🌎️hub/Dockerfile -t semio/os-hub .              # context = repository root
#   docker run --rm -p 8787:8787 -v semio-hub-data:/srv/semio-hub/data \
#     -e OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com \
#     -e OS_HUB_ALLOWED_ORIGINS=https://s.example.com \
#     -e OS_HUB_TRUSTED_FORWARDING=proxy semio/os-hub
#
# Seed the first user into the volume before the first start — the verb needs the data root, not a
# running hub, and there is no sign-up:
#
#   docker run --rm -it -v semio-hub-data:/srv/semio-hub/data semio/os-hub \
#     credential set --email ada@example.com --display-name Ada
#
# ─── what has actually been verified about this file ──────────────────────────────────────────────
#
# `docker build --check` (buildx 0.34, Docker 29.5) passes with no warnings, and
# `docker compose -f 🌎️hub/compose.yaml config` resolves. The image itself has **not** been built:
# the builder stage compiles the hub's full release dependency graph, which on the machine this was
# written on takes longer than the session that wrote it. The binary it would produce, however, has
# been built and run outside a container in exactly this posture — release profile, plain process,
# network bind, allowlist + proxy trust, credential sign-in, SIGTERM drain (ticket 26/09/18,
# `📓️rb1-release-builds-and-production-posture.md`).
#
# ─────────────────────────────────────────────────────────────────────────────────────────────────

ARG RUST_IMAGE=rust:1-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUST_IMAGE} AS builder

# 🧵️ Optional cap on the builder's parallel compiler jobs (`docker build --build-arg CARGO_BUILD_JOBS=2`),
# for a builder VM that shares its cores or memory with other work. Unset, cargo uses every core.
ARG CARGO_BUILD_JOBS

# 🧰️ The toolchain is pinned by `rust-toolchain.toml` (a dated nightly, with the wasm targets the
# plugin crates need); `rustup` in the base image honours it on the first `cargo` invocation.
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates curl unzip pkg-config libssl-dev cmake python3 git mold \
 && rm --recursive --force /var/lib/apt/lists/*
RUN curl --fail --silent --show-error --location https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"
ENV NX_DAEMON=false

WORKDIR /src
COPY . .

# 📦️ Dependencies, then the admin SPA, then the release binary. Each project's own `📜️script.ts`
# verb is invoked directly rather than through `nx run`: the targets are one-line
# `nx:run-commands` wrappers around exactly these commands, and the Nx project-graph pass costs
# minutes in a cold container for nothing. `os-hub:build-dev` is deliberately NOT built — the dev
# launcher is not part of this image.
RUN bun install --frozen-lockfile
RUN cd "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript" && bun ./📜️script.ts build
RUN cd "🌎️hub/📦️packages/🦀️rust" && bun ./📜️script.ts build

FROM ${RUNTIME_IMAGE} AS runtime

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates curl tini \
 && rm --recursive --force /var/lib/apt/lists/* \
 && useradd --system --create-home --home-dir /srv/semio-hub --shell /usr/sbin/nologin semio

# 🗿️ Two artifacts and nothing else: the binary, and the admin SPA it serves. `OS_HUB_ADMIN_DIR` is
# mandatory here — the compile-time default points at the crate's source tree, which this image does
# not carry.
COPY --from=builder /src/🌎️hub/📦️packages/🦀️rust/dist/build/os-hub /usr/local/bin/os-hub
COPY --from=builder /src/🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist /srv/semio-hub/admin

# 🗄️ The one path the process must be able to write. Everything else may be read-only.
RUN install --directory --owner=semio --group=semio /srv/semio-hub/data
VOLUME ["/srv/semio-hub/data"]

USER semio
WORKDIR /srv/semio-hub

ENV OS_HUB_DATA=/srv/semio-hub/data \
    OS_HUB_ADMIN_DIR=/srv/semio-hub/admin \
    OS_HUB_BIND=0.0.0.0 \
    OS_HUB_PORT=8787 \
    OS_HUB_MODE=production \
    OS_HUB_CREDENTIAL_SIGN_IN=true \
    OS_HUB_STORAGE_BACKEND=fs \
    OS_HUB_DIRECTORY_BACKEND=sqlite

# 🌐️ `OS_HUB_BIND=0.0.0.0` is the container's own interface — a loopback bind inside a container
# makes `-p` unreachable, which is what the previous revision of this file shipped. A non-loopback
# bind in production mode is admitted only with BOTH of the following, so the container refuses to
# start until the operator states them, by design:
#
#     OS_HUB_ALLOWED_ORIGINS=https://s.example.com     which browser origins may hold a session
#     OS_HUB_TRUSTED_FORWARDING=proxy                  TLS is terminated in front of this socket
#
# and with `OS_HUB_ADMIN_SUBJECTS=credential.password.v1:<the email you seeded>`, which production
# mode requires outright. See `🌎️hub/README.md` § "Read this first".

EXPOSE 8787

# 🩺️ `X-Forwarded-Proto: https` is not decoration: with `OS_HUB_TRUSTED_FORWARDING=proxy` the
# transport-security layer sits outermost on the whole router, `/healthz` included, and answers
# `403 x-semio-refusal: insecure-transport` to anything that does not speak as the proxy would.
HEALTHCHECK --interval=15s --timeout=5s --start-period=120s --retries=10 \
  CMD curl --fail --silent --show-error --max-time 4 \
      --header "X-Forwarded-Proto: https" \
      "http://127.0.0.1:${OS_HUB_PORT}/readyz" > /dev/null

# 🛑️ `tini` as PID 1 so `docker stop`'s SIGTERM reaches the hub, which stops accepting, lets
# in-flight requests finish, then drains its task owners and exits 0. Give it room:
# `stop_grace_period`/`--stop-timeout` of 30s, not Docker's default 10.
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/os-hub"]
