#!/usr/bin/env bun
/** 💬️ The agent's own voice, end to end, with nothing mocked on either side — ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AC1, audit `📓️g19-ai-user-experience-audit.md`
 * gap 1. A real `semio-os-mcp` stdio gateway launched exactly as `.mcp.json` launches it, a real
 * React `dev` session in a real browser, and between them only `🛰️rendezvous` and `🔗️AgentBridge`.
 *
 * What it proves, in order:
 *   0. rendezvous — this gateway's own offer becomes readable on the dev session;
 *   1. `conversation_reply` is in the live `tools/list` with an object input schema;
 *   2. the scope is real: a gateway granted `ui.control` but NOT `conversation.write` is refused
 *      with a typed `PERMISSION_DENIED` naming `shell.converse`, so the tool is not a free channel
 *      into the human's screen;
 *   3. a streamed turn (two chunks, one `replyId`) becomes ONE row in the LIVE shell, `streaming`
 *      then `complete`, carrying the concatenated text — the whole point of the slice;
 *   4. that row does NOT also appear as a `conversation_reply` tool-call row (the declared
 *      `SELF_PUBLISHING_CONVERSATION_TOOLS` suppression, measured in the real DOM);
 *   5. the reverse direction: a turn typed into the REAL composer reaches the agent, read back
 *      through `semio://ui/agent-messages` by the same MCP client.
 *
 * Two preconditions are refused before any step runs, because a gate that proceeds without them
 * reports a verdict about a different tree: the serve must answer `${SHELL_URL}`, and its
 * transformed `🔗️AgentBridge` must carry the `agentReply` frame branch this slice added.
 *
 * Environment: `S_OS_MCP_LIVE_SHELL_URL` (default `http://127.0.0.1:6197`) is an ALREADY RUNNING
 * dev session; `S_OS_MCP_LIVE_PLUGIN` (default `s`) is the variant it serves;
 * `S_AGENT_BRIDGE_DIR` is inherited untouched (the gateway must meet the session in the rendezvous
 * THAT session published into); `S_OS_MCP_AGENT_REPLY_SHOT` overrides where the screenshot lands.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, type Page } from "playwright";
import { parseAgentBridgeOffer, sameAgentBridgeOffer, type AgentBridgeConfig } from "../../../📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PROTOCOL_VERSION = "2025-06-18";
const PLUGIN = process.env.S_OS_MCP_LIVE_PLUGIN ?? "s";
const SHELL_ORIGIN = process.env.S_OS_MCP_LIVE_SHELL_URL ?? "http://127.0.0.1:6197";
const SHELL_URL = `${SHELL_ORIGIN}/?plugin=${PLUGIN}`;
const OFFER_URL = `${SHELL_ORIGIN}/__semio/agent-bridge`;
const CALL_BUDGET_MS = 120_000;
/** 🧱️ The one module the reply branch lives in. A serve transforming an older copy of it renders
 * nothing for a frame it has never heard of, and the gate would report on somebody else's code. */
const AGENT_BRIDGE_MODULE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx";
/** 📸️ `new URL(x, import.meta.url).pathname` percent-encodes emoji path segments and silently
 * writes outside the ticket folder; `fileURLToPath` is the one that does not. */
const SCREENSHOT = process.env.S_OS_MCP_AGENT_REPLY_SHOT ?? join(REPO_ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/ac1-agent-reply-panel.png");

type ServerEntry = { readonly command: string; readonly args: readonly string[] };

const outcomes: { step: string; state: "pass" | "fail" | "skip"; detail: string }[] = [];
function record(step: string, state: "pass" | "fail" | "skip", detail: string): void {
  outcomes.push({ step, state, detail });
  console.log(`${state.toUpperCase().padEnd(4)} ${step} :: ${detail}`);
}

/** 🔌️ One live `semio-os-mcp` stdio connection, driven exactly as a real MCP client drives it. */
class Peer {
  private readonly pending = new Map<string | number, (envelope: Record<string, unknown>) => void>();
  readonly diagnostics: string[] = [];
  private buffer = "";
  private nextId = 1;
  // 🧭️ No annotation: `Bun.spawn` is declared twice (bun-types and the repo's own ambient), so
  // `ReturnType<typeof Bun.spawn>` loses `stdin.write`. Inferring from the assignment binds the
  // signature this call actually resolved to.
  private readonly child;

  constructor(command: string, args: readonly string[], environment: Record<string, string> = {}) {
    this.child = Bun.spawn([command, ...args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, ...environment } });
    void this.pump();
    void this.collectDiagnostics();
  }

  private async pump(): Promise<void> {
    const decoder = new TextDecoder();
    for await (const chunk of this.child.stdout) {
      this.buffer += decoder.decode(chunk);
      let newline = this.buffer.indexOf("\n");
      while (newline >= 0) {
        const line = this.buffer.slice(0, newline).trim();
        this.buffer = this.buffer.slice(newline + 1);
        if (line) this.deliver(line);
        newline = this.buffer.indexOf("\n");
      }
    }
  }

  private deliver(line: string): void {
    let envelope: Record<string, any>;
    try {
      envelope = JSON.parse(line);
    } catch {
      this.diagnostics.push(`unparseable stdout line: ${line.slice(0, 200)}`);
      return;
    }
    if (typeof envelope.method === "string") return;
    const resolve = envelope.id !== undefined ? this.pending.get(envelope.id) : undefined;
    if (resolve) {
      this.pending.delete(envelope.id);
      resolve(envelope);
    }
  }

  private async collectDiagnostics(): Promise<void> {
    const decoder = new TextDecoder();
    for await (const chunk of this.child.stderr) for (const line of decoder.decode(chunk).split("\n")) if (line.trim()) this.diagnostics.push(line.trim());
  }

  private write(envelope: unknown): void {
    this.child.stdin.write(`${JSON.stringify(envelope)}\n`);
    void this.child.stdin.flush();
  }

  notify(method: string, params: unknown): void {
    this.write({ jsonrpc: "2.0", method, params });
  }

  async request(method: string, params: unknown, budgetMs = CALL_BUDGET_MS): Promise<Record<string, any>> {
    const id = this.nextId++;
    const answered = new Promise<Record<string, any>>((resolve, reject) => {
      this.pending.set(id, resolve as (envelope: Record<string, unknown>) => void);
      setTimeout(() => {
        if (!this.pending.delete(id)) return;
        reject(new Error(`${method} did not answer within ${budgetMs}ms; stderr tail: ${this.diagnostics.slice(-3).join(" | ")}`));
      }, budgetMs);
    });
    this.write({ jsonrpc: "2.0", id, method, params });
    return answered;
  }

  call(name: string, args: unknown, budgetMs = CALL_BUDGET_MS): Promise<Record<string, any>> {
    return this.request("tools/call", { name, arguments: args }, budgetMs);
  }

  stop(): void {
    this.child.kill();
  }
}

function semioEntry(): ServerEntry {
  const config = JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, ServerEntry> };
  const entry = config.mcpServers.semio;
  if (!entry) throw new Error(".mcp.json declares no `semio` server — the gate drives the shipped launch line, never a hand-rolled one");
  return entry;
}

/** 🔐️ The same launch line with `conversation.write` REMOVED — the negative control for step 2.
 * Built by rewriting the shipped `--scopes` rather than by hand, so it can never drift into
 * granting something the real config does not. */
function unscopedEntry(entry: ServerEntry): ServerEntry {
  const args = [...entry.args];
  const at = args.indexOf("--scopes");
  if (at < 0) throw new Error(".mcp.json's `semio` server passes no --scopes — the negative control has nothing to narrow");
  args[at + 1] = String(args[at + 1])
    .split(",")
    .filter((scope) => scope.trim() !== "conversation.write")
    .join(",");
  return { command: entry.command, args };
}

async function open(label: string, entry: ServerEntry, environment: Record<string, string> = {}): Promise<Peer> {
  console.log(`\n=== ${label}\n    ${entry.command} ${entry.args.join(" ")}`);
  const peer = new Peer(entry.command, entry.args, environment);
  const initialized = await peer.request("initialize", {
    protocolVersion: PROTOCOL_VERSION,
    capabilities: { roots: { listChanged: true } },
    clientInfo: { name: "os-mcp-agent-reply", title: "OS MCP Agent Reply", version: "1" },
  });
  if (initialized.error) throw new Error(`${label}: initialize failed: ${JSON.stringify(initialized.error)}`);
  peer.notify("notifications/initialized", {});
  return peer;
}

function structured(envelope: Record<string, any>): Record<string, any> {
  return (envelope.result?.structuredContent ?? {}) as Record<string, any>;
}

function describe(envelope: Record<string, any>): string {
  if (envelope.error) return `JSON-RPC error ${envelope.error.code}: ${envelope.error.message}`;
  return `isError=${envelope.result?.isError} :: ${JSON.stringify(envelope.result?.structuredContent ?? envelope.result?.content ?? {}).slice(0, 280)}`;
}

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

async function until<T>(label: string, budgetMs: number, intervalMs: number, probe: () => Promise<T | null>): Promise<T | null> {
  const deadline = Date.now() + budgetMs;
  while (Date.now() < deadline) {
    const value = await probe();
    if (value) return value;
    await sleep(intervalMs);
  }
  console.log(`  (timed out waiting for ${label} after ${budgetMs}ms)`);
  return null;
}

/** 🖥️ Everything this gate reads out of the live shell, in one evaluate. */
function readShell(page: Page) {
  return page.evaluate(() => ({
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    shellError: document.documentElement.getAttribute("data-semio-os-error"),
    presenceTone: document.querySelector("[data-semio-agent-presence-tone]")?.getAttribute("data-semio-agent-presence-tone") ?? null,
    chatPanel: document.querySelector("[data-semio-agent-chat-panel]") !== null,
    replies: [...document.querySelectorAll("[data-semio-agent-chat-entry='agentMessage']")].map((element) => ({
      id: element.id,
      state: element.getAttribute("data-agent-chat-state"),
      text: element.querySelector("[data-semio-agent-chat-reply]")?.textContent ?? "",
      inReplyTo: element.querySelector("[data-semio-agent-chat-reply]")?.getAttribute("data-semio-agent-chat-reply-to") ?? "",
      busy: element.querySelector("[data-semio-agent-chat-reply]")?.getAttribute("aria-busy") ?? "",
    })),
    toolRows: [...document.querySelectorAll("[data-semio-agent-chat-entry='toolCall']")].map((element) => element.textContent ?? ""),
    userRows: [...document.querySelectorAll("[data-semio-agent-chat-entry='userMessage']")].map((element) => element.textContent ?? ""),
    windowIds: [...new Set([...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))],
  }));
}

// ───────────────────────────── the run ─────────────────────────────
const entry = semioEntry();
const reachable = await fetch(SHELL_URL)
  .then((response) => response.ok)
  .catch(() => false);
if (!reachable) throw new Error(`no live dev session answers ${SHELL_URL}. Serve one first and re-run; point the gate elsewhere with S_OS_MCP_LIVE_SHELL_URL.`);

const servedBridge = await fetch(`${SHELL_ORIGIN}/@fs${join(REPO_ROOT, AGENT_BRIDGE_MODULE)}`)
  .then((response) => (response.ok ? response.text() : ""))
  .catch(() => "");
if (!servedBridge.includes("agentReply"))
  throw new Error(`${SHELL_ORIGIN} does not serve this tree's reply branch: the transformed \`🔗️AgentBridge\` carries no \`agentReply\` case. Restart that serve against this working tree before running the gate.`);

const browser = await chromium.launch({ headless: process.env.S_OS_MCP_LIVE_HEADED !== "1", args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();

let gateway: Peer | null = null;
let unscoped: Peer | null = null;
try {
  const readOffer = (): Promise<AgentBridgeConfig | null> =>
    fetch(OFFER_URL)
      .then(async (response) => (response.status === 200 ? parseAgentBridgeOffer(await response.json()) : null))
      .catch(() => null);
  const offerBefore = await readOffer();
  gateway = await open("gateway — .mcp.json args verbatim", entry);
  const offer = await until("the gateway's offer on the dev session", 60_000, 500, async () => {
    const published = await readOffer();
    return published === null || sameAgentBridgeOffer(published, offerBefore) ? null : published;
  });
  record("0 rendezvous", offer ? "pass" : "fail", offer ? `offer before=${offerBefore ? offerBefore.url : "(none)"}, after=${offer.url}` : `this gateway published no offer of its own; ${OFFER_URL} still answers ${offerBefore ? offerBefore.url : "no offer"}`);

  // 1. the tool exists on the live surface, with a real schema.
  const listed = await gateway.request("tools/list", {});
  const tools = ((listed.result?.tools ?? []) as Record<string, any>[]).map((tool) => tool);
  const reply = tools.find((tool) => tool.name === "conversation_reply") ?? null;
  record(
    "1 conversation_reply is on the live tool surface",
    reply && reply.inputSchema?.type === "object" && reply.outputSchema?.type === "object" ? "pass" : "fail",
    reply ? `title=${reply.title} input=${reply.inputSchema?.type} output=${reply.outputSchema?.type} (of ${tools.length} tools)` : `tools/list has ${tools.length} tools and none of them is conversation_reply`,
  );

  await page.goto(SHELL_URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  const booted = await until("shell boot", 240_000, 1000, async () => {
    const view = await readShell(page);
    return view.ready || view.shellError ? view : null;
  });
  record("0 boot", booted?.ready && !booted.shellError ? "pass" : "fail", booted ? `ready=${booted.ready} error=${booted.shellError} windows=${booted.windowIds.join(",")}` : "the shell never reached data-semio-os-ready");

  const attached = await until("the shell to register on /bridge", 90_000, 1000, async () => {
    const focused = await gateway!.call("ui_focus", {});
    return structured(focused).code === undefined ? focused : null;
  });
  record("0 the shell dials /bridge", attached ? "pass" : "fail", attached ? `ui_focus answered ${describe(attached)}` : "ui_focus never stopped answering PLUGIN_UNAVAILABLE — no shell registered");

  const revealed = await gateway.call("ui_reveal", { anchor: "right", path: ["framework.chat"] });
  const chatVisible = await until("the chat panel", 30_000, 500, async () => {
    const view = await readShell(page);
    return view.chatPanel ? view : null;
  });
  record("0 ui_reveal opens the chat panel", chatVisible ? "pass" : "fail", `${describe(revealed)}; chat panel in DOM=${chatVisible !== null}`);

  // 2. the scope is real — the NEGATIVE control, on its own gateway.
  unscoped = await open("gateway — same line, conversation.write removed", unscopedEntry(entry));
  const refused = await unscoped.call("conversation_reply", { text: "this must never reach a human's screen" });
  const refusal = structured(refused);
  record(
    "2 conversation.write gates the channel",
    refused.result?.isError === true && refusal.code === "PERMISSION_DENIED" && String(refusal.message ?? "").includes("shell.converse") ? "pass" : "fail",
    describe(refused),
  );

  // 3. a streamed turn becomes ONE row, streaming then complete.
  const toolRowsBefore = (await readShell(page)).toolRows.length;
  const replyId = `rep_gate_${Date.now()}`;
  const firstChunk = await gateway.call("conversation_reply", { text: "Widening that wall means", replyId, inReplyTo: "msg_gate", complete: false });
  const streaming = await until("the streaming reply row", 30_000, 200, async () => {
    const view = await readShell(page);
    return view.replies.find((row) => row.id.endsWith(replyId) && row.state === "streaming") ?? null;
  });
  const secondChunk = await gateway.call("conversation_reply", { text: " the 300 mm variant — shall I?", replyId });
  const settled = await until("the completed reply row", 30_000, 200, async () => {
    const view = await readShell(page);
    return view.replies.find((row) => row.id.endsWith(replyId) && row.state === "complete") ?? null;
  });
  const rowsForTurn = (await readShell(page)).replies.filter((row) => row.id.endsWith(replyId)).length;
  const expected = "Widening that wall means the 300 mm variant — shall I?";
  record(
    "3 a streamed turn is ONE row in the live shell",
    streaming && settled && rowsForTurn === 1 && settled.text === expected ? "pass" : "fail",
    `chunk1=${describe(firstChunk)} chunk2=${describe(secondChunk)}; streaming(busy=${streaming?.busy}) → complete(busy=${settled?.busy}); rows for this turn=${rowsForTurn}; inReplyTo=${settled?.inReplyTo}; text=${JSON.stringify(settled?.text ?? "")}`,
  );

  // 4. and it did NOT also print as a tool-call row.
  const toolRowsAfter = (await readShell(page)).toolRows;
  const duplicated = toolRowsAfter.filter((text) => text.includes("conversation_reply")).length;
  record(
    "4 the reply does not also print as a tool-call row",
    duplicated === 0 ? "pass" : "fail",
    `tool-call rows before=${toolRowsBefore} after=${toolRowsAfter.length}, of which naming conversation_reply=${duplicated} (SELF_PUBLISHING_CONVERSATION_TOOLS)`,
  );

  // 5. the reverse direction: a turn typed into the REAL composer reaches the agent.
  const typed = `gate turn ${Date.now()}`;
  const composer = page.locator("#framework\\.chat\\.draft").first();
  await composer.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  await composer.fill(typed).catch(() => undefined);
  await composer.press("Enter").catch(() => undefined);
  const echoed = await until("the human's own turn in the transcript", 20_000, 200, async () => {
    const view = await readShell(page);
    return view.userRows.some((text) => text.includes(typed)) ? view : null;
  });
  const inbox = await until("the agent's inbox to carry it", 30_000, 500, async () => {
    const read = await gateway!.request("resources/read", { uri: "semio://ui/agent-messages" });
    const body = String((read.result?.contents ?? [])[0]?.text ?? "");
    return body.includes(typed) ? body : null;
  });
  record(
    "5 a turn typed in the panel reaches the agent",
    echoed && inbox ? "pass" : "fail",
    echoed ? `echoed in the transcript; semio://ui/agent-messages = ${String(inbox ?? "(never carried it)").slice(0, 240)}` : "the composer never echoed the turn — the panel is not accepting input on this serve",
  );

  await page.screenshot({ path: SCREENSHOT, fullPage: false });
  console.log(`\nscreenshot → ${SCREENSHOT}`);
} finally {
  gateway?.stop();
  unscoped?.stop();
  await browser.close();
}

const failed = outcomes.filter((outcome) => outcome.state === "fail").length;
const skipped = outcomes.filter((outcome) => outcome.state === "skip").length;
console.log(`\nos-mcp-agent-reply: ${outcomes.length - failed - skipped} passed, ${failed} failed, ${skipped} skipped of ${outcomes.length}`);
process.exit(failed === 0 ? 0 : 1);
