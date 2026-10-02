/** 🚀️ Operator verbs of the architecture quiz: the site goes to a CDN at the site host, the proctor runs as a zero-touch
 * Docker stack (proctor + Caddy) at the proctor host, both hosts from `🔣️.json`.
 *
 * - `publish` builds the site with the proctor origin baked in, verifies the CDN artifact (entry points, sealed document,
 *   hashed assets, no development leftovers, size budget) and stages it with `_headers`.
 * - `docker-image-build` builds the proctor image with its version, revision and commit time as OCI labels;
 *   `docker-image-publish` checks that image and the stack, then pushes it as `sha-<revision>` (immutable), the workspace
 *   version and `latest` with the developer's own `docker login`, and prints the digest.
 * - `docker-image-check` runs the image as the stack does (no configuration, read-only root, no capabilities) and checks
 *   what it is (labels, user, no shell, the stack files it carries) and the API as the CDN-hosted site meets it: ready
 *   behind the proxy, cleartext refused, cross-origin preflight and POST granted to the site origin only, the request
 *   body limit, shared presence between two learners (a foreign origin refused), `docker stop` drains with exit code 0
 *   while a socket is open, the WAL folded into `proctor.sqlite` on the volume.
 * - `docker-stack-check` proves the stack exactly as a host gets it — `compose.yaml` and the `Caddyfile` copied out of the
 *   image into an empty directory: config, `caddy validate`, `up --wait` on a throw-away project with Caddy's internal
 *   CA on `localhost`, the hardening Docker applied, the API and the presence WebSocket over HTTPS through Caddy, a
 *   certificate the operator supplies served instead of Caddy's own, a backup while serving, an update without a failed
 *   request and with the data kept, a restore, an erasure on request, then `down --volumes`.
 * - `docker-stack-bundle` stages what the proctor host needs without a registry: the stack files out of the image, an
 *   empty `certificates` directory, an `.env` that pins the tag, the image as `proctor-image.tar` and the host's steps.
 * - `deploy-check` is the readiness gate: drift between `🔣️.json` and every file that repeats it, the catalog, `publish`,
 *   the image build, both checks and the end-to-end gate when the package has one.
 *
 * Every Docker call is one argv without a shell, so the verbs run unchanged on Windows, macOS and Linux; Ctrl-C/SIGTERM
 * stops the running client and removes what a check created.
 * @see ./🔣️.json — the site and proctor hosts, the image repository and port
 * @see ./🧬️schema/🔣️.json — the contract of that file
 * @see ./Dockerfile — the proctor image
 * @see ./compose.yaml — the stack
 * @see ./Caddyfile — the TLS-terminating proxy
 * @see ../README.md — the runbook
 * https://docs.docker.com/reference/cli/docker/ */
import { spawn, spawnSync } from "node:child_process";
import { X509Certificate, createHash, randomBytes } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { connect } from "node:tls";
import { gzipSync } from "node:zlib";
import { cargoTargetDirectory } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { workspaceCargoVersion } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { runRepositoryCommand as runOwnedCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { PRESENCE_PROTOCOL, decodeCommandOutcome, decodePresenceFrame, encodeCommandEnvelope, encodePresenceFrame, encodeQueryEnvelope, presenceSocketUrl, type PresenceFrame } from "../../../../🧰️framework/🛍️products/🖥️server/🟦️.ts";
import type { IdentityClaim } from "../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🟦️.ts";
import { TEACHING_WORKSPACE } from "../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import catalog from "../🔣️.json" with { type: "json" };
import deployment from "./🔣️.json" with { type: "json" };

//#region 🔣️Constants
/** 🌐️ The host the CDN serves the site at. */
export const QUIZ_SITE_HOST = deployment.site.host;

/** 🪪️ The site origin: the one browser origin the proctor grants. */
export const QUIZ_SITE_ORIGIN = `https://${QUIZ_SITE_HOST}`;

/** 🛂️ The host the Docker stack serves the proctor at. */
export const QUIZ_PROCTOR_HOST = deployment.proctor.host;

/** 🔗️ The proctor origin a release build of the site talks to. */
export const QUIZ_PROCTOR_ORIGIN = `https://${QUIZ_PROCTOR_HOST}`;

/** 🏷️ The registry repository every proctor image tag lives under. */
export const QUIZ_IMAGE_REPOSITORY = deployment.proctor.image;

/** 🔌️ The port the proctor binds inside the container (`PROCTOR_PORT`). */
export const QUIZ_IMAGE_PORT = deployment.proctor.port;

/** 🗄️ The one writable path of the image (`PROCTOR_DATA`). */
export const QUIZ_IMAGE_DATA_ROOT = "/srv/quiz/data";

/** 🗃️ The SQLite file the proctor keeps in `PROCTOR_DATA`. */
export const QUIZ_IMAGE_DATABASE = "proctor.sqlite";

/** 👤️ The unprivileged account the image runs as (`uid:gid`). */
export const QUIZ_IMAGE_USER = "65532:65532";

/** 💓️ The image's `HEALTHCHECK`: the proctor's own readiness probe. */
export const QUIZ_IMAGE_HEALTHCHECK = ["CMD", "/usr/local/bin/proctor", "health"] as const;

/** 🎒️ Where the image carries the stack files of its release. */
export const QUIZ_IMAGE_STACK_ROOT = "/srv/quiz/deploy";

/** 👣️ The whole footprint of a host besides Docker: these two files in one directory. */
export const QUIZ_STACK_FILES = ["compose.yaml", "Caddyfile"] as const;

/** 🔏️ The directory beside the stack files Caddy loads the operator's own certificates from (`*.pem` bundles). */
export const QUIZ_STACK_CERTIFICATES = "certificates";

/** 📦️ Where `publish` stages the CDN artifact, relative to the package's `dist`. */
export const QUIZ_PAGES_DIRECTORY = "pages/quizzes";

/** 🎁️ Where `docker-stack-bundle` stages the directory for the proctor host, relative to the package's `dist`. */
export const QUIZ_BUNDLE_DIRECTORY = "proctor";

/** 💾️ The file the bundle carries the proctor image in. */
export const QUIZ_BUNDLE_IMAGE = "proctor-image.tar";

/** 🏋️ What a release of the site may weigh: the entry script and stylesheet compressed as a CDN sends them, and the whole
 * artifact on disk. */
export const QUIZ_SITE_BUDGET = { scriptGzipBytes: 260_000, styleGzipBytes: 60_000, totalBytes: 4_000_000 } as const;

/** 📑️ `_headers` for CDNs that honour it (Cloudflare Pages, Netlify): every response hardened (the frame ban a `meta`
 * policy cannot carry included), hashed assets immutable, documents revalidated. No two rules set the same header for
 * one path, because those CDNs join the values of every matching rule. GitHub Pages ignores the file. */
export const QUIZ_SITE_HEADERS =
  [
    "/*",
    "  X-Content-Type-Options: nosniff",
    "  X-Frame-Options: DENY",
    "  Content-Security-Policy: frame-ancestors 'none'",
    "  Referrer-Policy: no-referrer",
    "/assets/*",
    "  Cache-Control: public, max-age=31536000, immutable",
    ...["/", "/index.html", "/404.html"].flatMap((path) => [path, "  Cache-Control: no-cache"]),
  ].join("\n") + "\n";

const SITE = "🎓️teaching/🏛️architecture/❓️quiz";
const DEPLOY = `${SITE}/🚀️deploy`;
const DOCKERFILE = `${DEPLOY}/Dockerfile`;
const BUNDLE = `${SITE}/📦️packages/🟦️typescript`;
const CATALOG = `${SITE}/🔣️.json`;
const PROCTOR_README = "🎓️teaching/🛂️proctor/README.md";
const WORKFLOW = ".github/workflows/architecture-quiz.yml";
const PROJECT = "@teaching/architecture-quiz";
const FOREIGN_ORIGIN = "https://foreign.example";
//#endregion 🔣️Constants

//#region 🧰️Tools
/** 🏳️ The value following `flag` in `segments`, if any. */
function flag(segments: readonly string[], name: string): string | undefined {
  const index = segments.indexOf(name);
  return index >= 0 ? segments[index + 1] : undefined;
}

/** 🛑️ An abort signal for Ctrl-C/SIGTERM and its release. */
function interruption(): { readonly signal: AbortSignal; readonly release: () => void } {
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  return { signal: controller.signal, release: () => { process.off("SIGINT", cancel); process.off("SIGTERM", cancel); } };
}

/** 🗣️ One progress line of these verbs. */
function say(line: string): void {
  console.log(`[deploy] ${line}`);
}

/** 💤️ Resolves after `ms`, or fails at once when `signal` is aborted. */
function pause(ms: number, signal: AbortSignal): Promise<void> {
  if (signal.aborted) return Promise.reject(new Error("cancelled"));
  return new Promise((accept) => setTimeout(accept, ms));
}

/** 🧾️ What one Docker client call answered: its exit code, both streams as text and stdout as the bytes it wrote. */
type DockerAnswer = Readonly<{ code: number; stdout: string; stderr: string; bytes: Buffer }>;

/** 🐳️ Runs one `docker` argv without a shell, with `env` merged over the process environment (a variable set to
 * `undefined` is removed) and `input` on stdin; an aborted `signal` interrupts the client. */
function docker(args: readonly string[], options: Readonly<{ signal?: AbortSignal; env?: NodeJS.ProcessEnv; input?: string | Uint8Array; cwd?: string }> = {}): Promise<DockerAnswer> {
  return new Promise((accept, reject) => {
    if (options.signal?.aborted) return reject(new Error("cancelled"));
    const child = spawn("docker", [...args], { shell: false, cwd: options.cwd, env: Object.fromEntries(Object.entries({ ...process.env, ...options.env }).filter(([, value]) => value !== undefined)), stdio: [options.input === undefined ? "ignore" : "pipe", "pipe", "pipe"], windowsHide: true });
    const written: Buffer[] = [];
    let stderr = "";
    child.stdout!.on("data", (chunk: Buffer) => written.push(chunk));
    child.stderr!.on("data", (chunk: Buffer) => (stderr += chunk.toString("utf8")));
    const abort = (): void => void child.kill("SIGINT");
    options.signal?.addEventListener("abort", abort, { once: true });
    child.once("error", (error) => reject(/ENOENT/u.test(String(error)) ? new Error("the docker client does not exist on PATH; install Docker Desktop or Docker Engine") : error));
    child.once("close", (code) => {
      options.signal?.removeEventListener("abort", abort);
      if (options.signal?.aborted) return reject(new Error("cancelled"));
      const bytes = Buffer.concat(written);
      accept({ code: code ?? -1, stdout: bytes.toString("utf8").trim(), stderr: stderr.trim(), bytes });
    });
    if (options.input !== undefined) child.stdin!.end(options.input);
  });
}

/** 🚢️ Runs one `docker` argv and fails with its output unless it exits 0. */
async function dockerOk(args: readonly string[], options: Parameters<typeof docker>[1] = {}): Promise<string> {
  const answer = await docker(args, options);
  if (answer.code !== 0) throw new Error(`docker ${args.slice(0, 3).join(" ")} exited ${answer.code}: ${(answer.stderr || answer.stdout).split("\n").slice(-6).join(" | ").slice(0, 900)}`);
  return answer.stdout;
}

/** 📜️ The last `lines` lines a container wrote to stdout and stderr. */
async function containerLog(container: string, lines: number): Promise<string> {
  const answer = await docker(["logs", "--tail", String(lines), container]).catch(() => ({ stdout: "", stderr: "" }));
  return [answer.stdout, answer.stderr].filter(Boolean).join("\n");
}

/** 🐋️ The daemon's server version, or an error naming the missing daemon — a precondition, never a quiz fault. */
async function dockerServerVersion(signal?: AbortSignal): Promise<string> {
  const answer = await docker(["version", "--format", "{{.Server.Version}}"], { signal });
  if (answer.code !== 0 || !answer.stdout) throw new Error(`the Docker daemon does not answer: ${(answer.stderr || answer.stdout).slice(0, 300)}`);
  return answer.stdout;
}

/** 🔖️ The commit a build of this checkout comes from: its hash (suffixed `-dirty` when tracked files differ from it) and
 * its time; `unknown` outside a git checkout. */
export function sourceRevision(repoRoot: string): Readonly<{ revision: string; created: string }> {
  const git = (...args: string[]): string | undefined => {
    const run = spawnSync("git", args, { cwd: repoRoot, encoding: "utf8", windowsHide: true });
    return run.status === 0 ? run.stdout.trim() : undefined;
  };
  const head = git("rev-parse", "HEAD");
  if (!head) return { revision: "unknown", created: "1970-01-01T00:00:00Z" };
  return { revision: git("status", "--porcelain", "--untracked-files=no") ? `${head}-dirty` : head, created: git("log", "-1", "--format=%cI") ?? "1970-01-01T00:00:00Z" };
}

/** ✅️ `proctor check <catalog>` over the site's catalog with the release binary the image ships: the Rust core validates the
 * catalog and every quiz it names and exits non-zero with the issues. The release profile and a private target directory keep
 * the check off the debug executable a running dev proctor holds open (Windows refuses to replace a running image, and
 * uplifted binaries are hard links into the shared build directory). */
export async function checkQuizCatalog(repoRoot: string): Promise<void> {
  const env = { ...process.env, CARGO_TARGET_DIR: `${cargoTargetDirectory(repoRoot)}-architecture-quiz` };
  await runOwnedCommand("cargo", ["run", "--release", "--quiet", "--package", "teaching-proctor", "--bin", "proctor", "--", "check", resolve(repoRoot, CATALOG)], join(repoRoot, TEACHING_WORKSPACE), "architecture-quiz:check", undefined, { env });
}
//#endregion 🧰️Tools

//#region 📐️Drift
/** 🧮️ A Caddy size (`8KB`, `2MiB`, `512`) in bytes. */
export function caddySizeBytes(size: string): number {
  const parsed = /^(\d+)\s*(B|KB|MB|GB|KiB|MiB|GiB)?$/iu.exec(size.trim());
  if (!parsed) throw new Error(`${JSON.stringify(size)} is not a size`);
  const factors: Record<string, number> = { b: 1, kb: 1e3, mb: 1e6, gb: 1e9, kib: 1024, mib: 1024 ** 2, gib: 1024 ** 3 };
  return Number(parsed[1]) * factors[(parsed[2] ?? "B").toLowerCase()]!;
}

/** 📨️ The request body Caddy lets through to the proctor by default, from the `Caddyfile`'s `request_body`: its size, or
 * the default of the environment placeholder that carries it (the `@oversized` matcher beside it compares a declared
 * length with the same placeholder). */
export function stackRequestBodyLimit(caddyfile: string): number {
  const limit = /request_body\s*\{[^}]*\bmax_size\s+(?:\{\$\w+:([^}\s]+)\}|(\S+))/u.exec(caddyfile);
  if (!limit) throw new Error("the Caddyfile caps no request body");
  return caddySizeBytes(limit[1] ?? limit[2]!);
}

/** 🗺️ The zone `host` lives in: the host without its first label. */
export function hostZone(host: string): string {
  return host.split(".").slice(1).join(".");
}

/** 📐️ Every place where a file of the deployment disagrees with its one authored source, `🔣️.json`: a host in the zone
 * of the site host or of the proctor host, or an image repository, other than the declared ones in the Dockerfile, the
 * stack, the workflow, the READMEs or the package; the baked port, origin, account and health probe of the image; the
 * certificates directory the stack mounts and loads; two defaults for the one request body limit of the `Caddyfile`;
 * unpinned base images or workflow actions; a workflow, README or
 * root script naming a target the package does not have; a Pages artifact path other than the staged one. Empty when
 * nothing drifted. */
export function deploymentDrift(repoRoot: string): string[] {
  const problems: string[] = [];
  const read = (path: string): string => readFileSync(join(repoRoot, path), "utf8");
  const [dockerfile, compose, caddyfile, workflow] = [read(DOCKERFILE), read(`${DEPLOY}/compose.yaml`), read(`${DEPLOY}/Caddyfile`), read(WORKFLOW)];
  const texts: Record<string, string> = { [DOCKERFILE]: dockerfile, [`${DEPLOY}/compose.yaml`]: compose, [`${DEPLOY}/Caddyfile`]: caddyfile, [WORKFLOW]: workflow, [`${SITE}/README.md`]: read(`${SITE}/README.md`), [PROCTOR_README]: read(PROCTOR_README), [`${BUNDLE}/package.json`]: read(`${BUNDLE}/package.json`) };
  const zones = [...new Set([QUIZ_SITE_HOST, QUIZ_PROCTOR_HOST].map(hostZone))].map((zone) => zone.replace(/\./gu, "\\.")).join("|");
  for (const [path, text] of Object.entries(texts)) {
    for (const host of new Set(text.match(new RegExp(`(?:[a-z0-9_][a-z0-9_-]*\\.)+(?:${zones})`, "gu")) ?? [])) if (host !== QUIZ_SITE_HOST && host !== QUIZ_PROCTOR_HOST && !host.startsWith("_github-pages-challenge-")) problems.push(`${path} names the host ${host}, which 🔣️.json does not declare`);
    for (const image of new Set(text.match(/ghcr\.io\/[a-z0-9._/-]+/gu) ?? [])) if (image !== QUIZ_IMAGE_REPOSITORY) problems.push(`${path} names the image ${image}, not ${QUIZ_IMAGE_REPOSITORY}`);
  }
  const expect = (path: string, text: string, fragments: readonly string[]): void => {
    for (const fragment of fragments) if (!text.includes(fragment)) problems.push(`${path} lacks ${JSON.stringify(fragment)}`);
  };
  expect(DOCKERFILE, dockerfile, [`PROCTOR_PORT=${QUIZ_IMAGE_PORT}`, `EXPOSE ${QUIZ_IMAGE_PORT}`, `PROCTOR_ALLOWED_ORIGINS=${QUIZ_SITE_ORIGIN}`, `PROCTOR_DATA=${QUIZ_IMAGE_DATA_ROOT}`, `VOLUME ["${QUIZ_IMAGE_DATA_ROOT}"]`, `USER ${QUIZ_IMAGE_USER}`, `CMD [${QUIZ_IMAGE_HEALTHCHECK.slice(1).map((word) => JSON.stringify(word)).join(", ")}]`, `/stage/deploy ${QUIZ_IMAGE_STACK_ROOT}`, ...QUIZ_STACK_FILES.map((file) => `${DEPLOY}/${file}`)]);
  expect(`${DEPLOY}/compose.yaml`, compose, [`image: ${QUIZ_IMAGE_REPOSITORY}:\${PROCTOR_TAG:-latest}`, `- "${QUIZ_IMAGE_PORT}"`, `\${PROCTOR_ALLOWED_ORIGINS:-${QUIZ_SITE_ORIGIN}}`, `\${PROCTOR_HOST:-${QUIZ_PROCTOR_HOST}}`, `${QUIZ_IMAGE_DATA_ROOT}`, `- ./${QUIZ_STACK_CERTIFICATES}:/${QUIZ_STACK_CERTIFICATES}:ro`]);
  expect(`${DEPLOY}/Caddyfile`, caddyfile, [`{$PROCTOR_HOST:${QUIZ_PROCTOR_HOST}} {`, `reverse_proxy proctor:${QUIZ_IMAGE_PORT}`, `load /${QUIZ_STACK_CERTIFICATES}`, "respond @oversized 413"]);
  const limits = new Set([...caddyfile.matchAll(/\{\$PROCTOR_LIMIT_BODY_BYTES:([^}\s]+)\}/gu)].map((placeholder) => placeholder[1]!));
  if (limits.size > 1) problems.push(`${DEPLOY}/Caddyfile defaults PROCTOR_LIMIT_BODY_BYTES to ${[...limits].join(" and ")}`);
  expect(WORKFLOW, workflow, [`path: ${BUNDLE}/dist/${QUIZ_PAGES_DIRECTORY}`]);
  for (const image of [...dockerfile.matchAll(/^ARG \w+_IMAGE=(\S+)$/gmu), ...compose.matchAll(/^\s+image:\s*(\S+)$/gmu)].map((line) => line[1]!)) if (!image.startsWith(QUIZ_IMAGE_REPOSITORY) && !/^[\w./-]+:[\w.-]+@sha256:[0-9a-f]{64}$/u.test(image)) problems.push(`the base image ${image} is not pinned to an exact tag and digest`);
  for (const action of new Set([...workflow.matchAll(/^\s*(?:- )?uses:\s*(\S+)/gmu)].map((line) => line[1]!))) if (!/@[0-9a-f]{40}$/u.test(action)) problems.push(`${WORKFLOW} uses ${action} without a commit hash`);
  const project = JSON.parse(read(`${BUNDLE}/📋️project.json`)) as { targets: Record<string, unknown> };
  for (const [path, text] of Object.entries({ ...texts, "package.json": read("package.json") })) for (const target of new Set([...text.matchAll(new RegExp(`${PROJECT}:([a-z0-9-]+)`, "gu"))].map((mention) => mention[1]!))) if (!(target in project.targets)) problems.push(`${path} names ${PROJECT}:${target}, which is not a target of the package`);
  try {
    stackRequestBodyLimit(caddyfile);
  } catch (error) {
    problems.push(`${DEPLOY}/Caddyfile: ${(error as Error).message}`);
  }
  return problems;
}

/** ⚠️ What an owner still has to decide before the site is public, none of which stops a deployment: the legal pages
 * the site links (`site.legal` of `🔣️.json`). */
export function deploymentWarnings(): string[] {
  const legal: Readonly<{ imprint?: string; privacy?: string }> = (deployment.site as { legal?: { imprint?: string; privacy?: string } }).legal ?? {};
  return (["imprint", "privacy"] as const).filter((page) => !legal[page]).map((page) => `site.legal.${page} is not set in ${DEPLOY}/🔣️.json: the public site links no ${page === "imprint" ? "imprint" : "privacy notice"}`);
}
//#endregion 📐️Drift

//#region 🌐️Site
/** 🧭️ The proctor origin a release build bakes in: `PROCTOR_URL`, else the proctor origin of `🔣️.json`. */
export function releaseProctorOrigin(env: NodeJS.ProcessEnv = process.env): string {
  return env.PROCTOR_URL ?? QUIZ_PROCTOR_ORIGIN;
}

/** 📦️ The directories of the package's `dist` that hold what a verb staged (`publish`, `docker-stack-bundle`), never the
 * build itself. */
const STAGED_DIRECTORIES: ReadonlySet<string> = new Set([QUIZ_PAGES_DIRECTORY.split("/")[0]!, QUIZ_BUNDLE_DIRECTORY]);

/** 📂️ Every file below `root`, as `/`-separated relative paths, skipping the directories named in `skip`. */
function siteFiles(root: string, skip: ReadonlySet<string> = new Set(), prefix = ""): string[] {
  return readdirSync(join(root, prefix), { withFileTypes: true }).flatMap((entry) => {
    const path = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) return skip.has(path) ? [] : siteFiles(root, skip, path);
    return entry.isFile() ? [path] : [];
  });
}

/** 🛡️ Every reason the release document `html` is not sealed for `proctorOrigin`: no Content-Security-Policy ahead of
 * the first script, a script policy that admits anything inline or evaluated beyond the document's own boot scripts (each
 * must be listed by its SHA-256), connections admitted anywhere but the proctor (HTTPS and its WebSocket twin), plugins or
 * a `base` allowed, no description, or a default language (the app sets `lang` from the learner's locale). */
export function siteDocumentProblems(html: string, proctorOrigin: string): string[] {
  const problems: string[] = [];
  const meta = /<meta http-equiv="Content-Security-Policy" content="([^"]*)"/u.exec(html);
  if (!meta) problems.push("the document declares no Content-Security-Policy");
  else {
    const directives = new Map(meta[1]!.split(";").map((directive) => directive.trim().split(/\s+/u)).map(([name, ...sources]) => [name!, sources]));
    const sources = (name: string): readonly string[] => directives.get(name) ?? directives.get("default-src") ?? [];
    if (meta.index > html.indexOf("<script")) problems.push("the Content-Security-Policy comes after the first script");
    for (const name of ["default-src", "script-src", "style-src"]) for (const source of directives.get(name) ?? ["(missing)"]) if (!/^'(?:self|none|sha256-[A-Za-z0-9+/]{43}=)'$/u.test(source)) problems.push(`${name} admits ${source}`);
    for (const block of html.matchAll(/<script(?![^>]*\ssrc=)[^>]*>([\s\S]*?)<\/script>/gu)) {
      const hash = `'sha256-${createHash("sha256").update(block[1]!).digest("base64")}'`;
      if (!sources("script-src").includes(hash)) problems.push(`script-src does not admit the inline script ${hash}`);
    }
    for (const block of html.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/gu)) {
      const hash = `'sha256-${createHash("sha256").update(block[1]!).digest("base64")}'`;
      if (!sources("style-src").includes(hash)) problems.push(`style-src does not admit the inline style ${hash}`);
    }
    const connections = [proctorOrigin, proctorOrigin.replace(/^http/u, "ws")];
    if ([...sources("connect-src")].sort().join(" ") !== [...connections].sort().join(" ")) problems.push(`connect-src admits ${sources("connect-src").join(" ") || "nothing"}, not exactly ${connections.join(" ")}`);
    for (const name of ["object-src", "base-uri"]) if (directives.get(name)?.join(" ") !== "'none'") problems.push(`${name} is not 'none'`);
    if (!directives.has("form-action")) problems.push("form-action is not restricted");
  }
  if (!/<meta name="description"/u.test(html)) problems.push("the document has no description");
  if (/<html[^>]*\slang=/u.test(html)) problems.push("the document declares a default language");
  return problems;
}

/** 🔎️ Every reason `dist` is not the CDN artifact of a release build baked for `proctorOrigin`: the host entry points and
 * markers (`404.html` being the same document as `index.html`), `CNAME` naming the site host, a sealed document
 * ({@link siteDocumentProblems}) whose scripts and stylesheets exist, content-hashed file names under `assets/` (they are
 * served as immutable), the proctor origin in the code and no loopback address outside it, no source map and no
 * development leftover, and the size budget ({@link QUIZ_SITE_BUDGET}). */
export function siteArtifactProblems(dist: string, proctorOrigin: string): string[] {
  const problems: string[] = [];
  const files = siteFiles(dist, STAGED_DIRECTORIES);
  const text = (path: string): string => readFileSync(join(dist, path), "utf8");
  for (const required of ["index.html", "404.html", ".nojekyll", "CNAME", "robots.txt", "manifest.webmanifest", "favicon.svg"]) if (!files.includes(required)) problems.push(`${required} is missing`);
  if (files.includes("CNAME") && text("CNAME").trim() !== QUIZ_SITE_HOST) problems.push(`CNAME names ${JSON.stringify(text("CNAME").trim())}, not ${QUIZ_SITE_HOST}`);
  if (files.includes("index.html")) {
    const html = text("index.html");
    if (files.includes("404.html") && text("404.html") !== html) problems.push("404.html is not the document of index.html");
    problems.push(...siteDocumentProblems(html, proctorOrigin));
    const linked = [...html.matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="\/(assets\/[^"]+)"/gu)].map((tag) => tag[1]!);
    for (const path of linked) if (!files.includes(path)) problems.push(`the document loads ${path}, which the artifact lacks`);
    for (const [kind, extension, budget] of [["script", ".js", QUIZ_SITE_BUDGET.scriptGzipBytes], ["stylesheet", ".css", QUIZ_SITE_BUDGET.styleGzipBytes]] as const) {
      const weight = linked.filter((path) => path.endsWith(extension) && files.includes(path)).reduce((sum, path) => sum + gzipSync(readFileSync(join(dist, path))).length, 0);
      if (weight > budget) problems.push(`the document's ${kind}s weigh ${weight} bytes compressed, over the budget of ${budget}`);
    }
  }
  for (const path of files.filter((file) => file.startsWith("assets/"))) if (!/-[A-Za-z0-9_-]{8}\.[a-z0-9]+$/u.test(path)) problems.push(`${path} carries no content hash, yet /assets/* is served as immutable`);
  for (const path of files.filter((file) => file.endsWith(".map"))) problems.push(`${path} is a source map`);
  const code = files.filter((path) => /\.(?:html|js|mjs|css)$/u.test(path)).map(text);
  if (!code.some((source) => source.includes(JSON.stringify(proctorOrigin)))) problems.push(`no script bakes the proctor origin ${proctorOrigin}`);
  const elsewhere = code.map((source) => [proctorOrigin, proctorOrigin.replace(/^http/u, "ws")].reduce((rest, origin) => rest.split(origin).join(""), source));
  for (const loopback of ["localhost", "127.0.0.1", "[::1]"]) if (elsewhere.some((source) => source.includes(loopback))) problems.push(`the site names ${loopback} outside the baked proctor origin`);
  for (const leftover of ["sourceMappingURL=", "/@vite/client", "react-refresh", "[DEBUG]"]) if (code.some((source) => source.includes(leftover))) problems.push(`the site carries the development leftover ${leftover}`);
  const total = files.reduce((sum, path) => sum + statSync(join(dist, path)).size, 0);
  if (total > QUIZ_SITE_BUDGET.totalBytes) problems.push(`the artifact weighs ${total} bytes, over the budget of ${QUIZ_SITE_BUDGET.totalBytes}`);
  return problems;
}

/** 🚚️ `publish` — a fresh release build of the site with the proctor origin baked in (`PROCTOR_URL`, else the production
 * proctor), verified as the CDN artifact and staged with `_headers` under `dist/pages/quizzes`, ready for any static host
 * (GitHub Pages, Cloudflare Pages, Netlify). Nothing is uploaded; a build for another proctor is flagged as a rehearsal. */
export function publishQuizSite(bundleRoot: string, build: () => void): void {
  const proctorOrigin = releaseProctorOrigin();
  say(`building the site for ${QUIZ_SITE_ORIGIN} against ${proctorOrigin}`);
  build();
  const dist = join(bundleRoot, "dist");
  const problems = siteArtifactProblems(dist, proctorOrigin);
  if (problems.length > 0) throw new Error(`the site artifact is not publishable:\n- ${problems.join("\n- ")}`);
  const pages = join(dist, QUIZ_PAGES_DIRECTORY);
  rmSync(join(dist, "pages"), { recursive: true, force: true });
  const files = siteFiles(dist, STAGED_DIRECTORIES);
  for (const path of files) {
    mkdirSync(dirname(join(pages, path)), { recursive: true });
    copyFileSync(join(dist, path), join(pages, path));
  }
  writeFileSync(join(pages, "_headers"), QUIZ_SITE_HEADERS);
  say(`staged ${files.length + 1} files (${files.reduce((sum, path) => sum + statSync(join(dist, path)).size, 0)} bytes) in ${pages}`);
  if (proctorOrigin !== QUIZ_PROCTOR_ORIGIN) say(`REHEARSAL artifact: it talks to ${proctorOrigin}, not ${QUIZ_PROCTOR_ORIGIN}; do not upload it`);
}
//#endregion 🌐️Site

//#region 🐳️Image
/** 🪧️ The labels of the image `reference`. */
async function imageLabels(reference: string, signal?: AbortSignal): Promise<Record<string, string>> {
  return (JSON.parse(await dockerOk(["image", "inspect", "--format", "{{json .Config.Labels}}", reference], { signal })) ?? {}) as Record<string, string>;
}

/** 🏗️ `docker-image-build [--tag <tag>] [--jobs <n>]` — `docker build` of the proctor image from the repository root with
 * the workspace version, the commit and the commit time as OCI labels, streaming every BuildKit step; `--jobs` caps the
 * builder's parallel compiler jobs. */
export async function buildQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const reference = `${QUIZ_IMAGE_REPOSITORY}:${flag(segments, "--tag") ?? "latest"}`;
  const jobs = flag(segments, "--jobs");
  const { revision, created } = sourceRevision(repoRoot);
  const labels = { VERSION: workspaceCargoVersion(repoRoot), REVISION: revision, CREATED: created };
  say(`docker ${await dockerServerVersion()}; building ${reference} (${labels.VERSION}, ${revision})`);
  await runOwnedCommand("docker", ["build", "--progress=plain", ...Object.entries({ ...labels, ...(jobs ? { CARGO_BUILD_JOBS: jobs } : {}) }).flatMap(([name, value]) => ["--build-arg", `${name}=${value}`]), "--file", DOCKERFILE, "--tag", reference, "."], repoRoot, "architecture-quiz:docker-image-build");
  say(`${reference} ${await dockerOk(["image", "inspect", "--format", "{{.Id}} {{.Os}}/{{.Architecture}} {{.Size}} bytes", reference])}`);
}

/** 📤️ `docker-image-publish [--tag <tag>]` — pushes the built image (default `latest`) to the registry of `🔣️.json` as
 * `sha-<revision>` (never moved again: the rollback target), the workspace version and `latest`, with the credentials of
 * the developer's own `docker login` (or CI's), and prints the digest. Only an image built from this exact commit, with
 * no uncommitted change, that passes `docker-image-check` and `docker-stack-check` is pushed. */
export async function publishQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const source = `${QUIZ_IMAGE_REPOSITORY}:${tag}`;
  say(`docker ${await dockerServerVersion()}; publishing ${source}`);
  if ((await docker(["image", "inspect", source])).code !== 0) throw new Error(`image ${source} does not exist; build it first with docker-image-build`);
  const built = (await imageLabels(source))["org.opencontainers.image.revision"] ?? "unknown";
  const { revision } = sourceRevision(repoRoot);
  if (!/^[0-9a-f]{40}$/u.test(built)) throw new Error(`${source} was built from ${built}: only an image of a commit without uncommitted changes is published`);
  if (built !== revision) throw new Error(`${source} was built from ${built}, the checkout is at ${revision}; build it again with docker-image-build`);
  await checkQuizImage(repoRoot, ["--tag", tag]);
  await checkQuizStack(["--tag", tag]);
  const references = [`sha-${built.slice(0, 12)}`, workspaceCargoVersion(repoRoot), "latest"].map((name) => `${QUIZ_IMAGE_REPOSITORY}:${name}`);
  for (const reference of references) {
    if (reference !== source) await dockerOk(["tag", source, reference]);
    const pushed = await docker(["push", reference]);
    if (pushed.code !== 0) throw new Error(`docker push ${reference} exited ${pushed.code}${/denied|unauthorized|authentication/iu.test(pushed.stderr) ? `; log in first: docker login ${QUIZ_IMAGE_REPOSITORY.split("/")[0]}` : ""}: ${pushed.stderr.slice(-600)}`);
    say(`pushed ${reference} ${/digest: (sha256:[0-9a-f]{64})/u.exec(pushed.stdout)?.[1] ?? ""}`);
  }
  say(`roll back to this release on a host with PROCTOR_TAG=sha-${built.slice(0, 12)} docker compose up -d`);
}

/** 📬️ One HTTP answer of the proctor. */
type Probe = Readonly<{ status: number; headers: Headers; body: string }>;

/** 📡️ One request to `url`; TLS verification is off only for the throw-away stack's internal CA. */
async function probe(url: string, init: Readonly<{ method?: string; headers?: Record<string, string>; body?: string | Uint8Array; insecure?: boolean; timeoutMs?: number }>, signal: AbortSignal): Promise<Probe> {
  const response = await fetch(url, { method: init.method ?? "GET", headers: init.headers, body: init.body, redirect: "manual", signal: AbortSignal.any([signal, AbortSignal.timeout(init.timeoutMs ?? 10_000)]), ...(init.insecure ? { tls: { rejectUnauthorized: false } } : {}) } as RequestInit);
  return { status: response.status, headers: response.headers, body: await response.text() };
}

/** 🚪️ How a check reaches a proctor: its base URL, the headers every request carries (the proxy's statement on a direct
 * container port, none through Caddy) and whether the certificate is Caddy's internal one. Every check speaks as the
 * site origin. */
type Door = Readonly<{ base: string; headers: Record<string, string>; insecure: boolean }>;

/** 🧑‍🎓️ The site's own proctor client. A check sends the envelopes this module builds (§9a), so it meets the proctor
 * exactly as the released site does; loaded only when a check runs. */
function siteClient(): Promise<typeof import("../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts")> {
  return import("../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts");
}

/** ❓️ One quiz query as the site sends it, answered with its status and the view. */
async function quizQuery(door: Door, query: Parameters<Awaited<ReturnType<typeof siteClient>>["queryEnvelope"]>[0], learner: string | undefined, signal: AbortSignal): Promise<Probe> {
  const envelope = encodeQueryEnvelope((await siteClient()).queryEnvelope(query, catalog.id, learner));
  return probe(`${door.base}/queries`, { method: "POST", headers: { ...door.headers, origin: QUIZ_SITE_ORIGIN, "content-type": "application/json" }, body: JSON.stringify(envelope), insecure: door.insecure }, signal);
}

/** 🙋️ Registers one learner as the site does — anonymous, or under the pseudonym `handle` — and answers its id. */
async function identifyLearner(door: Door, signal: AbortSignal, handle?: string): Promise<string> {
  const client = await siteClient();
  const identity: IdentityClaim = handle === undefined ? { kind: "anonymous" } : { kind: "pseudonym", handle };
  const command = { type: "identify-learner", id: client.newId(), learner: client.newId(), identity } as const;
  const answer = await probe(`${door.base}/commands`, { method: "POST", headers: { ...door.headers, origin: QUIZ_SITE_ORIGIN, "content-type": "application/json" }, body: JSON.stringify(encodeCommandEnvelope(client.commandEnvelope(command, catalog.id, Date.now()))), insecure: door.insecure }, signal);
  if (answer.status !== 200 || client.commandVerdict(decodeCommandOutcome(JSON.parse(answer.body))).kind !== "accepted") throw new Error(`POST /commands (identify-learner) answered ${answer.status}: ${answer.body.slice(0, 300)}`);
  return command.learner;
}

/** 🕵️ Whether the proctor knows `learner`: its learner view answers, or `404`. */
async function learnerKnown(door: Door, learner: string, signal: AbortSignal): Promise<boolean> {
  const answer = await quizQuery(door, { type: "learner", learner }, learner, signal);
  if (answer.status !== 200 && answer.status !== 404) throw new Error(`POST /queries (learner) answered ${answer.status}: ${answer.body.slice(0, 300)}`);
  return answer.status === 200;
}

/** 🤝️ The cross-origin contract of the CDN-hosted site against the proctor behind `door`: preflights of both POST routes
 * are granted to the site origin with a cache lifetime, a foreign origin gets no grant, and a real catalog query answers
 * with the grant. */
async function checkCrossOrigin(door: Door, signal: AbortSignal): Promise<void> {
  for (const route of ["/commands", "/queries"]) {
    const preflight = await probe(`${door.base}${route}`, { method: "OPTIONS", headers: { ...door.headers, origin: QUIZ_SITE_ORIGIN, "access-control-request-method": "POST", "access-control-request-headers": "content-type" }, insecure: door.insecure }, signal);
    const origin = preflight.headers.get("access-control-allow-origin");
    const maxAge = Number(preflight.headers.get("access-control-max-age") ?? "0");
    if (preflight.status !== 204 || origin !== QUIZ_SITE_ORIGIN || !/content-type/iu.test(preflight.headers.get("access-control-allow-headers") ?? "") || !(maxAge > 0)) throw new Error(`OPTIONS ${route} from ${QUIZ_SITE_ORIGIN} answered ${preflight.status}, allow-origin ${origin}, allow-headers ${preflight.headers.get("access-control-allow-headers")}, max-age ${preflight.headers.get("access-control-max-age")}`);
    say(`preflight ${route}: 204, allow-origin ${origin}, max-age ${maxAge}`);
  }
  const foreign = await probe(`${door.base}/queries`, { method: "OPTIONS", headers: { ...door.headers, origin: FOREIGN_ORIGIN, "access-control-request-method": "POST" }, insecure: door.insecure }, signal);
  if (foreign.headers.get("access-control-allow-origin")) throw new Error(`a preflight from ${FOREIGN_ORIGIN} was granted ${foreign.headers.get("access-control-allow-origin")}`);
  say(`preflight from ${FOREIGN_ORIGIN}: ${foreign.status} without a grant`);
  const query = await quizQuery(door, { type: "catalog" }, undefined, signal);
  if (query.status !== 200 || query.headers.get("access-control-allow-origin") !== QUIZ_SITE_ORIGIN || !query.body.includes("\"snapshot\"")) throw new Error(`POST /queries from ${QUIZ_SITE_ORIGIN} answered ${query.status}, allow-origin ${query.headers.get("access-control-allow-origin")}: ${query.body.slice(0, 300)}`);
  say(`POST /queries (catalog) from ${QUIZ_SITE_ORIGIN}: 200 snapshot with the grant`);
}

/** 📏️ The request body limit of `limit` bytes holds behind `door`: one byte more is refused with `413`, the limit itself
 * is read (and refused for what it is, not for its size). On a direct port with the Caddyfile's default this proves the
 * proctor's own default is the same number; through Caddy with a limit set in `.env` it proves both services follow
 * that one variable. */
async function checkBodyLimit(door: Door, limit: number, signal: AbortSignal): Promise<void> {
  const post = (bytes: number): Promise<Probe> => probe(`${door.base}/commands`, { method: "POST", headers: { ...door.headers, origin: QUIZ_SITE_ORIGIN, "content-type": "application/json" }, body: new Uint8Array(bytes).fill(0x20), insecure: door.insecure }, signal);
  const [over, at] = [await post(limit + 1), await post(limit)];
  if (over.status !== 413 || at.status === 413) throw new Error(`a body of ${limit + 1} bytes answered ${over.status} and one of ${limit} bytes ${at.status}; the limit of ${limit} bytes must refuse the first with 413 and read the second`);
  say(`request body limit: ${limit + 1} bytes refused with 413, ${limit} bytes read (${at.status})`);
}

/** 🪢️ One presence socket of a check: opens with the subprotocol and `headers`, and keeps every decoded frame. */
function presenceSocket(url: string, headers: Record<string, string>, insecure: boolean): Readonly<{ opened: Promise<WebSocket>; frames: PresenceFrame[] }> {
  const frames: PresenceFrame[] = [];
  const socket = new WebSocket(url, { protocols: [PRESENCE_PROTOCOL], headers, ...(insecure ? { tls: { rejectUnauthorized: false } } : {}) } as unknown as string[]);
  socket.addEventListener("message", (event) => frames.push(decodePresenceFrame(JSON.parse(String(event.data)))));
  const opened = new Promise<WebSocket>((accept, reject) => {
    socket.addEventListener("open", () => accept(socket), { once: true });
    socket.addEventListener("close", (event) => reject(new Error(`closed ${event.code} before opening`)), { once: true });
  });
  return { opened, frames };
}

/** ⏳️ The first frame in `frames` matching `wanted`, waiting up to 10 s for it to arrive. */
async function presenceFrame(frames: readonly PresenceFrame[], wanted: (frame: PresenceFrame) => boolean, label: string, signal: AbortSignal): Promise<PresenceFrame> {
  for (const started = Date.now(); Date.now() - started < 10_000; await pause(50, signal)) {
    const found = frames.find(wanted);
    if (found) return found;
  }
  throw new Error(`no presence frame: ${label}; got ${JSON.stringify(frames).slice(0, 600)}`);
}

/** 👥️ Shared presence as two CDN-hosted learners meet it on the catalog's roster room: both join with the site origin, one
 * shares a `PresenceState` and the other receives it in a batch, then its departure; a foreign origin never opens. The
 * second learner's socket is handed back open. */
async function checkPresence(door: Door, signal: AbortSignal): Promise<WebSocket> {
  const url = presenceSocketUrl(door.base, catalog.id, "check");
  const first = presenceSocket(url, { ...door.headers, origin: QUIZ_SITE_ORIGIN }, door.insecure);
  const ada = await first.opened;
  const welcome = await presenceFrame(first.frames, (frame) => frame.type === "welcome", "welcome of the first learner", signal);
  if (ada.protocol !== PRESENCE_PROTOCOL || welcome.type !== "welcome") throw new Error(`the presence socket spoke ${JSON.stringify(ada.protocol)}`);
  const second = presenceSocket(url, { ...door.headers, origin: QUIZ_SITE_ORIGIN }, door.insecure);
  const grace = await second.opened;
  const roster = await presenceFrame(second.frames, (frame) => frame.type === "welcome", "welcome of the second learner", signal);
  if (roster.type !== "welcome" || !roster.roster.some((entry) => entry.session === welcome.session)) throw new Error(`the second learner's roster lacks the first: ${JSON.stringify(roster)}`);
  const state = { tag: "0a1b2c3d", identity: { kind: "anonymous" }, place: { screen: "home" }, active: true };
  ada.send(JSON.stringify(encodePresenceFrame({ type: "state", state })));
  await presenceFrame(second.frames, (frame) => frame.type === "batch" && frame.entries.some((entry) => entry.session === welcome.session && Bun.deepEquals(entry.state, state)), "the first learner's state in a batch", signal);
  ada.close();
  await presenceFrame(second.frames, (frame) => frame.type === "batch" && frame.left.includes(welcome.session), "the first learner leaving", signal);
  say(`presence ${url.split("?")[0]}: two learners joined (colours ${welcome.colour}, ${roster.colour}), a shared state and a departure arrived in batches`);
  const foreign = await presenceSocket(url, { ...door.headers, origin: FOREIGN_ORIGIN }, door.insecure).opened.then((socket) => { socket.close(); return "opened"; }, (error: Error) => error.message);
  if (foreign === "opened") throw new Error(`a presence socket from ${FOREIGN_ORIGIN} was opened`);
  say(`presence from ${FOREIGN_ORIGIN}: refused (${foreign})`);
  return grace;
}

/** 🧳️ Copies the stack files the image `reference` carries into a fresh directory of the host — everything a host needs
 * besides Docker — and answers that directory. */
async function extractQuizStack(reference: string, signal: AbortSignal): Promise<string> {
  const directory = mkdtempSync(join(tmpdir(), "architecture-quiz-stack-"));
  const container = await dockerOk(["create", reference], { signal });
  try {
    await dockerOk(["cp", `${container}:${QUIZ_IMAGE_STACK_ROOT}/.`, directory], { signal });
  } finally {
    await docker(["rm", "--volumes", container]).catch(() => undefined);
  }
  return directory;
}

/** 🪞️ Fails unless the stack files in `directory`, copied out of an image, are those of this checkout. */
function expectCheckoutStack(repoRoot: string, directory: string): void {
  for (const file of QUIZ_STACK_FILES) if (!existsSync(join(directory, file)) || readFileSync(join(directory, file), "utf8") !== readFileSync(join(repoRoot, DEPLOY, file), "utf8")) throw new Error(`${QUIZ_IMAGE_STACK_ROOT}/${file} in the image is not ${DEPLOY}/${file}; build the image again`);
}

/** 🩺️ `docker-image-check [--tag <tag>] [--port <n>] [--ready-timeout-ms <n>] [--keep]` — inspects the image (labels,
 * account, health probe, no shell, the stack files of this checkout inside it), then runs it with nothing but a fresh
 * volume and a loopback port, read-only and without capabilities as the stack runs it (the baked production defaults are
 * the whole configuration), and fails on the first expectation it misses; the container and volume are removed afterwards
 * unless `--keep`. */
export async function checkQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const port = Number(flag(segments, "--port") ?? "18791");
  const readyTimeoutMs = Number(flag(segments, "--ready-timeout-ms") ?? "60000");
  const keep = segments.includes("--keep");
  const { signal, release } = interruption();
  const run = `${Date.now().toString(36)}-${randomBytes(3).toString("hex")}`;
  const image = `${QUIZ_IMAGE_REPOSITORY}:${tag}`;
  const container = `architecture-quiz-proctor-${run}`;
  const volume = `architecture-quiz-proctor-${run}-data`;
  const door: Door = { base: `http://127.0.0.1:${port}`, headers: { "x-forwarded-proto": "https", "x-forwarded-host": QUIZ_PROCTOR_HOST }, insecure: false };
  const scratch: string[] = [];
  let created = false;
  let running = false;
  try {
    say(`docker ${await dockerServerVersion(signal)}`);
    if ((await docker(["image", "inspect", image], { signal })).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    const config = JSON.parse(await dockerOk(["image", "inspect", "--format", "{{json .Config}}", image], { signal })) as { User: string; Labels: Record<string, string> | null; Healthcheck?: { Test: string[] }; ExposedPorts?: Record<string, unknown>; Volumes?: Record<string, unknown> };
    const label = (name: string): string => config.Labels?.[`org.opencontainers.image.${name}`] ?? "";
    const facts: Record<string, [unknown, unknown]> = { user: [config.User, QUIZ_IMAGE_USER], healthcheck: [config.Healthcheck?.Test, [...QUIZ_IMAGE_HEALTHCHECK]], ports: [Object.keys(config.ExposedPorts ?? {}), [`${QUIZ_IMAGE_PORT}/tcp`]], volumes: [Object.keys(config.Volumes ?? {}), [QUIZ_IMAGE_DATA_ROOT]], version: [label("version"), workspaceCargoVersion(repoRoot)], source: [`${label("source")}.git`, (JSON.parse(readFileSync(join(repoRoot, BUNDLE, "package.json"), "utf8")) as { repository: { url: string } }).repository.url] };
    for (const [fact, [found, expected]] of Object.entries(facts)) if (!Bun.deepEquals(found, expected)) throw new Error(`the image's ${fact} is ${JSON.stringify(found)}, not ${JSON.stringify(expected)}`);
    if (!/^[0-9a-f]{40}(?:-dirty)?$/u.test(label("revision")) || Number.isNaN(Date.parse(label("created")))) throw new Error(`the image names no source revision and time: ${label("revision")} ${label("created")}`);
    say(`image ${label("version")} of ${label("revision")} (${label("created")}), user ${config.User}, ${await dockerOk(["image", "inspect", "--format", "{{.Os}}/{{.Architecture}} {{.Size}} bytes", image], { signal })}`);
    if ((await docker(["run", "--rm", "--entrypoint", "/bin/sh", image, "-c", "true"], { signal })).code === 0) throw new Error("the image carries a shell");
    say("no shell in the image");
    scratch.push(await extractQuizStack(image, signal));
    expectCheckoutStack(repoRoot, scratch[0]!);
    say(`${QUIZ_IMAGE_STACK_ROOT} carries ${QUIZ_STACK_FILES.join(" and ")} of this checkout`);
    await dockerOk(["volume", "create", volume], { signal });
    created = true;
    const started = Date.now();
    await dockerOk(["run", "--detach", "--name", container, "--publish", `127.0.0.1:${port}:${QUIZ_IMAGE_PORT}`, "--volume", `${volume}:${QUIZ_IMAGE_DATA_ROOT}`, "--read-only", "--cap-drop", "ALL", "--security-opt", "no-new-privileges", "--stop-timeout", "30", image], { signal });
    running = true;
    say(`container ${container} (no configuration, read-only root, no capabilities) on ${door.base}, volume ${volume}`);
    let ready: Probe | undefined;
    while (!ready && Date.now() - started < readyTimeoutMs) {
      const state = (await docker(["inspect", "--format", "{{.State.Status}}", container], { signal })).stdout;
      if (state !== "running") throw new Error(`container ${state || "vanished"} before ready: ${(await containerLog(container, 20)).slice(-1200)}`);
      const answer = await probe(`${door.base}/instance`, { headers: door.headers }, signal).catch(() => undefined);
      if (answer?.status === 200) ready = answer;
      else await pause(500, signal);
    }
    if (!ready) throw new Error(`not ready within ${readyTimeoutMs} ms`);
    say(`/instance ready after ${Date.now() - started} ms: ${ready.body.slice(0, 120)}`);
    const cleartext = await probe(`${door.base}/instance`, {}, signal);
    if (cleartext.status !== 403 || cleartext.headers.get("x-semio-refusal") !== "insecure-transport") throw new Error(`/instance without the proxy's statement answered ${cleartext.status} ${cleartext.headers.get("x-semio-refusal")}; cleartext must be refused`);
    say("cleartext refused: 403 insecure-transport");
    await checkCrossOrigin(door, signal);
    await checkBodyLimit(door, stackRequestBodyLimit(readFileSync(join(repoRoot, DEPLOY, "Caddyfile"), "utf8")), signal);
    const learner = await identifyLearner(door, signal);
    const open = await checkPresence(door, signal);
    let health = "";
    const waiting = Date.now();
    while (health !== "healthy" && Date.now() - waiting < 60_000) {
      health = (await docker(["inspect", "--format", "{{.State.Health.Status}}", container], { signal })).stdout;
      if (health !== "healthy") await pause(500, signal);
    }
    if (health !== "healthy") throw new Error(`the image HEALTHCHECK reports ${health || "nothing"} after 60 s`);
    say(`HEALTHCHECK: healthy after ${Date.now() - started} ms`);
    const stopping = Date.now();
    await dockerOk(["stop", "--time", "30", container], { signal });
    running = false;
    const exitCode = Number(await dockerOk(["inspect", "--format", "{{.State.ExitCode}}", container], { signal }));
    if (exitCode !== 0) throw new Error(`docker stop drained with exit code ${exitCode}`);
    say(`docker stop drained in ${Date.now() - stopping} ms with a presence socket open (now ${open.readyState === WebSocket.OPEN ? "still open" : "closed"}), exit code 0`);
    open.close();
    scratch.push(mkdtempSync(join(tmpdir(), "architecture-quiz-data-")));
    await dockerOk(["cp", `${container}:${QUIZ_IMAGE_DATA_ROOT}/.`, scratch[1]!], { signal });
    const kept = readdirSync(scratch[1]!).map((name) => `${name} (${statSync(join(scratch[1]!, name)).size} bytes)`);
    if (!existsSync(join(scratch[1]!, QUIZ_IMAGE_DATABASE)) || (existsSync(join(scratch[1]!, `${QUIZ_IMAGE_DATABASE}-wal`)) && statSync(join(scratch[1]!, `${QUIZ_IMAGE_DATABASE}-wal`)).size > 0)) throw new Error(`after a clean stop ${QUIZ_IMAGE_DATA_ROOT} holds ${kept.join(", ")}: ${QUIZ_IMAGE_DATABASE} must be whole, with no write-ahead log left`);
    const events = await databaseEvents(join(scratch[1]!, QUIZ_IMAGE_DATABASE));
    if (events < 1) throw new Error(`${QUIZ_IMAGE_DATABASE} holds ${events} events after learner ${learner} registered`);
    say(`volume keeps ${kept.join(", ")}: ${events} events, the write-ahead log folded; ${image} passed`);
  } catch (error) {
    if (created) console.error(`[deploy] container log tail:\n${await containerLog(container, 30)}`);
    throw error;
  } finally {
    release();
    for (const directory of scratch) rmSync(directory, { recursive: true, force: true });
    if (running) await docker(["stop", "--time", "30", container]).catch(() => undefined);
    if (created && !keep) {
      await docker(["rm", "--force", container]).catch(() => undefined);
      await docker(["volume", "rm", "--force", volume]).catch(() => undefined);
    }
  }
}

/** 🧫️ How many events the proctor database at `file` holds, read by an engine that is not the proctor's (`bun:sqlite`),
 * after checking that the file declares the proctor's format. */
async function databaseEvents(file: string): Promise<number> {
  const { Database } = await import("bun:sqlite");
  const database = new Database(file, { readonly: true });
  try {
    const format = database.query("SELECT schema FROM proctor_format").get() as { schema: string } | null;
    if (!format?.schema.includes("proctor")) throw new Error(`${file} declares the format ${JSON.stringify(format)}`);
    return (database.query("SELECT COUNT(*) AS events FROM proctor_event").get() as { events: number }).events;
  } finally {
    database.close();
  }
}
//#endregion 🐳️Image

//#region 🧱️Stack
/** 🔖️ The SHA-256 fingerprint of the certificate the TLS listener on `localhost:<port>` presents for the name
 * `localhost`, whoever signed it. */
function servedCertificate(port: number, signal: AbortSignal): Promise<string> {
  return new Promise((accept, reject) => {
    if (signal.aborted) return reject(new Error("cancelled"));
    const socket = connect({ host: "localhost", port, servername: "localhost", rejectUnauthorized: false }, () => {
      const fingerprint = socket.getPeerCertificate().fingerprint256;
      socket.destroy();
      accept(fingerprint);
    });
    socket.setTimeout(10_000, () => socket.destroy(new Error(`no TLS handshake on localhost:${port} within 10 s`)));
    socket.once("error", reject);
    signal.addEventListener("abort", () => socket.destroy(new Error("cancelled")), { once: true });
  });
}

/** 🏭️ A certificate for `localhost` as an operator would supply one — one PEM bundle, the chain first and the private key
 * after it — minted by the internal CA of a throw-away container of `caddyImage`: another CA than the stack's own, so
 * never the certificate the stack obtains for itself, and no tool outside Docker. */
async function mintLocalCertificate(caddyImage: string, signal: AbortSignal): Promise<string> {
  const issued = "/data/caddy/certificates/local/localhost/localhost";
  const minted = await docker(["run", "--rm", "--interactive", "--entrypoint", "/bin/sh", caddyImage, "-c", `cat > /tmp/Caddyfile && caddy start --config /tmp/Caddyfile --adapter caddyfile > /dev/null 2>&1 && for attempt in $(seq 1 150); do [ -s ${issued}.crt ] && [ -s ${issued}.key ] && [ -s ${issued}.json ] && exec cat ${issued}.crt ${issued}.key; sleep 0.2; done; exit 1`], { signal, input: "{\n\tskip_install_trust\n}\n\nlocalhost {\n\trespond ok\n}\n" });
  if (minted.code !== 0 || !/-----BEGIN CERTIFICATE-----[\s\S]+PRIVATE KEY-----/u.test(minted.stdout)) throw new Error(`a throw-away ${caddyImage} minted no certificate for localhost (exit ${minted.code}): ${minted.stderr.slice(-600)}`);
  return `${minted.stdout}\n`;
}

/** 🧱️ `docker-stack-check [--tag <tag>] [--http-port <n>] [--https-port <n>] [--keep]` — the stack as a host gets and
 * runs it: the two stack files copied out of the image into an empty directory, an `.env` of overrides beside them (the
 * proctor host `localhost` — Caddy's internal CA, no public certificate —, the tag, the ports and a request body limit
 * for both services), `docker compose config`, `caddy fmt` and `caddy validate` in the Caddy image, `up --wait` on a
 * throw-away project, the hardening and limits Docker applied to both containers, the API over HTTPS through Caddy
 * (plain HTTP redirects, a forged `X-Forwarded-Proto` changes nothing, the response headers, the body limit of `.env`), the
 * cross-origin contract and shared presence (WebSocket) through Caddy, a backup taken while serving and read back, an
 * update (the proctor container replaced) during which no request fails and after which the data is still there, a
 * restore refused while serving and accepted once stopped, a learner erased on request, a certificate of the operator
 * ({@link mintLocalCertificate}) put into `certificates` and served instead of Caddy's own once the caddy service is
 * recreated (SHA-256 fingerprints of what the listener presents before and after, against the supplied file), then
 * `down --volumes` unless `--keep`. */
export async function checkQuizStack(segments: readonly string[]): Promise<void> {
  const httpPort = flag(segments, "--http-port") ?? "18080";
  const httpsPort = flag(segments, "--https-port") ?? "18443";
  const tag = flag(segments, "--tag") ?? "latest";
  const keep = segments.includes("--keep");
  const { signal, release } = interruption();
  const project = `architecture-quiz-check-${Date.now().toString(36)}`;
  const bodyLimit = 32_768;
  const overrides = { PROCTOR_HOST: "localhost", PROCTOR_TAG: tag, QUIZ_HTTP_PORT: httpPort, QUIZ_HTTPS_PORT: httpsPort, PROCTOR_LIMIT_BODY_BYTES: String(bodyLimit) };
  const env = Object.fromEntries([...Object.keys(overrides), "PROCTOR_ALLOWED_ORIGINS"].map((name) => [name, undefined]));
  const door: Door = { base: `https://localhost:${httpsPort}`, headers: {}, insecure: true };
  let directory = "";
  let started = false;
  const composed = (args: readonly string[], input?: Uint8Array): Promise<DockerAnswer> => docker(["compose", "--project-name", project, "--file", "compose.yaml", ...args], { signal, env, cwd: directory, input });
  const compose = async (args: readonly string[]): Promise<string> => {
    const answer = await composed(args);
    if (answer.code !== 0) throw new Error(`docker compose ${args.slice(0, 3).join(" ")} exited ${answer.code}: ${(answer.stderr || answer.stdout).split("\n").slice(-6).join(" | ").slice(0, 900)}`);
    return answer.stdout;
  };
  try {
    say(`docker ${await dockerServerVersion(signal)}; project ${project}`);
    directory = await extractQuizStack(`${QUIZ_IMAGE_REPOSITORY}:${tag}`, signal);
    const footprint = readdirSync(directory).sort();
    if (!Bun.deepEquals(footprint, [...QUIZ_STACK_FILES].sort())) throw new Error(`the image carries ${footprint.join(", ")} as its stack, not exactly ${QUIZ_STACK_FILES.join(" and ")}`);
    say(`host footprint: ${footprint.join(" and ")} from ${QUIZ_IMAGE_REPOSITORY}:${tag} in ${directory}`);
    mkdirSync(join(directory, QUIZ_STACK_CERTIFICATES));
    writeFileSync(join(directory, ".env"), Object.entries(overrides).map(([name, value]) => `${name}=${value}\n`).join(""));
    say(`.env overrides: ${Object.entries(overrides).map(([name, value]) => `${name}=${value}`).join(" ")}`);
    const config = await compose(["config"]);
    if (!config.includes("proctor") || !config.includes("caddy")) throw new Error("docker compose config lacks the proctor or the caddy service");
    say("docker compose config resolves");
    const caddyImage = /image:\s*(caddy:\S+)/u.exec(config)?.[1];
    if (!caddyImage) throw new Error("docker compose config names no caddy image");
    const caddyfile = readFileSync(join(directory, "Caddyfile"), "utf8");
    const validated = await docker(["run", "--rm", "--interactive", "--env", "PROCTOR_HOST=localhost", "--entrypoint", "/bin/sh", caddyImage, "-c", `cat > /tmp/Caddyfile && mkdir /${QUIZ_STACK_CERTIFICATES} && caddy fmt /tmp/Caddyfile > /dev/null && caddy validate --config /tmp/Caddyfile --adapter caddyfile`], { signal, input: caddyfile });
    if (validated.code !== 0 || /"level":"(?:warn|error)"/u.test(validated.stderr)) throw new Error(`caddy fmt and caddy validate do not accept the Caddyfile without a warning: ${(validated.stderr || validated.stdout).slice(-900)}`);
    say(`caddy fmt and caddy validate (${caddyImage}): formatted, ${validated.stderr.split("\n").filter((line) => /Valid configuration/u.test(line)).join(" ") || "valid"}, no warning`);
    started = true;
    const up = Date.now();
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    say(`up --wait: proctor and caddy healthy after ${Date.now() - up} ms`);
    for (const service of ["proctor", "caddy"]) {
      const host = JSON.parse(await dockerOk(["inspect", "--format", "{{json .HostConfig}}", `${project}-${service}-1`], { signal })) as { ReadonlyRootfs: boolean; CapDrop: string[] | null; SecurityOpt: string[] | null; Memory: number; PidsLimit: number | null; LogConfig: { Type: string; Config: Record<string, string> }; RestartPolicy: { Name: string }; PortBindings: Record<string, unknown> | null };
      const hardened = host.ReadonlyRootfs && host.CapDrop?.includes("ALL") && host.SecurityOpt?.some((option) => option.startsWith("no-new-privileges")) && host.Memory > 0 && (host.PidsLimit ?? 0) > 0 && host.LogConfig.Type === "json-file" && host.LogConfig.Config["max-size"] && host.LogConfig.Config["max-file"] && host.RestartPolicy.Name === "unless-stopped";
      if (!hardened) throw new Error(`the ${service} container is not hardened, limited and log-rotated as compose.yaml says: ${JSON.stringify(host).slice(0, 900)}`);
      if (service === "proctor" && Object.keys(host.PortBindings ?? {}).length > 0) throw new Error(`the proctor publishes ${JSON.stringify(host.PortBindings)} on the host`);
      say(`${service}: read-only root, capabilities dropped, no new privileges, ${host.Memory / 1024 ** 2} MiB, ${host.PidsLimit} pids, logs ${host.LogConfig.Config["max-file"]} × ${host.LogConfig.Config["max-size"]}, restart ${host.RestartPolicy.Name}`);
    }
    const instance = await probe(`${door.base}/instance`, { insecure: true }, signal);
    if (instance.status !== 200 || !instance.body.includes("teaching-proctor")) throw new Error(`GET ${door.base}/instance through Caddy answered ${instance.status}: ${instance.body.slice(0, 300)}`);
    const sent = Object.fromEntries(["strict-transport-security", "x-content-type-options", "x-frame-options", "content-security-policy", "referrer-policy", "server", "via"].map((name) => [name, instance.headers.get(name)]));
    if (!sent["strict-transport-security"] || sent["x-content-type-options"] !== "nosniff" || sent["x-frame-options"] !== "DENY" || !sent["content-security-policy"]?.includes("frame-ancestors 'none'") || !sent["referrer-policy"] || sent.server || sent.via) throw new Error(`GET /instance through Caddy carries the headers ${JSON.stringify(sent)}`);
    say(`GET ${door.base}/instance through Caddy: 200, ${Object.entries(sent).filter(([, value]) => value).map(([name, value]) => `${name}: ${value}`).join("; ")}`);
    const redirect = await probe(`http://localhost:${httpPort}/instance`, { headers: { "x-forwarded-proto": "https" } }, signal);
    if (redirect.status < 300 || redirect.status >= 400) throw new Error(`plain HTTP (claiming X-Forwarded-Proto: https) answered ${redirect.status} instead of redirecting to HTTPS`);
    say(`plain HTTP claiming X-Forwarded-Proto: https: ${redirect.status} → ${redirect.headers.get("location")}`);
    const forged = await probe(`${door.base}/instance`, { headers: { "x-forwarded-proto": "http" }, insecure: true }, signal);
    if (forged.status !== 200) throw new Error(`HTTPS with a client's X-Forwarded-Proto: http answered ${forged.status}; Caddy must replace the header`);
    say("HTTPS with a client's X-Forwarded-Proto: http: 200, Caddy replaced the header");
    await checkCrossOrigin(door, signal);
    await checkBodyLimit(door, bodyLimit, signal);
    (await checkPresence(door, signal)).close();
    const before = await identifyLearner(door, signal);
    const backup = await composed(["exec", "--no-TTY", "proctor", "proctor", "backup", "-"]);
    if (backup.code !== 0) throw new Error(`proctor backup - exited ${backup.code}: ${backup.stderr.slice(-600)}`);
    writeFileSync(join(directory, QUIZ_IMAGE_DATABASE), backup.bytes);
    const events = await databaseEvents(join(directory, QUIZ_IMAGE_DATABASE));
    if (events < 1) throw new Error(`the backup holds ${events} events after learner ${before} registered`);
    say(`backup while serving: ${backup.bytes.length} bytes, ${events} events (read with bun:sqlite)`);
    const after = await identifyLearner(door, signal);
    const answers: number[] = [];
    let updating = true;
    const watching = (async (): Promise<void> => {
      while (updating) {
        answers.push(await probe(`${door.base}/instance`, { insecure: true, timeoutMs: 60_000 }, signal).then((answer) => answer.status, () => 0));
        await pause(250, signal).catch(() => undefined);
      }
    })();
    const updated = Date.now();
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180", "--force-recreate", "proctor"]).finally(() => (updating = false));
    await watching;
    if (answers.some((status) => status !== 200)) throw new Error(`while the proctor container was replaced, GET /instance through Caddy answered ${answers.join(" ")}`);
    if (!(await learnerKnown(door, before, signal)) || !(await learnerKnown(door, after, signal))) throw new Error("a learner registered before the update is gone after it");
    say(`update (proctor container replaced in ${Date.now() - updated} ms): ${answers.length} requests through Caddy, none failed; both learners still known`);
    const refused = await composed(["exec", "--no-TTY", "proctor", "proctor", "restore", "-"], backup.bytes);
    if (refused.code === 0) throw new Error("proctor restore replaced the database of a serving proctor");
    say(`restore while serving: refused (${refused.stderr.split("\n").at(-1)?.slice(0, 160)})`);
    await compose(["stop", "proctor"]);
    const restored = await composed(["run", "--rm", "--no-TTY", "--no-deps", "proctor", "restore", "-"], backup.bytes);
    if (restored.code !== 0) throw new Error(`proctor restore - exited ${restored.code}: ${restored.stderr.slice(-600)}`);
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    if (!(await learnerKnown(door, before, signal)) || (await learnerKnown(door, after, signal))) throw new Error("the restored proctor does not answer the moment of the backup");
    say("restore once stopped: the proctor answers the moment of the backup (the earlier learner known, the later one not)");
    const handle = `Deploy Check ${randomBytes(3).toString("hex")}`;
    const named = await identifyLearner(door, signal, handle);
    if (!(await learnerKnown(door, named, signal))) throw new Error(`the learner registered as ${handle} is unknown`);
    await compose(["stop", "proctor"]);
    const erasure = ["run", "--rm", "--no-TTY", "--no-deps", "proctor", "erase", "--handle", handle];
    const [planned, erased] = [await composed([...erasure, "--dry-run"]), await composed(erasure)];
    if (planned.code !== 0 || erased.code !== 0) throw new Error(`proctor erase exited ${planned.code} (dry run) and ${erased.code}: ${(planned.stderr + erased.stderr).slice(-600)}`);
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    if ((await learnerKnown(door, named, signal)) || !(await learnerKnown(door, before, signal))) throw new Error(`after erasing ${handle} that learner is still known, or another one is gone`);
    say(`erase --handle ${JSON.stringify(handle)}: ${planned.stdout.split("\n").at(-1)}; then ${erased.stdout.split("\n").at(-1)?.slice(0, 60)}; that learner is unknown, the other one still known`);
    const own = await servedCertificate(Number(httpsPort), signal);
    const bundle = await mintLocalCertificate(caddyImage, signal);
    const supplied = new X509Certificate(bundle).fingerprint256;
    if (own === supplied) throw new Error(`the minted certificate ${supplied} is the one Caddy already serves`);
    writeFileSync(join(directory, QUIZ_STACK_CERTIFICATES, "localhost.pem"), bundle, { mode: 0o644 });
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180", "--force-recreate", "caddy"]);
    const served = await servedCertificate(Number(httpsPort), signal);
    if (served !== supplied) throw new Error(`with ${QUIZ_STACK_CERTIFICATES}/localhost.pem (${supplied}) supplied and the caddy service recreated, Caddy serves ${served} (its own was ${own})`);
    const logged = await composed(["logs", "--no-log-prefix", "caddy"]);
    const unmanaged = /skipping automatic certificate management[^\n]*"domain":"localhost"/u.test(`${logged.stdout}\n${logged.stderr}`);
    const through = await probe(`${door.base}/instance`, { insecure: true }, signal);
    if (!unmanaged || through.status !== 200) throw new Error(`with the supplied certificate Caddy ${unmanaged ? "manages none of its own" : "still manages its own"} and GET /instance answers ${through.status}`);
    say(`supplied certificate: Caddy served its own ${own}; with ${QUIZ_STACK_CERTIFICATES}/localhost.pem of another CA and the caddy service recreated it serves ${served}, the supplied one, manages none for localhost, and GET /instance answers 200`);
    say("stack passed");
  } catch (error) {
    if (started) console.error(`[deploy] stack logs:\n${(await docker(["compose", "--project-name", project, "--file", "compose.yaml", "logs", "--tail", "30"], { env, cwd: directory }).catch(() => ({ stdout: "", stderr: "" }))).stdout.slice(-3000)}`);
    throw error;
  } finally {
    release();
    if (started && !keep) await docker(["compose", "--project-name", project, "--file", "compose.yaml", "down", "--volumes", "--timeout", "30"], { env, cwd: directory }).catch(() => undefined);
    if (directory && !keep) rmSync(directory, { recursive: true, force: true });
  }
}
//#endregion 🧱️Stack

//#region 🎁️Bundle
/** 🎚️ The `.env` of the bundle: the tag of the image the host loads, pinned so that compose never looks for another
 * one, and every other default of `compose.yaml` an operator may override, as a comment holding that default. */
export function stackBundleEnvironment(tag: string): string {
  return ["# Overrides of compose.yaml; both services read this file as their environment.", `PROCTOR_TAG=${tag}`, `# PROCTOR_HOST=${QUIZ_PROCTOR_HOST}`, `# PROCTOR_ALLOWED_ORIGINS=${QUIZ_SITE_ORIGIN}`, "# QUIZ_HTTP_PORT=80", "# QUIZ_HTTPS_PORT=443"].map((line) => `${line}\n`).join("");
}

/** 📖️ The steps of the proctor host, as they travel with the bundle for the image tag `tag`. */
export function stackBundleReadme(tag: string): string {
  return [
    `Architecture quiz proctor: the Docker stack of ${QUIZ_PROCTOR_ORIGIN}`,
    "",
    `The backend of the site ${QUIZ_SITE_ORIGIN}:`,
    "the proctor (an API over one SQLite file on a Docker volume) behind Caddy, which terminates TLS. This directory is",
    "everything the host needs; nothing is pulled from a registry.",
    "",
    `  ${QUIZ_STACK_FILES.join(", ")}   the stack`,
    `  .env                      overrides of compose.yaml; pins the image tag ${tag}`,
    `  ${QUIZ_STACK_CERTIFICATES}/             your own certificate, when Caddy cannot obtain one (step 3)`,
    `  ${QUIZ_BUNDLE_IMAGE}         the image ${QUIZ_IMAGE_REPOSITORY}:${tag}`,
    "",
    "The host: linux/amd64 with Docker Engine 25 or newer and its compose plugin 2.24 or newer.",
    "",
    "1. Copy this whole directory to the host (scp -r, rsync, a USB stick) and work inside it.",
    `2. docker load --input ${QUIZ_BUNDLE_IMAGE}`,
    "3. The certificate, one of:",
    "   a. Let's Encrypt, nothing to do: Caddy obtains and renews it once ports 80/tcp, 443/tcp and 443/udp of",
    `      ${QUIZ_PROCTOR_HOST} are open from the internet.`,
    `   b. Your own: one file ${QUIZ_STACK_CERTIFICATES}/<any name>.pem with the certificate and its full chain first and the private`,
    "      key after it, owned by root with mode 0600 (Caddy does not start on a file it cannot read). Caddy serves",
    "      it and orders none. After replacing it: docker compose up --detach --force-recreate caddy",
    "   Either way a learner's browser reaches the proctor only where 443/tcp of the host is open to it.",
    "4. docker compose up --detach --wait",
    `5. curl --fail ${QUIZ_PROCTOR_ORIGIN}/instance`,
    '   answers JSON that starts {"id":"teaching-proctor"',
    "",
    "Backup while serving (keep copies off the host):",
    "  docker compose exec -T proctor proctor backup - > proctor.sqlite",
    "Stop and keep the data:",
    "  docker compose down",
    "NEVER docker compose down --volumes: it deletes the database and the certificates Caddy obtained.",
    "Logs:",
    "  docker compose logs --since 1h proctor caddy",
    "",
  ].join("\n");
}

/** 🚛️ `docker-stack-bundle [--tag <tag>] [--out <directory>]` — stages everything the proctor host needs without a
 * registry in `--out` (default the package's `dist/proctor`): `compose.yaml` and the `Caddyfile` copied out of the image
 * `--tag` (default `latest`, built beforehand with `docker-image-build`; an image whose stack files are not those of this
 * checkout is refused), the `certificates` directory (what it already holds stays), the `.env`
 * ({@link stackBundleEnvironment}), `README.txt` ({@link stackBundleReadme}) and the image itself as
 * `proctor-image.tar` (`docker save`). Nothing is uploaded: the directory is copied to the host by hand. Answers the
 * directory. */
export async function stageQuizStackBundle(repoRoot: string, bundleRoot: string, segments: readonly string[]): Promise<string> {
  const tag = flag(segments, "--tag") ?? "latest";
  const image = `${QUIZ_IMAGE_REPOSITORY}:${tag}`;
  const directory = resolve(flag(segments, "--out") ?? join(bundleRoot, "dist", QUIZ_BUNDLE_DIRECTORY));
  const { signal, release } = interruption();
  let scratch = "";
  try {
    say(`docker ${await dockerServerVersion(signal)}; bundling ${image}`);
    if ((await docker(["image", "inspect", image], { signal })).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    const labels = await imageLabels(image, signal);
    scratch = await extractQuizStack(image, signal);
    expectCheckoutStack(repoRoot, scratch);
    mkdirSync(join(directory, QUIZ_STACK_CERTIFICATES), { recursive: true });
    for (const file of QUIZ_STACK_FILES) copyFileSync(join(scratch, file), join(directory, file));
    writeFileSync(join(directory, ".env"), stackBundleEnvironment(tag));
    writeFileSync(join(directory, "README.txt"), stackBundleReadme(tag));
    say(`saving ${image} (${labels["org.opencontainers.image.version"]} of ${labels["org.opencontainers.image.revision"]}) as ${QUIZ_BUNDLE_IMAGE}`);
    await dockerOk(["save", "--output", join(directory, QUIZ_BUNDLE_IMAGE), image], { signal });
    say(`staged ${[...QUIZ_STACK_FILES, `${QUIZ_STACK_CERTIFICATES}/`, ".env", "README.txt"].join(", ")} and ${QUIZ_BUNDLE_IMAGE} (${Math.round(statSync(join(directory, QUIZ_BUNDLE_IMAGE)).size / 1024 ** 2)} MiB) in ${directory}`);
    return directory;
  } finally {
    release();
    if (scratch) rmSync(scratch, { recursive: true, force: true });
  }
}
//#endregion 🎁️Bundle

//#region ⚖️Readiness
const DEPLOY_CHECK_STEPS = ["no drift from 🚀️deploy/🔣️.json", "sources type-checked", "catalog valid in the Rust core", "site built and verified as the CDN artifact", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"] as const;
type DeployCheckStep = (typeof DEPLOY_CHECK_STEPS)[number];

/** 🪜️ The steps of `deploy-check` in order for a package with the nx `targets`: the cheap proofs first — no drift, then
 * the sources against the compiler when the package has a `typecheck` target, before anything is built — and the
 * end-to-end gate last, when the package has one. */
export function deployCheckSteps(targets: Readonly<Record<string, unknown>>): readonly DeployCheckStep[] {
  return DEPLOY_CHECK_STEPS.filter((step) => (step !== "sources type-checked" || "typecheck" in targets) && (step !== "end-to-end gate" || "test-e2e" in targets));
}

/** ⚖️ `deploy-check [--tag <tag>] [--jobs <n>]` — everything that must hold before the owner deploys, in the order of
 * {@link deployCheckSteps}, each step announced with its number and time, stopping at the first failure or at Ctrl-C:
 * no drift from `🔣️.json`, the package's sources against the compiler, the catalog valid in the Rust core, the site
 * built and verified as the CDN artifact, the image built, the image check, the stack check, and the package's
 * end-to-end gate. Nothing is pushed or uploaded. */
export async function checkQuizDeployment(repoRoot: string, bundleRoot: string, build: () => void, segments: readonly string[]): Promise<void> {
  const tagged = flag(segments, "--tag") ? ["--tag", flag(segments, "--tag")!] : [];
  const jobs = flag(segments, "--jobs") ? ["--jobs", flag(segments, "--jobs")!] : [];
  const targets = (JSON.parse(readFileSync(join(bundleRoot, "📋️project.json"), "utf8")) as { targets: Record<string, unknown> }).targets;
  const run: { readonly [S in DeployCheckStep]: () => Promise<void> | void } = {
    "no drift from 🚀️deploy/🔣️.json": () => {
      const drift = deploymentDrift(repoRoot);
      if (drift.length > 0) throw new Error(`the deployment drifted:\n- ${drift.join("\n- ")}`);
    },
    "sources type-checked": () => runOwnedCommand("bun", ["nx", "run", `${PROJECT}:typecheck`], repoRoot, "architecture-quiz:typecheck"),
    "catalog valid in the Rust core": () => checkQuizCatalog(repoRoot),
    "site built and verified as the CDN artifact": () => publishQuizSite(bundleRoot, build),
    "proctor image built": () => buildQuizImage(repoRoot, [...tagged, ...jobs]),
    "proctor image checked": () => checkQuizImage(repoRoot, tagged),
    "stack checked": () => checkQuizStack(tagged),
    "end-to-end gate": () => runOwnedCommand("bun", ["nx", "run", `${PROJECT}:test-e2e`], repoRoot, "architecture-quiz:test-e2e"),
  };
  const steps = deployCheckSteps(targets);
  const { signal, release } = interruption();
  const took: string[] = [];
  try {
    for (const [index, label] of steps.entries()) {
      if (signal.aborted) throw new Error("cancelled");
      say(`step ${index + 1}/${steps.length}: ${label}`);
      const started = Date.now();
      await run[label]();
      took.push(`${label} (${Math.round((Date.now() - started) / 1000)} s)`);
    }
  } finally {
    release();
  }
  for (const warning of deploymentWarnings()) say(`WARNING: ${warning}`);
  say(`ready to deploy: ${took.join("; ")}`);
}
//#endregion ⚖️Readiness
