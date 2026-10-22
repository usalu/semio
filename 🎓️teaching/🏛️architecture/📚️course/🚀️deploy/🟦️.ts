/** 🚀️ Operator verbs of the architecture quiz: the site goes to a CDN at the site host, the proctor runs as a zero-touch
 * Docker stack (proctor + Caddy) at the proctor host, both hosts from `🔣️.json`.
 *
 * - `publish` builds the site with the proctor origin baked in, verifies the CDN artifact (entry points, sealed document,
 *   hashed assets, no development leftovers, size budget) and stages it with `_headers`.
 * - `docker-image-build` builds the proctor image with its version, revision and commit time as OCI labels;
 *   `docker-image-publish` checks that image and the stack, then pushes it as `sha-<revision>-<image digest>`, the workspace
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
 * - `deploy-check` is the readiness gate: drift between `🔣️.json` and every file that repeats it, the course, `publish`,
 *   the image build, both checks and the end-to-end gate when the package has one.
 *
 * Runtime Docker checks use one argv without a shell on Windows, macOS and Linux. Image production requires a native
 * Linux release compiler owner. Ctrl-C/SIGTERM stops the running client and removes what a check created.
 * @see ./🔣️.json — the site and proctor hosts, the image repository and port
 * @see ./🧬️schema/🔣️.json — the contract of that file
 * @see ./Dockerfile — the proctor image
 * @see ./compose.yaml — the stack
 * @see ./Caddyfile — the TLS-terminating proxy
 * @see ../README.md — the runbook
 * https://docs.docker.com/reference/cli/docker/ */

import { courseOriginalBuildModeV1 } from "../🏗️builder/🎚️mode/🟦️.ts";

import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { requireIssuedScriptInvocationV1, type ScriptInvocation } from "@semio-tech/process/routing";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { type ProctorArtifact } from "../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import { acquireProctor } from "../../../🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts";
import course from "../🔣️.json" with { type: "json" };
import deployment from "./🔣️.json" with { type: "json" };
import { runToolSyncV1, startToolV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

//#region 🔣️Constants
/** 🌐️ The host the CDN serves the site at. */
export const COURSE_SITE_HOST = deployment.site.host;

/** 🪪️ The site origin: the one browser origin the proctor grants. */
export const COURSE_SITE_ORIGIN = `https://${COURSE_SITE_HOST}`;

/** 🛂️ The host the Docker stack serves the proctor at. */
export const COURSE_PROCTOR_HOST = deployment.proctor.host;

/** 🔗️ The proctor origin a release build of the site talks to. */
export const COURSE_PROCTOR_ORIGIN = `https://${COURSE_PROCTOR_HOST}`;

/** 🏷️ The registry repository every proctor image tag lives under. */
export const COURSE_IMAGE_REPOSITORY = deployment.proctor.image;

/** 🔌️ The port the proctor binds inside the container (`PROCTOR_PORT`). */
export const COURSE_IMAGE_PORT = deployment.proctor.port;

/** 🗄️ The one writable path of the image (`PROCTOR_DATA`). */
export const COURSE_IMAGE_DATA_ROOT = "/srv/course/data";

/** 🗃️ The SQLite file the proctor keeps in `PROCTOR_DATA`. */
export const COURSE_IMAGE_DATABASE = "proctor.sqlite";

/** 👤️ The unprivileged account the image runs as (`uid:gid`). */
export const COURSE_IMAGE_USER = "65532:65532";

/** 💓️ The image's `HEALTHCHECK`: the proctor's own readiness probe. */
export const COURSE_IMAGE_HEALTHCHECK = ["CMD", "/usr/local/bin/proctor", "health"] as const;

/** 🎒️ Where the image carries the stack files of its release. */
export const COURSE_IMAGE_STACK_ROOT = "/srv/course/deploy";

/** 👣️ The whole footprint of a host besides Docker: these two files in one directory. */
export const COURSE_STACK_FILES = ["compose.yaml", "Caddyfile"] as const;

/** 🔏️ The directory beside the stack files Caddy loads the operator's own certificates from (`*.pem` bundles). */
export const COURSE_STACK_CERTIFICATES = "certificates";

/** 📦️ Where `publish` stages the CDN artifact, relative to the package's `dist`. */
export const COURSE_PAGES_DIRECTORY = "pages/course";

/** 🎁️ Where `docker-stack-bundle` stages the directory for the proctor host, relative to the package's `dist`. */
export const COURSE_BUNDLE_DIRECTORY = "proctor";

/** 💾️ The file the bundle carries the proctor image in. */
export const COURSE_BUNDLE_IMAGE = "proctor-image.tar";

/** 🏋️ What a release of the site may weigh: the entry script and stylesheet compressed as a CDN sends them, and the whole
 * artifact on disk. */
export const COURSE_SITE_BUDGET = { scriptGzipBytes: 300_000, styleGzipBytes: 60_000, totalBytes: 5_000_000 } as const;

/** 📑️ `_headers` for CDNs that honour it (Cloudflare Pages, Netlify): every response hardened (the frame ban a `meta`
 * policy cannot carry included), hashed assets immutable, documents revalidated. No two rules set the same header for
 * one path, because those CDNs join the values of every matching rule. GitHub Pages ignores the file. */
export const COURSE_SITE_HEADERS =
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

const SITE = "🎓️teaching/🏛️architecture/📚️course";
export const DEPLOY = `${SITE}/🚀️deploy`;
export const DOCKERFILE = `${DEPLOY}/Dockerfile`;
export const BUNDLE = `${SITE}/📦️packages/🟦️typescript`;
export const COURSE = `${SITE}/🔣️.json`;
const PROCTOR_README = "🎓️teaching/🛂️proctor/README.md";
const WORKFLOW = ".github/workflows/architecture-course.yml";
const PROJECT = "@teaching/architecture-course";
export const FOREIGN_ORIGIN = "https://foreign.example";
//#endregion 🔣️Constants

//#region 🧰️Tools
/** 🏳️ The value following `flag` in `segments`, if any. */
export function flag(segments: readonly string[], name: string): string | undefined {
  const index = segments.indexOf(name);
  return index >= 0 ? segments[index + 1] : undefined;
}

/** 🛑️ Carries original cancellation alongside Ctrl-C/SIGTERM and releases its listeners. */
export function deploymentInterruption(parent: AbortSignal): { readonly signal: AbortSignal; readonly release: () => void } {
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  const originalCancelled = (): void => controller.abort(parent.reason);
  parent.addEventListener("abort", originalCancelled, { once: true });
  if (parent.aborted) originalCancelled();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  return { signal: controller.signal, release: () => { controller.abort(); parent.removeEventListener("abort", originalCancelled); process.off("SIGINT", cancel); process.off("SIGTERM", cancel); } };
}

/** 🪪️ Requires the exact live parent before lending cancellation to a deployment operation. */
export function interruption(invocation: ScriptInvocation): ReturnType<typeof deploymentInterruption> {
  requireIssuedScriptInvocationV1(invocation);
  return deploymentInterruption(invocation.control.signal);
}

/** 🗣️ One progress line of these verbs. */
export function say(line: string): void {
  console.log(`[deploy] ${line}`);
}

/** 💤️ Resolves after `ms`, or fails at once when `signal` is aborted. */
export function pause(ms: number, signal: AbortSignal): Promise<void> {
  if (signal.aborted) return Promise.reject(signal.reason);
  return new Promise((accept, reject) => {
    const cancel = (): void => { clearTimeout(timer); reject(signal.reason); };
    const timer = setTimeout(() => { signal.removeEventListener("abort", cancel); accept(); }, ms);
    signal.addEventListener("abort", cancel, { once: true });
  });
}

/** 🧾️ What one Docker client call answered: its exit code, both streams as text and stdout as the bytes it wrote. */
export type DockerAnswer = Readonly<{ code: number; stdout: string; stderr: string; bytes: Buffer }>;

/** 🧯️ Bounds cleanup after cancellation without reviving the canceled deployment work. */
export const COURSE_DEPLOY_CLEANUP_MS = 45_000;

/** 🐳️ Runs one `docker` argv without a shell, with `env` merged over the process environment (a variable set to
 * `undefined` is removed) and `input` on stdin; an aborted `signal` interrupts the client. */
export function docker(args: readonly string[], options: Readonly<{ signal?: AbortSignal; env?: NodeJS.ProcessEnv; input?: string | Uint8Array; cwd?: string }> = {}): Promise<DockerAnswer> {
  return new Promise((accept, reject) => {
    if (options.signal?.aborted) return reject(new Error("cancelled"));
    const child = startToolV1("docker", [...args], { shell: false, cwd: options.cwd, env: Object.fromEntries(Object.entries({ ...process.env, ...options.env }).filter(([, value]) => value !== undefined)), stdio: [options.input === undefined ? "ignore" : "pipe", "pipe", "pipe"], windowsHide: true });
    const written: Buffer[] = [];
    let stderr = "";
    child.stdout!.on("data", (chunk: Buffer) => written.push(chunk));
    child.stderr!.on("data", (chunk: Buffer) => (stderr += chunk.toString("utf8")));
    child.stdin?.on("error", error => { if (!options.signal?.aborted) reject(error); });
    const abort = (): void => terminateOwnedChildTree(child);
    options.signal?.addEventListener("abort", abort, { once: true });
    if (options.signal?.aborted) abort();
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
export async function dockerOk(args: readonly string[], options: Parameters<typeof docker>[1] = {}): Promise<string> {
  const answer = await docker(args, options);
  if (answer.code !== 0) throw new Error(`docker ${args.slice(0, 3).join(" ")} exited ${answer.code}: ${(answer.stderr || answer.stdout).split("\n").slice(-6).join(" | ").slice(0, 900)}`);
  return answer.stdout;
}

/** 📜️ The last `lines` lines a container wrote to stdout and stderr. */
export async function containerLog(container: string, lines: number, signal: AbortSignal): Promise<string> {
  const answer = await docker(["logs", "--tail", String(lines), container], { signal }).catch(() => ({ stdout: "", stderr: "" }));
  return [answer.stdout, answer.stderr].filter(Boolean).join("\n");
}

/** 🐋️ The daemon's server version, or an error naming the missing daemon — a precondition, never a quiz fault. */
export async function dockerServerVersion(signal?: AbortSignal): Promise<string> {
  const answer = await docker(["version", "--format", "{{.Server.Version}}"], { signal });
  if (answer.code !== 0 || !answer.stdout) throw new Error(`the Docker daemon does not answer: ${(answer.stderr || answer.stdout).slice(0, 300)}`);
  return answer.stdout;
}

/** 🔖️ The commit a build of this checkout comes from: its hash (suffixed `-dirty` when tracked files differ from it) and
 * its time; `unknown` outside a git checkout. */
export function sourceRevision(repoRoot: string): Readonly<{ revision: string; created: string }> {
  const git = (...args: string[]): string | undefined => {
    const run = runToolSyncV1("git", args, { cwd: repoRoot, encoding: "utf8", windowsHide: true });
    return run.status === 0 ? run.stdout.trim() : undefined;
  };
  const head = git("rev-parse", "HEAD");
  if (!head) return { revision: "unknown", created: "1970-01-01T00:00:00Z" };
  return { revision: git("status", "--porcelain", "--untracked-files=no") ? `${head}-dirty` : head, created: git("log", "-1", "--format=%cI") ?? "1970-01-01T00:00:00Z" };
}

/** ✅️ Validates current material while its genuine native producer and executable remain retained. */
export async function checkCourse(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
  const { signal, release } = interruption(invocation), phase = AbortSignal.any([signal, AbortSignal.timeout(1_800_000)]);
  let artifact: ProctorArtifact | undefined;
  try {
    artifact = await acquireProctor(repoRoot, invocation, process.env, phase, courseOriginalBuildModeV1(invocation, "dev"));
    const status = await artifact.launch(["check", resolve(repoRoot, COURSE)]).exited;
    phase.throwIfAborted();
    if (status !== 0) throw Error(`Authenticated proctor course check exited ${status}`);
  } finally { try { await artifact?.close(); } finally { release(); } }
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

/** 🗺️ The deployment's DNS zone: strip a host label while retaining an apex with two labels. */
export function hostZone(host: string): string {
  const labels = host.split(".");
  return labels.slice(labels.length > 2 ? 1 : 0).join(".");
}

/** 📐️ Every place where a file of the deployment disagrees with its one authored source, `🔣️.json`: a host in the zone
 * of the site host or of the proctor host, or an image repository, other than the declared ones in the Dockerfile, the
 * stack, the workflow, the READMEs or the package; the baked port, origin, account and health probe of the image; the
 * certificates directory the stack mounts and loads; two defaults for the one request body limit of the `Caddyfile`;
 * unpinned base images or workflow actions; a workflow, README or
 * root script naming a target the package does not have; a Pages artifact path other than the staged one. Empty when
 * nothing drifted. */
export function deploymentDrift(repoRoot: string, read: (path: string) => string = path => readFileSync(join(repoRoot, path), "utf8")): string[] {
  const problems: string[] = [];
  const [dockerfile, compose, caddyfile, workflow] = [read(DOCKERFILE), read(`${DEPLOY}/compose.yaml`), read(`${DEPLOY}/Caddyfile`), read(WORKFLOW)];
  const texts: Record<string, string> = { [DOCKERFILE]: dockerfile, [`${DEPLOY}/compose.yaml`]: compose, [`${DEPLOY}/Caddyfile`]: caddyfile, [WORKFLOW]: workflow, [`${SITE}/README.md`]: read(`${SITE}/README.md`), [PROCTOR_README]: read(PROCTOR_README), [`${BUNDLE}/package.json`]: read(`${BUNDLE}/package.json`) };
  const zones = [...new Set([COURSE_SITE_HOST, COURSE_PROCTOR_HOST].map(hostZone))].map((zone) => zone.replace(/\./gu, "\\.")).join("|");
  for (const [path, text] of Object.entries(texts)) {
    for (const host of new Set(text.match(new RegExp(`(?:[a-z0-9_][a-z0-9_-]*\\.)+(?:${zones})`, "gu")) ?? [])) if (host !== COURSE_SITE_HOST && host !== COURSE_PROCTOR_HOST && !host.startsWith("_github-pages-challenge-")) problems.push(`${path} names the host ${host}, which 🔣️.json does not declare`);
    for (const image of new Set(text.match(/ghcr\.io\/[a-z0-9._/-]+/gu) ?? [])) if (image !== COURSE_IMAGE_REPOSITORY) problems.push(`${path} names the image ${image}, not ${COURSE_IMAGE_REPOSITORY}`);
  }
  const expect = (path: string, text: string, fragments: readonly string[]): void => {
    for (const fragment of fragments) if (!text.includes(fragment)) problems.push(`${path} lacks ${JSON.stringify(fragment)}`);
  };
  expect(DOCKERFILE, dockerfile, [`PROCTOR_PORT=${COURSE_IMAGE_PORT}`, `EXPOSE ${COURSE_IMAGE_PORT}`, `PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}`, `PROCTOR_DATA=${COURSE_IMAGE_DATA_ROOT}`, `VOLUME ["${COURSE_IMAGE_DATA_ROOT}"]`, `USER ${COURSE_IMAGE_USER}`, `CMD [${COURSE_IMAGE_HEALTHCHECK.slice(1).map((word) => JSON.stringify(word)).join(", ")}]`, `COPY deploy ${COURSE_IMAGE_STACK_ROOT}`, 'COPY --chmod=0555 proctor /usr/local/bin/proctor', 'RUN ["/usr/local/bin/proctor", "check", "/srv/course/content/🏛️architecture/📚️course/🔣️.json"]']);
  expect(`${DEPLOY}/compose.yaml`, compose, [`image: ${COURSE_IMAGE_REPOSITORY}:\${PROCTOR_TAG:-latest}`, `- "${COURSE_IMAGE_PORT}"`, `\${PROCTOR_ALLOWED_ORIGINS:-${COURSE_SITE_ORIGIN}}`, `\${PROCTOR_HOST:-${COURSE_PROCTOR_HOST}}`, `${COURSE_IMAGE_DATA_ROOT}`, `- ./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`]);
  expect(`${DEPLOY}/Caddyfile`, caddyfile, [`{$PROCTOR_HOST:${COURSE_PROCTOR_HOST}} {`, `reverse_proxy proctor:${COURSE_IMAGE_PORT}`, `load /${COURSE_STACK_CERTIFICATES}`, "respond @oversized 413"]);
  const limits = new Set([...caddyfile.matchAll(/\{\$PROCTOR_LIMIT_BODY_BYTES:([^}\s]+)\}/gu)].map((placeholder) => placeholder[1]!));
  if (limits.size > 1) problems.push(`${DEPLOY}/Caddyfile defaults PROCTOR_LIMIT_BODY_BYTES to ${[...limits].join(" and ")}`);
  expect(WORKFLOW, workflow, [`path: ${BUNDLE}/dist/release/${COURSE_PAGES_DIRECTORY}`]);
  for (const image of [...dockerfile.matchAll(/^ARG \w+_IMAGE=(\S+)$/gmu), ...compose.matchAll(/^\s+image:\s*(\S+)$/gmu)].map((line) => line[1]!)) if (!image.startsWith(COURSE_IMAGE_REPOSITORY) && !/^[\w./-]+:[\w.-]+@sha256:[0-9a-f]{64}$/u.test(image)) problems.push(`the base image ${image} is not pinned to an exact tag and digest`);
  for (const action of new Set([...workflow.matchAll(/^\s*(?:- )?uses:\s*(\S+)/gmu)].map((line) => line[1]!))) if (!action.startsWith("./") && !/@[0-9a-f]{40}$/u.test(action)) problems.push(`${WORKFLOW} uses ${action} without a commit hash`);
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
