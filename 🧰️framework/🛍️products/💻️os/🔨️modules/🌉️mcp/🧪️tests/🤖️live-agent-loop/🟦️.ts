#!/usr/bin/env bun
/** 🤖️ The live agent loop, end to end, with nothing mocked on either side: a real `semio-os-mcp`
 * stdio gateway launched exactly as `.mcp.json` launches it, a real React `dev` session in a real
 * browser, and between them only the rendezvous (`🛰️rendezvous` + `🔌️vite-plugins`'
 * `semio-agent-bridge-rendezvous`) and the dialling seam in `🔗️AgentBridge`.
 *
 * What it proves, in order:
 *   0. rendezvous — the gateway's offer becomes readable on the dev session's `/__semio/agent-bridge`;
 *   a. the shell dials that offer and agent presence renders in the shell's own chrome;
 *   b. a tool call shows as a running row and then as a settled result row;
 *   c. the running row's Cancel button cancels the call in flight;
 *   d. `ui_reveal`/`ui_focus` move the REAL dock and the REAL active window, not a mirror;
 *   e. a capability whose policy gates on approval raises the approval affordance IN THE SHELL —
 *      this client advertises no `elicitation`, which is the lane most MCP clients are on — and
 *      Approve Once / Deny / a silent elicitation-capable client each resolve it into their own
 *      typed outcome.
 *
 * (c) and (e) state their own precondition instead of asserting into thin air: (e) needs at least
 * one capability in the LIVE catalog whose `effects.destructive` is true (`ApprovalMode`'s
 * `whenDestructive` is what every plugin descriptor declares). A step whose precondition is absent
 * is reported `SKIP` with the reason and does not fail the gate; the moment such a capability
 * exists the step runs and is required like every other.
 *
 * Environment: `S_OS_MCP_LIVE_SHELL_URL` (default `http://127.0.0.1:6080`) is an ALREADY RUNNING
 * dev session to drive — the gate reuses it rather than paying an activation, and says so.
 * `S_OS_MCP_LIVE_PLUGIN` (default `note`) is the variant that session serves.
 * `S_OS_MCP_LIVE_MUTATION`/`S_OS_MCP_LIVE_MUTATION_INPUT` (default `addBlock` and its `{kind,x,y}`)
 * name that plugin's own UNCONDITIONAL mutation for the (f) chain. `S_OS_MCP_LIVE_LOCALE` (`en`,
 * default, or `de`) is the language of the human's browser; the approval affordance must speak it
 * (step `(i18n)`), so running the gate once per locale proves both. `S_AGENT_BRIDGE_DIR` is
 * inherited untouched: the gateway must meet the session in the rendezvous THAT session published
 * into, so a gate that minted its own would always find a 404 where the offer should be.
 *
 * Two preconditions are refused before any step runs, because a gate that proceeds without them
 * reports a verdict about a different tree or a different session (AP1, 2026-09-20): the serve must
 * answer `${SHELL_URL}`, and its transformed `🏛️ShellHost` must carry `agentArtifactRouteRef`. A
 * third is asserted as step 0 rather than assumed: the offer at `/__semio/agent-bridge` must be
 * THIS gateway's, not a dead file or a peer gateway's, which the endpoint serves in preference to
 * nothing.
 */
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { inflateRawSync, inflateSync } from "node:zlib";
import { chromium, type Page } from "playwright";
import { parseAgentBridgeOffer, sameAgentBridgeOffer, type AgentBridgeConfig } from "../../../📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PROTOCOL_VERSION = "2025-06-18";
const PLUGIN = process.env.S_OS_MCP_LIVE_PLUGIN ?? "note";
const SHELL_ORIGIN = process.env.S_OS_MCP_LIVE_SHELL_URL ?? "http://127.0.0.1:6080";
const SHELL_URL = `${SHELL_ORIGIN}/?plugin=${PLUGIN}`;
const OFFER_URL = `${SHELL_ORIGIN}/__semio/agent-bridge`;
const CALL_BUDGET_MS = 240_000;
/** ✍️ The verb the (f) chain drives, and the arguments it needs. It is deliberately NOT the (e)
 * chain's destructive target: a destructive verb is by nature conditional — `note`'s
 * `deleteSelection` emits nothing at all when nothing is selected — so asserting "the head moved"
 * on it measures the fixture's selection state, not the route. This one must be a verb whose own
 * handler mutates unconditionally, named per plugin like `S_OS_MCP_LIVE_PLUGIN` already is. */
const MUTATION_VERB = process.env.S_OS_MCP_LIVE_MUTATION ?? "addBlock";
const MUTATION_INPUT = JSON.parse(process.env.S_OS_MCP_LIVE_MUTATION_INPUT ?? `{"kind":"text","x":40,"y":40}`) as Record<string, unknown>;
/** 🪟️ The program to SPAWN through the shell's command palette before the loop runs, for a host that
 * boots a landing app rather than the editor under test (the real `s` host: `ready=s`,
 * `windows=s-home-main`). Unset for every single-plugin `dev` session, which already boots the
 * program it serves. Ticket 26/09/18 S5 §8 / S6. */
const SPAWN_PLUGIN = process.env.S_OS_MCP_LIVE_SPAWN ?? null;
/** 🌐️ The language the human's shell runs in (`en` or `de`): the browser context's locale, which the
 * shell adopts when it holds no stored preference. Every surface the human reads — the transcript and
 * the approval affordance — must speak it; the oracle below is this gate's own, not the shell's bundle. */
const LOCALE = process.env.S_OS_MCP_LIVE_LOCALE === "de" ? "de" : "en";
const APPROVAL_WORDS: Readonly<Record<"en" | "de", Readonly<{ once: string; deny: string; withdrawnCancelled: string }>>> = {
  en: { once: "Approve Once", deny: "Deny", withdrawnCancelled: "the agent's request was cancelled" },
  de: { once: "Einmal genehmigen", deny: "Ablehnen", withdrawnCancelled: "die Anfrage des Agenten wurde abgebrochen" },
};
/** 🧱️ The one module the shell-bound artifact route lives in. A serve transforming an older copy
 * of it answers every (f) step from a route that no longer exists in the tree under test, which is
 * how two runs of this gate on two serves produced two different verdicts (LB1 §11.2). */
const SHELL_HOST_MODULE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx";
/** 🧷️ The adversarial text a collaborator plants in the note for the (g) chain: the canary of the
 * untrusted-content law, so this gate and the laws probe one and the same string. */
const UNTRUSTED_LAW = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗿️artifact/🧫️fixtures/🧷️untrusted-content-law.json";
const UNTRUSTED_CONTENT_SCHEMA = "semio.mcp.untrusted-content/v1";

/** 🧷️ The canary as text plus its base64 spelling for each of its three byte alignments. */
function canaryNeedles(canary: string): string[] {
  const bytes = Buffer.from(canary, "utf8");
  return [canary, ...[0, 1, 2].map((shift) => bytes.subarray(shift, shift + Math.floor((bytes.length - shift) / 3) * 3).toString("base64"))];
}

/** ✂️ `value` with every untrusted envelope cut out, collecting the envelopes it held. */
function outsideUntrusted(value: unknown, envelopes: Record<string, any>[]): unknown {
  if (Array.isArray(value)) return value.map((item) => outsideUntrusted(item, envelopes));
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>).flatMap(([key, member]) => {
      if (key === "untrusted" && (member as Record<string, unknown> | null)?.schema === UNTRUSTED_CONTENT_SCHEMA) {
        envelopes.push(member as Record<string, any>);
        return [];
      }
      return [[key, outsideUntrusted(member, envelopes)]];
    }),
  );
}

/** 📖️ The document bytes an envelope carries (pack then spr, or the export), as latin1 text to search
 * in, followed by every deflate stream found inside them inflated: a kind that compresses its pack
 * and event-log records (note does) holds its text only in those streams. */
function envelopeText(envelope: Record<string, any> | undefined): string {
  const content = (envelope?.content ?? {}) as Record<string, unknown>;
  const bytes = Buffer.concat(["packBase64", "sprBase64", "contentBase64"].map((field) => Buffer.from(String(content[field] ?? ""), "base64")));
  const inflated: string[] = [];
  for (let offset = 0; offset + 2 < bytes.length; offset += 1) {
    for (const inflate of [inflateRawSync, inflateSync]) {
      try {
        const text = inflate(bytes.subarray(offset)).toString("latin1");
        if (text.length >= 8) inflated.push(text);
      } catch {}
    }
  }
  return [bytes.toString("latin1"), ...inflated].join("\n");
}

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
  readonly serverRequests: Record<string, unknown>[] = [];
  private buffer = "";
  private nextId = 1;
  // 🧭️ No annotation: `Bun.spawn` is declared twice (bun-types and the repo's own `🌿️environment`
  // ambient), so `ReturnType<typeof Bun.spawn>` picks whichever overload comes last in the program and
  // loses `stdin.write`/`stdout`'s async iterator. Inferring the field from the constructor assignment
  // binds the signature this call actually resolved to.
  private readonly child;
  /** 🙋 How this client answers `elicitation/create`; `silent` answers nothing at all, which is what
   * exercises the gateway's wall-clock elicitation deadline. */
  elicitationAnswer: "accept" | "decline" | "silent" = "accept";

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
    if (typeof envelope.method === "string") {
      this.serverRequests.push(envelope);
      if (envelope.method === "elicitation/create" && envelope.id !== undefined && this.elicitationAnswer !== "silent") {
        const result = this.elicitationAnswer === "accept" ? { action: "accept", content: { approve: true } } : { action: "decline" };
        this.write({ jsonrpc: "2.0", id: envelope.id, result });
      }
      return;
    }
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

/** 🙋 `elicitation` is what decides which approval lane the gateway takes, so it is a parameter of
 * the connection rather than a constant: a client that advertises it settles every approval through
 * its own human and the shell is never asked, which is exactly how the shell lane went unmeasured
 * for as long as it did (M5a §8.13). The gate's MAIN client advertises none — the lane most real
 * MCP clients are actually on. */
async function open(label: string, entry: ServerEntry, environment: Record<string, string> = {}, elicitation = false): Promise<Peer> {
  console.log(`\n=== ${label}\n    ${entry.command} ${entry.args.join(" ")}`);
  const peer = new Peer(entry.command, entry.args, environment);
  const initialized = await peer.request("initialize", {
    protocolVersion: PROTOCOL_VERSION,
    capabilities: elicitation ? { roots: { listChanged: true }, elicitation: {} } : { roots: { listChanged: true } },
    clientInfo: { name: "os-mcp-live-agent-loop", title: "OS MCP Live Agent Loop", version: "1" },
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

/** ⏳️ Polls until `probe` answers truthy or the budget runs out — the only wait shape here, so
 * nothing ever sleeps longer than the thing it waits for. */
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
    entries: [...document.querySelectorAll("[data-semio-agent-chat-entry]")].map((element) => ({
      kind: element.getAttribute("data-semio-agent-chat-entry"),
      state: element.getAttribute("data-agent-chat-state"),
      id: element.id,
    })),
    cancelButtons: [...document.querySelectorAll("[data-semio-agent-chat-cancel]")].map((element) => element.getAttribute("data-semio-agent-chat-cancel")),
    // ⛩️ Every approval's ONE affordance (`🤖️AgentApprovals`' `AgentApprovalAffordance`, rendered in the agent
    //    conversation; ticket 26/09/18 session 11 U5 retired the modal copy whose veil blocked the inline one).
    approvals: [...document.querySelectorAll("[data-semio-agent-approval-id]")].map((element) => element.getAttribute("data-semio-agent-approval-id")),
    approvalCountdown: document.querySelector("[data-semio-agent-approval-countdown]")?.getAttribute("data-semio-agent-approval-countdown") ?? null,
    windowIds: [...new Set([...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))],
  }));
}

// ───────────────────────────── the run ─────────────────────────────
const entry = semioEntry();
const reachable = await fetch(SHELL_URL).then((response) => response.ok).catch(() => false);
if (!reachable)
  throw new Error(
    `no live dev session answers ${SHELL_URL}. Start one first (launch row "🛠️dev🗒️${PLUGIN}⚛️react", or \`bun nx run workspace:dev -- ${PLUGIN}\`) and re-run; point the gate elsewhere with S_OS_MCP_LIVE_SHELL_URL.`,
  );

// 🧱️ Precondition, not a step: a serve that is not transforming THIS tree's `🏛️ShellHost` cannot
// answer the (f) chain from the route under test, and a gate that ran anyway would report a verdict
// about somebody else's code. Refusing here is why two runs on two serves can now be compared.
const servedShellHost = await fetch(`${SHELL_ORIGIN}/@fs${join(REPO_ROOT, SHELL_HOST_MODULE)}`)
  .then((response) => (response.ok ? response.text() : ""))
  .catch(() => "");
if (!servedShellHost.includes("agentArtifactRouteRef"))
  throw new Error(
    `${SHELL_ORIGIN} does not serve this tree's shell-bound artifact route: the transformed \`🏛️ShellHost\` carries no \`agentArtifactRouteRef\`. Restart that serve against this working tree (LB1 §11.2 precondition 2) before running the gate.`,
  );

const browser = await chromium.launch({ headless: process.env.S_OS_MCP_LIVE_HEADED !== "1", args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" })).newPage();

let gateway: Peer | null = null;
try {
  // 🛰️ The dev session publishes its own record into the rendezvous the gateway reads, so the pairing
  //    is deterministic even with several gateways and several sessions alive at once.
  // 🛰️ The offer the endpoint already served is read IN FULL, not just its status: a dead gateway
  //    leaves its file behind and a peer's live one is served in preference to nothing, so a gate
  //    that only checked for `200` would happily hand this shell somebody else's bridge and then
  //    report on their session. The step passes only once the typed answer names a NEW gateway (url + proof).
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
  record(
    "0 rendezvous",
    offer ? "pass" : "fail",
    offer
      ? `offer before=${offerBefore ? offerBefore.url : "(none)"}, after=${offer.url}`
      : `this gateway published no offer of its own; ${OFFER_URL} still answers ${offerBefore ? `a pre-existing offer at ${offerBefore.url} — point S_AGENT_BRIDGE_DIR at the rendezvous THIS dev session publishes into` : "no offer"}`,
  );

/** 🪟️ Opens one program through the shell's OWN command palette — the route a human takes, and the
 * only one available: `spawnApp` is declared by the studio app, Home publishes no control for it, and
 * the palette chord is `Meta+p` (S3 measured `Meta+K` opening nothing). The studio is found by LABEL
 * because all three `🪐️space` apps share the palette id `spawn.space`; every other program is found
 * by `spawn.<pluginId>`. Returns the window ids the spawn added, empty when nothing opened. */
async function spawnProgramThroughPalette(page: Page, pluginId: string): Promise<readonly string[]> {
  const windowIdsNow = async (): Promise<readonly string[]> => (await readShell(page)).windowIds.filter((id): id is string => typeof id === "string");
  const openPalette = async (): Promise<boolean> => {
    for (let attempt = 0; attempt < 30; attempt += 1) {
      if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) break;
      const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip|überspringen/iu }).first();
      if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
      else await page.keyboard.press("Escape").catch(() => undefined);
      await page.waitForTimeout(500);
    }
    await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
    return (await input.count()) > 0;
  };
  const awaitNewWindows = async (before: readonly string[], budgetMs: number): Promise<readonly string[]> => {
    const deadline = Date.now() + budgetMs;
    while (Date.now() < deadline) {
      const opened = (await windowIdsNow()).filter((id) => !before.includes(id));
      if (opened.length > 0) {
        await page.waitForTimeout(6_000);
        return (await windowIdsNow()).filter((id) => !before.includes(id));
      }
      await page.waitForTimeout(500);
    }
    return [];
  };
  if (!(await windowIdsNow()).some((id) => /studio|s-workflow/iu.test(id))) {
    if (!(await openPalette())) return [];
    const studio = page.locator('[data-slot="command-item"]').filter({ hasText: /s\s*·\s*studio/iu }).first();
    if ((await studio.count()) === 0) await page.keyboard.press("Escape");
    else {
      const before = await windowIdsNow();
      await studio.click({ force: true }).catch(() => undefined);
      await awaitNewWindows(before, 180_000);
    }
  }
  if (!(await openPalette())) return [];
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(pluginId);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
  await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return [];
  }
  const before = await windowIdsNow();
  await item.click({ force: true }).catch(() => undefined);
  return awaitNewWindows(before, 90_000);
}

  await page.goto(SHELL_URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  // 🪧️ `data-semio-os-ready` carries the BOOTED PLUGIN'S ID, not `"true"`.
  const booted = await until("shell boot", 240_000, 1000, async () => {
    const view = await readShell(page);
    return view.ready || view.shellError ? view : null;
  });
  record("boot", booted?.ready && !booted.shellError ? "pass" : "fail", booted ? `ready=${booted.ready} error=${booted.shellError} windows=${booted.windowIds.join(",")}` : "the shell never reached data-semio-os-ready");

  // 🪟️ OPTIONAL: drive the loop against a SPAWNED foreign editor instead of the landing app.
  //
  // Every step below addresses whatever program owns the canvas, and against a single-plugin `dev`
  // session that is the booted plugin itself — which is what this gate has always measured. Against
  // the real `s` host it is `s`'s own Home: `ready=s`, `windows=s-home-main`, and the foreign editor
  // a human would drive only exists after a palette spawn. Ticket 26/09/18 S5 §8 named that gap as
  // the reason the gate could not be pointed at `:6071` at all. This is the step it named: with
  // `S_OS_MCP_LIVE_SPAWN=<pluginId>` the gate opens the studio and spawns that program through the
  // shell's own command palette before asserting anything, so `ui_focus`, `ui_reveal`,
  // `artifact_open`, `action_prepare`/`action_invoke` and `history_undo` all address a spawned
  // foreign editor. Unset — every existing single-plugin invocation — this is one skipped `if`.
  if (SPAWN_PLUGIN) {
    const spawnedWindows = await spawnProgramThroughPalette(page, SPAWN_PLUGIN);
    record(
      `0 spawn ${SPAWN_PLUGIN} through the command palette`,
      spawnedWindows.length > 0 ? "pass" : "fail",
      spawnedWindows.length > 0 ? `windows=${spawnedWindows.join(",")}` : `no window appeared for spawn.${SPAWN_PLUGIN}; the palette chord is Meta+p and only the STUDIO app declares spawnApp (S3/S4), so this host must be at a studio surface first`,
    );
  }

  // (a) the gateway's own witness that a shell attached: `ui_focus` stops answering the
  //     "no shell is attached to /bridge yet" refusal the moment the socket registers.
  const attached = await until("the shell to register on /bridge", 90_000, 1000, async () => {
    const focused = await gateway!.call("ui_focus", {});
    return structured(focused).code === undefined ? focused : null;
  });
  record("(a) the shell dials /bridge", attached ? "pass" : "fail", attached ? `ui_focus answered ${describe(attached)}` : "ui_focus never stopped answering PLUGIN_UNAVAILABLE — no shell registered");

  // 🧭️ WHICH lane is this run measuring? `context_resolve` is where the decision is TAKEN and it is
  //    sticky, so this call must come after the shell has registered (a resolve issued before the
  //    socket lands would pin `headless` for the whole session and every (f) step below would
  //    silently measure this process's own `--folder` workspace). The headless twin of this
  //    assertion is `client-e2e`'s step 0, which pins the opposite value.
  const context = await gateway.call("context_resolve", {});
  const resolvedChannel = String(structured(context).channel ?? "");
  record(
    "0 context_resolve pins the shell channel",
    resolvedChannel === "shell" ? "pass" : "fail",
    resolvedChannel === "shell"
      ? `channel=shell principal=${structured(context).principal} session=${structured(context).sessionId}`
      : `channel=${resolvedChannel || "<none>"} — a shell is attached but this session resolved elsewhere; \`🏛️ShellHost\` did not declare relayAppCommands, or the resolve happened before the socket registered`,
  );

  // (d) ui_reveal moves the real dock — and is how the conversation gets on screen at all.
  const revealed = await gateway.call("ui_reveal", { anchor: "right", path: ["framework.chat"] });
  const chatVisible = await until("the chat panel", 30_000, 500, async () => {
    const view = await readShell(page);
    return view.chatPanel ? view : null;
  });
  record("(d) ui_reveal moves the real dock", chatVisible ? "pass" : "fail", `${describe(revealed)}; chat panel in DOM=${chatVisible !== null}`);

  const presence = await until("agent presence", 20_000, 500, async () => {
    const view = await readShell(page);
    return view.presenceTone ? view : null;
  });
  record("(a) agent presence renders", presence ? "pass" : "fail", presence ? `tone=${presence.presenceTone}` : "no [data-semio-agent-presence-tone] in the shell");

  const windowId = presence?.windowIds.find((id) => id) ?? null;
  const focusedOne = windowId ? await gateway.call("ui_focus", { windowId }) : null;
  record("(d) ui_focus moves the real active window", focusedOne && structured(focusedOne).ok === true ? "pass" : "fail", focusedOne ? `${describe(focusedOne)} (windowId=${windowId})` : "the shell reported no window id to focus");

  // (b) a tool call shows as a running row, then as a settled result row.
  // 👁️ A running row is TRANSIENT — a gateway-owned read settles in milliseconds, and polling the
  //    DOM for it scores the shell's own promptness as a missing row (this step flaked exactly that
  //    way across two runs of identical code, 2026-09-20). An observer installed BEFORE the call
  //    sees every state the row passed through, so the assertion is about what the shell rendered
  //    rather than about when the poll happened to look.
  await page.evaluate(() => {
    const seen = new Set<string>();
    (window as unknown as { __semioRunningRows: string[] }).__semioRunningRows = [];
    const sweep = (): void => {
      for (const element of document.querySelectorAll("[data-semio-agent-chat-entry='toolCall'][data-agent-chat-state='running']")) {
        if (!seen.has(element.id)) {
          seen.add(element.id);
          (window as unknown as { __semioRunningRows: string[] }).__semioRunningRows.push(element.id);
        }
      }
    };
    new MutationObserver(sweep).observe(document.body, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-agent-chat-state"] });
    sweep();
  });
  const running = gateway.call("capabilities_search", { query: `${PLUGIN} block` });
  const sawRunning = await until("a running tool-call row", 20_000, 100, async () => {
    const observed = await page.evaluate(() => (window as unknown as { __semioRunningRows?: string[] }).__semioRunningRows ?? []);
    return observed.length > 0 ? { kind: "toolCall", state: "running", id: observed[observed.length - 1] ?? "" } : null;
  });
  const settledCall = await running;
  const sawResult = await until("the settled tool-call row", 30_000, 200, async () => {
    const view = await readShell(page);
    return view.entries.find((row) => row.kind === "toolCall" && (row.state === "ok" || row.state === "failed")) ?? null;
  });
  record("(b) tool call → running row → result row", sawRunning && sawResult ? "pass" : "fail", `running=${JSON.stringify(sawRunning)} settled=${JSON.stringify(sawResult)} call=${describe(settledCall)}`);

  // 🎯️ The mutation candidates, read off the LIVE catalog. A gateway-owned read verb settles in
  //    milliseconds and never gives the cancel affordance a frame to exist in, so (c) keeps a plugin
  //    `action_invoke` in flight instead — which is the honest shape of "an in-flight call" anyway.
  const found = await gateway.call("capabilities_search", { query: "delete selection clear", kind: ["mutation"] });
  const mutations = ((structured(found).results ?? []) as Record<string, any>[]).map((row) => String(row.capabilityId));
  const target = mutations.find((id) => id.startsWith(`${PLUGIN}.`)) ?? mutations[0] ?? null;
  // ✍️ The (f) chain's own verb, looked up in the LIVE catalog by the id it ends in. Separate from
  //    `target` on purpose — see MUTATION_VERB. `null` makes the whole chain SKIP with the reason,
  //    exactly as a missing destructive verb makes (e) skip, rather than asserting into thin air.
  const writeSearch = await gateway.call("capabilities_search", { query: MUTATION_VERB, kind: ["mutation"] });
  const writeTarget =
    ((structured(writeSearch).results ?? []) as Record<string, any>[]).map((row) => String(row.capabilityId)).find((id) => id.startsWith(`${PLUGIN}.`) && id.endsWith(`.${MUTATION_VERB}`)) ?? null;

  // (c) the running row's Cancel button cancels the call in flight.
  if (target === null) {
    record("(c) Cancel cancels an in-flight call", "skip", "the live catalog exposes no mutation capability to keep in flight");
  } else {
    const inFlight = gateway.call("action_invoke", { capabilityId: target, input: {} }).catch((error) => ({ error: { message: String(error) } }) as Record<string, any>);
    // 🖱️ Found and clicked inside ONE evaluate: a locator click is a second round trip, and an
    //    in-flight verb can settle in that gap — which scores the shell's own promptness as a
    //    missing cancel button. This clicks the affordance that exists at the instant it is seen.
    //    Only the `action_invoke` row qualifies: the search calls before it answer the gate before
    //    their own result frame reaches the shell, so "the first running row" was sometimes one of
    //    them — a click on an already-settled call that proved nothing (U5 §2, run 2).
    const cancelledAt = Date.now();
    const cancelled = await until("a cancellable row to click", 30_000, 100, async () =>
      page.evaluate(() => {
        const button = document.querySelector<HTMLElement>("[data-semio-agent-chat-cancel][data-semio-agent-chat-tool='action_invoke']");
        if (!button) return null;
        const id = button.getAttribute("data-semio-agent-chat-cancel");
        button.click();
        return id;
      }),
    );
    if (!cancelled) {
      await inFlight;
      record("(c) Cancel cancels an in-flight call", "fail", `no cancellable row appeared while ${target} was in flight`);
    } else {
      const cancelling = await until("the cancelling row", 20_000, 100, async () => {
        const row = (await readShell(page)).entries.find((entry) => entry.id.endsWith(cancelled));
        return row && row.state !== "running" ? row : null;
      });
      const settled = await inFlight;
      const settledMs = Date.now() - cancelledAt;
      // 🛑️ The call must END as cancelled, promptly — not merely show a `cancelling` row while it
      //    waits out the 120 s approval deadline as `APPROVAL_REQUIRED`, which is what a parked
      //    approval did before the gateway's wait observed the tool call's own job (U5 §2, run 1).
      const endedCancelled = structured(settled as Record<string, any>).code === "CANCELLED" && settledMs < 30_000;
      record("(c) Cancel cancels an in-flight call", cancelling && endedCancelled ? "pass" : "fail", `clicked=${cancelled} row=${JSON.stringify(cancelling)} settledMs=${settledMs} call=${describe(settled as Record<string, any>)}`);
      // 🪦️ The call parked on an approval, so the shell was showing a decidable affordance for it. The
      //    gateway withdraws it with the cancel (`ApprovalWithdrawn{cancelled}`), and the shell must
      //    retire it — no countdown, no decision — and say why in the human's language, instead of
      //    leaving it decidable until its own countdown runs out (U5 §7).
      const approvalId = String(structured(settled as Record<string, any>).details?.approvalHandle ?? "");
      const retired = await until("the withdrawn approval affordance", 15_000, 150, async () =>
        page.evaluate((id: string) => {
          const affordance = [...document.querySelectorAll<HTMLElement>("[data-semio-agent-approval-id]")].filter((element) => id === "" || element.getAttribute("data-semio-agent-approval-id") === id).at(-1);
          if (!affordance || affordance.getAttribute("data-semio-agent-approval-state") !== "withdrawn") return null;
          return {
            id: affordance.getAttribute("data-semio-agent-approval-id"),
            withdrawal: affordance.getAttribute("data-semio-agent-approval-withdrawal"),
            countdown: affordance.querySelector("[data-semio-agent-approval-countdown]") !== null,
            decisions: affordance.querySelectorAll("[id^='framework.approvals.deny.'], [id^='framework.approvals.once.'], [id^='framework.approvals.session.']").length,
            text: (affordance.querySelector("[role='status']") as HTMLElement | null)?.innerText ?? "",
          };
        }, approvalId),
      );
      record(
        `(c2) the cancelled call's approval is withdrawn in the shell (${LOCALE})`,
        retired !== null && retired.withdrawal === "cancelled" && !retired.countdown && retired.decisions === 0 && retired.text.includes(APPROVAL_WORDS[LOCALE].withdrawnCancelled) ? "pass" : "fail",
        retired ? `approval=${retired.id} withdrawal=${retired.withdrawal} countdown=${retired.countdown} decisions=${retired.decisions} text="${retired.text}"` : `no approval affordance turned withdrawn (approvalHandle=${approvalId || "<none in the answer>"})`,
      );
    }
  }

  // (f) the mutation chain, against the SAME live gateway the shell is dialled into:
  //     create/open → prepare/invoke → snapshot → undo/redo → transaction → export. Every leg
  //     asserts on returned wire data and each one states its own cascade reason rather than
  //     inheriting a green from the leg before it.
  const safely = async (label: string, run: () => Promise<Record<string, any>>): Promise<Record<string, any>> => {
    try {
      return await run();
    } catch (error) {
      return { error: { code: -1, message: `${label}: ${String(error)}` } };
    }
  };
  const failure = (envelope: Record<string, any>): string | null => {
    if (envelope.error) return `JSON-RPC error ${envelope.error.code}: ${String(envelope.error.message).slice(0, 200)}`;
    if (envelope.result?.isError) return `${structured(envelope).code ?? "ERROR"}: ${String(structured(envelope).message ?? "").slice(0, 200)}`;
    return null;
  };
  // 🗣️ `<plugin>.<artifactKind>@<schemaVersion>/*#<app>.<action>` — the artifact kind a mutation
  //    capability belongs to is the segment its own id carries, never a kind invented here.
  const artifactKind = writeTarget ? (/^[^.]+\.([^@]+)@/.exec(writeTarget)?.[1] ?? null) : null;
  let plantedIn: { artifactId: string } | null = null;
  if (!writeTarget || !artifactKind) {
    const reason = `the live catalog exposes no \`${PLUGIN}\`…\`${MUTATION_VERB}\` mutation capability to drive (writeTarget=${writeTarget ?? "none"}); name the plugin's own unconditional mutation with S_OS_MCP_LIVE_MUTATION/S_OS_MCP_LIVE_MUTATION_INPUT`;
    for (const step of ["(f1) artifact_create/open", "(f2) action_prepare", "(f3) action_invoke changes the head", "(f4) artifact_snapshot shows the change", "(f5) the live shell shows the same artifact", "(f6) history_undo/redo", "(f7) transaction begin/rollback/commit", "(f8) artifact_export"]) record(step, "skip", reason);
  } else {
    const artifactId = `live-agent-loop-${Date.now().toString(36)}`;
    const created = await safely("artifact_create", () => gateway!.call("artifact_create", { artifactId, kind: artifactKind }));
    const opened = failure(created) ? created : await safely("artifact_open", () => gateway!.call("artifact_open", { artifactId }));
    const createFault = failure(created) ?? failure(opened);
    record("(f1) artifact_create/open", createFault ? "fail" : "pass", createFault ? `kind=${artifactKind}: ${createFault}` : `artifactId=${artifactId} kind=${artifactKind} ${describe(opened)}`);

    // 🧬️ The document as it stands BEFORE the agent touches it, kept whole: (f4) is only a real
    //    assertion if it can say the bytes CHANGED, and a pack whose change lands past a preview's
    //    cut looks identical to a truncated read. Measured 2026-09-20 (AP1): `artifact_snapshot`
    //    answered a byte-identical pack across a mutation the same chain reported as applied,
    //    because the snapshot came from the folder row seeded at create time rather than from the
    //    session's own channel — a `packBytes > 0` assertion could never have caught it.
    const snapshotBefore = createFault ? created : await safely("artifact_snapshot", () => gateway!.call("artifact_snapshot", { artifactId }));
    // 🧬️ The WHOLE observable snapshot, not its pack alone: an event-sourced document keeps its
    //    genesis container in `pack` and every edit in the `spr` sidecar, so a note that gained a
    //    block answers an unchanged `packBytes` and a grown `sprBytes` (515/223 → 515/612, measured
    //    2026-09-20). A witness that watched only the pack would call that "no change".
    const witness = (envelope: Record<string, any>): string => `${structured(envelope).packBytes ?? "?"}:${structured(envelope).sprBytes ?? "?"}:${envelopeText(structured(envelope).untrusted)}`;
    const before = witness(snapshotBefore);

    const prepared = createFault ? created : await safely("action_prepare", () => gateway!.call("action_prepare", { capabilityId: writeTarget, input: MUTATION_INPUT }));
    const prepareFault = createFault ? `cascaded from (f1): ${createFault}` : failure(prepared);
    const handle = String(structured(prepared).preparedHandle ?? "");
    const revisionBefore = JSON.stringify(structured(prepared).expectedRevision ?? null);
    record("(f2) action_prepare", prepareFault ? "fail" : "pass", prepareFault ?? `preparedHandle=${handle} expectedRevision=${revisionBefore}`);

    const invoked = prepareFault ? prepared : await safely("action_invoke", () => gateway!.call("action_invoke", { preparedActionHandle: handle }));
    const invokeFault = prepareFault ? `cascaded from (f2)` : failure(invoked);
    const status = String(structured(invoked).status ?? "");
    const revisionAfter = JSON.stringify(structured(invoked).revisionAfter ?? null);
    const undoToken = String(structured(invoked).undoToken ?? "");
    const headMoved = status === "SUCCEEDED" && revisionAfter !== JSON.stringify(structured(invoked).revisionBefore ?? null);
    record("(f3) action_invoke changes the head", !invokeFault && headMoved ? "pass" : "fail", invokeFault ?? `status=${status} before=${JSON.stringify(structured(invoked).revisionBefore ?? null)} after=${revisionAfter} undoToken=${undoToken || "(none)"}`);

    const snapshot = invokeFault ? invoked : await safely("artifact_snapshot", () => gateway!.call("artifact_snapshot", { artifactId }));
    const packBytes = Number(structured(snapshot).packBytes ?? 0);
    const after = witness(snapshot);
    const snapshotFault = invokeFault ? `cascaded from (f3)` : (failure(snapshot) ?? (packBytes > 0 ? null : "the snapshot carries no pack at all"));
    // 🧬️ "Shows the change" means the snapshot differs from the pre-invoke read. An identical one
    //    after an applied mutation means the snapshot is answering some other copy of the document.
    record(
      "(f4) artifact_snapshot shows the change",
      !snapshotFault && after !== before ? "pass" : "fail",
      snapshotFault ??
        (after !== before
          ? `packBytes=${packBytes} sprBytes=${structured(snapshot).sprBytes ?? "?"} (was ${before.split(":").slice(0, 2).join("/")}) — the snapshot moved with the mutation`
          : `the snapshot is byte-identical to the pre-invoke read (packBytes=${packBytes}, sprBytes=${structured(snapshot).sprBytes ?? "?"}) — it is not reading the store the mutation landed in`),
    );

    // 🖥️ The acceptance criterion outcome 4 actually names: the change the agent made is the change
    //    a human sees. Checked against the live DOM by the artifact's own id, so it can only pass if
    //    the shell really owns the document the gateway mutated.
    //    🪪️ TWO ids name one document and the gate asserts the bridge between them. `artifactId` is
    //    what this client INVENTED and handed the gateway; `expectedRevision.artifactId` is what the
    //    shell's own artifact route answered for it (`${pluginId}:${appId}:${instanceId}`) — the
    //    shell publishes exactly that string as `data-semio-artifact-id`, so a green row means the
    //    id the agent holds resolves to the element the human is looking at.
    const shellArtifactRef = String(structured(prepared).expectedRevision?.artifactId ?? "");
    const shellShows = shellArtifactRef
      ? await page.evaluate((id: string) => document.querySelector(`[data-semio-artifact-id="${id}"], [data-semio-document-id="${id}"]`) !== null, shellArtifactRef)
      : false;
    record(
      "(f5) the live shell shows the same artifact",
      shellShows ? "pass" : "fail",
      shellShows
        ? `agent id ${artifactId} → shell route ref ${shellArtifactRef}, carried by [data-semio-artifact-id] in the live DOM`
        : shellArtifactRef
          ? `action_prepare resolved the shell route ref ${shellArtifactRef}, but no [data-semio-artifact-id="${shellArtifactRef}"] is in the live DOM — this serve's 🏛️ShellHost does not publish the identity its own artifact route hands out`
          : `action_prepare returned no expectedRevision.artifactId, so the agent holds no shell-resolvable identity for ${artifactId} at all`,
    );

    // ↩️ `history_undo`/`redo` are keyed by the invocation's OWN undo token, not by the artifact —
    //    an agent may only walk back a mutation it can name.
    const undone = snapshotFault || !undoToken ? snapshot : await safely("history_undo", () => gateway!.call("history_undo", { undoToken }));
    const redone = !undoToken || failure(undone) ? undone : await safely("history_redo", () => gateway!.call("history_redo", { undoToken }));
    const historyFault = snapshotFault ? `cascaded from (f4)` : !undoToken ? "(f3) returned no undoToken, so there is nothing an agent could undo" : (failure(undone) ?? failure(redone));
    record("(f6) history_undo/redo", historyFault ? "fail" : "pass", historyFault ?? `undo=${describe(undone)} redo=${describe(redone)}`);

    // 🧾️ A saga is begun over PREPARED handles, so each leg needs its own fresh prepare.
    const prepareFor = async (leg: string): Promise<string> => String(structured(await safely(`action_prepare/${leg}`, () => gateway!.call("action_prepare", { capabilityId: target, input: {} }))).preparedHandle ?? "");
    const rollbackHandle = historyFault ? "" : await prepareFor("rollback");
    const begun = rollbackHandle ? await safely("transaction_begin", () => gateway!.call("transaction_begin", { preparedHandles: [rollbackHandle] })) : undone;
    const txnHandle = String(structured(begun).transactionHandle ?? "");
    const rolled = txnHandle ? await safely("transaction_rollback", () => gateway!.call("transaction_rollback", { transactionHandle: txnHandle })) : begun;
    const commitHandle = failure(rolled) ? "" : await prepareFor("commit");
    const begun2 = commitHandle ? await safely("transaction_begin", () => gateway!.call("transaction_begin", { preparedHandles: [commitHandle] })) : rolled;
    const txnHandle2 = String(structured(begun2).transactionHandle ?? "");
    const committed = txnHandle2 ? await safely("transaction_commit", () => gateway!.call("transaction_commit", { transactionHandle: txnHandle2 })) : begun2;
    const txnFault = historyFault ? `cascaded from (f6)` : (failure(begun) ?? failure(rolled) ?? failure(begun2) ?? failure(committed));
    record("(f7) transaction begin/rollback/commit", txnFault ? "fail" : "pass", txnFault ?? `rollback=${txnHandle} ${describe(rolled)} commit=${txnHandle2} ${describe(committed)}`);

    const exported = txnFault ? committed : await safely("artifact_export", () => gateway!.call("artifact_export", { artifactId }));
    const exportFault = txnFault ? `cascaded from (f7)` : failure(exported);
    const contentBytes = String(structured(exported).untrusted?.content?.contentBase64 ?? "").length;
    record("(f8) artifact_export", !exportFault && contentBytes > 0 ? "pass" : "fail", exportFault ?? `untrusted.content.contentBase64=${contentBytes} char(s) mimeType=${structured(exported).mimeType ?? "null"}`);
    if (!exportFault) plantedIn = { artifactId };
  }

  // (e) approval. `capabilities_describe` is the precondition oracle: a capability that does not
  //     gate cannot raise an affordance, and asserting against it would be theatre.
  const described = target ? await gateway.call("capabilities_describe", { capabilityId: target }) : null;
  const policy = (structured(described ?? {}).capability ?? structured(described ?? {})) as Record<string, any>;
  const gates = policy?.policy?.approval === "always" || (policy?.policy?.approval === "whenDestructive" && policy?.effects?.destructive === true);
  if (!target || !gates) {
    const reason = `no capability in the live catalog gates on approval (${target ?? "no mutation verb"}: approval=${policy?.policy?.approval ?? "?"}, destructive=${policy?.effects?.destructive ?? "?"}); author one with ActionDefinition::destructive() and this step runs`;
    record("(e1) Approve Once lets it through", "skip", reason);
    record("(e2) Deny returns the typed refusal", "skip", reason);
    record("(e3) a silent client returns the typed timeout", "skip", reason);
    for (const step of ["(g1) a collaborator's instruction reaches the agent only inside `untrusted`", "(g2) the destructive action it asks for still waits for a human", "(g3) the planted block survives the denial"]) record(step, "skip", reason);
  } else {
    // 🚦️ Which channel resolves the approval is itself the measurement (M5a, 2026-09-20). The
    //    coordinator offers ELICITATION first, so an elicitation-capable client settles every
    //    approval through its own human and the shell is never asked — which is how the shell lane
    //    went unmeasured. This gateway's client therefore advertises no `elicitation` at all (see
    //    `open`), the lane most real MCP clients are on: the gateway must then publish
    //    `ApprovalRequested` over `/bridge`, the shell must render a decidable affordance, and the
    //    human's click must come back as the typed outcome. `sawAffordance` is an assertion here,
    //    not a note: without it the human was never given the chance to decide.
    const decide = async (decision: "once" | "deny", capabilityId: string = String(target), input: Record<string, unknown> = {}): Promise<{ answer: Record<string, any>; surface: string; countdown: string | null; words: Readonly<{ once: string; deny: string }> | null }> => {
      // 🗂️ Every approval this run already parked stays on screen — `(c)` cancels its call while the
      //    gateway is still waiting for a human, so a row from it outlives the step. Deciding "the
      //    first row" would answer THAT one, whose budget has been running since, and leave this
      //    invocation waiting on a decision nobody made. Only an id that was not there a moment ago
      //    belongs to the call this step just made.
      const before = new Set((await readShell(page)).approvals.filter((id): id is string => id !== null));
      const invoked = gateway!.call("action_invoke", { capabilityId, input });
      const shown = await until("the approval affordance", 30_000, 150, async () => {
        const view = await readShell(page);
        const fresh = view.approvals.find((id) => id !== null && !before.has(id));
        return fresh ? { id: fresh, controlId: `framework.approvals.${decision}.${fresh}`, surface: "conversation", countdown: view.approvalCountdown } : null;
      });
      // ⏳️ The countdown is part of the affordance, not decoration: a human asked to decide inside
      //    a deadline they cannot see is being asked to guess. Read from THIS approval's own row,
      //    so an older row's spent budget is never mistaken for this one's.
      const countdown = shown
        ? await page.evaluate(
            (id: string) =>
              document.querySelector(`[data-semio-agent-approval-id="${id}"] [data-semio-agent-approval-countdown]`)?.getAttribute("data-semio-agent-approval-countdown") ?? null,
            shown.id,
          )
        : null;
      const words = shown
        ? await page.evaluate(
            (id: string) => ({
              once: (document.querySelector(`[id="framework.approvals.once.${id}"]`) as HTMLElement | null)?.innerText.trim() ?? "",
              deny: (document.querySelector(`[id="framework.approvals.deny.${id}"]`) as HTMLElement | null)?.innerText.trim() ?? "",
            }),
            shown.id,
          )
        : null;
      if (shown) await page.locator(`[id="${shown.controlId}"]`).first().click({ timeout: 10_000 });
      return { answer: await invoked, surface: shown?.surface ?? "none", countdown, words };
    };
    const once = await decide("once");
    record(
      "(e1) Approve Once lets it through",
      once.surface !== "none" && structured(once.answer).code !== "APPROVAL_REQUIRED" && structured(once.answer).code !== "PERMISSION_DENIED" ? "pass" : "fail",
      `affordance=${once.surface} countdown=${once.countdown ?? "(none)"} answer=${describe(once.answer)}`,
    );
    const lang = await page.evaluate(() => document.documentElement.lang);
    const spoken = once.words !== null && once.words.once === APPROVAL_WORDS[LOCALE].once && once.words.deny === APPROVAL_WORDS[LOCALE].deny;
    record(`(i18n) the approval affordance speaks ${LOCALE}`, spoken ? "pass" : "fail", `lang=${lang} once="${once.words?.once ?? ""}" deny="${once.words?.deny ?? ""}" expected="${APPROVAL_WORDS[LOCALE].once}"/"${APPROVAL_WORDS[LOCALE].deny}"`);
    const denied = await decide("deny");
    record(
      "(e2) Deny returns the typed refusal",
      denied.surface !== "none" && structured(denied.answer).code === "PERMISSION_DENIED" ? "pass" : "fail",
      `affordance=${denied.surface} channel=${structured(denied.answer).details?.channel ?? "?"} answer=${describe(denied.answer)}`,
    );

    // (g) prompt injection. A collaborator's document carries an instruction; the agent reads the
    //     document back. The text must reach it only inside the `untrusted` envelope, and the
    //     destructive action it asks for must still stop at the human. The plant is the law's shell
    //     recipe (`🧷️untrusted-content-law.json`): a whole-document load the human approves.
    const untrustedLaw = JSON.parse(readFileSync(join(REPO_ROOT, UNTRUSTED_LAW), "utf8"));
    const adversarial = String(untrustedLaw.canary);
    const needles = canaryNeedles(adversarial);
    const recipe = (untrustedLaw.plants as Record<string, any>[]).find((row) => row.lane === "shell");
    const gSteps = ["(g1) a collaborator's instruction reaches the agent only inside `untrusted`", "(g2) the destructive action it asks for still waits for a human", "(g3) the planted block survives the denial"];
    if (!plantedIn || !recipe || recipe.artifactKind !== artifactKind) {
      const reason = !plantedIn ? "the (f) chain created no document to plant the collaborator's text in" : `the law has no shell recipe for ${artifactKind}`;
      for (const step of gSteps) record(step, "skip", reason);
    } else {
      const noteId = plantedIn.artifactId;
      const readBack = async (): Promise<{ envelopes: Record<string, any>[]; outside: string; carried: string }> => {
        const reply = await safely("artifact_snapshot", () => gateway!.call("artifact_snapshot", { artifactId: noteId }));
        const envelopes: Record<string, any>[] = [];
        const outside = JSON.stringify(outsideUntrusted(reply, envelopes));
        return { envelopes, outside, carried: envelopes.map(envelopeText).join("\n") };
      };
      const plant = await decide("once", String(recipe.capabilityId), JSON.parse(JSON.stringify(recipe.input).replaceAll("{{canary}}", adversarial)));
      const read = await readBack();
      const leaked = needles.some((needle) => read.outside.includes(needle));
      const carried = read.carried.includes(adversarial);
      const provenance = read.envelopes[0]?.provenance;
      const plantFault = plant.surface === "none" ? `the collaborator's load raised no approval: ${describe(plant.answer)}` : failure(plant.answer);
      record(
        gSteps[0]!,
        !plantFault && carried && !leaked && read.envelopes.length === 1 && typeof read.envelopes[0]?.notice === "string" ? "pass" : "fail",
        plantFault ?? `plant=${describe(plant.answer)} envelopes=${read.envelopes.length} carriedInside=${carried} leakedOutside=${leaked} source=${provenance?.source} authors=${JSON.stringify(provenance?.authors)} contentSha256=${String(provenance?.revision?.contentSha256 ?? "").slice(0, 12)}…`,
      );
      const obeyed = await decide("deny");
      record(gSteps[1]!, obeyed.surface !== "none" && structured(obeyed.answer).code === "PERMISSION_DENIED" ? "pass" : "fail", `the agent tried ${target} as the note demanded: affordance=${obeyed.surface} countdown=${obeyed.countdown ?? "(none)"} answer=${describe(obeyed.answer)}`);
      const after = await readBack();
      record(gSteps[2]!, after.carried.includes(adversarial) ? "pass" : "fail", `the snapshot still carries the collaborator's text inside untrusted: ${after.carried.includes(adversarial)}`);
    }

    // ⏱️ A client that advertises `elicitation` and then never answers must get the typed timeout —
    //    not an approval, and not a wedged agent. Its own empty rendezvous closes the SHELL lane by
    //    construction, so the only budget this step spends is the elicitation deadline itself.
    const silent = await open("(e3) elicitation timeout — a client that never answers", entry, { S_AGENT_BRIDGE_DIR: mkdtempSync(join(tmpdir(), "semio-os-mcp-silent-")) }, true);
    silent.elicitationAnswer = "silent";
    const timedOut = await silent.call("action_invoke", { capabilityId: target, input: {} });
    const details = JSON.stringify(structured(timedOut).details ?? {});
    record("(e3) a silent client returns the typed timeout", structured(timedOut).code === "APPROVAL_REQUIRED" && details.includes("did not answer the elicitation in time") ? "pass" : "fail", `${describe(timedOut)} details=${details.slice(0, 240)}`);
    silent.stop();
  }

  console.log(`\n  server-initiated requests seen: ${gateway.serverRequests.map((row) => row.method).join(", ") || "(none)"}`);
} finally {
  gateway?.stop();
  await browser.close();
}

console.log("\n=== transcript");
for (const outcome of outcomes) console.log(`  ${outcome.state.toUpperCase().padEnd(4)}  ${outcome.step}`);
const failed = outcomes.filter((outcome) => outcome.state === "fail").length;
const skipped = outcomes.filter((outcome) => outcome.state === "skip").length;
console.log(`\nos-mcp-live-agent-loop: ${outcomes.length - failed - skipped} passed, ${failed} failed, ${skipped} skipped of ${outcomes.length}`);
if (failed > 0) process.exit(1);
