# syntax=docker/dockerfile:1.7
#
# 🐳️ quizze.architektur-und-technologie.de — one image, one process: the `proctor` release binary serving the built quiz
# website, the architecture catalog with every quiz it names, and the gateway routes, over one SQLite file on one volume.
# No repository, no Rust toolchain, no bun and no Nx in the runtime layer.
#
#   docker build -f 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile -t semio/architecture-quiz .   # context = repository root
#   docker run --rm -p 127.0.0.1:8791:8791 -v architecture-quiz-data:/srv/quiz/data \
#     -e PROCTOR_ALLOWED_ORIGINS=https://quizze.architektur-und-technologie.de \
#     -e PROCTOR_TRUSTED_FORWARDING=proxy semio/architecture-quiz
#
# The context is the repository root because the builder compiles from source; the root `.dockerignore` keeps host-local
# state (`.🧬semio`, `node_modules`, `dist`, cargo output) out of it. `docker-image-build` and `docker-image-check` of
# `@teaching/architecture-quiz` build this file cold and run the result in its production posture.

ARG RUST_IMAGE=rust:1-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUST_IMAGE} AS builder

# 🧵️ Optional cap on the builder's parallel compiler jobs (`--build-arg CARGO_BUILD_JOBS=2`); unset, cargo uses every core.
ARG CARGO_BUILD_JOBS

# 🧰️ The toolchain is pinned by `rust-toolchain.toml`; `rustup` in the base image honours it on the first `cargo` call.
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates curl unzip \
 && rm --recursive --force /var/lib/apt/lists/*
RUN curl --fail --silent --show-error --location https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"
ENV NX_DAEMON=false

WORKDIR /src
COPY . .

# 📦️ Dependencies, the site, then the proctor. The site's own `📜️script.ts` verb is invoked directly rather than through
# `nx run` (the target is a one-line wrapper around it, and the project-graph pass costs minutes in a cold container).
# A private `CARGO_TARGET_DIR` puts the one uplifted binary at a fixed path.
RUN bun install --frozen-lockfile
RUN cd "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript" && bun ./📜️script.ts build
RUN CARGO_TARGET_DIR=/out cargo build --release --locked --package teaching-proctor --bin proctor

# 📚️ The catalog and every quiz, keeping the relative layout the catalog's quiz paths are written against.
RUN mkdir --parents /stage/content \
 && cd "🎓️teaching" \
 && find "🏛️architecture" -path "*/❓️quiz/🔣️.json" -not -path "*/node_modules/*" -print0 \
  | tar --create --null --files-from=- \
  | tar --extract --directory=/stage/content

FROM ${RUNTIME_IMAGE} AS runtime

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install --yes --no-install-recommends ca-certificates curl tini \
 && rm --recursive --force /var/lib/apt/lists/* \
 && useradd --system --create-home --home-dir /srv/quiz --shell /usr/sbin/nologin quiz

# 🗿️ Three artifacts and nothing else: the binary, the built site and the content.
COPY --from=builder /out/release/proctor /usr/local/bin/proctor
COPY --from=builder "/src/🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/dist" /srv/quiz/site
COPY --from=builder /stage/content /srv/quiz/content

# 🗄️ The one path the process writes: `proctor.sqlite` with its WAL and SHM siblings. Everything else is read-only.
RUN install --directory --owner=quiz --group=quiz --mode=0700 /srv/quiz/data
VOLUME ["/srv/quiz/data"]

USER quiz
WORKDIR /srv/quiz

# 🌐️ `PROCTOR_BIND=0.0.0.0` is the container's own interface; a loopback bind inside a container makes `-p` unreachable.
# A network bind in production mode is admitted only with BOTH of the following, so the container refuses to start until
# the operator states them:
#
#     PROCTOR_ALLOWED_ORIGINS=https://quizze.architektur-und-technologie.de   which browser origins may call the gateway
#     PROCTOR_TRUSTED_FORWARDING=proxy                                        TLS is terminated in front of this socket
ENV PROCTOR_DATA=/srv/quiz/data \
    PROCTOR_SITE=/srv/quiz/site \
    PROCTOR_CATALOG="/srv/quiz/content/🏛️architecture/❓️quiz/🔣️.json" \
    PROCTOR_BIND=0.0.0.0 \
    PROCTOR_PORT=8791 \
    PROCTOR_MODE=production

EXPOSE 8791

# 🩺️ `X-Forwarded-Proto: https` speaks as the proxy would: with `PROCTOR_TRUSTED_FORWARDING=proxy` cleartext is refused.
HEALTHCHECK --interval=15s --timeout=5s --start-period=20s --retries=5 \
  CMD curl --fail --silent --show-error --max-time 4 \
      --header "X-Forwarded-Proto: https" \
      "http://127.0.0.1:${PROCTOR_PORT}/instance" > /dev/null

# 🛑️ `tini` as PID 1 so `docker stop`'s SIGTERM reaches the proctor, which stops accepting and closes the database.
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/proctor"]
CMD ["serve"]
