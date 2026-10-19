import { type ChildProcess, spawn } from "node:child_process";
import { createInterface } from "node:readline";
import type { Readable, Writable } from "node:stream";

export type WarmMcpFrame = { readonly jsonrpc?: "2.0"; readonly id?: number | string | null; readonly method?: string; readonly params?: unknown; readonly result?: unknown; readonly error?: unknown };

/** 🪪️ The `serverInfo.name` the gateway answers with while the real server is still being staged. */
export const WARM_MCP_SERVER_NAME = "semio";

/** 🪪️ The id the gateway handshakes with the real server under; it never collides with a client id. */
export const WARM_MCP_HANDSHAKE_ID = "semio-gateway-initialize";

const LIST_FIELDS: Readonly<Record<string, string>> = { "tools/list": "tools", "resources/list": "resources", "resources/templates/list": "resourceTemplates", "prompts/list": "prompts" };
const LIST_CHANGED: readonly string[] = ["notifications/tools/list_changed", "notifications/resources/list_changed", "notifications/prompts/list_changed"];

/** 🌡️ What the gateway answers by itself while the real server is not up: `initialize`, `ping` and every list method. `null` means the request must wait for the real server. */
export function warmMcpReply(frame: WarmMcpFrame): WarmMcpFrame | null {
  if (frame.method === undefined || frame.id === undefined || frame.id === null) return null;
  const reply = (result: unknown): WarmMcpFrame => ({ jsonrpc: "2.0", id: frame.id, result });
  if (frame.method === "initialize") {
    const requested = (frame.params as { protocolVersion?: unknown } | undefined)?.protocolVersion;
    const listChanged = { listChanged: true };
    return reply({ protocolVersion: typeof requested === "string" ? requested : "2025-06-18", capabilities: { tools: listChanged, resources: listChanged, prompts: listChanged }, serverInfo: { name: WARM_MCP_SERVER_NAME, version: "warming" }, instructions: "The semio server is starting; its tools appear through list_changed notifications once its binary is staged." });
  }
  if (frame.method === "ping") return reply({});
  const field = LIST_FIELDS[frame.method];
  return field === undefined ? null : reply({ [field]: [] });
}

export type WarmMcpServerPort = Readonly<{ send: (frame: WarmMcpFrame) => void; stop: () => void }>;

export type WarmMcpPorts = Readonly<{ toClient: (frame: WarmMcpFrame) => void; startServer: () => WarmMcpServerPort; log: (line: string) => void }>;

type WarmState = "warming" | "handshaking" | "ready" | "failed";

/** 🚪️ A protocol-level gateway that answers the client's handshake at once, queues every other request, and splices in the real server once it is staged. Pure state machine: all IO flows through {@link WarmMcpPorts}. */
export class WarmMcpGateway {
  private state: WarmState = "warming";
  private staged = false;
  private initialize: WarmMcpFrame | null = null;
  private initialized = false;
  private server: WarmMcpServerPort | null = null;
  private failure = "";
  private readonly queue: WarmMcpFrame[] = [];

  constructor(private readonly ports: WarmMcpPorts) {}

  get phase(): WarmState {
    return this.state;
  }

  fromClient(frame: WarmMcpFrame): void {
    if (this.state === "ready") return this.server!.send(frame);
    if (this.state === "failed") return this.refuse(frame);
    if (frame.method === "initialize" && this.initialize === null) this.initialize = frame;
    if (frame.method === "notifications/initialized") this.initialized = true;
    const reply = warmMcpReply(frame);
    if (reply !== null) this.ports.toClient(reply);
    else if (frame.method !== "notifications/initialized") this.queue.push(frame);
    this.begin();
  }

  stagingFinished(status: number): void {
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
    if (this.state === "handshaking") this.fail(`the server exited with ${code}`);
  }

  clientClosed(): void {
    this.server?.stop();
  }

  private begin(): void {
    if (this.state !== "warming" || !this.staged || this.initialize === null) return;
    this.state = "handshaking";
    this.server = this.ports.startServer();
    this.server.send({ jsonrpc: "2.0", id: WARM_MCP_HANDSHAKE_ID, method: "initialize", params: this.initialize.params });
  }

  private fail(reason: string): void {
    this.failure = reason;
    this.state = "failed";
    this.ports.log(`[semio-os-mcp] unavailable: ${reason}`);
    for (const queued of this.queue.splice(0)) this.refuse(queued);
  }

  private refuse(frame: WarmMcpFrame): void {
    const reply = warmMcpReply(frame);
    if (reply !== null) return this.ports.toClient(reply);
    if (frame.method !== undefined && frame.id !== undefined && frame.id !== null) this.ports.toClient({ jsonrpc: "2.0", id: frame.id, error: { code: -32603, message: `semio-os-mcp is unavailable: ${this.failure}` } });
  }
}

export type WarmMcpLaunch = Readonly<{ stage: (progress: (line: string) => void, signal: AbortSignal) => Promise<number>; command: () => string; args: readonly string[]; cwd: string; env: NodeJS.ProcessEnv; input: Readable; output: Writable }>;

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
    let finished = false;
    const staging = new AbortController();
    const signals = ["SIGINT", "SIGTERM", "SIGHUP"] as const;
    const interrupt = (): void => {
      child?.kill();
      finish(143);
    };
    for (const signal of signals) process.once(signal, interrupt);
    const finish = (code: number): void => {
      if (finished) return;
      finished = true;
      for (const signal of signals) process.off(signal, interrupt);
      staging.abort();
      resolve(code);
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
        spawned.on("exit", (code) => {
          gateway.serverExited(code);
          finish(code ?? 1);
        });
        spawned.on("error", (error) => {
          gateway.serverExited(null);
          log(`[semio-os-mcp] ${error.message}`);
          finish(1);
        });
        return { send: (frame) => void spawned.stdin!.write(`${JSON.stringify(frame)}\n`), stop: () => void spawned.stdin!.end() };
      },
    });
    createInterface({ input: launch.input }).on("line", (line) => {
      const frame = parseFrame(line);
      if (frame !== null) gateway.fromClient(frame);
    }).on("close", () => {
      gateway.clientClosed();
      if (child === null) finish(0);
    });
    void launch.stage(log, staging.signal).then(
      (status) => gateway.stagingFinished(status),
      (error: unknown) => {
        log(`[semio-os-mcp] ${String(error)}`);
        gateway.stagingFinished(1);
      },
    );
  });
}
