import policy from "./🧬️schema/🔣️.json" with {type:"json"};
import serverInstructions from "../../🧭️protocol/🧬️schema/📜️instructions.json" with {type:"json"};
import protocolVersions from "../../🧭️protocol/🧬️schema/🗓️versions.json" with {type:"json"};
import { type ChildProcess, spawn } from "node:child_process";
import { createInterface } from "node:readline";
import type { Readable, Writable } from "node:stream";

export type WarmMcpFrame = { readonly jsonrpc?: "2.0"; readonly id?: number | string | null; readonly method?: string; readonly params?: unknown; readonly result?: unknown; readonly error?: unknown };

/** 🪪️ The `serverInfo.name` the gateway answers with while the real server is still being staged. */
export const WARM_MCP_SERVER_NAME = "semio";

/** 🪪️ The id the gateway handshakes with the real server under; it never collides with a client id. */
export const WARM_MCP_HANDSHAKE_ID = "semio-gateway-initialize";

const LIST_CHANGED: readonly string[] = ["notifications/tools/list_changed", "notifications/resources/list_changed", "notifications/prompts/list_changed"];

/** 🌡️ Answers `initialize` and `ping` during staging. Other requests wait for the native server. */
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
      instructions: `${serverInstructions}\nThe semio server is starting. Discovery and tool requests wait for the native server.`,
    });
  }
  if (frame.method === "ping") return reply({});
  return null;
}

export type WarmMcpServerPort = Readonly<{ send: (frame: WarmMcpFrame) => void; stop: () => void }>;

export type WarmMcpPorts = Readonly<{ toClient: (frame: WarmMcpFrame) => void; startServer: () => WarmMcpServerPort; log: (line: string) => void }>;

type WarmState = "warming" | "handshaking" | "ready" | "failed";

/** 🚪️ Answers the initial handshake and pings, queues discovery and tool requests, and relays frames once the native server is ready. */
export class WarmMcpGateway {
  private state: WarmState = "warming";
  private staged = false;
  private closed = false;
  private initialize: WarmMcpFrame | null = null;
  private initialized = false;
  private modern = false;
  private server: WarmMcpServerPort | null = null;
  private failure = "";
  private readonly queue: WarmMcpFrame[] = [];

  private readonly ports: WarmMcpPorts;

  constructor(ports: WarmMcpPorts) {
    this.ports = ports;
  }

  get phase(): WarmState {
    return this.state;
  }

  fromClient(frame: WarmMcpFrame): void {
    if (this.closed) return;
    if (this.state === "ready") return this.server!.send(frame);
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
    const reply = this.modern ? null : warmMcpReply(frame);
    if (reply !== null) this.ports.toClient(reply);
    else if (frame.method !== "notifications/initialized") this.queue.push(frame);
    this.begin();
  }

  stagingFinished(status: number): void {
    if (this.closed) return;
    if (status !== 0) return this.fail(`staging exited with status ${status}`);
    this.staged = true;
    this.begin();
  }

  fromServer(frame: WarmMcpFrame): void {
    if (this.state === "ready") return this.ports.toClient(frame);
    if (this.state !== "handshaking" || frame.id !== WARM_MCP_HANDSHAKE_ID) return this.ports.log(`[semio-os-mcp] dropped a frame received before the handshake finished`);
    if (frame.error !== undefined) return this.fail(`the server refused the handshake: ${JSON.stringify(frame.error)}`);
    this.server!.send({ jsonrpc: "2.0", method: "notifications/initialized" });
    this.state = "ready";
    for (const queued of this.queue.splice(0)) this.server!.send(queued);
    for (const method of LIST_CHANGED) this.ports.toClient({ jsonrpc: "2.0", method });
  }

  serverExited(code: number | null): void {
    if (!this.closed && this.state === "handshaking") this.fail(`the server exited with ${code}`);
  }

  clientClosed(): void {
    this.closed = true;
    this.queue.length = 0;
    this.server?.stop();
  }

  private begin(): void {
    if (this.state !== "warming" || this.closed || !this.staged || !this.modern && (!this.initialized || this.initialize === null)) return;
    this.state = this.modern ? "ready" : "handshaking";
    try {
      this.server = this.ports.startServer();
      if (this.modern) for (const queued of this.queue.splice(0)) this.server.send(queued);
      else this.server.send({ jsonrpc: "2.0", id: WARM_MCP_HANDSHAKE_ID, method: "initialize", params: this.initialize!.params });
    } catch (error) {
      this.fail(error instanceof Error ? error.message : String(error));
    }
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

export type WarmMcpLaunch = Readonly<{ stage: (progress: (line: string) => void, signal: AbortSignal) => Promise<number>; command: () => string; args: readonly string[]; cwd: string; env: NodeJS.ProcessEnv; input: Readable; output: Writable; onStop?: () => void }>;

const parseFrame = (line: string): WarmMcpFrame | null => {
  try {
    const value: unknown = JSON.parse(line);
    return value !== null && typeof value === "object" && !Array.isArray(value) ? (value as WarmMcpFrame) : null;
  } catch {
    return null;
  }
};

/** 🚀️ Serves MCP over `launch.input`/`launch.output` at once, stages the real server in the background through `launch.stage` and resolves with the exit status once the client or the real server ends the session. stdout carries protocol frames only. */
export function serveWarmMcpGateway(launch: WarmMcpLaunch): Promise<number> {
  return new Promise((resolve) => {
    const log = (line: string): void => void process.stderr.write(`${line}\n`);
    let child: ChildProcess | null = null;
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
      if (child === null) return finish(code);
      timers.push(setTimeout(() => {
        child?.kill("SIGTERM");
        timers.push(setTimeout(() => {
          child?.kill("SIGKILL");
          timers.push(setTimeout(() => finish(1), shutdown.killMilliseconds));
        }, shutdown.terminationMilliseconds));
      }, shutdown.graceMilliseconds));
    };
    const gateway = new WarmMcpGateway({
      toClient: (frame) => void launch.output.write(`${JSON.stringify(frame)}\n`),
      log,
      startServer: () => {
        const spawned = spawn(launch.command(), [...launch.args], { cwd: launch.cwd, env: launch.env, stdio: ["pipe", "pipe", "inherit"] });
        child = spawned;
        createInterface({ input: spawned.stdout! }).on("line", (line) => {
          const frame = parseFrame(line);
          if (frame === null) log(`[semio-os-mcp] non-protocol stdout line ignored: ${line.slice(0, 200)}`);
          else gateway.fromServer(frame);
        });
        spawned.on("close", (code) => {
          gateway.serverExited(code);
          const status = code === 0 && gateway.phase === "failed" ? 1 : code ?? 1;
          finish(requestedExit ?? status);
        });
        spawned.on("error", (error) => {
          gateway.serverExited(null);
          log(`[semio-os-mcp] ${error.message}`);
          finish(1);
        });
        spawned.stdin!.on("error", error => { if (!stopping) { log(`[semio-os-mcp] ${error.message}`); stop(1); } });
        return { send: (frame) => void spawned.stdin!.write(`${JSON.stringify(frame)}\n`), stop: () => void spawned.stdin!.end() };
      },
    });
    createInterface({ input: launch.input }).on("line", (line) => {
      const frame = parseFrame(line);
      if (frame !== null) gateway.fromClient(frame);
    }).on("close", () => {
      stop(0);
    });
    void launch.stage(log, staging.signal).then(
      (status) => { if (!finished) gateway.stagingFinished(status); },
      (error: unknown) => {
        if (!staging.signal.aborted) { log(`[semio-os-mcp] ${String(error)}`); gateway.stagingFinished(1); }
      },
    );
  });
}
