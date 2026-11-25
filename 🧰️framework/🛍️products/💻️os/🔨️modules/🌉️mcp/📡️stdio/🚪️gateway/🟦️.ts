import policy from "./🧬️schema/🔣️.json" with {type:"json"};
import serverInstructions from "../../🧭️protocol/🧬️schema/📜️instructions.json" with {type:"json"};
import protocolVersions from "../../🧭️protocol/🧬️schema/🗓️versions.json" with {type:"json"};
import { type MissingMcpModelV1, readMissingMcpModelV1 } from "../📦️component-models/🕳️missing/🟦️.ts";
import { type ChildProcess, spawn } from "node:child_process";
import { createInterface } from "node:readline";
import type { Readable, Writable } from "node:stream";

export type WarmMcpFrame = { readonly jsonrpc?: "2.0"; readonly id?: number | string | null; readonly method?: string; readonly params?: unknown; readonly result?: unknown; readonly error?: unknown };

/** 🪪️ The `serverInfo.name` the gateway answers with while the real server is still being staged. */
export const WARM_MCP_SERVER_NAME = "semio";

/** 🪪️ The id the gateway handshakes with the real server under; it never collides with a client id. */
export const WARM_MCP_HANDSHAKE_ID = "semio-gateway-initialize";

const LIST_CHANGED: readonly string[] = ["notifications/tools/list_changed", "notifications/resources/list_changed", "notifications/prompts/list_changed"];

const WARM_DISCOVERY: Readonly<Record<string, unknown>> = policy.definitions.WarmDiscovery.const;

/** 🌡️ Answers `initialize`, `ping` and discovery with empty lists during staging. Other requests wait for the native server. */
export function warmMcpReply(frame: WarmMcpFrame): WarmMcpFrame | null {
  if (frame.method === undefined || frame.id === undefined || frame.id === null) return null;
  const reply = (result: unknown): WarmMcpFrame => ({ jsonrpc: "2.0", id: frame.id, result });
  if (frame.method === "initialize") {
    const requested = (frame.params as { protocolVersion?: unknown } | undefined)?.protocolVersion;
    const listChanged = { listChanged: true };
    return reply({
      protocolVersion: typeof requested === "string" && protocolVersions.includes(requested) ? requested : protocolVersions[0],
      capabilities: { tools: listChanged, resources: { ...listChanged, subscribe: true }, prompts: listChanged },
      serverInfo: { name: WARM_MCP_SERVER_NAME, version: "warming" },
      instructions: `${serverInstructions}\nThe semio server is starting. Its tools, resources and prompts appear through list_changed notifications once it is ready; tool requests wait for it.`,
    });
  }
  if (frame.method === "ping") return reply({});
  return Object.hasOwn(WARM_DISCOVERY, frame.method) ? reply(WARM_DISCOVERY[frame.method]) : null;
}

export type WarmMcpServerPort = Readonly<{ send: (frame: WarmMcpFrame) => void; stop: () => void }>;

/** 🕳️ Builds and publishes the component model of one plugin; resolves whether the published model set changed (so a swap is due), rejects on failure, and stops when `signal` aborts. */
export type WarmMcpModelBuilder = (model: MissingMcpModelV1, signal: AbortSignal) => Promise<boolean>;

export type WarmMcpPorts = Readonly<{ toClient: (frame: WarmMcpFrame) => void; startServer: () => WarmMcpServerPort; log: (line: string) => void; schedule?: (milliseconds: number, run: () => void) => () => void; prepareModel?: WarmMcpModelBuilder; now?: () => number; abandon?: (reason: string) => void; expecting?: boolean }>;

type WarmState = "warming" | "handshaking" | "ready" | "failed";

type WarmChild = Readonly<{ port: WarmMcpServerPort; pending: Set<number | string>; asked: Set<number | string>; generation: number }>;

/** 🧭️ What a native server is to the gateway: serving the client, handshaking to replace the serving one, or finishing the requests it already holds. */
export type WarmMcpRole = "current" | "candidate" | "retiring" | "unknown";

const REFRESH: Readonly<{ failedPrefix: string; droppedFrame: string; replacedExit: string }> = policy.definitions.Refresh.const;

const SUPERVISION: Readonly<{ maximumRestarts: number; windowMilliseconds: number; restartingPrefix: string; exhaustedPrefix: string; recoveredLine: string; message: string; cause: string; summary: { en: string; de: string } }> = policy.definitions.Supervision.const;

const MODELS: Readonly<{ preparingPrefix: string; readyPrefix: string; unchangedPrefix: string; failedPrefix: string }> = policy.definitions.Models.const;

const HANDSHAKE: Readonly<{ candidateMilliseconds: number }> = policy.definitions.Handshake.const;

const DISCOVERY_HOLD: Readonly<{ maximumMilliseconds: number }> = policy.definitions.DiscoveryHold.const;

const REPLAY_PREFIX = "semio-gateway-replay-";

const FORWARD_PREFIX = "semio-gateway-request-";

const isResponse = (frame: WarmMcpFrame): frame is WarmMcpFrame & { id: number | string } => frame.method === undefined && frame.id !== undefined && frame.id !== null;

/** 🚪️ Answers the initial handshake, pings and discovery, queues tool requests, and relays frames once the native server is ready. A usable previously published server starts before staging ends; a refresh that changed it is swapped in behind the client without dropping in-flight requests. */
export class WarmMcpGateway {
  private state: WarmState = "warming";
  private staged = false;
  private usable = false;
  private expecting = false;
  private swapWanted = false;
  private swapAgain = false;
  private restarting = false;
  private restarts: number[] = [];
  private readonly models = new Set<string>();
  private modelQueue: Promise<void> = Promise.resolve();
  private readonly stopping = new AbortController();
  private closed = false;
  private initialize: WarmMcpFrame | null = null;
  private initialized = false;
  private modern = false;
  private server: WarmChild | null = null;
  private candidate: WarmChild | null = null;
  private readonly retiring: WarmChild[] = [];
  private generations = 0;
  private requests = 0;
  private replays = 0;
  private level: unknown = null;
  private rootsChanged = false;
  private readonly subscriptions = new Set<string>();
  private readonly forwarded = new Map<string, { child: WarmChild; id: number | string }>();
  private cancelHandshake: () => void = () => {};
  private failure = "";
  private readonly queue: WarmMcpFrame[] = [];

  private readonly ports: WarmMcpPorts;

  constructor(ports: WarmMcpPorts) {
    this.ports = ports;
    this.expecting = ports.expecting ?? false;
  }

  get phase(): WarmState {
    return this.state;
  }

  role(port: WarmMcpServerPort): WarmMcpRole {
    if (this.server?.port === port) return "current";
    if (this.candidate?.port === port) return "candidate";
    return this.retiring.some(child => child.port === port) ? "retiring" : "unknown";
  }

  fromClient(frame: WarmMcpFrame): void {
    if (this.closed) return;
    this.observe(frame);
    if (this.state === "ready" && this.restarting) return this.hold(frame);
    if (this.state === "ready") return this.dispatch(frame);
    if (this.state === "failed") return this.refuse(frame);
    if (frame.method === "notifications/cancelled" && frame.id === undefined) {
      const requestId = (frame.params as { requestId?: unknown } | null | undefined)?.requestId;
      if (typeof requestId === "number" || typeof requestId === "string") {
        const index = this.queue.findIndex(queued => queued.method !== undefined && queued.id === requestId);
        if (index !== -1) {
          this.queue.splice(index, 1);
          return;
        }
      }
    }
    if (frame.method === "server/discover" && this.initialize === null) this.modern = true;
    if (frame.method === "initialize" && this.initialize === null) this.initialize = frame;
    if (frame.method === "notifications/initialized") this.initialized = true;
    const held = !this.modern && this.holdsDiscovery(frame);
    const reply = this.modern || held ? null : warmMcpReply(frame);
    if (reply !== null) this.ports.toClient(reply);
    else if (frame.method !== "notifications/initialized") this.queue.push(frame);
    if (held) this.ports.schedule?.(DISCOVERY_HOLD.maximumMilliseconds, () => this.answerEmpty(frame));
    this.begin();
  }

  private holdsDiscovery(frame: WarmMcpFrame): boolean {
    return frame.id !== undefined && frame.id !== null && frame.method !== undefined && Object.hasOwn(WARM_DISCOVERY, frame.method) && (this.expecting || this.usable || this.staged || this.state === "handshaking");
  }

  /** 🕳️ No published server can serve soon: the held discovery requests are answered with the empty lists at once. */
  discoveryUnavailable(): void {
    this.expecting = false;
    if (this.usable || this.staged || this.state !== "warming") return;
    for (const frame of this.queue.filter((queued) => queued.method !== undefined && Object.hasOwn(WARM_DISCOVERY, queued.method))) this.answerEmpty(frame);
  }

  private answerEmpty(frame: WarmMcpFrame): void {
    const index = this.queue.indexOf(frame);
    if (index === -1 || this.closed) return;
    this.queue.splice(index, 1);
    const reply = warmMcpReply(frame);
    if (reply !== null) this.ports.toClient(reply);
  }

  markUsable(): void {
    if (this.closed) return;
    this.usable = true;
    if (this.server !== null) return this.swap();
    this.begin();
  }

  stagingFinished(status: number, changed = true): void {
    if (this.closed) return;
    if (status !== 0) {
      if (this.usable) return this.ports.log(`${REFRESH.failedPrefix} staging exited with status ${status}`);
      return this.fail(`staging exited with status ${status}`);
    }
    this.staged = true;
    if (this.server === null) return this.begin();
    if (changed) this.swap();
  }

  fromServer(frame: WarmMcpFrame, from?: WarmMcpServerPort): void {
    const child = from === undefined ? this.candidate ?? this.server : [this.server, this.candidate, ...this.retiring].find(row => row?.port === from) ?? null;
    if (child === null) return this.ports.log(`[semio-os-mcp] dropped a frame received before the handshake finished`);
    if (child === this.candidate) return this.candidateFrame(child, frame);
    if (child !== this.server) return this.retiringFrame(child, frame);
    if (this.state === "ready") {
      if (!isResponse(frame)) return this.relayInitiated(child, frame);
      if (typeof frame.id === "string" && frame.id.startsWith(REPLAY_PREFIX)) return;
      child.pending.delete(frame.id);
      this.ports.toClient(frame);
      return this.noteMissingModel(frame);
    }
    if (this.state !== "handshaking" || frame.id !== WARM_MCP_HANDSHAKE_ID) return this.ports.log(`[semio-os-mcp] dropped a frame received before the handshake finished`);
    if (frame.error !== undefined) return this.fail(`the server refused the handshake: ${JSON.stringify(frame.error)}`);
    child.port.send({ jsonrpc: "2.0", method: "notifications/initialized" });
    this.state = "ready";
    for (const queued of this.queue.splice(0)) this.send(child, queued);
    this.announce();
    if (this.swapWanted) this.swap();
  }

  /** Tells whether the gateway keeps the session alive by restarting the serving server; `false` leaves the end of the session to the caller. */
  serverExited(code: number | null, from?: WarmMcpServerPort, signal?: string | null): boolean {
    if (from !== undefined && this.candidate?.port === from) {
      this.refreshFailed(`the new server exited with ${code}`);
      return false;
    }
    const retired = from === undefined ? undefined : this.retiring.find(child => child.port === from);
    if (retired !== undefined) {
      this.retiring.splice(this.retiring.indexOf(retired), 1);
      for (const id of retired.pending) this.ports.toClient({ jsonrpc: "2.0", id, error: { code: -32603, message: `${REFRESH.replacedExit} ${code}` } });
      retired.pending.clear();
      this.forget(retired);
      return false;
    }
    if (from !== undefined && this.server?.port === from && !this.closed && this.state === "ready" && !this.restarting && (code !== 0 || signal)) return this.crashed(this.server, code, signal ?? null);
    if (!this.closed && this.state === "handshaking") this.fail(`the server exited with ${code}`);
    return false;
  }

  private withinBudget(): boolean {
    const now = this.ports.now?.() ?? Date.now();
    this.restarts = this.restarts.filter(at => at > now - SUPERVISION.windowMilliseconds);
    if (this.restarts.length >= SUPERVISION.maximumRestarts) return false;
    this.restarts.push(now);
    return true;
  }

  private crashed(child: WarmChild, code: number | null, signal: string | null): boolean {
    const exit = signal ? `signal ${signal}` : `status ${code}`;
    if (!this.withinBudget()) {
      this.ports.log(`${SUPERVISION.exhaustedPrefix} ${exit}`);
      return false;
    }
    const text = (template: string): string => template.replace("{exit}", exit);
    const error = { code: -32603, message: text(SUPERVISION.message), data: { cause: SUPERVISION.cause, retryable: true, exit: { code, signal }, summary: { en: text(SUPERVISION.summary.en), de: text(SUPERVISION.summary.de) } } };
    for (const id of child.pending) this.ports.toClient({ jsonrpc: "2.0", id, error });
    child.pending.clear();
    child.asked.clear();
    this.forget(child);
    this.restarting = true;
    this.ports.log(`${SUPERVISION.restartingPrefix} ${exit}`);
    if (this.candidate === null) this.swap();
    return true;
  }

  private recover(reason: string): void {
    if (this.closed || !this.restarting) return;
    if (this.withinBudget()) return this.swap();
    this.ports.log(`${SUPERVISION.exhaustedPrefix} ${reason}`);
    this.restarting = false;
    this.fail(reason);
    this.ports.abandon?.(reason);
  }

  private hold(frame: WarmMcpFrame): void {
    if (isResponse(frame)) return;
    if (frame.method === "notifications/cancelled" && frame.id === undefined) {
      const requestId = (frame.params as { requestId?: unknown } | null | undefined)?.requestId;
      const index = this.queue.findIndex(queued => queued.method !== undefined && queued.id === requestId);
      if (index !== -1) {
        this.queue.splice(index, 1);
        return;
      }
    }
    this.queue.push(frame);
  }

  portFailed(port: WarmMcpServerPort, reason: string): void {
    const role = this.role(port);
    if (role === "candidate") this.refreshFailed(reason);
    else if (role === "retiring") {
      this.ports.log(`${REFRESH.failedPrefix} ${reason}`);
      this.serverExited(null, port);
      port.stop();
    }
  }

  clientClosed(): void {
    this.closed = true;
    this.stopping.abort();
    this.queue.length = 0;
    for (const child of [this.server, this.candidate, ...this.retiring]) child?.port.stop();
  }

  private begin(): void {
    if (this.state !== "warming" || this.closed || !this.staged && !this.usable || !this.modern && (!this.initialized || this.initialize === null)) return;
    this.state = this.modern ? "ready" : "handshaking";
    try {
      const child = this.startWarm();
      this.server = child;
      if (this.modern) for (const queued of this.queue.splice(0)) this.send(child, queued);
      else child.port.send(this.handshake());
    } catch (error) {
      this.fail(error instanceof Error ? error.message : String(error));
    }
  }

  private startWarm(): WarmChild {
    return { port: this.ports.startServer(), pending: new Set(), asked: new Set(), generation: this.generations++ };
  }

  private handshake(): WarmMcpFrame {
    return { jsonrpc: "2.0", id: WARM_MCP_HANDSHAKE_ID, method: "initialize", params: this.initialize!.params };
  }

  private noteMissingModel(frame: WarmMcpFrame): void {
    const missing = readMissingMcpModelV1(frame);
    if (missing === undefined || this.ports.prepareModel === undefined || this.closed || this.models.has(missing.pluginId)) return;
    const build = this.ports.prepareModel;
    this.models.add(missing.pluginId);
    this.modelQueue = this.modelQueue.then(async () => {
      if (this.closed) return;
      this.ports.log(`${MODELS.preparingPrefix} ${missing.pluginId}`);
      let changed: boolean;
      try {
        changed = await build(missing, this.stopping.signal);
      } catch (error) {
        this.models.delete(missing.pluginId);
        if (!this.closed) this.ports.log(`${MODELS.failedPrefix} ${missing.pluginId}: ${error instanceof Error ? error.message : String(error)}`);
        return;
      }
      if (this.closed) return;
      this.ports.log(`${changed ? MODELS.readyPrefix : MODELS.unchangedPrefix} ${missing.pluginId}`);
      if (changed) this.swap(true);
    });
  }

  private swap(again = false): void {
    if (this.closed) return;
    if (this.candidate !== null) {
      this.swapAgain ||= again;
      return;
    }
    if (this.state !== "ready") {
      this.swapWanted = this.state !== "failed";
      return;
    }
    this.swapWanted = false;
    try {
      const child = this.startWarm();
      this.candidate = child;
      if (this.modern) this.promote(child);
      else {
        child.port.send(this.handshake());
        this.cancelHandshake = this.ports.schedule?.(HANDSHAKE.candidateMilliseconds, () => {
          if (this.candidate === child) this.refreshFailed(`the new server did not answer the handshake within ${HANDSHAKE.candidateMilliseconds} ms`);
        }) ?? (() => {});
      }
    } catch (error) {
      this.refreshFailed(error instanceof Error ? error.message : String(error));
    }
  }

  private candidateFrame(child: WarmChild, frame: WarmMcpFrame): void {
    if (typeof frame.id === "string" && frame.id.startsWith(REPLAY_PREFIX)) return;
    if (frame.id !== WARM_MCP_HANDSHAKE_ID) return this.ports.log(`[semio-os-mcp] dropped a frame received before the handshake finished`);
    if (frame.error !== undefined) return this.refreshFailed(`the new server refused the handshake: ${JSON.stringify(frame.error)}`);
    child.port.send({ jsonrpc: "2.0", method: "notifications/initialized" });
    this.replay(child);
    this.promote(child);
  }

  private observe(frame: WarmMcpFrame): void {
    const uri = (frame.params as { uri?: unknown } | null | undefined)?.uri;
    if (frame.method === "logging/setLevel") this.level = frame.params;
    else if (frame.method === "resources/subscribe" && typeof uri === "string") this.subscriptions.add(uri);
    else if (frame.method === "resources/unsubscribe" && typeof uri === "string") this.subscriptions.delete(uri);
    else if (frame.method === "notifications/roots/list_changed") this.rootsChanged = true;
  }

  private replay(child: WarmChild): void {
    const request = (method: string, params: unknown): void => child.port.send({ jsonrpc: "2.0", id: `${REPLAY_PREFIX}${++this.replays}`, method, params });
    if (this.level !== null) request("logging/setLevel", this.level);
    for (const uri of this.subscriptions) request("resources/subscribe", { uri });
    if (this.rootsChanged) child.port.send({ jsonrpc: "2.0", method: "notifications/roots/list_changed" });
  }

  private forget(child: WarmChild): void {
    for (const [id, entry] of this.forwarded) if (entry.child === child) this.forwarded.delete(id);
  }

  private relayInitiated(child: WarmChild, frame: WarmMcpFrame): void {
    if (frame.method === undefined) return;
    const mapped = (id: number | string): string | undefined => [...this.forwarded].find(([, entry]) => entry.child === child && entry.id === id)?.[0];
    if (frame.id === undefined || frame.id === null) {
      const requestId = (frame.params as { requestId?: unknown } | null | undefined)?.requestId;
      const gateway = frame.method === "notifications/cancelled" && (typeof requestId === "number" || typeof requestId === "string") ? mapped(requestId) : undefined;
      return this.ports.toClient(gateway === undefined ? frame : { ...frame, params: { ...(frame.params as object), requestId: gateway } });
    }
    if (child === this.server && child.generation === 0) {
      child.asked.add(frame.id);
      return this.ports.toClient(frame);
    }
    const id = `${FORWARD_PREFIX}${++this.requests}`;
    this.forwarded.set(id, { child, id: frame.id });
    this.ports.toClient({ ...frame, id });
  }

  private settleCandidate(): void {
    if (!this.swapAgain) return;
    this.swapAgain = false;
    this.swap();
  }

  private promote(child: WarmChild): void {
    this.cancelHandshake();
    const previous = this.server!;
    this.candidate = null;
    this.server = child;
    this.retiring.push(previous);
    this.release(previous);
    if (this.restarting) {
      this.restarting = false;
      this.ports.log(SUPERVISION.recoveredLine);
      for (const queued of this.queue.splice(0)) this.send(child, queued);
    }
    this.announce();
    this.settleCandidate();
  }

  private release(child: WarmChild): void {
    if (child.pending.size !== 0) return;
    this.retiring.splice(this.retiring.indexOf(child), 1);
    this.forget(child);
    child.port.stop();
  }

  private retiringFrame(child: WarmChild, frame: WarmMcpFrame): void {
    if (!isResponse(frame)) return this.relayInitiated(child, frame);
    if (!child.pending.delete(frame.id)) return this.ports.log(REFRESH.droppedFrame);
    this.ports.toClient(frame);
    this.release(child);
  }

  private refreshFailed(reason: string): void {
    this.cancelHandshake();
    this.ports.log(`${REFRESH.failedPrefix} ${reason}`);
    this.candidate?.port.stop();
    this.candidate = null;
    this.recover(reason);
    this.settleCandidate();
  }

  private announce(): void {
    for (const method of LIST_CHANGED) this.ports.toClient({ jsonrpc: "2.0", method });
  }

  private dispatch(frame: WarmMcpFrame): void {
    const children = [this.server!, ...this.retiring];
    if (isResponse(frame)) {
      const entry = typeof frame.id === "string" ? this.forwarded.get(frame.id) : undefined;
      if (entry !== undefined) {
        this.forwarded.delete(frame.id as string);
        return entry.child.port.send({ ...frame, id: entry.id });
      }
      const asker = children.find(child => child.asked.delete(frame.id));
      return (asker ?? this.server!).port.send(frame);
    }
    const requestId = frame.method === "notifications/cancelled" ? (frame.params as { requestId?: unknown } | null | undefined)?.requestId : undefined;
    const owner = typeof requestId === "number" || typeof requestId === "string" ? children.find(child => child.pending.has(requestId)) : undefined;
    if (owner === undefined) return this.send(this.server!, frame);
    owner.port.send(frame);
    owner.pending.delete(requestId as number | string);
    if (owner !== this.server) this.release(owner);
  }

  private send(child: WarmChild, frame: WarmMcpFrame): void {
    if (frame.method !== undefined && frame.id !== undefined && frame.id !== null) child.pending.add(frame.id);
    child.port.send(frame);
  }

  private fail(reason: string): void {
    this.failure = reason;
    this.state = "failed";
    this.ports.log(`[semio-os-mcp] unavailable: ${reason}`);
    for (const queued of this.queue.splice(0)) this.refuse(queued);
  }

  private refuse(frame: WarmMcpFrame): void {
    if (frame.method !== undefined && frame.id !== undefined && frame.id !== null) this.ports.toClient({ jsonrpc: "2.0", id: frame.id, error: { code: -32603, message: `semio-os-mcp is unavailable: ${this.failure}` } });
  }
}

/** 🛰️ How a launch brings the real server up. `stage` runs the refresh and reports the exit status; it may call `serving` as soon as a previously published usable server can already run (`command()` is then valid), so the gateway starts it without waiting. After a successful stage `changed` says whether the published binary, catalog or component model set differ from what is serving; absent means changed. `models` builds and publishes the component model of a plugin the serving server reported missing and says whether the model set changed. */
export type WarmMcpLaunch = Readonly<{ expected?: Promise<boolean>; stage: (progress: (line: string) => void, signal: AbortSignal, serving: () => void) => Promise<number>; changed?: () => boolean; models?: (model: MissingMcpModelV1, progress: (line: string) => void, signal: AbortSignal) => Promise<boolean>; command: () => string; args: readonly string[]; cwd: string; env: NodeJS.ProcessEnv; input: Readable; output: Writable; onStop?: () => void }>;

const parseFrame = (line: string): WarmMcpFrame | null => {
  try {
    const value: unknown = JSON.parse(line);
    return value !== null && typeof value === "object" && !Array.isArray(value) ? (value as WarmMcpFrame) : null;
  } catch {
    return null;
  }
};

/** 🚀️ Serves MCP over `launch.input`/`launch.output` at once, stages the real server in the background through `launch.stage` and resolves with the exit status once the client or the real server ends the session. When `stage` reports a usable published server first, that server answers at once and the refresh swaps in behind it. stdout carries protocol frames only. */
export function serveWarmMcpGateway(launch: WarmMcpLaunch): Promise<number> {
  return new Promise((resolve) => {
    const log = (line: string): void => void process.stderr.write(`${line}\n`);
    const live = new Set<ChildProcess>();
    let finished = false, stopping = false;
    let requestedExit: number | undefined;
    const timers: ReturnType<typeof setTimeout>[] = [];
    const shutdown = policy.definitions.Shutdown.const;
    const staging = new AbortController();
    const signals = ["SIGINT", "SIGTERM", "SIGHUP"] as const;
    const interrupt = (): void => {
      stop(143);
    };
    for (const signal of signals) process.once(signal, interrupt);
    const finish = (code: number): void => {
      if (finished) return;
      finished = true;
      for (const signal of signals) process.off(signal, interrupt);
      for (const timer of timers) clearTimeout(timer);
      staging.abort();
      launch.onStop?.();
      resolve(code);
    };
    const stop = (code: number): void => {
      if (finished || stopping) return;
      stopping = true;
      requestedExit = code;
      staging.abort();
      launch.onStop?.();
      gateway.clientClosed();
      if (live.size === 0) return finish(code);
      timers.push(setTimeout(() => finish(1), shutdown.graceMilliseconds + shutdown.terminationMilliseconds + shutdown.killMilliseconds));
    };
    const gateway = new WarmMcpGateway({
      expecting: launch.expected !== undefined,
      toClient: (frame) => void launch.output.write(`${JSON.stringify(frame)}\n`),
      log,
      abandon: () => stop(1),
      ...(launch.models === undefined ? {} : { prepareModel: (model: MissingMcpModelV1, signal: AbortSignal) => launch.models!(model, log, signal) }),
      schedule: (milliseconds, run) => {
        const timer = setTimeout(run, milliseconds);
        timers.push(timer);
        return () => clearTimeout(timer);
      },
      startServer: () => {
        const spawned = spawn(launch.command(), [...launch.args], { windowsHide: true, cwd: launch.cwd, env: launch.env, stdio: ["pipe", "pipe", "inherit"] });
        live.add(spawned);
        let ended = false;
        const reaper: ReturnType<typeof setTimeout>[] = [];
        const port: WarmMcpServerPort = {
          send: (frame) => void spawned.stdin!.write(`${JSON.stringify(frame)}\n`),
          stop: () => {
            if (ended) return;
            ended = true;
            spawned.stdin!.end();
            reaper.push(setTimeout(() => {
              spawned.kill("SIGTERM");
              reaper.push(setTimeout(() => spawned.kill("SIGKILL"), shutdown.terminationMilliseconds));
            }, shutdown.graceMilliseconds));
          },
        };
        const settle = (): void => {
          ended = true;
          live.delete(spawned);
          for (const timer of reaper) clearTimeout(timer);
        };
        createInterface({ input: spawned.stdout! }).on("line", (line) => {
          const frame = parseFrame(line);
          if (frame === null) log(`[semio-os-mcp] non-protocol stdout line ignored: ${line.slice(0, 200)}`);
          else gateway.fromServer(frame, port);
        });
        spawned.on("close", (code, signal) => {
          settle();
          const role = gateway.role(port);
          const restarted = gateway.serverExited(code, port, signal);
          if (stopping) return live.size === 0 ? finish(requestedExit ?? 1) : undefined;
          if (role === "current" && !restarted) stop(code === 0 && gateway.phase === "failed" ? 1 : code ?? 1);
        });
        spawned.on("error", (error) => {
          settle();
          const role = gateway.role(port);
          gateway.serverExited(null, port);
          log(`[semio-os-mcp] ${error.message}`);
          if (role === "current") stop(1);
        });
        spawned.stdin!.on("error", error => {
          if (stopping || ended) return;
          const role = gateway.role(port);
          log(`[semio-os-mcp] ${error.message}`);
          if (role === "current") stop(1);
          else gateway.portFailed(port, error.message);
        });
        return port;
      },
    });
    createInterface({ input: launch.input }).on("line", (line) => {
      const frame = parseFrame(line);
      if (frame !== null) gateway.fromClient(frame);
    }).on("close", () => {
      stop(0);
    });
    void launch.expected?.then((expected) => { if (!expected) gateway.discoveryUnavailable(); });
    void launch.stage(log, staging.signal, () => gateway.markUsable()).then(
      (status) => { if (!finished) gateway.stagingFinished(status, status === 0 ? launch.changed?.() ?? true : true); },
      (error: unknown) => {
        if (!staging.signal.aborted) { log(`[semio-os-mcp] ${String(error)}`); gateway.stagingFinished(1); }
      },
    );
  });
}
