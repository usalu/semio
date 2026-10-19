# 🐳️ Runtime-only packaging of an authenticated release built by the canonical native Linux compiler owner.
# The owned image command supplies a finite tar context on stdin: executable, selected course JSON, stack files and data.
# No repository source, compiler, toolchain, shell or package manager enters this recipe.
ARG RUNTIME_IMAGE=gcr.io/distroless/cc-debian12:nonroot@sha256:9dac0a79194e45a7da0158a9c6da57b217585af0786db3845d1f0ec1a0dd182f

FROM ${RUNTIME_IMAGE} AS runtime

COPY --chmod=0555 proctor /usr/local/bin/proctor
COPY content /srv/course/content
COPY deploy /srv/course/deploy
COPY --chown=65532:65532 --chmod=0700 data /srv/course/data
USER 65532:65532
WORKDIR /srv/course

# 📚️ Validates the actual final runtime's loader, architecture and all selected course content without a shell.
RUN ["/usr/local/bin/proctor", "check", "/srv/course/content/🏛️architecture/📚️course/🔣️.json"]
VOLUME ["/srv/course/data"]
ENV PROCTOR_DATA=/srv/course/data \
    PROCTOR_COURSE="/srv/course/content/🏛️architecture/📚️course/🔣️.json" \
    PROCTOR_BIND=0.0.0.0 \
    PROCTOR_PORT=8791 \
    PROCTOR_MODE=production \
    PROCTOR_ALLOWED_ORIGINS=https://architektur-und-technologie.de \
    PROCTOR_TRUSTED_FORWARDING=proxy
EXPOSE 8791
HEALTHCHECK --interval=15s --timeout=5s --start-period=5m --start-interval=2s --retries=3 \
  CMD ["/usr/local/bin/proctor", "health"]
STOPSIGNAL SIGTERM
ENTRYPOINT ["/usr/local/bin/proctor"]
CMD ["serve"]

ARG VERSION=0.0.0
ARG REVISION=unknown
ARG CREATED=1970-01-01T00:00:00Z
LABEL org.opencontainers.image.title="architecture-course-proctor" \
      org.opencontainers.image.description="Proctor API of the architecture course: quiz lifecycle, leaderboard and shared presence over one SQLite file" \
      org.opencontainers.image.source="https://github.com/usalu/semio" \
      org.opencontainers.image.licenses="AGPL-3.0-or-later" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${REVISION}" \
      org.opencontainers.image.created="${CREATED}"
