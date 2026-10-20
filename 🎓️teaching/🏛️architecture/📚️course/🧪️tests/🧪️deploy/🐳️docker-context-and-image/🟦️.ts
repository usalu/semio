/** 🐳️ The docker context, apex domain, schema contract, drift and proctor image tests. */
import Ajv from "ajv";
import picomatch from "picomatch";
import { existsSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { expect, it } from "vitest";
import { TEACHING_WORKSPACE } from "../../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import deployment from "../../../🚀️deploy/🔣️.json" with { type: "json" };
import courseContract from "../../../🧫️fixtures/📚️course/🔣️.json" with { type: "json" };
import dockerContext from "../../../🧫️fixtures/🐳️context/🔣️.json" with { type: "json" };
import { courseImageContent } from "../../../🚀️deploy/📦️image/🟦️.ts";
import architectureCourse from "../../../🔣️.json" with { type: "json" };
import schema from "../../../🚀️deploy/🧬️schema/🔣️.json" with { type: "json" };
import { COURSE_IMAGE_HEALTHCHECK, COURSE_IMAGE_USER, COURSE_PAGES_DIRECTORY, COURSE_PROCTOR_ORIGIN, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, deploymentDrift, hostZone } from "../../../🚀️deploy/🟦️.ts";
import { releaseProctorOrigin } from "../../../🚀️deploy/🌐️site/🟦️.ts";
import { compose, deploy, drifted, read, repoRoot, site } from "../🧰️deploy-fixtures/🟦️.ts";

/** 📦 Registers the docker context, apex domain, schema contract, source drift, image gate and runtime role tests. */
export function registerDockerContextAndImageTests(): void {
  it("includes every course quiz in the Docker context and excludes generated content", () => {
    const rules = read("Dockerfile.dockerignore").split(/\r?\n/u).filter(line => line && !line.startsWith("#"));
    for (const row of dockerContext.files) {
      const ancestors = row.path.split("/").map((_, index, parts) => parts.slice(0, index + 1).join("/"));
      const included = rules.reduce((admitted, rule) => {
        const include = rule.startsWith("!");
        const matches = picomatch(include ? rule.slice(1) : rule, { dot: true });
        return ancestors.some(path => matches(path)) ? include : admitted;
      }, true);
      expect(included, row.path).toBe(row.included);
    }
    const quizzes = architectureCourse.quizzes.map(quiz => "content/" + relative(join(repoRoot, TEACHING_WORKSPACE), resolve(site, quiz)).split(sep).join("/"));
    const covered = dockerContext.files.filter(row => row.included).map(row => row.path);
    expect(covered).toEqual(expect.arrayContaining(quizzes));
  });
  it("publishes the architecture course at its apex domain", () => {
    expect(deployment.site.host).toBe(courseContract.host);
    expect(hostZone(courseContract.host)).toBe(courseContract.host);
    expect(COURSE_PAGES_DIRECTORY).toBe(courseContract.pages);
    expect(compose.services.proctor?.environment?.PROCTOR_ALLOWED_ORIGINS).toContain(`https://${courseContract.host}`);
    expect(read("Dockerfile.dockerignore")).toContain("!content/**/🔣️.json");
    expect(read("Dockerfile")).toContain('/srv/course/content/🏛️architecture/📚️course/🔣️.json');
  });
  it("declares its hosts, image and port under the contract of its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(deployment), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...deployment, site: { ...deployment.site, legal: { imprint: "https://example.org/impressum", privacy: "https://example.org/datenschutz" } } })).toBe(true);
    const { host: _host, ...hostless } = deployment.proctor;
    for (const invalid of [{ ...deployment, proctor: hostless }, { ...deployment, proctor: { ...deployment.proctor, port: 80 } }, { ...deployment, proctor: { ...deployment.proctor, image: `${deployment.proctor.image}:latest` } }, { ...deployment, site: { host: "https://quizzes.example" } }, { ...deployment, site: { ...deployment.site, legal: { imprint: "impressum" } } }, { ...deployment, cdn: "pages" }]) expect(validate(invalid), JSON.stringify(invalid)).toBe(false);
    expect(deployment.proctor.host).not.toBe(deployment.site.host);
    expect([hostZone("quizze.example.org"), hostZone("semio.institute.example.org")]).toEqual(["example.org", "institute.example.org"]);
    expect(COURSE_SITE_ORIGIN).toBe(`https://${deployment.site.host}`);
    expect(COURSE_PROCTOR_ORIGIN).toBe(`https://${deployment.proctor.host}`);
    expect(releaseProctorOrigin({})).toBe(COURSE_PROCTOR_ORIGIN);
    expect(releaseProctorOrigin({ PROCTOR_URL: "https://staging.example" })).toBe("https://staging.example");
  });
  it("has no file that drifted from that source, and finds every seeded drift", () => {
    expect(deploymentDrift(repoRoot)).toEqual([]);
    for (const undeclared of [`quizzes.${hostZone(deployment.site.host)}`, `proctor.${deployment.site.host}`, `www.${hostZone(deployment.proctor.host)}`, `staging.${deployment.proctor.host}`]) expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/README.md", (text) => `${text}\nhttps://${undeclared}\n`))).toEqual([`🎓️teaching/🏛️architecture/📚️course/README.md names the host ${undeclared}, which 🔣️.json does not declare`]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/README.md", (text) => `${text}\n_github-pages-challenge-owner.${deployment.site.host} and https://${deployment.site.host} call https://${deployment.proctor.host}\n`))).toEqual([]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml", (text) => text.replace(`PROCTOR_HOST:-${deployment.proctor.host}`, `PROCTOR_HOST:-proctor.${hostZone(deployment.proctor.host)}`)))).toEqual([`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml names the host proctor.${hostZone(deployment.proctor.host)}, which 🔣️.json does not declare`, `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml lacks "\${PROCTOR_HOST:-${deployment.proctor.host}}"`]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml", (text) => text.replace(`./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`, `./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}`)))).toEqual([`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml lacks "- ./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro"`]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile", (text) => text.replace(`load /${COURSE_STACK_CERTIFICATES}`, "load /etc/ssl/private")))).toEqual([`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile lacks "load /${COURSE_STACK_CERTIFICATES}"`]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile", (text) => text.replace("> {$PROCTOR_LIMIT_BODY_BYTES:16384}", "> {$PROCTOR_LIMIT_BODY_BYTES:8192}").replace("respond @oversized 413", "respond @oversized 400")))).toEqual(['🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile lacks "respond @oversized 413"', "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile defaults PROCTOR_LIMIT_BODY_BYTES to 16384 and 8192"]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🛂️proctor/README.md", (text) => `${text}\nghcr.io/usalu/proctor:latest\n`))).toEqual([`🎓️teaching/🛂️proctor/README.md names the image ghcr.io/usalu/proctor, not ${deployment.proctor.image}`]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Dockerfile", (text) => text.replace(`EXPOSE ${deployment.proctor.port}`, "EXPOSE 8080")))).toEqual([`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Dockerfile lacks "EXPOSE ${deployment.proctor.port}"`]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Dockerfile", (text) => text.replace(/^(ARG RUNTIME_IMAGE=[^@]+)@sha256:[0-9a-f]+$/mu, "$1")))).toEqual([expect.stringMatching(/^the base image gcr\.io\/distroless\/\S+ is not pinned to an exact tag and digest$/u)]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml", (text) => text.replace(/image: caddy:\S+/u, "image: caddy:2")))).toEqual(["the base image caddy:2 is not pinned to an exact tag and digest"]);
    expect(deploymentDrift(repoRoot, drifted("🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile", (text) => text.replace(`proctor:${deployment.proctor.port}`, "proctor:8080").replace(/request_body \{[^}]*\}/u, "")))).toEqual([`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile lacks "reverse_proxy proctor:${deployment.proctor.port}"`, "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile: the Caddyfile caps no request body"]);
    expect(deploymentDrift(repoRoot, drifted(".github/workflows/architecture-course.yml", (text) => text.replace(/actions\/checkout@[0-9a-f]{40}/gu, "actions/checkout@v7").replace(":publish --configuration=release\n", ":release --configuration=release\n").replace(COURSE_PAGES_DIRECTORY, "pages")))).toEqual([expect.stringContaining(`dist/release/${COURSE_PAGES_DIRECTORY}`), ".github/workflows/architecture-course.yml uses actions/checkout@v7 without a commit hash", ".github/workflows/architecture-course.yml names @teaching/architecture-course:release, which is not a target of the package"]);
    expect(deploymentDrift(repoRoot, drifted("package.json", (text) => `${text}\n{"run":"@teaching/architecture-course:stack-check"}\n`))).toEqual(["package.json names @teaching/architecture-course:stack-check, which is not a target of the package"]);
  });
  it("bakes the production gate into a shell-less proctor image of one pinned runtime base", () => {
    const dockerfile = read("Dockerfile");
    for (const line of ["PROCTOR_MODE=production", `PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}`, "PROCTOR_TRUSTED_FORWARDING=proxy", "PROCTOR_BIND=0.0.0.0", `PROCTOR_PORT=${deployment.proctor.port}`, `EXPOSE ${deployment.proctor.port}`, `USER ${COURSE_IMAGE_USER}`, 'VOLUME ["/srv/course/data"]', "STOPSIGNAL SIGTERM", 'ENTRYPOINT ["/usr/local/bin/proctor"]', 'COPY --chmod=0555 proctor /usr/local/bin/proctor', 'RUN ["/usr/local/bin/proctor", "check", "/srv/course/content/🏛️architecture/📚️course/🔣️.json"]', "org.opencontainers.image.revision", "org.opencontainers.image.version", "org.opencontainers.image.source"]) expect(dockerfile).toContain(line);
    expect(/^HEALTHCHECK [^\n]*\\\n\s+CMD (\[.*\])$/mu.exec(dockerfile)?.[1]).toBe(JSON.stringify(COURSE_IMAGE_HEALTHCHECK.slice(1)).replace(",", ", "));
    expect([...dockerfile.matchAll(/^ARG (\w+_IMAGE)=\S+@sha256:[0-9a-f]{64}$/gmu)].map((line) => line[1])).toEqual(["RUNTIME_IMAGE"]);
    const runtime = dockerfile.slice(dockerfile.indexOf("AS runtime"));
    for (const absent of ["cargo", "rustup", "apt-get", "curl", "tini", "PROCTOR_SITE", "bun", "COPY . ."]) expect(runtime).not.toContain(absent);
    expect([...runtime.matchAll(/^RUN (.+)$/gmu)].map(row => JSON.parse(row[1]!))).toEqual([["/usr/local/bin/proctor", "check", "/srv/course/content/🏛️architecture/📚️course/🔣️.json"]]);
    const context = read("Dockerfile.dockerignore").split("\n").filter((line) => line && !line.startsWith("#"));
    expect(context[0]).toBe("*");
    expect(context).not.toContain("!Cargo.lock");
    for (const admitted of ["!proctor", "!Dockerfile", "!content/**/🔣️.json", "!deploy/compose.yaml", "!deploy/Caddyfile", "!data/"]) expect(context).toContain(admitted);
    expect(context.filter(line => line.startsWith("!")).some(line => /cargo|rust|\.rs$/iu.test(line))).toBe(false);
    for (const excluded of [".cargo", "**/.cargo", ".git", ".🧬semio", "**/node_modules", "**/target"]) expect(context.indexOf(excluded)).toBeGreaterThan(context.findLastIndex((line) => line.startsWith("!")));
  });
  it("packages only finite runtime roles and validates the actual nonroot runtime without a compiler", () => {
    const directives = read("Dockerfile").split(/\r?\n/u).filter(line => !line.trimStart().startsWith("#")).join("\n");
    expect([...directives.matchAll(/^FROM (.+)$/gmu)].map(row => row[1])).toEqual(["${RUNTIME_IMAGE} AS runtime"]);
    expect([...directives.matchAll(/^COPY .+$/gmu)].map(row => row[0])).toEqual(dockerContext.copies);
    expect([...directives.matchAll(/^RUN (.+)$/gmu)].map(row => JSON.parse(row[1]!))).toEqual([dockerContext.validation]);
    expect(directives).not.toMatch(/\b(?:cargo|rustup|rustc|apt-get|curl|bun)\b/u);
    expect(directives).not.toContain("COPY . .");
    expect(existsSync(join(deploy, "Dockerfile.cargo.toml"))).toBe(false);
    const selected = courseImageContent(repoRoot, join(site, "🔣️.json"), AbortSignal.timeout(5000)).map(row => row.path);
    expect(selected).toEqual(dockerContext.files.filter(row => row.included && row.path.startsWith("content/")).map(row => row.path));
  });
}
