/** 🖼️ The `docker-image-check` verb: runs the image as the stack does (no configuration, read-only root, no capabilities) and checks what it is (labels, user, no shell, the stack files it carries) and the API as the CDN-hosted site meets it — ready behind the proxy, cleartext refused, cross-origin preflight and POST granted to the site origin only, the request body limit, shared presence between two learners, `docker stop` draining with exit code 0 while a socket is open, the WAL folded into `proctor.sqlite` on the volume.
 * @see ../🟦️.ts — the constants, tools and drift check shared by the operator verbs */

import { randomBytes } from "node:crypto";
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { request as httpRequest } from "node:http";
import { request as httpsRequest } from "node:https";
import { workspaceCargoVersion } from "@semio-tech/repo-lib/cargo";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import { PRESENCE_PROTOCOL, type PresenceFrame } from "../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧬️wire-types/🟦️.ts";
import { decodeCommandOutcome, decodePresenceFrame, encodeCommandEnvelope, encodePresenceFrame, encodeQueryEnvelope } from "../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🔤️codec/🟦️.ts";
import { presenceSocketUrl } from "../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🛣️routes-and-sockets/🟦️.ts";
import type { IdentityClaim } from "../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🧬️schema/🟦️.ts";
import course from "../../🔣️.json" with { type: "json" };
import { BUNDLE, COURSE_DEPLOY_CLEANUP_MS, COURSE_IMAGE_DATABASE, COURSE_IMAGE_DATA_ROOT, COURSE_IMAGE_HEALTHCHECK, COURSE_IMAGE_PORT, COURSE_IMAGE_REPOSITORY, COURSE_IMAGE_STACK_ROOT, COURSE_IMAGE_USER, COURSE_PROCTOR_HOST, COURSE_SITE_ORIGIN, COURSE_STACK_FILES, DEPLOY, FOREIGN_ORIGIN, containerLog, docker, dockerOk, dockerServerVersion, flag, interruption, pause, say, stackRequestBodyLimit } from "../🟦️.ts";

//#region 🐳️Image
/** 📬️ One HTTP answer of the proctor. */
type Probe = Readonly<{ status: number; headers: Headers; body: string }>;

/** 📡️ One request to `url`; TLS verification is off only for the throw-away stack's internal CA. */
export async function probe(url: string, init: Readonly<{ method?: string; headers?: Record<string, string>; body?: string | Uint8Array; insecure?: boolean; timeoutMs?: number }>, signal: AbortSignal): Promise<Probe> {
  const response = await fetch(url, { method: init.method ?? "GET", headers: init.headers, body: init.body, redirect: "manual", signal: AbortSignal.any([signal, AbortSignal.timeout(init.timeoutMs ?? 10_000)]), ...(init.insecure ? { tls: { rejectUnauthorized: false } } : {}) } as RequestInit);
  return { status: response.status, headers: response.headers, body: await response.text() };
}

/** 🚪️ How a check reaches a proctor: its base URL, the headers every request carries (the proxy's statement on a direct
 * container port, none through Caddy) and whether the certificate is Caddy's internal one. Every check speaks as the
 * site origin. */
export type Door = Readonly<{ base: string; headers: Record<string, string>; insecure: boolean }>;

/** 🧑‍🎓️ The site's own proctor client. A check sends the envelopes this module builds (§9a), so it meets the proctor
 * exactly as the released site does; loaded only when a check runs. */
function siteClient(): Promise<typeof import("../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts")> {
  return import("../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts");
}

/** ❓️ One quiz query as the site sends it, answered with its status and the view. */
async function courseQuery(door: Door, query: Parameters<Awaited<ReturnType<typeof siteClient>>["queryEnvelope"]>[0], learner: string | undefined, signal: AbortSignal): Promise<Probe> {
  const envelope = encodeQueryEnvelope((await siteClient()).queryEnvelope(query, course.id, learner));
  return probe(`${door.base}/queries`, { method: "POST", headers: { ...door.headers, origin: COURSE_SITE_ORIGIN, "content-type": "application/json" }, body: JSON.stringify(envelope), insecure: door.insecure }, signal);
}

/** 🙋️ Registers one learner as the site does — anonymous, or under the pseudonym `handle` — and answers its id. */
export async function identifyLearner(door: Door, signal: AbortSignal, handle?: string): Promise<string> {
  const client = await siteClient();
  const identity: IdentityClaim = handle === undefined ? { kind: "anonymous" } : { kind: "pseudonym", handle };
  const command = { type: "identify-learner", id: client.newId(), learner: client.newId(), identity } as const;
  const answer = await probe(`${door.base}/commands`, { method: "POST", headers: { ...door.headers, origin: COURSE_SITE_ORIGIN, "content-type": "application/json" }, body: JSON.stringify(encodeCommandEnvelope(client.commandEnvelope(command, course.id, Date.now()))), insecure: door.insecure }, signal);
  if (answer.status !== 200 || client.commandVerdict(decodeCommandOutcome(JSON.parse(answer.body))).kind !== "accepted") throw new Error(`POST /commands (identify-learner) answered ${answer.status}: ${answer.body.slice(0, 300)}`);
  return command.learner;
}

/** 🕵️ Whether the proctor knows `learner`: its learner view answers, or `404`. */
export async function learnerKnown(door: Door, learner: string, signal: AbortSignal): Promise<boolean> {
  const answer = await courseQuery(door, { type: "learner", learner }, learner, signal);
  if (answer.status !== 200 && answer.status !== 404) throw new Error(`POST /queries (learner) answered ${answer.status}: ${answer.body.slice(0, 300)}`);
  return answer.status === 200;
}

/** 🤝️ The cross-origin contract of the CDN-hosted site against the proctor behind `door`: preflights of both POST routes
 * are granted to the site origin with a cache lifetime, a foreign origin gets no grant, and a real course query answers
 * with the grant. */
export async function checkCrossOrigin(door: Door, signal: AbortSignal): Promise<void> {
  for (const route of ["/commands", "/queries"]) {
    const preflight = await probe(`${door.base}${route}`, { method: "OPTIONS", headers: { ...door.headers, origin: COURSE_SITE_ORIGIN, "access-control-request-method": "POST", "access-control-request-headers": "content-type" }, insecure: door.insecure }, signal);
    const origin = preflight.headers.get("access-control-allow-origin");
    const maxAge = Number(preflight.headers.get("access-control-max-age") ?? "0");
    if (preflight.status !== 204 || origin !== COURSE_SITE_ORIGIN || !/content-type/iu.test(preflight.headers.get("access-control-allow-headers") ?? "") || !(maxAge > 0)) throw new Error(`OPTIONS ${route} from ${COURSE_SITE_ORIGIN} answered ${preflight.status}, allow-origin ${origin}, allow-headers ${preflight.headers.get("access-control-allow-headers")}, max-age ${preflight.headers.get("access-control-max-age")}`);
    say(`preflight ${route}: 204, allow-origin ${origin}, max-age ${maxAge}`);
  }
  const foreign = await probe(`${door.base}/queries`, { method: "OPTIONS", headers: { ...door.headers, origin: FOREIGN_ORIGIN, "access-control-request-method": "POST" }, insecure: door.insecure }, signal);
  if (foreign.headers.get("access-control-allow-origin")) throw new Error(`a preflight from ${FOREIGN_ORIGIN} was granted ${foreign.headers.get("access-control-allow-origin")}`);
  say(`preflight from ${FOREIGN_ORIGIN}: ${foreign.status} without a grant`);
  const query = await courseQuery(door, { type: "course" }, undefined, signal);
  if (query.status !== 200 || query.headers.get("access-control-allow-origin") !== COURSE_SITE_ORIGIN || !query.body.includes("\"snapshot\"")) throw new Error(`POST /queries from ${COURSE_SITE_ORIGIN} answered ${query.status}, allow-origin ${query.headers.get("access-control-allow-origin")}: ${query.body.slice(0, 300)}`);
  say(`POST /queries (course) from ${COURSE_SITE_ORIGIN}: 200 snapshot with the grant`);
}

/** 📏️ The request body limit of `limit` bytes holds behind `door`: one byte more is refused with `413`, the limit itself
 * is read (and refused for what it is, not for its size). On a direct port with the Caddyfile's default this proves the
 * proctor's own default is the same number; through Caddy with a limit set in `.env` it proves both services follow
 * that one variable. */
export async function checkBodyLimit(door: Door, limit: number, signal: AbortSignal): Promise<void> {
  const post = (bytes: number): Promise<Probe> => probe(`${door.base}/commands`, { method: "POST", headers: { ...door.headers, origin: COURSE_SITE_ORIGIN, "content-type": "application/json" }, body: new Uint8Array(bytes).fill(0x20), insecure: door.insecure }, signal);
  const [over, at] = [await post(limit + 1), await post(limit)];
  if (over.status !== 413 || at.status === 413) throw new Error(`a body of ${limit + 1} bytes answered ${over.status} and one of ${limit} bytes ${at.status}; the limit of ${limit} bytes must refuse the first with 413 and read the second`);
  say(`request body limit: ${limit + 1} bytes refused with 413, ${limit} bytes read (${at.status})`);
}

/** 🔌️ The platform socket lifecycle needed by a bounded deployment handshake. */
export interface PresenceCheckSocket {
  readonly readyState: number;
  addEventListener(type: "open" | "error" | "close", listener: () => void): void;
  removeEventListener(type: "open" | "error" | "close", listener: () => void): void;
  close(): void;
}

/** 🚦️ Reads the actual upgrade verdict so transport failure can never count as a foreign-origin refusal. */
export async function presenceUpgradeStatus(url: string, headers: Record<string, string>, insecure: boolean, signal: AbortSignal, timeoutMs = 10_000): Promise<number> {
  signal.throwIfAborted();
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 30_000) throw Error("Finite presence handshake deadline required");
  const target = new URL(url);
  if (target.protocol !== "ws:" && target.protocol !== "wss:") throw Error("Presence WebSocket URL required");
  target.protocol = target.protocol === "wss:" ? "https:" : "http:";
  const deadline = AbortSignal.timeout(timeoutMs);
  return new Promise((accept, reject) => {
    const request = (target.protocol === "https:" ? httpsRequest : httpRequest)(target, { method: "GET", agent: false, headers: { ...headers, connection: "Upgrade", upgrade: "websocket", "sec-websocket-version": "13", "sec-websocket-key": randomBytes(16).toString("base64"), "sec-websocket-protocol": PRESENCE_PROTOCOL }, rejectUnauthorized: !insecure, signal: AbortSignal.any([signal, deadline]) });
    request.once("response", response => { response.destroy(); accept(response.statusCode ?? 0); });
    request.once("upgrade", (response, socket) => { socket.destroy(); accept(response.statusCode ?? 0); });
    request.once("error", error => reject(signal.aborted ? signal.reason : deadline.aborted ? Error(`presence upgrade handshake timed out within ${timeoutMs} ms`) : error));
    request.end();
  });
}

/** ⏱️ Settles a presence handshake on opening, refusal, original cancellation or its finite deadline. */
export function waitForPresenceSocket(socket: PresenceCheckSocket, signal: AbortSignal, timeoutMs = 10_000): Promise<void> {
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 30_000) throw Error("Finite presence handshake deadline required");
  return new Promise((accept, reject) => {
    let settled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const finish = (error?: unknown): void => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      socket.removeEventListener("open", opened);
      socket.removeEventListener("error", failed);
      socket.removeEventListener("close", closed);
      signal.removeEventListener("abort", cancelled);
      if (error !== undefined) {
        try { socket.close(); } catch {}
        reject(error);
      } else accept();
    };
    const opened = (): void => finish();
    const failed = (): void => finish(Error("presence handshake failed"));
    const closed = (): void => finish(Error("presence socket closed before opening"));
    const cancelled = (): void => finish(signal.reason ?? new DOMException("Presence handshake cancelled", "AbortError"));
    socket.addEventListener("open", opened);
    socket.addEventListener("error", failed);
    socket.addEventListener("close", closed);
    signal.addEventListener("abort", cancelled, { once: true });
    timer = setTimeout(() => finish(Error(`presence handshake did not open within ${timeoutMs} ms`)), timeoutMs);
    if (signal.aborted) cancelled();
    else if (socket.readyState === 1) opened();
    else if (socket.readyState > 1) closed();
  });
}

/** 🪢️ One cancellable presence socket of a check, retaining every decoded frame. */
function presenceSocket(url: string, headers: Record<string, string>, insecure: boolean, signal: AbortSignal): Readonly<{ opened: Promise<WebSocket>; frames: PresenceFrame[] }> {
  signal.throwIfAborted();
  const frames: PresenceFrame[] = [];
  const socket = new WebSocket(url, { protocols: [PRESENCE_PROTOCOL], headers, ...(insecure ? { tls: { rejectUnauthorized: false } } : {}) } as unknown as string[]);
  socket.addEventListener("message", (event) => frames.push(decodePresenceFrame(JSON.parse(String(event.data)))));
  const cancel = (): void => socket.close();
  signal.addEventListener("abort", cancel, { once: true });
  socket.addEventListener("close", () => signal.removeEventListener("abort", cancel), { once: true });
  const opened = waitForPresenceSocket(socket, signal).then(() => socket);
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

/** 👥️ Shared presence as two CDN-hosted learners meet it on the course's roster room: both join with the site origin, one
 * shares a `PresenceState` and the other receives it in a batch, then its departure; a foreign origin never opens. The
 * second learner's socket is handed back open. */
export async function checkPresence(door: Door, signal: AbortSignal): Promise<WebSocket> {
  const url = presenceSocketUrl(door.base, course.id, "check");
  const first = presenceSocket(url, { ...door.headers, origin: COURSE_SITE_ORIGIN }, door.insecure, signal);
  const ada = await first.opened;
  const welcome = await presenceFrame(first.frames, (frame) => frame.type === "welcome", "welcome of the first learner", signal);
  if (ada.protocol !== PRESENCE_PROTOCOL || welcome.type !== "welcome") throw new Error(`the presence socket spoke ${JSON.stringify(ada.protocol)}`);
  const second = presenceSocket(url, { ...door.headers, origin: COURSE_SITE_ORIGIN }, door.insecure, signal);
  const grace = await second.opened;
  const roster = await presenceFrame(second.frames, (frame) => frame.type === "welcome", "welcome of the second learner", signal);
  if (roster.type !== "welcome" || !roster.roster.some((entry) => entry.session === welcome.session)) throw new Error(`the second learner's roster lacks the first: ${JSON.stringify(roster)}`);
  const state = { tag: "0a1b2c3d", identity: { kind: "anonymous" }, place: { screen: "home" }, active: true };
  ada.send(JSON.stringify(encodePresenceFrame({ type: "state", state })));
  await presenceFrame(second.frames, (frame) => frame.type === "batch" && frame.entries.some((entry) => entry.session === welcome.session && Bun.deepEquals(entry.state, state)), "the first learner's state in a batch", signal);
  ada.close();
  await presenceFrame(second.frames, (frame) => frame.type === "batch" && frame.left.includes(welcome.session), "the first learner leaving", signal);
  say(`presence ${url.split("?")[0]}: two learners joined (colours ${welcome.colour}, ${roster.colour}), a shared state and a departure arrived in batches`);
  const foreign = await presenceUpgradeStatus(url, { ...door.headers, origin: FOREIGN_ORIGIN }, door.insecure, signal);
  signal.throwIfAborted();
  if (foreign !== 403) throw new Error(`presence upgrade from ${FOREIGN_ORIGIN} answered ${foreign}; expected 403`);
  say(`presence from ${FOREIGN_ORIGIN}: refused (${foreign})`);
  return grace;
}

/** 🧳️ Copies the stack files the image `reference` carries into a fresh directory of the host — everything a host needs
 * besides Docker — and answers that directory. */
export async function extractCourseStack(reference: string, signal: AbortSignal): Promise<string> {
  signal.throwIfAborted();
  const directory = mkdtempSync(join(tmpdir(), "architecture-course-stack-"));
  const container = `architecture-course-export-${randomBytes(8).toString("hex")}`;
  let completed = false;
  try {
    await dockerOk(["create", "--name", container, reference], { signal });
    await dockerOk(["cp", `${container}:${COURSE_IMAGE_STACK_ROOT}/.`, directory], { signal });
    completed = true;
    return directory;
  } finally {
    let retained = false;
    try { await dockerOk(["rm", "--force", "--volumes", container], { signal: AbortSignal.timeout(COURSE_DEPLOY_CLEANUP_MS) }); retained = completed; }
    finally { if (!retained) rmSync(directory, { recursive: true, force: true }); }
  }
}

/** 🪞️ Fails unless the stack files in `directory`, copied out of an image, are those of this checkout. */
export function expectCheckoutStack(repoRoot: string, directory: string): void {
  for (const file of COURSE_STACK_FILES) if (!existsSync(join(directory, file)) || readFileSync(join(directory, file), "utf8") !== readFileSync(join(repoRoot, DEPLOY, file), "utf8")) throw new Error(`${COURSE_IMAGE_STACK_ROOT}/${file} in the image is not ${DEPLOY}/${file}; build the image again`);
}

/** 🩺️ `docker-image-check [--tag <tag>] [--port <n>] [--ready-timeout-ms <n>] [--keep]` — inspects the image (labels,
 * account, health probe, no shell, the stack files of this checkout inside it), then runs it with nothing but a fresh
 * volume and a loopback port, read-only and without capabilities as the stack runs it (the baked production defaults are
 * the whole configuration), and fails on the first expectation it misses; the container and volume are removed afterwards
 * unless `--keep`. */
export async function checkCourseImage(repoRoot: string, segments: readonly string[], invocation: ScriptInvocation): Promise<void> {
  const tag = flag(segments, "--tag") ?? "latest";
  const port = Number(flag(segments, "--port") ?? "18791");
  const readyTimeoutMs = Number(flag(segments, "--ready-timeout-ms") ?? "60000");
  const keep = segments.includes("--keep");
  const { signal, release } = interruption(invocation);
  const run = `${Date.now().toString(36)}-${randomBytes(3).toString("hex")}`;
  const image = `${COURSE_IMAGE_REPOSITORY}:${tag}`;
  const container = `architecture-course-proctor-${run}`;
  const volume = `architecture-course-proctor-${run}-data`;
  const door: Door = { base: `http://127.0.0.1:${port}`, headers: { "x-forwarded-proto": "https", "x-forwarded-host": COURSE_PROCTOR_HOST }, insecure: false };
  const scratch: string[] = [];
  let created = false;
  let running = false;
  try {
    say(`docker ${await dockerServerVersion(signal)}`);
    if ((await docker(["image", "inspect", image], { signal })).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    const config = JSON.parse(await dockerOk(["image", "inspect", "--format", "{{json .Config}}", image], { signal })) as { User: string; Labels: Record<string, string> | null; Healthcheck?: { Test: string[] }; ExposedPorts?: Record<string, unknown>; Volumes?: Record<string, unknown> };
    const label = (name: string): string => config.Labels?.[`org.opencontainers.image.${name}`] ?? "";
    const facts: Record<string, [unknown, unknown]> = { user: [config.User, COURSE_IMAGE_USER], healthcheck: [config.Healthcheck?.Test, [...COURSE_IMAGE_HEALTHCHECK]], ports: [Object.keys(config.ExposedPorts ?? {}), [`${COURSE_IMAGE_PORT}/tcp`]], volumes: [Object.keys(config.Volumes ?? {}), [COURSE_IMAGE_DATA_ROOT]], version: [label("version"), workspaceCargoVersion(repoRoot)], source: [`${label("source")}.git`, (JSON.parse(readFileSync(join(repoRoot, BUNDLE, "package.json"), "utf8")) as { repository: { url: string } }).repository.url] };
    for (const [fact, [found, expected]] of Object.entries(facts)) if (!Bun.deepEquals(found, expected)) throw new Error(`the image's ${fact} is ${JSON.stringify(found)}, not ${JSON.stringify(expected)}`);
    if (!/^[0-9a-f]{40}(?:-dirty)?$/u.test(label("revision")) || Number.isNaN(Date.parse(label("created")))) throw new Error(`the image names no source revision and time: ${label("revision")} ${label("created")}`);
    say(`image ${label("version")} of ${label("revision")} (${label("created")}), user ${config.User}, ${await dockerOk(["image", "inspect", "--format", "{{.Os}}/{{.Architecture}} {{.Size}} bytes", image], { signal })}`);
    if ((await docker(["run", "--rm", "--entrypoint", "/bin/sh", image, "-c", "true"], { signal })).code === 0) throw new Error("the image carries a shell");
    say("no shell in the image");
    scratch.push(await extractCourseStack(image, signal));
    expectCheckoutStack(repoRoot, scratch[0]!);
    say(`${COURSE_IMAGE_STACK_ROOT} carries ${COURSE_STACK_FILES.join(" and ")} of this checkout`);
    created = true;
    await dockerOk(["volume", "create", volume], { signal });
    const started = Date.now();
    running = true;
    await dockerOk(["run", "--detach", "--name", container, "--publish", `127.0.0.1:${port}:${COURSE_IMAGE_PORT}`, "--volume", `${volume}:${COURSE_IMAGE_DATA_ROOT}`, "--read-only", "--cap-drop", "ALL", "--security-opt", "no-new-privileges", "--stop-timeout", "30", image], { signal });
    say(`container ${container} (no configuration, read-only root, no capabilities) on ${door.base}, volume ${volume}`);
    let ready: Probe | undefined;
    while (!ready && Date.now() - started < readyTimeoutMs) {
      const state = (await docker(["inspect", "--format", "{{.State.Status}}", container], { signal })).stdout;
      if (state !== "running") throw new Error(`container ${state || "vanished"} before ready: ${(await containerLog(container, 20, signal)).slice(-1200)}`);
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
    scratch.push(mkdtempSync(join(tmpdir(), "architecture-course-data-")));
    await dockerOk(["cp", `${container}:${COURSE_IMAGE_DATA_ROOT}/.`, scratch[1]!], { signal });
    const kept = readdirSync(scratch[1]!).map((name) => `${name} (${statSync(join(scratch[1]!, name)).size} bytes)`);
    if (!existsSync(join(scratch[1]!, COURSE_IMAGE_DATABASE)) || (existsSync(join(scratch[1]!, `${COURSE_IMAGE_DATABASE}-wal`)) && statSync(join(scratch[1]!, `${COURSE_IMAGE_DATABASE}-wal`)).size > 0)) throw new Error(`after a clean stop ${COURSE_IMAGE_DATA_ROOT} holds ${kept.join(", ")}: ${COURSE_IMAGE_DATABASE} must be whole, with no write-ahead log left`);
    const events = await databaseEvents(join(scratch[1]!, COURSE_IMAGE_DATABASE));
    if (events < 1) throw new Error(`${COURSE_IMAGE_DATABASE} holds ${events} events after learner ${learner} registered`);
    say(`volume keeps ${kept.join(", ")}: ${events} events, the write-ahead log folded; ${image} passed`);
  } catch (error) {
    if (created) console.error(`[deploy] container log tail:\n${await containerLog(container, 30, AbortSignal.timeout(5_000))}`);
    throw error;
  } finally {
    release();
    const removalMilliseconds = 5_000;
    for (const directory of scratch) rmSync(directory, { recursive: true, force: true });
    if (running) await docker(["stop", "--time", "30", container], { signal: AbortSignal.timeout(COURSE_DEPLOY_CLEANUP_MS - removalMilliseconds * 2) }).catch(() => undefined);
    if (created && !keep) {
      await docker(["rm", "--force", container], { signal: AbortSignal.timeout(removalMilliseconds) }).catch(() => undefined);
      await docker(["volume", "rm", "--force", volume], { signal: AbortSignal.timeout(removalMilliseconds) }).catch(() => undefined);
    }
  }
}

/** 🧫️ How many events the proctor database at `file` holds, read by an engine that is not the proctor's (`bun:sqlite`),
 * after checking that the file declares the proctor's format. */
export async function databaseEvents(file: string): Promise<number> {
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
