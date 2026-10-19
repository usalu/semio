/** 🧪️ The split deployment agrees with its one authored source (`🚀️deploy/🔣️.json`, held to its JSON Schema by the
 * third-party `ajv`): nothing in the repository drifts from it and every seeded drift is found; the Dockerfile bakes the
 * production gate into a shell-less image of one pinned runtime base; `compose.yaml` (parsed by the third-party `yaml` library) runs
 * the proctor unexposed, hardened, limited and log-rotated behind a pinned Caddy; the Caddyfile serves the proctor host with
 * a body cap, timeouts and response headers and loads the operator's certificates from the directory the stack mounts
 * read-only; the workflow (parsed by `yaml`) releases from the repository's default branch only with pinned actions and the least permissions;
 * `publish` accepts exactly a CDN artifact whose document is sealed for the baked proctor origin, with hashed assets, no
 * development leftover and within its size budget; and the bundle for the proctor host pins its tag, lists exactly the
 * defaults of `compose.yaml` and tells every step with the hosts of the one source. The readiness gate type-checks the
 * package's sources as its second step — the steps, the package's targets and the README agree.
 * @see ../../🚀️deploy/🔣️.json — the hosts, image and port
 * @see ../../🚀️deploy/🧬️schema/🔣️.json — their contract
 * @see ../../🚀️deploy/🟦️.ts — the operator verbs under test */
import Ajv from "ajv";
import { sha256 } from "@noble/hashes/sha2.js";
import picomatch from "picomatch";
import { createHash, randomBytes } from "node:crypto";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import { afterEach, describe, expect, it } from "vitest";
import { TEACHING_WORKSPACE } from "../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import { writeAssetArtifacts } from "../../../../../🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts";
import publication from "../../../../../🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🧫️fixtures/🔣️.json" with { type: "json" };
import deployment from "../../🚀️deploy/🔣️.json" with { type: "json" };
import courseContract from "../../🧫️fixtures/📚️course/🔣️.json" with { type: "json" };
import dockerContext from "../../🧫️fixtures/🐳️context/🔣️.json" with { type: "json" };
import { courseImageContent } from "../../🚀️deploy/📦️image/🟦️.ts";
import bundleCases from "../../🧫️fixtures/🚛️bundle/🔣️.json" with { type: "json" };
import { stackBundleImages, stackBundlePlan } from "../../🚀️deploy/🟦️.ts";
import { COURSE_DEPLOY_CLEANUP_MS, deploymentInterruption } from "../../🚀️deploy/🟦️.ts";
import cancellation from "../../🧫️fixtures/🛑️cancellation/🔣️.json" with { type: "json" };
import { getEventListeners } from "node:events";
import architectureCourse from "../../🔣️.json" with { type: "json" };
import readiness from "../../🧫️fixtures/🚀️readiness/🔣️.json" with { type: "json" };
import sitePublication from "../../🧫️fixtures/🌐️publication/🔣️.json" with { type: "json" };
import schema from "../../🚀️deploy/🧬️schema/🔣️.json" with { type: "json" };
import { COURSE_BUNDLE_DIRECTORY, COURSE_BUNDLE_IMAGE, COURSE_IMAGE_HEALTHCHECK, COURSE_IMAGE_USER, COURSE_PAGES_DIRECTORY, COURSE_PROCTOR_ORIGIN, COURSE_SITE_BUDGET, COURSE_SITE_HEADERS, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, COURSE_STACK_FILES, caddySizeBytes, courseReleaseBuildPlan, deployCheckSteps, deploymentDrift, hostZone, releaseProctorOrigin, siteArtifactProblems, siteDocumentProblems, stageCourseSiteArtifact, stackBundleEnvironment, stackBundleReadme, stackRequestBodyLimit } from "../../🚀️deploy/🟦️.ts";

const site = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(site, "../../..");
const deploy = join(site, "🚀️deploy");
const read = (name: string): string => readFileSync(join(deploy, name), "utf8");
const DRIFT_SOURCES = ["🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Dockerfile", "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml", "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile", ".github/workflows/architecture-course.yml", "🎓️teaching/🏛️architecture/📚️course/README.md", "🎓️teaching/🛂️proctor/README.md", "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json", "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json", "package.json"] as const;
type Limits = { cpus: string; memory: string; pids: number };
type Service = { image: string; pull_policy?: string; build?: unknown; expose?: string[]; ports?: string[]; env_file?: { path: string; required: boolean }[]; environment?: Record<string, string>; volumes?: string[]; depends_on?: Record<string, { condition: string }>; healthcheck?: { test: string[] }; read_only?: boolean; cap_drop?: string[]; cap_add?: string[]; security_opt?: string[]; stop_grace_period?: string; restart?: string; logging?: { driver: string; options: Record<string, string> }; deploy?: { resources: { limits: Limits } } };
type Step = { uses?: string; run?: string; with?: Record<string, unknown>; "continue-on-error"?: boolean };
type Job = { if?: string; needs?: string; outputs?: Record<string,string>; "runs-on": string; "timeout-minutes"?: number; "continue-on-error"?: boolean; permissions?: Record<string, string>; steps: Step[] };
const compose = parse(read("compose.yaml")) as { name: string; services: Record<string, Service>; volumes: Record<string, unknown> };
const workflow = parse(readFileSync(join(repoRoot, ".github/workflows/architecture-course.yml"), "utf8")) as { on: Record<string, unknown>; permissions: Record<string, string>; concurrency: Record<string, unknown>; jobs: Record<string, Job> };
const scratch: string[] = [];

for (const row of publication.cases) it(`publishes ${row.id} assets without rewriting unchanged files`, () => {
  const parent = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated", "nx", "asset-publication") : join(tmpdir(), "asset-publication");
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(parent, "run-"));
  scratch.push(root);
  const file = join(root, "asset.txt");
  const before = new Date("2000-01-01T00:00:00Z");
  if (row.before !== null) { writeFileSync(file, row.before); utimesSync(file, before, before); }
  writeAssetArtifacts([{ path: file, content: row.output }]);
  expect(createHash("sha256").update(readFileSync(file)).digest("hex")).toBe(createHash("sha256").update(Buffer.from(row.output, "utf8")).digest("hex"));
  expect(statSync(file).mtimeMs !== before.getTime()).toBe(row.rewrite);
});

/** 📂️ A throw-away directory holding `files` (relative path → content). */
function tree(files: Record<string, string | Uint8Array>): string {
  const parent = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : tmpdir();
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(parent, "course-deploy-"));
  scratch.push(root);
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

/** 🪞️ Holds the drift sources in memory with one seeded change and no checkout files copied into ticket output. */
function drifted(path: (typeof DRIFT_SOURCES)[number], change: (text: string) => string): (source: string) => string {
  const sources = new Map<string, string>(DRIFT_SOURCES.map(source => [source, readFileSync(join(repoRoot, source), "utf8")]));
  sources.set(path, change(sources.get(path)!));
  return source => {
    const text = sources.get(source);
    if (text === undefined) throw Error("Unseeded deployment drift source: " + source);
    return text;
  };
}

const digest = (text: string): string => `'sha256-${createHash("sha256").update(text).digest("base64")}'`;
const BOOT = "document.documentElement.dataset.ready='1'";
const STYLE = "body{margin:0}";

/** 📄️ A release document sealed for `origin` the way the site's builder seals it: `policy` replaces the whole policy. */
function sealed(origin: string, policy = `default-src 'self'; script-src 'self' ${digest(BOOT)}; style-src 'self' ${digest(STYLE)}; style-src-attr 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src ${origin} ${origin.replace(/^http/u, "ws")}; manifest-src 'self'; object-src 'none'; base-uri 'none'; form-action 'self'`): string {
  return `<!doctype html>\n<html>\n  <head>\n    <meta charset="UTF-8" />\n    <meta http-equiv="Content-Security-Policy" content="${policy}" />\n    <title>Architecture and Technology</title>\n    <meta name="description" lang="en" content="Quizzes" />\n    <script>${BOOT}</script>\n    <style>${STYLE}</style>\n    <script type="module" crossorigin src="/assets/app-AbCd1234.js"></script>\n    <link rel="stylesheet" crossorigin href="/assets/app-EfGh5678.css">\n  </head>\n  <body><div id="root"></div></body>\n</html>\n`;
}

/** 📦️ A whole CDN artifact baked for `origin`. */
function released(origin: string): Record<string, string | Uint8Array> {
  return { "index.html": sealed(origin), "404.html": sealed(origin), ".nojekyll": "", CNAME: `${deployment.site.host}\n`, "robots.txt": "User-agent: *\nAllow: /\n", "manifest.webmanifest": "{}", "favicon.svg": "<svg xmlns=\"http://www.w3.org/2000/svg\"/>", "assets/app-AbCd1234.js": `const proctor = ${JSON.stringify(origin)};`, "assets/app-EfGh5678.css": STYLE };
}

afterEach(() => {
  for (const root of scratch.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("architecture course deployment", () => {
  it("retains parent cancellation until release and closes its local lifetime like native AbortController", () => {
    expect(COURSE_DEPLOY_CLEANUP_MS).toBe(cancellation.cleanupTimeoutMilliseconds);
    const validate = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["initial", "during", "released"], properties: { initial: { type: "boolean" }, during: { type: "boolean" }, released: { const: true } } });
    for (const row of cancellation.cases) {
      const parent = new AbortController(), oracle = new AbortController(), local = new AbortController();
      const expected = AbortSignal.any([oracle.signal, local.signal]);
      const reason = Error("parent deadline exhausted");
      if (row.phase === "before") { parent.abort(reason); oracle.abort(reason); }
      const before = [process.listenerCount("SIGINT"), process.listenerCount("SIGTERM"), getEventListeners(parent.signal, "abort").length];
      const owned = deploymentInterruption(parent.signal);
      const initial = owned.signal.aborted;
      expect(initial).toBe(expected.aborted);
      if (row.phase === "during") { parent.abort(reason); oracle.abort(reason); }
      const during = owned.signal.aborted;
      expect(during).toBe(expected.aborted);
      if (row.phase !== "release") expect(owned.signal.reason).toBe(reason);
      owned.release();
      local.abort();
      expect(parent.signal.aborted).toBe(row.phase !== "release");
      const result = { initial, during, released: owned.signal.aborted };
      expect(result).toEqual({ initial: row.initial, during: row.during, released: row.released });
      expect(result.released).toBe(expected.aborted);
      expect(validate(result), JSON.stringify(validate.errors)).toBe(true);
      expect([process.listenerCount("SIGINT"), process.listenerCount("SIGTERM"), getEventListeners(parent.signal, "abort").length]).toEqual(before);
    }
  });
  it("builds publication as a release even when the dashboard selected prototype", () => {
    const bundle = join(site, "📦️packages/🟦️typescript");
    const plan = courseReleaseBuildPlan(repoRoot, bundle, { SEMIO_BUILD_MODE: "prototype", PROCTOR_URL: COURSE_PROCTOR_ORIGIN });
    expect({ mode: plan.mode, owner: plan.owner }).toEqual(sitePublication.build);
    expect(plan.environment.SEMIO_BUILD_MODE).toBe(sitePublication.build.mode);
    expect(plan.environment.PROCTOR_URL).toBe(COURSE_PROCTOR_ORIGIN);
    expect(plan.workspace).toBe(repoRoot);
    expect(plan.config).toBe(join(site, "🏗️builder/🌐️vite/🟦️.ts"));
    const project = JSON.parse(readFileSync(join(bundle, "📋️project.json"), "utf8")) as { targets: { build: { cache: boolean; outputs: string[] } } };
    expect(project.targets.build).toMatchObject({ cache: false, outputs: [] });
  });
  for (const row of sitePublication.cases) it(`stages ${row.id} from the current artifact without exporting its internal receipt`, () => {
    const files = { ...released(row.origin), ".semio-artifact.json": JSON.stringify({ version: 1, owner: "course", files: [] }) };
    const source = tree(files);
    const destination = join(tree({ "old.txt": "obsolete" }), "pages");
    mkdirSync(destination);
    writeFileSync(join(destination, "obsolete.txt"), "old");
    const before = Object.fromEntries(Object.keys(files).map(path => [path, createHash("sha256").update(readFileSync(join(source, path))).digest("hex")]));
    stageCourseSiteArtifact(source, destination, row.origin);
    const actual = readdirSync(destination, { recursive: true, withFileTypes: true }).filter(entry => entry.isFile()).map(entry => join(entry.parentPath, entry.name).slice(destination.length + 1).replaceAll("\\", "/")).sort();
    const expected = Object.keys(files).filter(path => !picomatch(sitePublication.internalFiles, { dot: true })(path));
    expect(actual).toEqual([...expected, ...sitePublication.addedFiles].sort());
    for (const path of expected) expect(createHash("sha256").update(readFileSync(join(destination, path))).digest("hex")).toBe(Buffer.from(sha256(readFileSync(join(source, path)))).toString("hex"));
    expect(readFileSync(join(destination, "_headers"), "utf8")).toBe(COURSE_SITE_HEADERS);
    expect(Object.fromEntries(Object.keys(files).map(path => [path, createHash("sha256").update(readFileSync(join(source, path))).digest("hex")]))).toEqual(before);
  });

  it("preserves the immutable source and earlier staging when export is refused or cancelled", () => {
    const source = tree(released(sitePublication.cases[0]!.origin));
    const destination = tree({ "old.txt": "preserved" });
    for (const pages of [source, join(source, "pages"), dirname(source)]) expect(() => stageCourseSiteArtifact(source, pages, COURSE_PROCTOR_ORIGIN)).toThrow("separate");
    expect(() => stageCourseSiteArtifact(source, destination, "https://wrong.example")).toThrow("not publishable");
    const cancellation = new AbortController();
    cancellation.abort(new Error("publication cancelled"));
    expect(() => stageCourseSiteArtifact(source, destination, COURSE_PROCTOR_ORIGIN, cancellation.signal)).toThrow("publication cancelled");
    expect(readFileSync(join(destination, "old.txt"), "utf8")).toBe("preserved");
    expect(siteArtifactProblems(source, COURSE_PROCTOR_ORIGIN)).toEqual([]);
  });

  it("refuses a staging junction or symlink that redirects into the immutable source", () => {
    const source = tree(released(COURSE_PROCTOR_ORIGIN));
    const redirect = join(tree({}), "redirect");
    symlinkSync(source, redirect, process.platform === "win32" ? "junction" : "dir");
    expect(() => stageCourseSiteArtifact(source, join(redirect, "pages"), COURSE_PROCTOR_ORIGIN)).toThrow("links");
  });

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
  it("runs the proctor unexposed, hardened and limited behind a pinned Caddy", () => {
    const { proctor, caddy } = compose.services as { proctor: Service; caddy: Service };
    expect(compose.name).toBe("architecture-course");
    expect(proctor.image).toBe(`${deployment.proctor.image}:\${PROCTOR_TAG:-latest}`);
    expect(proctor.pull_policy).toBe("missing");
    expect(proctor.build).toBeUndefined();
    expect(proctor.expose).toEqual([String(deployment.proctor.port)]);
    expect(proctor.ports).toBeUndefined();
    expect(proctor.environment!.PROCTOR_ALLOWED_ORIGINS).toBe(`\${PROCTOR_ALLOWED_ORIGINS:-${COURSE_SITE_ORIGIN}}`);
    expect(proctor.volumes).toEqual(["proctor-data:/srv/course/data"]);
    expect(proctor.stop_grace_period).toBe("30s");
    expect(proctor.cap_add).toBeUndefined();
    expect(caddy.image).toMatch(/^caddy:\d+\.\d+\.\d+@sha256:[0-9a-f]{64}$/u);
    expect(caddy.environment!.PROCTOR_HOST).toBe(`\${PROCTOR_HOST:-${deployment.proctor.host}}`);
    expect(caddy.depends_on).toEqual({ proctor: { condition: "service_healthy" } });
    expect(caddy.ports).toEqual(["${COURSE_HTTP_PORT:-80}:80", "${COURSE_HTTPS_PORT:-443}:443", "${COURSE_HTTPS_PORT:-443}:443/udp"]);
    expect(caddy.volumes).toEqual(["./Caddyfile:/etc/caddy/Caddyfile:ro", `./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`, "caddy-data:/data", "caddy-config:/config"]);
    expect(caddy.cap_add).toEqual(["NET_BIND_SERVICE"]);
    expect(caddy.healthcheck!.test[0]).toBe("CMD");
    for (const service of [proctor, caddy]) {
      expect(service.restart).toBe("unless-stopped");
      expect(service.env_file).toEqual([{ path: ".env", required: false }]);
      expect(service.read_only).toBe(true);
      expect(service.cap_drop).toEqual(["ALL"]);
      expect(service.security_opt).toEqual(["no-new-privileges:true"]);
      expect(service.logging).toEqual({ driver: "json-file", options: { "max-size": "10m", "max-file": "5" } });
      expect(Number(service.deploy!.resources.limits.cpus)).toBeGreaterThan(0);
      expect(service.deploy!.resources.limits.memory).toMatch(/^\d+[MG]$/u);
      expect(service.deploy!.resources.limits.pids).toBeGreaterThan(0);
    }
    expect(Object.keys(compose.volumes).sort()).toEqual(["caddy-config", "caddy-data", "proctor-data"]);
  });

  it("proxies the proctor host to the proctor service with a body cap, timeouts and response headers", () => {
    const caddyfile = read("Caddyfile");
    const directives = caddyfile.split("\n").filter((line) => !line.trimStart().startsWith("#")).join("\n");
    expect(directives).toContain(`{$PROCTOR_HOST:${deployment.proctor.host}} {`);
    expect(directives).toContain(`reverse_proxy proctor:${deployment.proctor.port}`);
    expect(directives).toContain(`\ttls {\n\t\tload /${COURSE_STACK_CERTIFICATES}\n\t}\n`);
    expect(directives).not.toMatch(/Access-Control/iu);
    for (const directive of ["skip_install_trust", "read_header ", "read_body ", "idle ", "lb_try_duration ", "dial_timeout ", "response_header_timeout ", "encode zstd gzip", 'Strict-Transport-Security "max-age=31536000"', 'X-Content-Type-Options "nosniff"', 'X-Frame-Options "DENY"', "frame-ancestors 'none'", "-Server", "-Via"]) expect(directives).toContain(directive);
    expect(directives).toContain("max_size {$PROCTOR_LIMIT_BODY_BYTES:16384}");
    expect(directives).toContain('\t@oversized `{http.request.header.Content-Length} != "" && int({http.request.header.Content-Length}) > {$PROCTOR_LIMIT_BODY_BYTES:16384}`\n\trespond @oversized 413\n');
    expect(stackRequestBodyLimit(caddyfile)).toBe(16_384);
    expect(stackRequestBodyLimit("example.org {\n\trequest_body {\n\t\tmax_size 8KB\n\t}\n}\n")).toBe(8000);
    expect([caddySizeBytes("512"), caddySizeBytes("8KB"), caddySizeBytes("8KiB"), caddySizeBytes("2MiB"), caddySizeBytes("1mb")]).toEqual([512, 8000, 8192, 2_097_152, 1_000_000]);
    expect(() => caddySizeBytes("eight")).toThrow();
    expect(() => stackRequestBodyLimit("example.org {\n\treverse_proxy proctor:8791\n}\n")).toThrow("caps no request body");
  });

  it("releases from the actual repository default branch only, with pinned actions and the least permissions", () => {
    expect(Object.keys(workflow.on)).toEqual(["workflow_dispatch"]);
    expect(workflow.permissions).toEqual({ contents: "read" });
    expect(workflow.concurrency).toEqual({ group: "architecture-course", "cancel-in-progress": false });
    const { configuration, check, site: siteJob, pages, proctor } = workflow.jobs as { configuration: Job; check: Job; site: Job; pages: Job; proctor: Job };
    expect(Object.keys(workflow.jobs)).toEqual([readiness.release.configurationJob, readiness.release.job, "site", "pages", "proctor"]);
    expect(configuration.if).toBe(readiness.release.condition);
    expect(configuration.outputs).toEqual({runner:`\${{ vars.${readiness.release.runnerVariable} }}`});
    expect(configuration.steps.map(step=>step.run)).toEqual(['test -n "$ARCHITECTURE_COURSE_RUNNER"']);
    expect(check.needs).toBe(readiness.release.configurationJob);
    expect(check.outputs).toEqual({runner:readiness.release.runner});
    expect(check["runs-on"]).toBe(readiness.release.runner);
    expect(check.if).toBe(readiness.release.condition);
    expect(check.steps.filter((step) => step.run).map((step) => step.run)).toEqual(readiness.release.commands);
    expect(check.permissions).toBeUndefined();
    expect(check["continue-on-error"]).not.toBe(true);
    for (const step of check.steps) expect(step["continue-on-error"]).not.toBe(true);
    for (const consumer of readiness.release.consumers) expect(workflow.jobs[consumer]?.needs).toBe(readiness.release.job);
    expect(siteJob.if).toBe(readiness.release.siteCondition);
    expect(proctor.if).toBe(readiness.release.proctorCondition);
    expect(siteJob.permissions).toBeUndefined();
    expect([pages.needs, pages.permissions]).toEqual(["site", { pages: "write", "id-token": "write" }]);
    expect(proctor.permissions).toEqual({ contents: "read", packages: "write" });
    expect(configuration["runs-on"]).toBe("ubuntu-24.04");expect(pages["runs-on"]).toBe("ubuntu-24.04");
    for(const job of[siteJob,proctor])expect(job["runs-on"]).toBe(readiness.release.consumerRunner);
    for (const job of [configuration, check, siteJob, pages, proctor]) {
      expect(job["timeout-minutes"]).toBeGreaterThan(0);
      for (const step of job.steps) if (step.uses) expect(step.uses).toMatch(/^[\w-]+\/[\w-]+@[0-9a-f]{40}$/u);
      for (const step of job.steps) if (step.uses?.startsWith("actions/checkout@")) expect(step.with).toEqual({ "persist-credentials": false });
      for (const step of job.steps) if (step.run?.includes(" bun repo dependencies sync")) expect(step.run).toBe(readiness.release.commands[1]);
    }
    expect(siteJob.steps.filter((step) => step.run).map((step) => step.run)).toEqual(readiness.release.siteCommands);
    expect(siteJob.steps.at(-1)!.with).toEqual({ path: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/release/${COURSE_PAGES_DIRECTORY}` });
    const release = proctor.steps.filter(step => step.run || step.uses?.startsWith("docker/login-action@")).map(step => step.run ?? step.uses!.split("@")[0]);
    expect(release).toEqual(readiness.release.proctorSteps);
  });

  it("accepts exactly a document sealed for the baked proctor origin", () => {
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN), COURSE_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteDocumentProblems(sealed("https://localhost:18443"), "https://localhost:18443")).toEqual([]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN), "https://staging.example")).toEqual([`connect-src admits ${COURSE_PROCTOR_ORIGIN} wss://${deployment.proctor.host}, not exactly https://staging.example wss://staging.example`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(/\s*<meta http-equiv[^>]*>/u, ""), COURSE_PROCTOR_ORIGIN)).toEqual(["the document declares no Content-Security-Policy"]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(BOOT, `${BOOT};fetch('/')`), COURSE_PROCTOR_ORIGIN)).toEqual([`script-src does not admit the inline script ${digest(`${BOOT};fetch('/')`)}`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(STYLE, "body{margin:1px}"), COURSE_PROCTOR_ORIGIN)).toEqual([`style-src does not admit the inline style ${digest("body{margin:1px}")}`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace("<html>", "<html lang=\"en\">").replace(/\s*<meta name="description"[^>]*>/u, ""), COURSE_PROCTOR_ORIGIN)).toEqual(["the document has no description", "the document declares a default language"]);
    const moved = sealed(COURSE_PROCTOR_ORIGIN);
    const meta = /\s*<meta http-equiv[^>]*>/u.exec(moved)![0];
    expect(siteDocumentProblems(moved.replace(meta, "").replace("<style>", `${meta.trim()}<style>`), COURSE_PROCTOR_ORIGIN)).toEqual(["the Content-Security-Policy comes after the first script"]);
    const loose = `default-src *; script-src 'self' 'unsafe-inline' 'unsafe-eval' https:; style-src 'self' 'unsafe-inline'; connect-src *; object-src 'self'`;
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN, loose), COURSE_PROCTOR_ORIGIN)).toEqual(["default-src admits *", "script-src admits 'unsafe-inline'", "script-src admits 'unsafe-eval'", "script-src admits https:", "style-src admits 'unsafe-inline'", `script-src does not admit the inline script ${digest(BOOT)}`, `style-src does not admit the inline style ${digest(STYLE)}`, `connect-src admits *, not exactly ${COURSE_PROCTOR_ORIGIN} wss://${deployment.proctor.host}`, "object-src is not 'none'", "base-uri is not 'none'", "form-action is not restricted"]);
  });

  it("publishes exactly a CDN artifact that bakes the proctor origin", () => {
    expect(siteArtifactProblems(tree(released(COURSE_PROCTOR_ORIGIN)), COURSE_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteArtifactProblems(tree(released("https://localhost:18443")), "https://localhost:18443")).toEqual([]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), CNAME: "quizze.example\n" }), COURSE_PROCTOR_ORIGIN)).toEqual([`CNAME names "quizze.example", not ${deployment.site.host}`]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/dev-AbCd1234.js": "fetch('http://127.0.0.1:8791')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["the site names 127.0.0.1 outside the baked proctor origin"]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "404.html": "<!doctype html>" }), COURSE_PROCTOR_ORIGIN)).toEqual(["404.html is not the document of index.html"]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js.map": "{}", "assets/chunk.js": "//# sourceMappingURL=chunk.js.map\nconsole.log('[DEBUG] x')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["assets/app-AbCd1234.js.map carries no content hash, yet /assets/* is served as immutable", "assets/chunk.js carries no content hash, yet /assets/* is served as immutable", "assets/app-AbCd1234.js.map is a source map", "the site carries the development leftover sourceMappingURL=", "the site carries the development leftover [DEBUG]"]);
    const { "assets/app-EfGh5678.css": _stylesheet, "robots.txt": _robots, ...partial } = released(COURSE_PROCTOR_ORIGIN);
    expect(siteArtifactProblems(tree(partial), COURSE_PROCTOR_ORIGIN)).toEqual(["robots.txt is missing", "the document loads assets/app-EfGh5678.css, which the artifact lacks"]);
    expect(siteArtifactProblems(tree({ "index.html": sealed(COURSE_PROCTOR_ORIGIN), CNAME: `${deployment.site.host}\n` }), COURSE_PROCTOR_ORIGIN)).toEqual(["404.html is missing", ".nojekyll is missing", "robots.txt is missing", "manifest.webmanifest is missing", "favicon.svg is missing", "the document loads assets/app-AbCd1234.js, which the artifact lacks", "the document loads assets/app-EfGh5678.css, which the artifact lacks", `no script bakes the proctor origin ${COURSE_PROCTOR_ORIGIN}`]);
    const heavy = `${released(COURSE_PROCTOR_ORIGIN)["assets/app-AbCd1234.js"] as string}\n/* ${randomBytes(COURSE_SITE_BUDGET.scriptGzipBytes * 2).toString("base64")} */`;
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js": heavy }), COURSE_PROCTOR_ORIGIN)).toEqual([expect.stringMatching(/^the document's scripts weigh \d+ bytes compressed, over the budget of \d+$/u)]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "🖼️assets/font.woff2": randomBytes(COURSE_SITE_BUDGET.totalBytes) }), COURSE_PROCTOR_ORIGIN)).toEqual([expect.stringMatching(/^the artifact weighs \d+ bytes, over the budget of \d+$/u)]);
  });

  it("bundles the proctor stack for a host without a registry: the tag pinned, the defaults of compose.yaml listed, every step told", () => {
    const defaults = Object.fromEntries([...read("compose.yaml").matchAll(/\$\{([A-Z_]+):-([^}]*)\}/gu)].map((substitution) => [substitution[1]!, substitution[2]!]));
    expect(Object.keys(defaults).sort()).toEqual(["COURSE_HTTPS_PORT", "COURSE_HTTP_PORT", "PROCTOR_ALLOWED_ORIGINS", "PROCTOR_HOST", "PROCTOR_TAG"]);
    const settings = (tag: string): string[] => stackBundleEnvironment(tag).split("\n").filter((line) => line.includes("="));
    expect(settings(defaults.PROCTOR_TAG!).map((line) => line.replace(/^# /u, "")).sort()).toEqual(Object.entries(defaults).map(([name, value]) => `${name}=${value}`).sort());
    expect(settings("sha-0123456789ab").filter((line) => !line.startsWith("#"))).toEqual(["PROCTOR_TAG=sha-0123456789ab"]);
    expect(stackBundleEnvironment("latest")).toBe(`# Overrides of compose.yaml; both services read this file as their environment.\nPROCTOR_TAG=latest\n# PROCTOR_HOST=${deployment.proctor.host}\n# PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}\n# COURSE_HTTP_PORT=80\n# COURSE_HTTPS_PORT=443\n`);
    const readme = stackBundleReadme("sha-0123456789ab", compose.services.caddy!.image, bundleCases.proxy.image);
    for (const row of bundleCases.cases) {
      const images = stackBundleImages(row.tag, read("compose.yaml"));
      expect(images).toEqual(row.images);
      expect(images).toEqual(Object.values(compose.services).map(service => service.image.replace("${PROCTOR_TAG:-latest}", row.tag)));
      const plan = stackBundlePlan(row.tag, read("compose.yaml"));
      expect(plan.images).toEqual([...row.images, bundleCases.proxy.image]);
      expect(plan.caddy).toBe(bundleCases.proxy.image);
      const staged = parse(plan.compose) as { services: Record<string, Service> };
      expect(staged.services.caddy).toEqual({ ...compose.services.caddy, ...bundleCases.proxy });
      expect(staged.services.proctor).toEqual(compose.services.proctor);
      for (const image of plan.images) expect(stackBundleReadme(row.tag, images[1], plan.caddy)).toContain(image);
    }
    for (const row of bundleCases.invalid) {
      const changed = "replace" in row ? read("compose.yaml").replace(row.replace!, row.with!) : read("compose.yaml") + row.append;
      expect(() => stackBundleImages("latest", changed), row.id).toThrow(row.error);
    }
    const steps = ["1. Copy this whole directory to the host", `2. docker load --input ${COURSE_BUNDLE_IMAGE}`, "3. The certificate, one of:", "4. docker compose up --detach --wait", `5. curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`];
    expect(readme.match(/^\d+\. /gmu)).toEqual(steps.map((step) => step.slice(0, 3)));
    const told = steps.map((step) => readme.indexOf(`\n${step}`));
    expect(told.every((at, index) => at > (told[index - 1] ?? 0)), JSON.stringify(told)).toBe(true);
    for (const fact of [COURSE_SITE_ORIGIN, "linux/amd64", "Docker Engine 25 or newer", "compose plugin 2.24 or newer", `${deployment.proctor.image}:sha-0123456789ab`, "pins the image tag sha-0123456789ab", "80/tcp, 443/tcp and 443/udp", `${deployment.proctor.host} are open from the internet`, `${COURSE_STACK_CERTIFICATES}/<any name>.pem`, "full chain first", "key after it", "owned by root with mode 0600", "docker compose up --detach --force-recreate caddy", "docker compose exec -T proctor proctor backup - > proctor.sqlite", "docker compose down\n", "NEVER docker compose down --volumes", "docker compose logs --since 1h proctor caddy", ...COURSE_STACK_FILES, ".env", `${COURSE_STACK_CERTIFICATES}/`, COURSE_BUNDLE_IMAGE]) expect(readme, fact).toContain(fact);
    const named = new Set(readme.match(new RegExp(`(?:[a-z0-9_-]+\\.)*(?:${[deployment.site.host, deployment.proctor.host].map(hostZone).join("|").replace(/\./gu, "\\.")})`, "gu")));
    expect([...named].sort()).toEqual([deployment.site.host, deployment.proctor.host].sort());
    expect(readme.split("\n").every((line) => line.length <= 120)).toBe(true);
    expect(compose.services.caddy!.volumes).toContain(`./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`);
    expect(compose.services.proctor!.pull_policy).toBe("missing");
    expect(compose.services.caddy!.pull_policy).toBe("missing");
    const origin = COURSE_PROCTOR_ORIGIN;
    expect(siteArtifactProblems(tree({ ...released(origin), [`${COURSE_BUNDLE_DIRECTORY}/probe.js`]: "fetch('http://localhost')", [`${COURSE_PAGES_DIRECTORY}/index.html`]: "<!doctype html>" }), origin)).toEqual([]);
    expect(siteArtifactProblems(tree({ ...released(origin), "elsewhere/probe.js": "fetch('http://localhost')" }), origin)).toEqual(["the site names localhost outside the baked proctor origin"]);
  });

  it("checks the sources against the compiler early in the readiness gate, before anything is built, and ends with the end-to-end gate", () => {
    const project = JSON.parse(readFileSync(join(site, "📦️packages/🟦️typescript/📋️project.json"), "utf8")) as { targets: Record<string, { options?: { command?: string }; dependsOn?: string[] }> };
    expect(project.targets.typecheck?.options?.command).toBe("bun ./📜️script.ts typecheck");
    expect(project.targets["deploy-check"]?.dependsOn).toContain(readiness.sourceProducer);
    expect(project.targets.typecheck?.dependsOn).toEqual(["@semio-tech/framework-rs:generate", "@semio-tech/ui-contract-rs:generate", "@semio-tech/ui-rs:generate", "@semio-tech/framework-actor-rs:typegen", "@semio-tech/ui-styling-tokens:generate", "@semio-tech/assets:build"]);
    const steps = deployCheckSteps(project.targets);
    expect(steps).toEqual(["no drift from 🚀️deploy/🔣️.json", "sources type-checked", "course valid in the Rust core", "site built and verified as the CDN artifact", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"]);
    expect(deployCheckSteps({})).toEqual(steps.filter((step) => step !== "sources type-checked" && step !== "end-to-end gate"));
    const readme = readFileSync(join(site, "README.md"), "utf8");
    const gate = readme.slice(readme.indexOf("### Readiness gate"));
    const told = gate.slice(0, gate.indexOf("\n#", 1)).match(/^\d+\. /gmu) ?? [];
    expect(told).toEqual(steps.map((_, index) => `${index + 1}. `));
    const include = (JSON.parse(readFileSync(join(site, "📦️packages/🟦️typescript/tsconfig.json"), "utf8")) as { include: string[] }).include;
    for (const covered of ["./📜️script.ts", "../../🟦️.ts", "../../🏗️builder/**/*.ts", "../../🧱️stack/**/*.ts", "../../🚀️deploy/**/*.ts", "../../🎭️e2e/**/*.ts", "../../🧪️tests/**/*.ts"]) expect(include).toContain(covered);
  });

  it("hardens every response and caches hashed assets, without two rules setting one header for a path", () => {
    const rules = COURSE_SITE_HEADERS.trimEnd().split("\n").filter((line) => !line.startsWith(" "));
    expect(rules).toEqual(["/*", "/assets/*", "/", "/index.html", "/404.html"]);
    expect(COURSE_SITE_HEADERS).toContain("/assets/*\n  Cache-Control: public, max-age=31536000, immutable\n");
    expect(COURSE_SITE_HEADERS.match(/Cache-Control: no-cache/gu)).toHaveLength(3);
    const everywhere = COURSE_SITE_HEADERS.slice(0, COURSE_SITE_HEADERS.indexOf("/assets/*"));
    expect(everywhere).not.toContain("Cache-Control");
    for (const header of ["X-Content-Type-Options: nosniff", "X-Frame-Options: DENY", "Content-Security-Policy: frame-ancestors 'none'", "Referrer-Policy: no-referrer"]) expect(everywhere).toContain(`  ${header}\n`);
  });
});
