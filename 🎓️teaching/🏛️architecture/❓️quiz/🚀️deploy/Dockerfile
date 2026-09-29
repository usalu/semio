# syntax=docker/dockerfile:1.7
#
# 🐳️ The architecture quiz proctor (`proctor.quizzes.architektur-und-technologie.de`, see `🔣️.json`): one image, one
# process — the `proctor` release binary with the architecture catalog and every quiz it names, over one SQLite file on one
# volume. API only: the site is a static build on a CDN. No repository, no Rust toolchain, no bun and no Nx at runtime.
#
#   docker build -f 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile -t ghcr.io/usalu/architecture-quiz-proctor .
#
# Zero-touch: the production configuration is baked in, so the image boots with nothing but a volume. It binds the
# container's interface in production mode, grants CORS to the site origin only and trusts the TLS-terminating proxy in
# front of it (compose.yaml puts Caddy there and exposes the port to nothing else). Every value stays overridable by env.
#
# The context is the repository root because the builder compiles from source; the root `.dockerignore` keeps host-local
# state (`.🧬semio`, `node_modules`, `dist`, cargo output) out of it. `docker-image-build`, `docker-image-check` and
# `docker-stack-check` of `@teaching/architecture-quiz` build and prove this file.

ARG RUST_IMAGE=rust:1-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUST_IMAGE} AS builder

# 🧵️ Optional cap on the builder's parallel compiler jobs (`--build-arg CARGO_BUILD_JOBS=2`); unset, cargo uses every core.
ARG CARGO_BUILD_JOBS

# 🧰️ The toolchain pinned by `rust-toolchain.toml`, in its own layer so a source change does not reinstall it.
WORKDIR /src
COPY rust-toolchain.toml ./
RUN rustup toolchain install

# 🦀️ The proctor alone. A private `CARGO_TARGET_DIR` puts the one uplifted binary at a fixed path; the registry and the
# shared build directory of `.cargo/config.toml` are cache mounts, so a rebuild compiles only what changed.
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/.🧬semio/🦑️repo/⚡️cache/cargo/build \
    CARGO_TARGET_DIR=/out cargo build --release --locked --package teaching-proctor --bin proctor

# 📚️ The catalog and every quiz, keeping the relative layout the catalog's quiz paths are written against.
RUN mkdir --parents /stage/content \
 && cd "🎓️teaching" \
 && find "🏛️architecture" -path "*/❓️quiz/🔣️.json" -not -path "*/node_modules/*" -print0 \
  | tar --create --null --files-from=- \
  | tar --extract --directory=/stage/content

FROM ${RUNTIME_IMAGE} AS runtime

# 🏷️ Links the registry package to its source repository.
LABEL org.opencontainers.image.source="https://github.com/usalu/semio" \
      org.opencontainers.image.title="architecture-quiz-proctor"

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates curl tini \
 && rm --recursive --force /var/lib/apt/lists/* \
 && useradd --system --create-home --home-dir /srv/quiz --shell /usr/sbin/nologin quiz

# 🗿️ Two artifacts and nothing else: the binary and the content.
COPY --from=builder /out/release/proctor /usr/local/bin/proctor
COPY --from=builder /stage/content /srv/quiz/content

# 🗄️ The one path the process writes: `proctor.sqlite` with its WAL and SHM siblings. Everything else is read-only.
RUN install --directory --owner=quiz --group=quiz --mode=0700 /srv/quiz/data
VOLUME ["/srv/quiz/data"]

USER quiz
WORKDIR /srv/quiz

# 🌐️ The production gate in full: `PROCTOR_BIND=0.0.0.0` is the container's own interface (a loopback bind inside a
# container makes a published or exposed port unreachable); a network bind in production mode needs the origin allowlist
# (the CDN-hosted site) and the statement that a TLS-terminating proxy is the only client.
ENV PROCTOR_DATA=/srv/quiz/data \
    PROCTOR_CATALOG="/srv/quiz/content/🏛️architecture/❓️quiz/🔣️.json" \
    PROCTOR_BIND=0.0.0.0 \
    PROCTOR_PORT=8791 \
    PROCTOR_MODE=production \
    PROCTOR_ALLOWED_ORIGINS=https://quizzes.architektur-und-technologie.de \
    PROCTOR_TRUSTED_FORWARDING=proxy

EXPOSE 8791

# 🩺️ `X-Forwarded-Proto: https` speaks as the proxy would: with `PROCTOR_TRUSTED_FORWARDING=proxy` cleartext is refused.
HEALTHCHECK --interval=15s --timeout=5s --start-period=20s --retries=5 \
  CMD curl --fail --silent --show-error --max-time 4 \
      --header "X-Forwarded-Proto: https" \
      "http://127.0.0.1:${PROCTOR_PORT}/instance" > /dev/null

# 🛑️ `tini` as PID 1 so `docker stop`'s SIGTERM reaches the proctor, which stops accepting and closes the database.
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/proctor"]
CMD ["serve"]
