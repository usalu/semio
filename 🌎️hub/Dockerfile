# syntax=docker/dockerfile:1.7
#
# 🐳️ `os-hub` container image — UNBUILT.
#
# This file was authored against the real build commands (`bun nx run os-hub-admin:build`,
# `os-hub:build`, `os-hub:build-dev`, `os-hub:dev`) on a machine with no Docker installed, so it has
# never been built or run. Treat every line as a reviewed proposal, not a proven recipe.
#
# ─── why the image carries the repository and not just a binary ───────────────────────────────────
#
# `os-hub` is not a standalone daemon today. `validate_auth_startup` (`🏗️bootstrap/🦀️.rs`) refuses
# `production` without an `IdentityAssertionVerifier` adapter — of which there is none — and
# `development` requires a protected `LocalBootstrapTransport`, which the binary obtains by reading
# an `initialize` frame off **inherited file descriptor 3** that the Nx launcher creates. A runtime
# image containing only the compiled binary would therefore exit at boot with
# `development requires a protected LocalBootstrapTransport adapter`. The launcher is the entrypoint
# because it is the only topology that boots; see `🌎️hub/README.md` § "Read this first".
#
# What the builder stage does buy: the runtime stage needs no Rust toolchain and no `cargo` at all,
# because `DevScript` execs the already-staged `dist/build-dev` binary and only re-stages it when it
# is missing.
#
#   docker build -f 🌎️hub/Dockerfile -t semio/os-hub:dev .          # context = repository root
#   docker run --rm -p 8787:8787 -v semio-hub-data:/srv/semio-hub/data semio/os-hub:dev
#
# ─────────────────────────────────────────────────────────────────────────────────────────────────

ARG RUST_IMAGE=rust:1-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUST_IMAGE} AS builder

# 🧰️ The toolchain is pinned by `rust-toolchain.toml` (a dated nightly, with the wasm targets the
# plugin crates need); `rustup` in the base image honours it on the first `cargo` invocation.
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates curl unzip pkg-config libssl-dev cmake python3 git \
 && rm --recursive --force /var/lib/apt/lists/*
RUN curl --fail --silent --show-error --location https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"
ENV NX_DAEMON=false

WORKDIR /src
COPY . .

# 📦️ Dependencies, then the admin SPA, then both hub binaries. `os-hub:build` is the release target
# the distribution tarball ships; `os-hub:build-dev` is what the launcher actually execs, and both
# are staged so the runtime stage never compiles anything.
RUN bun install --frozen-lockfile
RUN bun nx run os-hub-admin:build
RUN bun nx run os-hub:build
RUN bun nx run os-hub:build-dev

FROM ${RUNTIME_IMAGE} AS runtime

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates tini \
 && rm --recursive --force /var/lib/apt/lists/* \
 && useradd --system --create-home --home-dir /srv/semio-hub --shell /usr/sbin/nologin semio

COPY --from=builder /root/.bun/bin/bun /usr/local/bin/bun
COPY --from=builder --chown=semio:semio /src /srv/semio-hub/repo

# 🗄️ The one path the process must be able to write. Everything else may be read-only.
RUN install --directory --owner=semio --group=semio /srv/semio-hub/data
VOLUME ["/srv/semio-hub/data"]

USER semio
WORKDIR /srv/semio-hub/repo

ENV NX_DAEMON=false \
    OS_HUB_DATA=/srv/semio-hub/data \
    OS_HUB_BIND=127.0.0.1 \
    OS_HUB_PORT=8787 \
    OS_HUB_MODE=development \
    OS_HUB_CREDENTIAL_SIGN_IN=true \
    OS_HUB_STORAGE_BACKEND=fs \
    OS_HUB_DIRECTORY_BACKEND=sqlite

# 🌐️ Left unset on purpose: with a loopback bind the hub defaults to the `loopback-development`
# cross-origin policy. Set `OS_HUB_ALLOWED_ORIGINS` to the origins your `s` bundle is served from
# before putting this behind a proxy — see `🌎️hub/README.md` § "Cross-origin access".

EXPOSE 8787

HEALTHCHECK --interval=15s --timeout=5s --start-period=180s --retries=10 \
  CMD bun --eval "const r = await fetch('http://127.0.0.1:' + (process.env.OS_HUB_PORT ?? '8787') + '/readyz'); process.exit(r.ok ? 0 : 1)"

# 🛑️ `tini` as PID 1 so `docker stop`'s SIGTERM is forwarded rather than delivered to a shell that
# ignores it. The hub itself now installs a SIGTERM/SIGINT handler that stops accepting and then
# drains its task owners; whether Nx's own process tree forwards the signal all the way down to the
# launched binary is the part of this file that most needs a real `docker stop` to confirm.
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["bun", "nx", "run", "os-hub:dev"]
