/** 🚀️ Operator verbs of `quizze.architektur-und-technologie.de`: the proctor's own catalog check, a cold build of the
 * production image, and a check that runs the built image the way an operator meets it — production posture on a fresh
 * volume, published on host loopback only, answering behind the stated TLS proxy, refusing cleartext without it, serving
 * the site with its SPA fallback, draining on `docker stop` with exit code 0 and leaving the SQLite file on the volume.
 * Every Docker call is one argv without a shell, so the verbs run unchanged on Windows, macOS and Linux; Ctrl-C/SIGTERM
 * stops the running client and removes the container and volume a check created.
 * @see ./Dockerfile — the image
 * @see ./compose.yaml — the service
 * @see ./Caddyfile — the TLS-terminating proxy
 * @see ../README.md — the operator guide
 * https://docs.docker.com/reference/cli/docker/ */
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { resolve } from "node:path";
import { cargoTargetDirectory } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { runOwnedCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🌐️ The public host the image serves. */
export const QUIZ_SITE_HOST = "quizze.architektur-und-technologie.de";

/** 🏷️ The repository every image tag of the site lives under. */
export const QUIZ_IMAGE_REPOSITORY = "semio/architecture-quiz";

/** 🔌️ The port the proctor binds inside the container (`PROCTOR_PORT`). */
export const QUIZ_IMAGE_PORT = 8791;

/** 🗄️ The one writable path of the image (`PROCTOR_DATA`). */
export const QUIZ_IMAGE_DATA_ROOT = "/srv/quiz/data";

/** 🗃️ The SQLite file the proctor keeps in `PROCTOR_DATA`. */
export const QUIZ_IMAGE_DATABASE = "proctor.sqlite";

const DOCKERFILE = "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile";
const CATALOG = "🎓️teaching/🏛️architecture/❓️quiz/🔣️.json";
const SITE_TITLE = "Quizze · Architektur und Technologie";

/** 🏳️ The value following `flag` in `segments`, if any. */
function flag(segments: readonly string[], name: string): string | undefined {
  const index = segments.indexOf(name);
  return index >= 0 ? segments[index + 1] : undefined;
}

/** 🧾️ What one Docker client call answered. */
type DockerAnswer = Readonly<{ code: number; stdout: string; stderr: string }>;

/** 🐳️ Runs one `docker` argv without a shell; an aborted `signal` interrupts the client. */
function docker(args: readonly string[], signal?: AbortSignal): Promise<DockerAnswer> {
  return new Promise((accept, reject) => {
    if (signal?.aborted) return reject(new Error("cancelled"));
    const child = spawn("docker", [...args], { shell: false, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (chunk: Buffer) => (stdout += chunk.toString("utf8")));
    child.stderr.on("data", (chunk: Buffer) => (stderr += chunk.toString("utf8")));
    const abort = (): void => void child.kill("SIGINT");
    signal?.addEventListener("abort", abort, { once: true });
    child.once("error", (error) => reject(/ENOENT/u.test(String(error)) ? new Error("the docker client does not exist on PATH; install Docker Desktop or Docker Engine") : error));
    child.once("close", (code) => {
      signal?.removeEventListener("abort", abort);
      if (signal?.aborted) return reject(new Error("cancelled"));
      accept({ code: code ?? -1, stdout: stdout.trim(), stderr: stderr.trim() });
    });
  });
}

/** 🐳️ Runs one `docker` argv and fails with its stderr unless it exits 0. */
async function dockerOk(args: readonly string[], signal?: AbortSignal): Promise<string> {
  const answer = await docker(args, signal);
  if (answer.code !== 0) throw new Error(`docker ${args[0]} exited ${answer.code}: ${(answer.stderr || answer.stdout).split("\n").slice(-4).join(" | ").slice(0, 600)}`);
  return answer.stdout;
}

/** 📜️ The last `lines` lines a container wrote to stdout and stderr. */
async function containerLog(container: string, lines: number): Promise<string> {
  const answer = await docker(["logs", "--tail", String(lines), container]).catch(() => ({ stdout: "", stderr: "" }));
  return [answer.stdout, answer.stderr].filter(Boolean).join("\n");
}

/** 🩺️ The daemon's server version, or an error naming the missing daemon — a precondition, never a site fault. */
async function dockerServerVersion(signal?: AbortSignal): Promise<string> {
  const answer = await docker(["version", "--format", "{{.Server.Version}}"], signal);
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

/** 🏗️ `docker-image-build [--tag <tag>] [--jobs <n>]` — a cold `docker build` of the site image from the repository root,
 * streaming every BuildKit step; `--jobs` caps the builder's parallel compiler jobs. */
export async function buildQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const jobs = flag(segments, "--jobs");
  console.log(`[docker-image] docker ${await dockerServerVersion()}; building ${QUIZ_IMAGE_REPOSITORY}:${tag}`);
  await runOwnedCommand("docker", ["build", "--progress=plain", ...(jobs ? ["--build-arg", `CARGO_BUILD_JOBS=${jobs}`] : []), "--file", DOCKERFILE, "--tag", `${QUIZ_IMAGE_REPOSITORY}:${tag}`, "."], repoRoot, "architecture-quiz:docker-image-build");
  console.log(`[docker-image] ${QUIZ_IMAGE_REPOSITORY}:${tag} ${await dockerOk(["image", "inspect", "--format", "{{.Id}} {{.Size}} bytes", `${QUIZ_IMAGE_REPOSITORY}:${tag}`])}`);
}

/** 🌐️ One HTTP answer of the running image. */
type Probe = Readonly<{ status: number; type: string; body: string }>;

/** 🌐️ `GET path` on the published port, stating the TLS proxy in front unless `cleartext`. */
async function probe(port: number, path: string, cleartext: boolean, signal: AbortSignal): Promise<Probe> {
  const response = await fetch(`http://127.0.0.1:${port}${path}`, { headers: cleartext ? {} : { "x-forwarded-proto": "https", "x-forwarded-host": QUIZ_SITE_HOST }, signal: AbortSignal.any([signal, AbortSignal.timeout(10_000)]) });
  return { status: response.status, type: response.headers.get("content-type") ?? "", body: await response.text() };
}

/** 🩺️ `docker-image-check [--tag <tag>] [--port <n>] [--ready-timeout-ms <n>] [--keep]` — runs the built image in its
 * production posture on a fresh volume and fails on the first expectation it misses; the container and volume are removed
 * afterwards unless `--keep`. */
export async function checkQuizImage(repoRoot: string, segments: readonly string[]): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const port = Number(flag(segments, "--port") ?? "18791");
  const readyTimeoutMs = Number(flag(segments, "--ready-timeout-ms") ?? "60000");
  const keep = segments.includes("--keep");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const say = (line: string): void => console.log(`[docker-image] ${line}`);
  const signal = controller.signal;
  const run = `${Date.now().toString(36)}-${randomBytes(3).toString("hex")}`;
  const image = `${QUIZ_IMAGE_REPOSITORY}:${tag}`;
  const container = `architecture-quiz-${run}`;
  const volume = `architecture-quiz-${run}-data`;
  let created = false;
  let running = false;
  try {
    say(`docker ${await dockerServerVersion(signal)} in ${repoRoot}`);
    if ((await docker(["image", "inspect", image], signal)).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    await dockerOk(["volume", "create", volume], signal);
    created = true;
    const started = Date.now();
    await dockerOk(["run", "--detach", "--name", container, "--publish", `127.0.0.1:${port}:${QUIZ_IMAGE_PORT}`, "--volume", `${volume}:${QUIZ_IMAGE_DATA_ROOT}`, "--env", `PROCTOR_ALLOWED_ORIGINS=https://${QUIZ_SITE_HOST}`, "--env", "PROCTOR_TRUSTED_FORWARDING=proxy", "--stop-timeout", "30", image], signal);
    running = true;
    say(`container ${container} on 127.0.0.1:${port}, volume ${volume}`);
    let ready: Probe | undefined;
    while (!ready && Date.now() - started < readyTimeoutMs) {
      const state = (await docker(["inspect", "--format", "{{.State.Status}}", container], signal)).stdout;
      if (state !== "running") throw new Error(`container ${state || "vanished"} before ready: ${(await containerLog(container, 20)).slice(-1200)}`);
      const answer = await probe(port, "/instance", false, signal).catch(() => undefined);
      if (answer?.status === 200) ready = answer;
      else await new Promise((wait) => setTimeout(wait, 1_000));
    }
    if (!ready) throw new Error(`not ready within ${readyTimeoutMs} ms`);
    say(`/instance ready after ${Date.now() - started} ms: ${ready.body.slice(0, 160)}`);
    const site = await probe(port, "/", false, signal);
    if (site.status !== 200 || !site.body.includes(`<title>${SITE_TITLE}</title>`)) throw new Error(`/ answered ${site.status} ${site.type} without the site title`);
    const deep = await probe(port, "/leaderboard", false, signal);
    if (deep.status !== 200 || !deep.body.includes(`<title>${SITE_TITLE}</title>`)) throw new Error(`/leaderboard answered ${deep.status} ${deep.type} instead of the SPA fallback`);
    say(`site: / ${site.status} ${site.type}, /leaderboard ${deep.status} (SPA fallback)`);
    const cleartext = await probe(port, "/instance", true, signal);
    if (cleartext.status < 400) throw new Error(`/instance without the proxy's statement answered ${cleartext.status}; cleartext must be refused`);
    say(`cleartext refused: ${cleartext.status} ${cleartext.body.slice(0, 120)}`);
    const stopping = Date.now();
    await dockerOk(["stop", "--time", "30", container], signal);
    running = false;
    const exitCode = Number(await dockerOk(["inspect", "--format", "{{.State.ExitCode}}", container], signal));
    if (exitCode !== 0) throw new Error(`docker stop drained with exit code ${exitCode}`);
    say(`docker stop drained in ${Date.now() - stopping} ms, exit code ${exitCode}`);
    const files = await dockerOk(["run", "--rm", "--volume", `${volume}:${QUIZ_IMAGE_DATA_ROOT}`, "--entrypoint", "/bin/ls", image, QUIZ_IMAGE_DATA_ROOT], signal);
    if (!files.split("\n").includes(QUIZ_IMAGE_DATABASE)) throw new Error(`${QUIZ_IMAGE_DATA_ROOT} holds ${JSON.stringify(files)} without ${QUIZ_IMAGE_DATABASE}`);
    say(`volume keeps ${files.split("\n").join(", ")}`);
    say(`${image} passed`);
  } catch (error) {
    if (created) console.error(`[docker-image] container log tail:\n${await containerLog(container, 30)}`);
    throw error;
  } finally {
    process.off("SIGINT", cancel);
    process.off("SIGTERM", cancel);
    if (running) await docker(["stop", "--time", "30", container]).catch(() => undefined);
    if (created && !keep) {
      await docker(["rm", "--force", container]).catch(() => undefined);
      await docker(["volume", "rm", "--force", volume]).catch(() => undefined);
    }
  }
}
