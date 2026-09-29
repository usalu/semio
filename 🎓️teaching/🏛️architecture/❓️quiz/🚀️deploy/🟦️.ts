/** 🚀️ Operator verbs of the architecture quiz: the site goes to a CDN at the site host, the proctor runs as a zero-touch
 * Docker stack (proctor + Caddy) at the proctor host, both hosts from `🔣️.json`.
 *
 * - `publish` builds the site with the proctor origin baked in, verifies the CDN artifact and stages it with `_headers`.
 * - `docker-image-build` / `docker-image-publish` build the proctor image and push it to the registry named in `🔣️.json`
 *   with the developer's own `docker login`.
 * - `docker-image-check` runs the image with no configuration at all (its baked production defaults) and checks the API as
 *   the CDN-hosted site meets it: ready behind the proxy, cleartext refused, cross-origin preflight and POST granted to the
 *   site origin only, shared presence between two learners (a foreign origin refused), `docker stop` drains with exit code
 *   0, `proctor.sqlite` on the volume.
 * - `docker-stack-check` proves `compose.yaml` and the `Caddyfile`: config, `caddy validate`, `up --wait` on a throw-away
 *   project with Caddy's internal CA on `localhost`, the API and the presence WebSocket over HTTPS through Caddy, then
 *   `down --volumes`.
 *
 * Every Docker call is one argv without a shell, so the verbs run unchanged on Windows, macOS and Linux; Ctrl-C/SIGTERM
 * stops the running client and removes what a check created.
 * @see ./🔣️.json — the site and proctor hosts, the image repository and port
 * @see ./Dockerfile — the proctor image
 * @see ./compose.yaml — the stack
 * @see ./Caddyfile — the TLS-terminating proxy
 * @see ../README.md — the operator guide
 * https://docs.docker.com/reference/cli/docker/ */
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { copyFileSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { cargoTargetDirectory } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { workspaceCargoVersion } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { runOwnedCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { PRESENCE_PROTOCOL, decodePresenceFrame, encodePresenceFrame, presenceSocketUrl, type PresenceFrame } from "../../../../🧰️framework/🛍️products/🖥️server/🟦️.ts";
import deployment from "./🔣️.json" with { type: "json" };

/** 🌐️ The host the CDN serves the site at. */
export const QUIZ_SITE_HOST = deployment.site.host;

/** 🌐️ The site origin: the one browser origin the proctor grants. */
export const QUIZ_SITE_ORIGIN = `https://${QUIZ_SITE_HOST}`;

/** 🛂️ The host the Docker stack serves the proctor at. */
export const QUIZ_PROCTOR_HOST = deployment.proctor.host;

/** 🛂️ The proctor origin a release build of the site talks to. */
export const QUIZ_PROCTOR_ORIGIN = `https://${QUIZ_PROCTOR_HOST}`;

/** 🏷️ The registry repository every proctor image tag lives under. */
export const QUIZ_IMAGE_REPOSITORY = deployment.proctor.image;

/** 🔌️ The port the proctor binds inside the container (`PROCTOR_PORT`). */
export const QUIZ_IMAGE_PORT = deployment.proctor.port;

/** 🗄️ The one writable path of the image (`PROCTOR_DATA`). */
export const QUIZ_IMAGE_DATA_ROOT = "/srv/quiz/data";

/** 🗃️ The SQLite file the proctor keeps in `PROCTOR_DATA`. */
export const QUIZ_IMAGE_DATABASE = "proctor.sqlite";

/** 📦️ Where `publish` stages the CDN artifact, relative to the package's `dist`. */
export const QUIZ_PAGES_DIRECTORY = "pages/quizzes";

/** 🧾️ `_headers` for CDNs that honour it (Cloudflare Pages, Netlify): hashed assets immutable, documents revalidated. The
 * rules never overlap, because those CDNs join the values of every matching rule. GitHub Pages ignores the file. */
export const QUIZ_SITE_HEADERS = ["/assets/*", "  Cache-Control: public, max-age=31536000, immutable", ...["/", "/index.html", "/404.html"].flatMap((path) => [path, "  Cache-Control: no-cache"])].join("\n") + "\n";

const DEPLOY = "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy";
const DOCKERFILE = `${DEPLOY}/Dockerfile`;
const CATALOG = "🎓️teaching/🏛️architecture/❓️quiz/🔣️.json";
const FOREIGN_ORIGIN = "https://foreign.example";

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

/** 🧾️ What one Docker client call answered. */
type DockerAnswer = Readonly<{ code: number; stdout: string; stderr: string }>;

/** 🐳️ Runs one `docker` argv without a shell, with `env` merged over the process environment and `input` on stdin; an
 * aborted `signal` interrupts the client. */
function docker(args: readonly string[], options: Readonly<{ signal?: AbortSignal; env?: NodeJS.ProcessEnv; input?: string; cwd?: string }> = {}): Promise<DockerAnswer> {
  return new Promise((accept, reject) => {
    if (options.signal?.aborted) return reject(new Error("cancelled"));
    const child = spawn("docker", [...args], { shell: false, cwd: options.cwd, env: { ...process.env, ...options.env }, stdio: [options.input === undefined ? "ignore" : "pipe", "pipe", "pipe"], windowsHide: true });
    let stdout = "";
    let stderr = "";
    child.stdout!.on("data", (chunk: Buffer) => (stdout += chunk.toString("utf8")));
    child.stderr!.on("data", (chunk: Buffer) => (stderr += chunk.toString("utf8")));
    const abort = (): void => void child.kill("SIGINT");
    options.signal?.addEventListener("abort", abort, { once: true });
    child.once("error", (error) => reject(/ENOENT/u.test(String(error)) ? new Error("the docker client does not exist on PATH; install Docker Desktop or Docker Engine") : error));
    child.once("close", (code) => {
      options.signal?.removeEventListener("abort", abort);
      if (options.signal?.aborted) return reject(new Error("cancelled"));
      accept({ code: code ?? -1, stdout: stdout.trim(), stderr: stderr.trim() });
    });
    if (options.input !== undefined) child.stdin!.end(options.input);
  });
}

/** 🐳️ Runs one `docker` argv and fails with its output unless it exits 0. */
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

/** 🩺️ The daemon's server version, or an error naming the missing daemon — a precondition, never a quiz fault. */
async function dockerServerVersion(signal?: AbortSignal): Promise<string> {
  const answer = await docker(["version", "--format", "{{.Server.Version}}"], { signal });
  if (answer.code !== 0 || !answer.stdout) throw new Error(`the Docker daemon does not answer: ${(answer.stderr || answer.stdout).slice(0, 300)}`);
  return answer.stdout;
}

/** ✅️ `proctor check <catalog>` over the site's catalog with the release binary the image ships: the Rust core validates the
 * catalog and every quiz it names and exits non-zero with the issues. The release profile and a private target directory keep
 * the check off the debug executable a running dev proctor holds open (Windows refuses to replace a running image, and
 * uplifted binaries are hard links into the shared build directory). */
export async function checkQuizCatalog(repoRoot: string): Promise<void> {
  const env = { ...process.env, CARGO_TARGET_DIR: `${cargoTargetDirectory(repoRoot)}-architecture-quiz` };
  await runOwnedCommand("cargo", ["run", "--release", "--quiet", "--package", "teaching-proctor", "--bin", "proctor", "--", "check", resolve(repoRoot, CATALOG)], repoRoot, "architecture-quiz:check", undefined, { env });
}

//#region 🌐️Site
/** 🧭️ The proctor origin a release build bakes in: `PROCTOR_URL`, else the proctor origin of `🔣️.json`. */
export function releaseProctorOrigin(env: NodeJS.ProcessEnv = process.env): string {
  return env.PROCTOR_URL ?? QUIZ_PROCTOR_ORIGIN;
}

/** 📂️ Every file below `root`, as `/`-separated relative paths, skipping the directories named in `skip`. */
function siteFiles(root: string, skip: ReadonlySet<string> = new Set(), prefix = ""): string[] {
  return readdirSync(join(root, prefix), { withFileTypes: true }).flatMap((entry) => {
    const path = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) return skip.has(path) ? [] : siteFiles(root, skip, path);
    return entry.isFile() ? [path] : [];
  });
}

/** 🔎️ Every reason `dist` is not the CDN artifact of a release build baked for `proctorOrigin`: the host entry points and
 * markers, `CNAME` naming the site host, the proctor origin in the code, and no loopback address outside that origin. */
export function siteArtifactProblems(dist: string, proctorOrigin: string): string[] {
  const problems: string[] = [];
  const files = siteFiles(dist, new Set(["pages"]));
  for (const required of ["index.html", "404.html", ".nojekyll", "CNAME"]) if (!files.includes(required)) problems.push(`${required} is missing`);
  if (files.includes("CNAME") && readFileSync(join(dist, "CNAME"), "utf8").trim() !== QUIZ_SITE_HOST) problems.push(`CNAME names ${JSON.stringify(readFileSync(join(dist, "CNAME"), "utf8").trim())}, not ${QUIZ_SITE_HOST}`);
  const code = files.filter((path) => /\.(?:html|js|mjs|css)$/u.test(path)).map((path) => readFileSync(join(dist, path), "utf8"));
  if (!code.some((text) => text.includes(JSON.stringify(proctorOrigin)))) problems.push(`no script bakes the proctor origin ${proctorOrigin}`);
  for (const loopback of ["localhost", "127.0.0.1", "[::1]"]) if (code.some((text) => text.split(proctorOrigin).join("").includes(loopback))) problems.push(`the site names ${loopback} outside the baked proctor origin`);
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
  const files = siteFiles(dist, new Set(["pages"]));
  for (const path of files) {
    mkdirSync(dirname(join(pages, path)), { recursive: true });
    copyFileSync(join(dist, path), join(pages, path));
  }
  writeFileSync(join(pages, "_headers"), QUIZ_SITE_HEADERS);
  say(`staged ${files.length + 1} files in ${pages}`);
  if (proctorOrigin !== QUIZ_PROCTOR_ORIGIN) say(`REHEARSAL artifact: it talks to ${proctorOrigin}, not ${QUIZ_PROCTOR_ORIGIN}; do not upload it`);
}
//#endregion 🌐️Site

//#region 🐳️Image
/** 🏗️ `docker-image-build [--tag <tag>] [--jobs <n>]` — a cold `docker build` of the proctor image from the repository root,
 * streaming every BuildKit step; `--jobs` caps the builder's parallel compiler jobs. */
export async function buildQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const jobs = flag(segments, "--jobs");
  say(`docker ${await dockerServerVersion()}; building ${QUIZ_IMAGE_REPOSITORY}:${tag}`);
  await runOwnedCommand("docker", ["build", "--progress=plain", ...(jobs ? ["--build-arg", `CARGO_BUILD_JOBS=${jobs}`] : []), "--file", DOCKERFILE, "--tag", `${QUIZ_IMAGE_REPOSITORY}:${tag}`, "."], repoRoot, "architecture-quiz:docker-image-build");
  say(`${QUIZ_IMAGE_REPOSITORY}:${tag} ${await dockerOk(["image", "inspect", "--format", "{{.Id}} {{.Size}} bytes", `${QUIZ_IMAGE_REPOSITORY}:${tag}`])}`);
}

/** 🚚️ `docker-image-publish [--tag <tag>]` — tags the built image (default `latest`) with the workspace version and pushes
 * both tags to the registry of `🔣️.json`, with the credentials of the developer's own `docker login` (or CI's). */
export async function publishQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const source = `${QUIZ_IMAGE_REPOSITORY}:${flag(segments, "--tag") ?? "latest"}`;
  const versioned = `${QUIZ_IMAGE_REPOSITORY}:${workspaceCargoVersion(repoRoot)}`;
  say(`docker ${await dockerServerVersion()}; publishing ${source}`);
  if ((await docker(["image", "inspect", source])).code !== 0) throw new Error(`image ${source} does not exist; build it first with docker-image-build`);
  await dockerOk(["tag", source, versioned]);
  for (const reference of [versioned, `${QUIZ_IMAGE_REPOSITORY}:latest`]) {
    if (reference !== source) await dockerOk(["tag", source, reference]);
    const pushed = await docker(["push", reference]);
    if (pushed.code !== 0) throw new Error(`docker push ${reference} exited ${pushed.code}${/denied|unauthorized|authentication/iu.test(pushed.stderr) ? `; log in first: docker login ${QUIZ_IMAGE_REPOSITORY.split("/")[0]}` : ""}: ${pushed.stderr.slice(-600)}`);
    say(`pushed ${reference}`);
  }
}

/** 🌐️ One HTTP answer of the proctor. */
type Probe = Readonly<{ status: number; headers: Headers; body: string }>;

/** 🌐️ One request to `url`; TLS verification is off only for the throw-away stack's internal CA. */
async function probe(url: string, init: Readonly<{ method?: string; headers?: Record<string, string>; body?: string; insecure?: boolean }>, signal: AbortSignal): Promise<Probe> {
  const response = await fetch(url, { method: init.method ?? "GET", headers: init.headers, body: init.body, signal: AbortSignal.any([signal, AbortSignal.timeout(10_000)]), ...(init.insecure ? { tls: { rejectUnauthorized: false } } : {}) } as RequestInit);
  return { status: response.status, headers: response.headers, body: await response.text() };
}

/** 🧾️ A catalog query envelope as the site sends it (§9a). */
function catalogQuery(): string {
  const query = JSON.stringify({ type: "catalog" });
  return JSON.stringify({ queryId: randomBytes(16).toString("hex"), kind: "quiz.catalog", version: 1, scope: "architecture", principal: { kind: "anonymous" }, arguments: [...new TextEncoder().encode(query)], consistency: { kind: "authority" }, cursor: null });
}

/** 🤝️ The cross-origin contract of the CDN-hosted site against the proctor at `base`: preflights of both POST routes are
 * granted to the site origin with a cache lifetime, a foreign origin gets no grant, and a real catalog query answers with
 * the grant. `forwarded` adds the proxy's statement for a direct container port. */
async function checkCrossOrigin(base: string, forwarded: Record<string, string>, insecure: boolean, signal: AbortSignal): Promise<void> {
  for (const route of ["/commands", "/queries"]) {
    const preflight = await probe(`${base}${route}`, { method: "OPTIONS", headers: { ...forwarded, origin: QUIZ_SITE_ORIGIN, "access-control-request-method": "POST", "access-control-request-headers": "content-type" }, insecure }, signal);
    const origin = preflight.headers.get("access-control-allow-origin");
    const maxAge = Number(preflight.headers.get("access-control-max-age") ?? "0");
    if (preflight.status !== 204 || origin !== QUIZ_SITE_ORIGIN || !/content-type/iu.test(preflight.headers.get("access-control-allow-headers") ?? "") || !(maxAge > 0)) throw new Error(`OPTIONS ${route} from ${QUIZ_SITE_ORIGIN} answered ${preflight.status}, allow-origin ${origin}, allow-headers ${preflight.headers.get("access-control-allow-headers")}, max-age ${preflight.headers.get("access-control-max-age")}`);
    say(`preflight ${route}: 204, allow-origin ${origin}, max-age ${maxAge}`);
  }
  const foreign = await probe(`${base}/queries`, { method: "OPTIONS", headers: { ...forwarded, origin: FOREIGN_ORIGIN, "access-control-request-method": "POST" }, insecure }, signal);
  if (foreign.headers.get("access-control-allow-origin")) throw new Error(`a preflight from ${FOREIGN_ORIGIN} was granted ${foreign.headers.get("access-control-allow-origin")}`);
  say(`preflight from ${FOREIGN_ORIGIN}: ${foreign.status} without a grant`);
  const query = await probe(`${base}/queries`, { method: "POST", headers: { ...forwarded, origin: QUIZ_SITE_ORIGIN, "content-type": "application/json" }, body: catalogQuery(), insecure }, signal);
  if (query.status !== 200 || query.headers.get("access-control-allow-origin") !== QUIZ_SITE_ORIGIN || !query.body.includes("\"snapshot\"")) throw new Error(`POST /queries from ${QUIZ_SITE_ORIGIN} answered ${query.status}, allow-origin ${query.headers.get("access-control-allow-origin")}: ${query.body.slice(0, 300)}`);
  say(`POST /queries (catalog) from ${QUIZ_SITE_ORIGIN}: 200 snapshot with the grant`);
}

/** 👥️ One presence socket of a check: opens with the subprotocol and `headers`, and keeps every decoded frame. */
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
  for (const started = Date.now(); Date.now() - started < 10_000; await new Promise((wait) => setTimeout(wait, 50))) {
    if (signal.aborted) throw new Error("cancelled");
    const found = frames.find(wanted);
    if (found) return found;
  }
  throw new Error(`no presence frame: ${label}; got ${JSON.stringify(frames).slice(0, 600)}`);
}

/** 👥️ Shared presence as two CDN-hosted learners meet it on the catalog's roster room: both join with the site origin, one
 * shares a `PresenceState` and the other receives it in a batch, then its departure; a foreign origin never opens. */
async function checkPresence(base: string, forwarded: Record<string, string>, insecure: boolean, signal: AbortSignal): Promise<void> {
  const url = presenceSocketUrl(base, "architecture", "check");
  const first = presenceSocket(url, { ...forwarded, origin: QUIZ_SITE_ORIGIN }, insecure);
  const ada = await first.opened;
  const welcome = await presenceFrame(first.frames, (frame) => frame.type === "welcome", "welcome of the first learner", signal);
  if (ada.protocol !== PRESENCE_PROTOCOL || welcome.type !== "welcome") throw new Error(`the presence socket spoke ${JSON.stringify(ada.protocol)}`);
  const second = presenceSocket(url, { ...forwarded, origin: QUIZ_SITE_ORIGIN }, insecure);
  const grace = await second.opened;
  const roster = await presenceFrame(second.frames, (frame) => frame.type === "welcome", "welcome of the second learner", signal);
  if (roster.type !== "welcome" || !roster.roster.some((entry) => entry.session === welcome.session)) throw new Error(`the second learner's roster lacks the first: ${JSON.stringify(roster)}`);
  const state = { tag: "0a1b2c3d", identity: { kind: "anonymous" }, place: { screen: "home" }, active: true };
  ada.send(JSON.stringify(encodePresenceFrame({ type: "state", state })));
  await presenceFrame(second.frames, (frame) => frame.type === "batch" && frame.entries.some((entry) => entry.session === welcome.session && Bun.deepEquals(entry.state, state)), "the first learner's state in a batch", signal);
  ada.close();
  await presenceFrame(second.frames, (frame) => frame.type === "batch" && frame.left.includes(welcome.session), "the first learner leaving", signal);
  grace.close();
  say(`presence ${url.split("?")[0]}: two learners joined (colours ${welcome.colour}, ${roster.colour}), a shared state and a departure arrived in batches`);
  const foreign = await presenceSocket(url, { ...forwarded, origin: FOREIGN_ORIGIN }, insecure).opened.then((socket) => { socket.close(); return "opened"; }, (error: Error) => error.message);
  if (foreign === "opened") throw new Error(`a presence socket from ${FOREIGN_ORIGIN} was opened`);
  say(`presence from ${FOREIGN_ORIGIN}: refused (${foreign})`);
}

/** 🩺️ `docker-image-check [--tag <tag>] [--port <n>] [--ready-timeout-ms <n>] [--keep]` — runs the image with nothing but a
 * fresh volume and a loopback port (the baked production defaults are the whole configuration) and fails on the first
 * expectation it misses; the container and volume are removed afterwards unless `--keep`. */
export async function checkQuizImage(segments: readonly string[]): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const port = Number(flag(segments, "--port") ?? "18791");
  const readyTimeoutMs = Number(flag(segments, "--ready-timeout-ms") ?? "60000");
  const keep = segments.includes("--keep");
  const { signal, release } = interruption();
  const run = `${Date.now().toString(36)}-${randomBytes(3).toString("hex")}`;
  const image = `${QUIZ_IMAGE_REPOSITORY}:${tag}`;
  const container = `architecture-quiz-proctor-${run}`;
  const volume = `architecture-quiz-proctor-${run}-data`;
  const base = `http://127.0.0.1:${port}`;
  const forwarded = { "x-forwarded-proto": "https", "x-forwarded-host": QUIZ_PROCTOR_HOST };
  let created = false;
  let running = false;
  try {
    say(`docker ${await dockerServerVersion(signal)}`);
    if ((await docker(["image", "inspect", image], { signal })).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    await dockerOk(["volume", "create", volume], { signal });
    created = true;
    const started = Date.now();
    await dockerOk(["run", "--detach", "--name", container, "--publish", `127.0.0.1:${port}:${QUIZ_IMAGE_PORT}`, "--volume", `${volume}:${QUIZ_IMAGE_DATA_ROOT}`, "--stop-timeout", "30", image], { signal });
    running = true;
    say(`container ${container} (no configuration) on ${base}, volume ${volume}`);
    let ready: Probe | undefined;
    while (!ready && Date.now() - started < readyTimeoutMs) {
      const state = (await docker(["inspect", "--format", "{{.State.Status}}", container], { signal })).stdout;
      if (state !== "running") throw new Error(`container ${state || "vanished"} before ready: ${(await containerLog(container, 20)).slice(-1200)}`);
      const answer = await probe(`${base}/instance`, { headers: forwarded }, signal).catch(() => undefined);
      if (answer?.status === 200) ready = answer;
      else await new Promise((wait) => setTimeout(wait, 1_000));
    }
    if (!ready) throw new Error(`not ready within ${readyTimeoutMs} ms`);
    say(`/instance ready after ${Date.now() - started} ms: ${ready.body.slice(0, 120)}`);
    const cleartext = await probe(`${base}/instance`, {}, signal);
    if (cleartext.status !== 403 || cleartext.headers.get("x-semio-refusal") !== "insecure-transport") throw new Error(`/instance without the proxy's statement answered ${cleartext.status} ${cleartext.headers.get("x-semio-refusal")}; cleartext must be refused`);
    say("cleartext refused: 403 insecure-transport");
    await checkCrossOrigin(base, forwarded, false, signal);
    await checkPresence(base, forwarded, false, signal);
    let health = "";
    const waiting = Date.now();
    while (health !== "healthy" && Date.now() - waiting < 60_000) {
      health = (await docker(["inspect", "--format", "{{.State.Health.Status}}", container], { signal })).stdout;
      if (health !== "healthy") await new Promise((wait) => setTimeout(wait, 1_000));
    }
    if (health !== "healthy") throw new Error(`the image HEALTHCHECK reports ${health || "nothing"} after 60 s`);
    say(`HEALTHCHECK: healthy after ${Date.now() - started} ms`);
    const stopping = Date.now();
    await dockerOk(["stop", "--time", "30", container], { signal });
    running = false;
    const exitCode = Number(await dockerOk(["inspect", "--format", "{{.State.ExitCode}}", container], { signal }));
    if (exitCode !== 0) throw new Error(`docker stop drained with exit code ${exitCode}`);
    say(`docker stop drained in ${Date.now() - stopping} ms, exit code 0`);
    const files = await dockerOk(["run", "--rm", "--volume", `${volume}:${QUIZ_IMAGE_DATA_ROOT}`, "--entrypoint", "/bin/ls", image, QUIZ_IMAGE_DATA_ROOT], { signal });
    if (!files.split("\n").includes(QUIZ_IMAGE_DATABASE)) throw new Error(`${QUIZ_IMAGE_DATA_ROOT} holds ${JSON.stringify(files)} without ${QUIZ_IMAGE_DATABASE}`);
    say(`volume keeps ${files.split("\n").join(", ")}; ${image} passed`);
  } catch (error) {
    if (created) console.error(`[deploy] container log tail:\n${await containerLog(container, 30)}`);
    throw error;
  } finally {
    release();
    if (running) await docker(["stop", "--time", "30", container]).catch(() => undefined);
    if (created && !keep) {
      await docker(["rm", "--force", container]).catch(() => undefined);
      await docker(["volume", "rm", "--force", volume]).catch(() => undefined);
    }
  }
}
//#endregion 🐳️Image

//#region 🧱️Stack
/** 🧱️ `docker-stack-check [--tag <tag>] [--http-port <n>] [--https-port <n>] [--keep]` — the stack as an operator starts
 * it: `docker compose config`, `caddy validate` in the Caddy image, `up --wait` on a throw-away project whose proctor host
 * is `localhost` (Caddy's internal CA, no public certificate), the API over HTTPS through Caddy (plain HTTP redirects),
 * the cross-origin contract and shared presence (WebSocket) through Caddy, then `down --volumes` unless `--keep`. */
export async function checkQuizStack(repoRoot: string, segments: readonly string[]): Promise<void> {
  const httpPort = flag(segments, "--http-port") ?? "18080";
  const httpsPort = flag(segments, "--https-port") ?? "18443";
  const keep = segments.includes("--keep");
  const { signal, release } = interruption();
  const directory = join(repoRoot, DEPLOY);
  const project = `architecture-quiz-check-${Date.now().toString(36)}`;
  const env = { PROCTOR_HOST: "localhost", PROCTOR_TAG: flag(segments, "--tag") ?? "latest", QUIZ_HTTP_PORT: httpPort, QUIZ_HTTPS_PORT: httpsPort };
  const compose = (args: readonly string[]): Promise<string> => dockerOk(["compose", "--project-name", project, "--file", "compose.yaml", ...args], { signal, env, cwd: directory });
  let started = false;
  try {
    say(`docker ${await dockerServerVersion(signal)}; project ${project}`);
    const config = await compose(["config"]);
    if (!config.includes("proctor") || !config.includes("caddy")) throw new Error("docker compose config lacks the proctor or the caddy service");
    say("docker compose config resolves");
    const caddyImage = /image:\s*(caddy:\S+)/u.exec(config)?.[1] ?? "caddy:2";
    const validated = await docker(["run", "--rm", "--interactive", "--env", "PROCTOR_HOST=localhost", "--entrypoint", "/bin/sh", caddyImage, "-c", "cat > /tmp/Caddyfile && caddy validate --config /tmp/Caddyfile --adapter caddyfile"], { signal, input: readFileSync(join(directory, "Caddyfile"), "utf8") });
    if (validated.code !== 0) throw new Error(`caddy validate failed: ${(validated.stderr || validated.stdout).slice(-900)}`);
    say(`caddy validate (${caddyImage}): ${validated.stderr.split("\n").filter((line) => /Valid configuration/u.test(line)).join(" ") || "valid"}`);
    started = true;
    const up = Date.now();
    await compose(["up", "--detach", "--wait", "--wait-timeout", "180"]);
    say(`up --wait healthy after ${Date.now() - up} ms`);
    const base = `https://localhost:${httpsPort}`;
    const instance = await probe(`${base}/instance`, { insecure: true }, signal);
    if (instance.status !== 200 || !instance.body.includes("teaching-proctor")) throw new Error(`GET ${base}/instance through Caddy answered ${instance.status}: ${instance.body.slice(0, 300)}`);
    say(`GET ${base}/instance through Caddy: 200, strict-transport-security ${instance.headers.get("strict-transport-security")}`);
    const redirect = await fetch(`http://localhost:${httpPort}/instance`, { redirect: "manual", signal: AbortSignal.any([signal, AbortSignal.timeout(10_000)]) });
    if (redirect.status < 300 || redirect.status >= 400) throw new Error(`plain HTTP answered ${redirect.status} instead of redirecting to HTTPS`);
    say(`plain HTTP: ${redirect.status} → ${redirect.headers.get("location")}`);
    await checkCrossOrigin(base, {}, true, signal);
    await checkPresence(base, {}, true, signal);
    say("stack passed");
  } catch (error) {
    if (started) console.error(`[deploy] stack logs:\n${(await docker(["compose", "--project-name", project, "--file", "compose.yaml", "logs", "--tail", "30"], { env, cwd: directory }).catch(() => ({ stdout: "", stderr: "" }))).stdout.slice(-3000)}`);
    throw error;
  } finally {
    release();
    if (started && !keep) await docker(["compose", "--project-name", project, "--file", "compose.yaml", "down", "--volumes", "--timeout", "30"], { env, cwd: directory }).catch(() => undefined);
  }
}
//#endregion 🧱️Stack
