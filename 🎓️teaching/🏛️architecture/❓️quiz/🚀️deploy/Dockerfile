# 🐳️ The architecture quiz proctor (`semio.iek.uni-hannover.de`, see `🔣️.json`): one image, one
# process — the `proctor` release binary with the architecture catalog and every quiz it names, over one SQLite file on one
# volume. API only: the site is a static build on a CDN. No shell, no package manager, no repository, no Rust toolchain,
# no bun and no Nx at runtime: a distroless base (glibc and nothing else), the binary and the content.
#
#   docker build -f 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile -t ghcr.io/usalu/architecture-quiz-proctor .
#
# Zero-touch: the production configuration is baked in, so the image boots with nothing but a volume. It binds the
# container's interface in production mode, grants CORS to the site origin only and trusts the TLS-terminating proxy in
# front of it (compose.yaml puts Caddy there and exposes the port to nothing else). Every value stays overridable by env.
# The stack that runs it travels inside it: `/srv/quiz/deploy` holds this release's `compose.yaml` and `Caddyfile`, so a
# host needs neither a clone nor a download (`docker create` + `docker cp`, see ../README.md).
#
# The context is the repository root because the builder compiles from source; `Dockerfile.dockerignore` beside this file
# admits only what the build reads (Rust sources, manifests, the quiz content, the stack files), so the context stays small
# and the compile layer is reused until one of those changes. Both base images are pinned by digest; `linux/amd64` is what
# the workflow publishes, and the same file builds natively on `linux/arm64`. `docker-image-build`, `docker-image-check`
# and `docker-stack-check` of `@teaching/architecture-quiz` build and prove this file.

ARG RUST_IMAGE=rust:1.98.1-bookworm@sha256:93ce27a88655056a51dbdd8f5f2d7ddc071c7b0070fb288a37b5a285fc83971e
ARG RUNTIME_IMAGE=gcr.io/distroless/cc-debian12:nonroot@sha256:9dac0a79194e45a7da0158a9c6da57b217585af0786db3845d1f0ec1a0dd182f

FROM ${RUST_IMAGE} AS builder

# 🧵️ Optional cap on the builder's parallel compiler jobs (`--build-arg CARGO_BUILD_JOBS=2`); unset, cargo uses every core.
ARG CARGO_BUILD_JOBS

# 🧰️ The toolchain pinned by `rust-toolchain.toml`, in its own layer so a source change does not reinstall it.
WORKDIR /src
COPY rust-toolchain.toml ./
RUN rustup toolchain install

# 🦀️ The proctor alone, from the locked dependency graph of its workspace (`🎓️teaching/Cargo.toml` with its own
# `Cargo.lock`; the framework crates it depends on by path inherit from the root manifest). A private `CARGO_TARGET_DIR`
# puts the one uplifted binary at a fixed path; the registry and the shared build directory of `.cargo/config.toml` are
# cache mounts, so a rebuild compiles only what changed. The build is attempted twice: the pinned nightly compiler crashed
# once in 13 builds of this crate (SIGSEGV in LLVM while linking the binary with thin LTO, 2026-10-01), and cargo resumes
# from every unit already compiled, so the second attempt repeats only what crashed — and a real compile error is simply
# reported twice.
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/.🧬semio/🦑️repo/⚡️cache/cargo/build \
    cd "🎓️teaching" && for attempt in 1 2; do \
      CARGO_TARGET_DIR=/out cargo build --release --locked --package teaching-proctor --bin proctor && exit 0; \
    done; exit 1

# 📚️ What the runtime stage copies besides the binary: the catalog and every quiz in the relative layout the catalog's
# quiz paths are written against, the stack files of this release, and the empty data directory whose owner and mode a
# fresh volume inherits. The binary validates the staged catalog here, so an image with content it would refuse to
# serve is never built.
RUN mkdir --parents /stage/content /stage/deploy /stage/data \
 && cp "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/compose.yaml" "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Caddyfile" /stage/deploy/ \
 && cd "🎓️teaching" \
 && find "🏛️architecture" -path "*/❓️quiz/🔣️.json" -print0 \
  | tar --create --null --files-from=- \
  | tar --extract --directory=/stage/content \
 && /out/release/proctor check "/stage/content/🏛️architecture/❓️quiz/🔣️.json"

FROM ${RUNTIME_IMAGE} AS runtime

COPY --from=builder /out/release/proctor /usr/local/bin/proctor
COPY --from=builder /stage/content /srv/quiz/content
COPY --from=builder /stage/deploy /srv/quiz/deploy

# 🗄️ The one path the process writes: `proctor.sqlite` with its WAL and SHM siblings. Everything else is read-only, so the
# container runs with a read-only root filesystem.
COPY --from=builder --chown=65532:65532 --chmod=0700 /stage/data /srv/quiz/data
VOLUME ["/srv/quiz/data"]

# 👤️ The base image's unprivileged `nonroot` account, by number so an orchestrator can verify it.
USER 65532:65532
WORKDIR /srv/quiz

# 🌐️ The production gate in full: `PROCTOR_BIND=0.0.0.0` is the container's own interface (a loopback bind inside a
# container makes a published or exposed port unreachable); a network bind in production mode needs the origin allowlist
# (the CDN-hosted site) and the statement that a TLS-terminating proxy is the only client.
ENV PROCTOR_DATA=/srv/quiz/data \
    PROCTOR_CATALOG="/srv/quiz/content/🏛️architecture/❓️quiz/🔣️.json" \
    PROCTOR_BIND=0.0.0.0 \
    PROCTOR_PORT=8791 \
    PROCTOR_MODE=production \
    PROCTOR_ALLOWED_ORIGINS=https://quizze.architektur-und-technologie.de \
    PROCTOR_TRUSTED_FORWARDING=proxy

EXPOSE 8791

# 🩺️ `proctor health` asks the listener for `GET /instance` with `X-Forwarded-Proto: https`, as the proxy would: it answers
# once the catalog is validated and the read models are current. During the start period (long enough for a boot that
# refolds every read model after an update) the probe runs every 2 s and a failure does not count, so a start is
# reported healthy as soon as it is and never unhealthy while it still catches up.
HEALTHCHECK --interval=15s --timeout=5s --start-period=5m --start-interval=2s --retries=3 \
  CMD ["/usr/local/bin/proctor", "health"]

# 🛑️ The proctor is PID 1 and handles SIGTERM itself: it stops accepting, settles once more and closes the database.
STOPSIGNAL SIGTERM
ENTRYPOINT ["/usr/local/bin/proctor"]
CMD ["serve"]

# 🏷️ What the image is and where it came from; last, so a new revision reuses every layer above. `docker-image-build`
# passes the workspace version, the commit and the commit time.
ARG VERSION=0.0.0
ARG REVISION=unknown
ARG CREATED=1970-01-01T00:00:00Z
LABEL org.opencontainers.image.title="architecture-quiz-proctor" \
      org.opencontainers.image.description="Proctor API of the architecture quiz: quiz lifecycle, leaderboard and shared presence over one SQLite file" \
      org.opencontainers.image.source="https://github.com/usalu/semio" \
      org.opencontainers.image.licenses="AGPL-3.0-or-later" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${REVISION}" \
      org.opencontainers.image.created="${CREATED}"
