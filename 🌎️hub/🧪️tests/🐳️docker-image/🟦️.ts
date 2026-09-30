/** 🐳️ The hub's container image, proven the way an operator meets it: a cold `docker build` of `🌎️hub/Dockerfile` from the
 * repository root, then the built image run in its production posture (network bind inside the container, credential
 * sign-in, origin allowlist, `OS_HUB_TRUSTED_FORWARDING=proxy`) on a fresh named volume seeded with a copy of a published
 * trusted catalog and one credential, reached through the loopback forwarding proxy, answering `/readyz` and `/healthz`,
 * refusing cleartext without the proxy's statement, carrying a two-client document relay (two independent sessions,
 * each client's edit reaching the other's socket), and draining on `docker stop` with exit code 0.
 * Every Docker call is one argv (no shell) so the harness runs unchanged on macOS, Linux and Windows; nothing is
 * bind-mounted (the catalog and the credential cross into the volume through `docker cp` and stdin), so a Docker Desktop
 * that may not read the repository's folder still runs it. Cancellation (Ctrl-C/SIGTERM) stops the running Docker
 * client and removes the container and volume the run created.
 * https://docs.docker.com/reference/cli/docker/ */
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { terminateOwnedChildTree } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { hubForwardingProxy, hubSeedTrustedCatalog } from "../../🤝️integration-harness/🟦️.ts";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenDocument, hubProbeSignIn } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️testkit/📡️client-probe/🟦️.ts";

/** 🏷️ The image repository every tag of this harness lives under. */
export const HUB_IMAGE_REPOSITORY = "semio/os-hub";

/** 🗄️ The one writable path of the image (`OS_HUB_DATA`), where the named volume is mounted. */
export const HUB_IMAGE_DATA_ROOT = "/srv/semio-hub/data";

/** 🌐️ The port the image's hub binds inside the container (`OS_HUB_PORT`). */
export const HUB_IMAGE_PORT = 8787;

/** 🧾️ Every name one run owns, derived from its run id so concurrent runs never collide. */
export type HubImageRunNames = Readonly<{ image: string; container: string; seed: string; volume: string }>;

/** 🧾️ The names of one check run of `tag`. */
export function hubImageRunNames(tag: string, runId: string): HubImageRunNames {
  return { image: `${HUB_IMAGE_REPOSITORY}:${tag}`, container: `semio-hub-image-${runId}`, seed: `semio-hub-image-${runId}-seed`, volume: `semio-hub-image-${runId}-data` };
}

/** 🏗️ `docker build` of `🌎️hub/Dockerfile` with the repository root as context, optionally capping the builder's compiler
 * jobs (a Docker VM that shares its memory: a release build of the hub's codec crates needs ~2.5 GiB per job). */
export function hubImageBuildArguments(input: Readonly<{ tag: string; jobs: number | null }>): string[] {
  return ["build", "--progress=plain", ...(input.jobs ? ["--build-arg", `CARGO_BUILD_JOBS=${input.jobs}`] : []), "--file", "🌎️hub/Dockerfile", "--tag", `${HUB_IMAGE_REPOSITORY}:${input.tag}`, "."];
}

/** 🌱️ The stopped helper container that owns the fresh volume while the catalog is copied in, then hands it to `semio`. */
export function hubImageSeedArguments(names: HubImageRunNames): string[] {
  return ["create", "--name", names.seed, "--user", "root", "--entrypoint", "/bin/sh", "--volume", `${names.volume}:${HUB_IMAGE_DATA_ROOT}`, names.image, "-c", `chown -R semio:semio ${HUB_IMAGE_DATA_ROOT} && chmod 700 ${HUB_IMAGE_DATA_ROOT} ${HUB_IMAGE_DATA_ROOT}/trusted-catalog`];
}

/** 🔑️ `os-hub credential set` against the volume with no hub running; the password travels on stdin only. */
export function hubImageCredentialArguments(names: HubImageRunNames, email: string, displayName: string): string[] {
  return ["run", "--rm", "--interactive", "--volume", `${names.volume}:${HUB_IMAGE_DATA_ROOT}`, names.image, "credential", "set", "--email", email, "--display-name", displayName];
}

/** 🚀️ The production-posture hub: published on host loopback only, the seeded subject as admin, one allowed origin, TLS
 * stated as terminated by the proxy in front, and room for the SIGTERM drain. */
export function hubImageRunArguments(names: HubImageRunNames, input: Readonly<{ port: number; email: string; allowedOrigin: string }>): string[] {
  return ["run", "--detach", "--name", names.container, "--publish", `127.0.0.1:${input.port}:${HUB_IMAGE_PORT}`, "--volume", `${names.volume}:${HUB_IMAGE_DATA_ROOT}`, "--env", `OS_HUB_ADMIN_SUBJECTS=credential.password.v1:${input.email}`, "--env", `OS_HUB_ALLOWED_ORIGINS=${input.allowedOrigin}`, "--env", "OS_HUB_TRUSTED_FORWARDING=proxy", "--stop-timeout", "30", names.image];
}

/** 📟️ What one Docker client call answered. */
export type DockerAnswer = Readonly<{ code: number; stdout: string; stderr: string }>;

/** 🐳️ Runs one `docker` argv without a shell, feeding `input` on stdin and every output line to `onLine`; an aborted
 * `signal` interrupts the client (a cancelled `docker build` cancels the build) and removes its process tree after 10 s. */
export function runDocker(args: readonly string[], options: Readonly<{ input?: string; signal?: AbortSignal; onLine?: (line: string) => void }> = {}): Promise<DockerAnswer> {
  return new Promise((resolveAnswer, rejectAnswer) => {
    if (options.signal?.aborted) return rejectAnswer(new Error("cancelled"));
    const child = spawn("docker", [...args], { shell: false, stdio: [options.input === undefined ? "ignore" : "pipe", "pipe", "pipe"], windowsHide: true });
    let stdout = "";
    let stderr = "";
    const lines = (chunk: Buffer, sink: "out" | "err"): void => {
      const text = chunk.toString("utf8");
      if (sink === "out") stdout += text;
      else stderr += text;
      if (options.onLine) for (const line of text.split(/\r?\n/u)) if (line.trim()) options.onLine(line);
    };
    child.stdout!.on("data", (chunk: Buffer) => lines(chunk, "out"));
    child.stderr!.on("data", (chunk: Buffer) => lines(chunk, "err"));
    let escalation: ReturnType<typeof setTimeout> | undefined;
    const abort = (): void => {
      child.kill("SIGINT");
      escalation = setTimeout(() => terminateOwnedChildTree(child), 10_000);
    };
    options.signal?.addEventListener("abort", abort, { once: true });
    child.once("error", (error) => {
      options.signal?.removeEventListener("abort", abort);
      rejectAnswer(/ENOENT/u.test(String(error)) ? new Error("the docker client does not exist on PATH; install Docker Desktop or Docker Engine") : error);
    });
    child.once("close", (code) => {
      if (escalation) clearTimeout(escalation);
      options.signal?.removeEventListener("abort", abort);
      if (options.signal?.aborted) return rejectAnswer(new Error("cancelled"));
      resolveAnswer({ code: code ?? -1, stdout, stderr });
    });
    if (options.input !== undefined) child.stdin!.end(options.input);
  });
}

/** 🐳️ Runs one `docker` argv and fails loudly with its stderr unless it exits 0. */
async function docker(args: readonly string[], options: Parameters<typeof runDocker>[1] = {}): Promise<string> {
  const answer = await runDocker(args, options);
  if (answer.code !== 0) throw new Error(`docker ${args[0]} exited ${answer.code}: ${(answer.stderr || answer.stdout).trim().split("\n").slice(-4).join(" | ").slice(0, 600)}`);
  return answer.stdout.trim();
}

/** 🩺️ The daemon's server version, or an error naming a stopped daemon — a missing precondition, never a hub fault. */
export async function dockerServerVersion(signal?: AbortSignal): Promise<string> {
  const answer = await runDocker(["version", "--format", "{{.Server.Version}}"], { signal });
  if (answer.code !== 0 || !answer.stdout.trim()) throw new Error(`the Docker daemon does not answer: ${(answer.stderr || answer.stdout).trim().slice(0, 300)}`);
  return answer.stdout.trim();
}

/** 🏗️ One cold image build. */
export type HubImageBuildReport = Readonly<{ image: string; exitCode: number; seconds: number; steps: number; imageId: string; imageBytes: number; logPath: string; tail: string }>;

/** 🏗️ Builds `semio/os-hub:<tag>` from `repoRoot`, streaming every BuildKit step header to `onProgress` and the whole client
 * output to `logPath`; answers the build's exit code, duration and the resulting image's id and size. */
export async function buildHubImage(options: Readonly<{ repoRoot: string; tag: string; jobs: number | null; logPath: string; signal: AbortSignal; onProgress: (line: string) => void }>): Promise<HubImageBuildReport> {
  options.onProgress(`docker ${await dockerServerVersion(options.signal)}; building ${HUB_IMAGE_REPOSITORY}:${options.tag}${options.jobs ? ` with ${options.jobs} compiler job(s)` : ""}`);
  mkdirSync(dirname(options.logPath), { recursive: true });
  const log: string[] = [];
  let steps = 0;
  const started = performance.now();
  const answer = await new Promise<DockerAnswer>((resolveBuild, rejectBuild) => {
    const child = spawn("docker", hubImageBuildArguments(options), { cwd: options.repoRoot, shell: false, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
    let escalation: ReturnType<typeof setTimeout> | undefined;
    const abort = (): void => {
      child.kill("SIGINT");
      escalation = setTimeout(() => terminateOwnedChildTree(child), 10_000);
    };
    options.signal.addEventListener("abort", abort, { once: true });
    const onChunk = (chunk: Buffer): void => {
      for (const line of chunk.toString("utf8").split(/\r?\n/u)) {
        if (!line) continue;
        log.push(line);
        if (/^#\d+ \[[^\]]+\]/u.test(line) || /^#\d+ (DONE|ERROR|CANCELED)/u.test(line) || /^(ERROR|error):/u.test(line)) {
          if (/^#\d+ \[/u.test(line)) steps += 1;
          options.onProgress(`${Math.round((performance.now() - started) / 1000)} s ${line.slice(0, 240)}`);
        }
      }
    };
    child.stdout.on("data", onChunk);
    child.stderr.on("data", onChunk);
    child.once("error", rejectBuild);
    child.once("close", (code) => {
      if (escalation) clearTimeout(escalation);
      options.signal.removeEventListener("abort", abort);
      resolveBuild({ code: code ?? -1, stdout: "", stderr: "" });
    });
  });
  writeFileSync(options.logPath, `${log.join("\n")}\n`);
  if (options.signal.aborted) throw new Error("cancelled");
  const seconds = Math.round((performance.now() - started) / 1000);
  const image = `${HUB_IMAGE_REPOSITORY}:${options.tag}`;
  const inspected = answer.code === 0 ? (await docker(["image", "inspect", "--format", "{{.Id}} {{.Size}}", image], { signal: options.signal })).split(" ") : [];
  return { image, exitCode: answer.code, seconds, steps, imageId: inspected[0] ?? "", imageBytes: Number(inspected[1] ?? -1), logPath: options.logPath, tail: log.slice(-12).join("\n") };
}

/** 🤝️ One two-client relay over one document of a running hub. */
export type HubTwoClientSmokeReport = Readonly<{ spaceId: string; kindId: string; artifactId: string; creationMs: number; aToBMs: number; bToAMs: number; undecodable: number }>;

/** 🤝️ Two independent sessions of one signed-in person (two devices) open one freshly created `kind` document; A's edit
 * must reach B's socket and B's edit, chained on it, must reach A's. `origin` is the proxy in front of a production hub or
 * a loopback development hub. */
export async function runHubTwoClientSmoke(options: Readonly<{ origin: string; email: string; password: string; kind: string; signal: AbortSignal; onProgress: (line: string) => void }>): Promise<HubTwoClientSmokeReport> {
  const step = (label: string): void => {
    if (options.signal.aborted) throw new Error("cancelled");
    options.onProgress(label);
  };
  step("sign-in: client A and client B");
  const tokenA = await hubProbeSignIn(options.origin, options.email, options.password, "docker-smoke-a");
  const tokenB = await hubProbeSignIn(options.origin, options.email, options.password, "docker-smoke-b");
  step("client A creates a space");
  const spaceId = await hubProbeCreateSpace(options.origin, tokenA, `Docker smoke ${new Date().toISOString()}`);
  const catalog = await hubProbeCreationCatalog(options.origin, tokenA, spaceId);
  const kindId = catalog.kinds.find((row) => row.kindId === options.kind)?.kindId;
  if (!kindId) throw new Error(`kind ${options.kind} does not exist in the seeded catalog (creatable: ${catalog.kinds.map((row) => row.kindId).join(", ") || "none"})`);
  step(`client A creates a ${kindId} document`);
  const created = await hubProbeCreateArtifact(options.origin, tokenA, spaceId, catalog.generationId, kindId, "Docker smoke");
  step(`both clients open ${created.artifactId}`);
  const a = await hubProbeOpenDocument(options.origin, tokenA, spaceId, created.artifactId, `docker-smoke-a-${randomBytes(4).toString("hex")}`);
  const b = await hubProbeOpenDocument(options.origin, tokenB, spaceId, created.artifactId, `docker-smoke-b-${randomBytes(4).toString("hex")}`);
  try {
    step("client A edits; client B must receive it");
    const sentA = Date.now();
    const fromA = await a.edit(0, "");
    const aToBMs = (await b.relayed(fromA)) - sentA;
    step("client B edits on top; client A must receive it");
    const sentB = Date.now();
    const fromB = await b.edit(1, fromA);
    const bToAMs = (await a.relayed(fromB)) - sentB;
    return { spaceId, kindId, artifactId: created.artifactId, creationMs: created.ms, aToBMs, bToAMs, undecodable: a.undecodableFrames() + b.undecodableFrames() };
  } finally {
    a.close();
    b.close();
  }
}

/** 🩺️ One image check run. */
export type HubImageCheckReport = {
  image: string;
  imageId: string;
  imageBytes: number;
  generation: string;
  container: string;
  readyMs: number;
  healthzStatus: number;
  directHealthzStatus: number;
  directRefusal: string;
  smoke?: HubTwoClientSmokeReport;
  stopMs: number;
  exitCode: number;
  kept: boolean;
  logTail: string;
};

/** 🩺️ Runs `semio/os-hub:<tag>` in its production posture on a fresh volume (see the module docstring) and answers what it
 * measured; the container, volume and staged catalog are removed afterwards unless `keep`. `password` is never logged. */
export async function checkHubImage(options: Readonly<{ tag: string; port: number; catalogRoot: string; kind: string; email: string; password: string; allowedOrigin: string; readyTimeoutMs: number; keep: boolean; signal: AbortSignal; onProgress: (line: string) => void }>): Promise<HubImageCheckReport> {
  const say = options.onProgress;
  say(`docker ${await dockerServerVersion(options.signal)}`);
  const names = hubImageRunNames(options.tag, `${Date.now().toString(36)}-${randomBytes(3).toString("hex")}`);
  const inspected = await runDocker(["image", "inspect", "--format", "{{.Id}} {{.Size}}", names.image], { signal: options.signal });
  if (inspected.code !== 0) throw new Error(`image ${names.image} does not exist; build it first with docker-image-build --tag ${options.tag}`);
  const [imageId = "", imageBytes = "-1"] = inspected.stdout.trim().split(" ");
  const stage = mkdtempSync(join(tmpdir(), "semio-hub-image-"));
  const report: HubImageCheckReport = { image: names.image, imageId, imageBytes: Number(imageBytes), generation: "", container: names.container, readyMs: -1, healthzStatus: -1, directHealthzStatus: -1, directRefusal: "", stopMs: -1, exitCode: -1, kept: options.keep, logTail: "" };
  let proxy: ReturnType<typeof hubForwardingProxy> | undefined;
  let running = false;
  try {
    report.generation = hubSeedTrustedCatalog(options.catalogRoot, stage);
    say(`volume ${names.volume}: trusted catalog ${report.generation.slice(0, 12)} and one credential`);
    await docker(["volume", "create", names.volume], { signal: options.signal });
    await docker(hubImageSeedArguments(names), { signal: options.signal });
    await docker(["cp", join(stage, "trusted-catalog"), `${names.seed}:${HUB_IMAGE_DATA_ROOT}/`], { signal: options.signal });
    await docker(["start", "--attach", names.seed], { signal: options.signal });
    await docker(["rm", names.seed], { signal: options.signal });
    await docker(hubImageCredentialArguments(names, options.email, "Docker Smoke"), { input: `${options.password}\n`, signal: options.signal });
    const started = Date.now();
    await docker(hubImageRunArguments(names, options), { signal: options.signal });
    running = true;
    proxy = hubForwardingProxy(`http://127.0.0.1:${options.port}`);
    say(`container ${names.container} on 127.0.0.1:${options.port}, forwarding proxy ${proxy.origin}`);
    let last = "";
    while (Date.now() - started < options.readyTimeoutMs) {
      if (options.signal.aborted) throw new Error("cancelled");
      const state = (await runDocker(["inspect", "--format", "{{.State.Status}}", names.container])).stdout.trim();
      if (state !== "running") throw new Error(`container ${state || "vanished"} before ready: ${(await runDocker(["logs", "--tail", "12", names.container])).stderr.trim().slice(-800)}`);
      const ready = await hubProbeCall(proxy.origin, "GET", "/readyz").catch((error: unknown) => ({ status: 0, text: String(error), json: null, bytes: new Uint8Array() }));
      const line = `${ready.status} ${ready.text.slice(0, 200)}`;
      if (line !== last) say(`/readyz after ${Date.now() - started} ms: ${line}`);
      last = line;
      if (ready.status === 200) {
        report.readyMs = Date.now() - started;
        break;
      }
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 2_000));
    }
    if (report.readyMs < 0) throw new Error(`not ready within ${options.readyTimeoutMs} ms; last /readyz ${last}`);
    report.healthzStatus = (await hubProbeCall(proxy.origin, "GET", "/healthz")).status;
    const direct = await fetch(`http://127.0.0.1:${options.port}/healthz`, { signal: AbortSignal.timeout(10_000) });
    report.directHealthzStatus = direct.status;
    report.directRefusal = direct.headers.get("x-semio-refusal") ?? "";
    say(`/healthz through the proxy ${report.healthzStatus}; without the proxy's statement ${report.directHealthzStatus} ${report.directRefusal}`);
    report.smoke = await runHubTwoClientSmoke({ origin: proxy.origin, email: options.email, password: options.password, kind: options.kind, signal: options.signal, onProgress: (line) => say(`smoke: ${line}`) });
    say(`smoke: A → B ${report.smoke.aToBMs} ms, B → A ${report.smoke.bToAMs} ms`);
    const stopping = Date.now();
    await docker(["stop", "--time", "30", names.container], { signal: options.signal });
    running = false;
    report.stopMs = Date.now() - stopping;
    report.exitCode = Number(await docker(["inspect", "--format", "{{.State.ExitCode}}", names.container]));
    say(`docker stop took ${report.stopMs} ms, exit code ${report.exitCode}`);
    return report;
  } catch (error) {
    report.logTail = (await runDocker(["logs", "--tail", "20", names.container]).catch(() => ({ code: -1, stdout: "", stderr: "" }))).stderr.trim().slice(-1600);
    throw Object.assign(error instanceof Error ? error : new Error(String(error)), { report });
  } finally {
    proxy?.stop();
    if (running) await runDocker(["stop", "--time", "30", names.container]).catch(() => undefined);
    if (!options.keep) {
      await runDocker(["rm", "--force", names.container, names.seed]).catch(() => undefined);
      await runDocker(["volume", "rm", "--force", names.volume]).catch(() => undefined);
    }
    rmSync(stage, { recursive: true, force: true });
  }
}
