/** 🤝️ Two humans collaborate on ONE hub document through the wgpu shells — the wasm32 browser shell and the native winit
 * shell — and across shell TYPES with the React `s` shell; an AI agent's edit is rendered, not only decoded.
 *
 * Journeys (`--journey`), each filed as one acceptance check (`<check>-<locale>`):
 * - `wasm32` (`wgpu-collaboration-wasm32`): two wasm32 wgpu browser shells, each through its own severable relay in front of
 *   the hub, sign in, attach one hub note from the Sync card, see each other in the roster, edit both ways (latency measured),
 *   survive a short cut without a frozen frame worker (an offline edit is admitted as pending and delivered after the relink),
 *   speak a medium cut as a short shortage, expire a long cut in the session's tongue and never relink by themselves; a late
 *   joiner attaching afterwards shows every committed block.
 * - `wasm32-react` (`wgpu-collaboration-wasm32-react`): a wasm32 wgpu shell and a React `s` shell on one block2d document:
 *   open, presence both ways, edits both ways, per-shell undo, and late joiners of BOTH shell types converge.
 * - `wasm32-native` / `native-react` (`wgpu-collaboration-wasm32-native`, `wgpu-collaboration-native-react`): the native wgpu
 *   user is the renderer's live law `a_native_and_a_react_user_collaborate_on_one_hub_document` (it creates the space and a
 *   block2d document through its own door); the browser user — wasm32 or React — follows the law's file handshake step by step:
 *   open, presence both ways, native edit ingested, own edit ingested natively, each undoes only their own edit, a reload
 *   converges.
 * - `native-react-cursors` (`wgpu-peer-cursors-native-react`): the native law
 *   `a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board` with a React follower: each user's pointer over the shared
 *   board is painted as a peer cursor on the other's board.
 * - `cursors` (`wgpu-peer-cursors`): two wasm32 shells on one puzzle2d board; each user's pointer over its own board is painted
 *   as a peer cursor on the other's board (pixel deltas over a control pass).
 * - `agent-pixels` (`wgpu-agent-reply-pixels`): a human holds a hub note in the wasm32 shell while a delegated agent, its own
 *   principal over the stdio semio MCP, adds a block; the human's roster names the agent AS an agent, the block is decoded
 *   into the Artifact panel without a reload, and the live frame changes by at least `agentEditMinPixels` from the frame before
 *   the agent's edit while equalling, within `agentSameMaxPixels` (or twice the measured frame noise), a cold reference render
 *   of the same committed state by a fresh session of the same human opened after the first one closed.
 *
 * Every control of the wasm32 shell is reached through its accessibility mirror (`#semio-wgpu-accessibility`), so each run is
 * also a keyboard-reachability proof. Humans come from `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}` only (never argv, never
 * logged); a missing credential or hub, or a serve that can neither be reused nor started, is a `blocked` record.
 *
 * Promoted from the session-12–14 ticket harnesses (ticket 26/09/23: `wp-wg7`/`wp-wg9` `wg9-browser-collab.mjs` +
 * `wg9-agent-probe.ts`, `wp-wg8`/`wp-wg10` `cross-shell.mjs` + `run-cross-shell.sh`, `wp-wg11`).
 * @see ../../../../🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs — the native live laws and their file handshake
 * @see ../../../../../../🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts — the React two-human matrix (same credential variables)
 */

import { type ChildProcess, spawn } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { type AddressInfo, connect, createServer, type Socket } from "node:net";
import { join, resolve } from "node:path";
import type { Browser, BrowserContext, Page } from "playwright";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeSignIn } from "../../../../../../📇️directory/🧪️testkit/📡️client-probe/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../../../../📇️directory/🧬️schema/🟦️.ts";
import { requireMcpBinary, spawnRawMcp } from "../../../../../../🌉️mcp/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../../../../🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { devServePortV1, ensureDevServe } from "../../../../../../🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";

//#region 🔖️Contract
/** 🧭️ The journeys this harness runs, each one acceptance check. */
export type HubCollaborationJourneyV1 = "wasm32" | "wasm32-react" | "wasm32-native" | "native-react" | "native-react-cursors" | "cursors" | "agent-pixels";

/** 🧾️ The acceptance check each journey files its record under (the gate appends `-<locale>`). */
export const HUB_COLLABORATION_CHECKS: Readonly<Record<HubCollaborationJourneyV1, string>> = {
  wasm32: "wgpu-collaboration-wasm32",
  "wasm32-react": "wgpu-collaboration-wasm32-react",
  "wasm32-native": "wgpu-collaboration-wasm32-native",
  "native-react": "wgpu-collaboration-native-react",
  "native-react-cursors": "wgpu-peer-cursors-native-react",
  cursors: "wgpu-peer-cursors",
  "agent-pixels": "wgpu-agent-reply-pixels",
};

/** 🧩️ The hub kind (creation-catalog schema) each journey's document is, the wasm32 playground variant that edits it, the
 * browser serves the journey drives (`wgpu` release serve of that variant, the React `s` dev serve) and whether the native wgpu
 * shell (the renderer's live law, which creates its own space and document) is one of the users. */
export const HUB_COLLABORATION_KINDS: Readonly<Record<HubCollaborationJourneyV1, Readonly<{ schema: string; variant: string; wgpu: boolean; react: boolean; native: boolean }>>> = {
  wasm32: { schema: "note.document", variant: "note", wgpu: true, react: false, native: false },
  "wasm32-react": { schema: "block.2d", variant: "block2d", wgpu: true, react: true, native: false },
  "wasm32-native": { schema: "block.2d", variant: "block2d", wgpu: true, react: false, native: true },
  "native-react": { schema: "block.2d", variant: "block2d", wgpu: false, react: true, native: true },
  "native-react-cursors": { schema: "block.2d", variant: "block2d", wgpu: false, react: true, native: true },
  cursors: { schema: "puzzle.2d.fixture", variant: "puzzle2d", wgpu: true, react: false, native: false },
  "agent-pixels": { schema: "note.document", variant: "note", wgpu: true, react: false, native: false },
};

/** ⏱️ Bounds every judgement uses (one place, so a slower machine changes numbers, never code paths). */
export const HUB_COLLABORATION_BOUNDS = {
  bootMs: 180_000,
  liveMs: 120_000,
  presenceMs: 45_000,
  editSeenMs: 60_000,
  shortCutMs: 15_000,
  mediumCutMs: 20_000,
  expiryMs: 120_000,
  relinkMs: 60_000,
  frozenBeatMs: 2_000,
  keptLineMs: 8_000,
  cursorSettleMs: 3_500,
  agentSeenMs: 60_000,
  agentEditMinPixels: 256,
  agentSameMaxPixels: 64,
  nativeStepMs: 900_000,
} as const;

/** 👤️ One human: a hub credential and the tongue their browser boots in. */
export type HubCollaborationHuman = Readonly<{ label: string; email: string; password: string }>;

/** 🔑️ The two humans, from `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}` only (the goal gate's variables, shared with the React
 * two-human matrix). Absent → the run is `blocked`. */
export function hubCollaborationHumans(env: NodeJS.ProcessEnv = process.env): readonly [HubCollaborationHuman, HubCollaborationHuman] {
  const humans = [1, 2].map((index) => ({ label: index === 1 ? "A" : "B", email: env[`SEMIO_TWO_HUMAN_USER${index}_EMAIL`] ?? "", password: env[`SEMIO_TWO_HUMAN_USER${index}_PASSWORD`] ?? "" }));
  if (!humans.every((human) => human.email && human.password)) throw new HubCollaborationBlocked("SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD} are not set");
  return humans as unknown as readonly [HubCollaborationHuman, HubCollaborationHuman];
}

/** 🚧️ A missing precondition (credential, hub, serve, native runtime): the record is `blocked`, not `fail`. */
export class HubCollaborationBlocked extends Error {}

/** 🎛️ One run. `wgpuServe` / `reactServe` name the serves to drive; `null` → the run starts its own through `ensureDevServe`
 * (see {@link provisionServe}); `nativeBinary` runs a prebuilt renderer test binary instead of `cargo test`. */
export type HubCollaborationOptionsV1 = Readonly<{
  repoRoot: string;
  journey: HubCollaborationJourneyV1;
  hub: string;
  wgpuServe: string | null;
  reactServe: string | null;
  locale: "en" | "de";
  humans: readonly [HubCollaborationHuman, HubCollaborationHuman];
  spaceId: string | null;
  documentId: string | null;
  nativeBinary: string | null;
  nativeModules: string | null;
  outDir: string;
  tag: string;
  signal: AbortSignal;
}>;

/** 📒️ One judged step. */
export type HubCollaborationStepV1 = Readonly<{ name: string; pass: boolean; atMs: number; detail: unknown }>;

/** 📜️ One run's report (`report.json`). */
export type HubCollaborationReportV1 = {
  schema: "semio.wgpu.hub-collaboration-report/v1";
  journey: HubCollaborationJourneyV1;
  locale: "en" | "de";
  hub: string;
  spaceId: string | null;
  documentId: string | null;
  startedAt: string;
  finishedAt: string | null;
  steps: HubCollaborationStepV1[];
  measured: Record<string, number | string | boolean>;
  fatal: string | null;
};
//#endregion 🔖️Contract

//#region 🔖️Relay
/** ✂️️ A severable TCP relay in front of the hub: a session that signs in through `origin` reaches the hub until `sever()`
 * destroys every open connection (the document socket included — a browser's offline switch leaves an open WebSocket
 * alive, ticket 26/09/23 run s14a) and refuses new ones until `heal()`. A real connection shortage, not an emulated one. */
export type SeverableRelayV1 = Readonly<{ origin: string; sever: () => number; heal: () => void; close: () => Promise<void> }>;

export async function startSeverableRelay(hubOrigin: string): Promise<SeverableRelayV1> {
  const target = new URL(hubOrigin);
  const sockets = new Set<Socket>();
  let severed = false;
  const server = createServer((client) => {
    if (severed) {
      client.destroy();
      return;
    }
    const upstream = connect(Number(target.port || 80), target.hostname);
    for (const socket of [client, upstream]) {
      sockets.add(socket);
      socket.on("close", () => sockets.delete(socket));
      socket.on("error", () => {
        client.destroy();
        upstream.destroy();
      });
    }
    client.pipe(upstream);
    upstream.pipe(client);
  });
  await new Promise<void>((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
  const { port } = server.address() as AddressInfo;
  return {
    origin: `http://127.0.0.1:${port}`,
    sever: () => {
      severed = true;
      const cut = sockets.size;
      for (const socket of sockets) socket.destroy();
      return cut;
    },
    heal: () => {
      severed = false;
    },
    close: () =>
      new Promise<void>((resolveClose) => {
        for (const socket of sockets) socket.destroy();
        server.close(() => resolveClose());
      }),
  };
}
//#endregion 🔖️Relay

//#region 🔖️Pixels
/** 🖼️ The RGBA pixels of one page region (a PNG screenshot decoded in the page, so no image library is needed). */
async function regionPixels(page: Page, clip: Readonly<{ x: number; y: number; width: number; height: number }>, path?: string): Promise<number[]> {
  const png = await page.screenshot({ clip, type: "png", ...(path ? { path } : {}) });
  return page.evaluate(async (bytes) => {
    const bitmap = await createImageBitmap(new Blob([new Uint8Array(bytes)], { type: "image/png" }));
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext("2d")!;
    context.drawImage(bitmap, 0, 0);
    return Array.from(context.getImageData(0, 0, bitmap.width, bitmap.height).data);
  }, [...png]);
}

/** 🔢️ How many pixels differ by more than a visible amount (sum of the RGB channel deltas > 60). */
export function changedPixelCount(left: readonly number[], right: readonly number[]): number {
  let changed = Math.abs(left.length - right.length) / 4;
  for (let index = 0; index < Math.min(left.length, right.length); index += 4) {
    if (Math.abs(left[index]! - right[index]!) + Math.abs(left[index + 1]! - right[index + 1]!) + Math.abs(left[index + 2]! - right[index + 2]!) > 60) changed += 1;
  }
  return changed;
}

/** 🎨️ How many distinct colours a sampled screenshot holds — a painted frame is not a single colour. */
async function distinctColours(page: Page, path: string): Promise<number> {
  const png = await page.screenshot({ path, type: "png" });
  return page.evaluate(async (bytes) => {
    const bitmap = await createImageBitmap(new Blob([new Uint8Array(bytes)], { type: "image/png" }));
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext("2d")!;
    context.drawImage(bitmap, 0, 0);
    const data = context.getImageData(0, 0, bitmap.width, bitmap.height).data;
    const colours = new Set<string>();
    for (let index = 0; index < data.length && colours.size < 64; index += 4 * 97) colours.add(`${data[index]},${data[index + 1]},${data[index + 2]}`);
    return colours.size;
  }, [...png]);
}
//#endregion 🔖️Pixels

//#region 🔖️Wasm32Shell
/** 🔍️ One node of the wasm32 shell's accessibility projection. */
export type MirrorNode = Readonly<{ key: string; label?: string; role?: string; disabled?: boolean; checked?: boolean; valueText?: string; windowId: string; rect?: readonly number[] }>;

/** 👁️ One reading of a wasm32 shell: its sync label, roster lines, accessibility projection and painted document texts. */
export type Wasm32View = Readonly<{ sync: string; peers: string[]; nodes: MirrorNode[]; texts: string[] }>;

const MIRROR = "#semio-wgpu-accessibility";
const LIVE_SYNC = /live|connected|persisted|verbunden|gespeichert/iu;
const LINK_KEY = /(?:^|\/)framework\.sync\.link\.([a-z-]+)$/u;
const SYNC_CARD = "s-sync-status";
const ABSENT_PEERS = /^(No one else is here|Niemand sonst ist hier)$/u;

/** 🌐️ One human in one isolated browser profile on the wasm32 wgpu shell, signed in to `hubOrigin` (the hub or a relay). */
export class Wasm32Shell {
  readonly lines: string[] = [];
  readonly hubLines: string[] = [];
  readonly documentFrames: { at: number; direction: string; bytes: number; namesDocument: boolean }[] = [];
  private constructor(
    readonly human: HubCollaborationHuman,
    readonly context: BrowserContext,
    readonly page: Page,
    readonly hubOrigin: string,
    private readonly clock: () => number,
    private readonly documentId: () => string,
  ) {}

  static async boot(browser: Browser, human: HubCollaborationHuman, serve: string, hubOrigin: string, locale: "en" | "de", clock: () => number, documentId: () => string): Promise<Wasm32Shell> {
    const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1, locale: locale === "de" ? "de-DE" : "en-US" });
    const shell = new Wasm32Shell(human, context, await context.newPage(), hubOrigin, clock, documentId);
    const { page } = shell;
    page.on("console", (message) => shell.lines.push(`${clock()} ${message.type()} ${message.text().slice(0, 600)}`));
    page.on("pageerror", (error) => shell.lines.push(`${clock()} pageerror ${String(error).slice(0, 600)}`));
    page.on("response", (response) => {
      if (response.url().startsWith(hubOrigin)) shell.hubLines.push(`${clock()} ${response.status()} ${response.request().method()} ${response.url().replace(hubOrigin, "")}`);
    });
    page.on("websocket", (socket) => {
      shell.hubLines.push(`${clock()} ws-open ${socket.url().replace(/^wss?:\/\/[^/]+/u, "")}`);
      if (!socket.url().includes("/document/ws")) return;
      const keep = (direction: string) => (frame: { payload: string | Buffer }) => {
        const body = Buffer.from(frame.payload as Buffer);
        shell.documentFrames.push({ at: clock(), direction, bytes: body.length, namesDocument: body.toString("latin1").includes(documentId()) });
      };
      socket.on("framesent", keep("sent"));
      socket.on("framereceived", keep("received"));
      socket.on("close", () => shell.hubLines.push(`${clock()} ws-closed ${socket.url().replace(/^wss?:\/\/[^/]+/u, "")}`));
    });
    await page.goto(serve, { waitUntil: "domcontentloaded", timeout: HUB_COLLABORATION_BOUNDS.bootMs });
    await page.waitForFunction(() => typeof (globalThis as { semioWgpuIntrospection?: { dumpStructure?: unknown } }).semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: HUB_COLLABORATION_BOUNDS.bootMs });
    await page.waitForTimeout(8_000);
    return shell;
  }

  async projection(): Promise<MirrorNode[]> {
    const raw = await this.page.evaluate(async () => (await (globalThis as { semioWgpuIntrospection?: { dumpAccessibility?: () => Promise<string> } }).semioWgpuIntrospection?.dumpAccessibility?.()) ?? "");
    try {
      const parsed = JSON.parse(raw) as { windows?: { windowId: string; nodes?: Omit<MirrorNode, "windowId">[] }[] };
      return (parsed.windows ?? []).flatMap((surface) => (surface.nodes ?? []).map((node) => ({ ...node, windowId: surface.windowId })));
    } catch {
      return [];
    }
  }

  async keyEndingWith(suffix: string): Promise<string | undefined> {
    return (await this.projection()).find((node) => String(node.key).endsWith(suffix))?.key;
  }

  async awaitKey(suffix: string, budgetMs = 40_000): Promise<string | undefined> {
    const started = Date.now();
    let key = await this.keyEndingWith(suffix);
    while (!key && Date.now() - started < budgetMs) {
      await this.page.waitForTimeout(1_000);
      key = await this.keyEndingWith(suffix);
    }
    return key;
  }

  async activate(key: string | undefined, settleMs = 2_500): Promise<string> {
    if (!key) return "absent";
    const outcome = await this.page.evaluate(({ selector, nodeKey }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`) as HTMLButtonElement | null;
      if (!element) return "absent";
      if (element.disabled === true) return "disabled";
      element.focus();
      element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
      return "activated";
    }, { selector: MIRROR, nodeKey: key });
    await this.page.waitForTimeout(settleMs);
    return outcome;
  }

  async typeInto(key: string | undefined, value: string, verify = false, budgetMs = 15_000): Promise<boolean> {
    if (!key) return false;
    const started = Date.now();
    while (Date.now() - started < budgetMs) {
      const applied = await this.page.evaluate(({ selector, nodeKey, text }) => {
        const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
        if (!(element instanceof HTMLInputElement) && !(element instanceof HTMLTextAreaElement)) return false;
        element.focus();
        element.value = text;
        element.dispatchEvent(new Event("input", { bubbles: true }));
        element.dispatchEvent(new Event("change", { bubbles: true }));
        return true;
      }, { selector: MIRROR, nodeKey: key, text: value });
      await this.page.waitForTimeout(applied ? 1_500 : 500);
      if (applied && (!verify || (await this.projection()).some((node) => node.key === key && node.valueText === value))) return true;
    }
    return false;
  }

  async submitInput(key: string | undefined): Promise<boolean> {
    if (!key) return false;
    const submitted = await this.page.evaluate(({ selector, nodeKey }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`) as HTMLElement | null;
      if (!element) return false;
      element.focus();
      element.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", code: "Enter", bubbles: true }));
      return true;
    }, { selector: MIRROR, nodeKey: key });
    await this.page.waitForTimeout(3_000);
    return submitted;
  }

  /** 🔁️ Re-selects a hub connection (the mirror's enabled state is republished by the next hub verb). */
  private async republish(preferRemote: boolean): Promise<string> {
    const connections = (await this.projection()).filter((node) => String(node.key).includes("sign-in.connection"));
    const connection = (preferRemote ? connections.find((node) => String(node.key).includes("remote:")) : undefined) ?? connections[0];
    return this.activate(connection?.key, 2_000);
  }

  /** 🔑️ Signs in through the shell's own hub workspace (address → add connection → credentials → submit). */
  async signIn(): Promise<{ minted: boolean; attempts: number; chrome: string[] }> {
    await this.activate("framework.hub.signIn", 2_000);
    await this.typeInto(await this.awaitKey("framework.hub.address"), this.hubOrigin);
    for (let attempt = 0; attempt < 4; attempt += 1) {
      await this.republish(false);
      if ((await this.activate(await this.awaitKey("framework.hub.sign-in.add", 5_000), 4_000)) === "activated") break;
    }
    const minted = (): boolean => this.hubLines.some((line) => / 20[01] POST \/auth\/sessions$/u.test(line));
    let attempts = 0;
    for (; attempts < 4 && !minted(); attempts += 1) {
      await this.republish(true);
      await this.typeInto(await this.awaitKey("framework.hub.email", 5_000), this.human.email);
      await this.typeInto(await this.awaitKey("framework.hub.password", 5_000), this.human.password);
      await this.republish(true);
      await this.activate(await this.awaitKey("framework.hub.sign-in.submit", 5_000), 12_000);
    }
    await this.republish(true);
    await this.page.waitForTimeout(3_000);
    const nodes = await this.projection();
    const chrome = nodes.filter((node) => node.windowId === "framework.hub" && node.role === "button").map((node) => String(node.label ?? "")).filter(Boolean).slice(0, 6);
    await this.activate(nodes.find((node) => String(node.key).endsWith("framework.hub.close"))?.key, 1_500);
    return { minted: minted(), attempts, chrome };
  }

  /** 🔄️ Opens the Sync card only when its dock switch is off — activating an open card's switch closes it. */
  async openSyncCard(): Promise<void> {
    const pill = (await this.projection()).find((node) => node.key === SYNC_CARD && node.windowId === "shell.chrome");
    if (pill?.checked !== true) await this.activate(SYNC_CARD, 2_000);
  }

  /** 🔄️ Closes the Sync card, judged by its painted content (the `framework.sync.panel` group), not the pill's switch state —
   * a pixel comparison needs the same chrome in both frames. */
  async closeSyncCard(): Promise<boolean> {
    for (let attempt = 0; attempt < 3; attempt += 1) {
      if (!(await this.projection()).some((node) => String(node.key).endsWith("framework.sync.panel"))) return true;
      await this.activate(SYNC_CARD, 2_000);
    }
    return !(await this.projection()).some((node) => String(node.key).endsWith("framework.sync.panel"));
  }

  /** 🔗️ Attaches `remote://<hub host>/<space>/<document>` from the Sync card. */
  async attach(spaceId: string, documentId: string): Promise<{ uri: string; pressed: string }> {
    await this.openSyncCard();
    await this.activate(await this.awaitKey("framework.sync.remote", 45_000), 2_000);
    const path = await this.awaitKey("framework.sync.remote.path", 30_000);
    const uri = `${this.hubOrigin.replace(/^https?:\/\//u, "")}/${spaceId}/${documentId}`;
    await this.typeInto(path, uri, true);
    await this.submitInput(path);
    let attach = (await this.projection()).find((node) => String(node.key).endsWith("framework.sync.attach"));
    for (let tick = 0; tick < 30 && attach?.disabled !== false; tick += 1) {
      await this.page.waitForTimeout(500);
      attach = (await this.projection()).find((node) => String(node.key).endsWith("framework.sync.attach"));
    }
    return { uri, pressed: await this.activate(attach?.key, 4_000) };
  }

  /** 🖼️ The painted text runs of the shell's main document window (`dumpStructure`) — what a sighted user reads. */
  async paintedTexts(): Promise<string[]> {
    const raw = await this.page.evaluate(async () => (await (globalThis as { semioWgpuIntrospection?: { dumpStructure?: () => Promise<string> } }).semioWgpuIntrospection?.dumpStructure?.()) ?? "");
    try {
      return ((JSON.parse(raw) as { nodes?: { text?: string | null }[] }).nodes ?? []).flatMap((node) => (typeof node.text === "string" ? [node.text] : []));
    } catch {
      return [];
    }
  }

  async view(): Promise<Wasm32View> {
    const nodes = await this.projection();
    return {
      sync: String(nodes.find((node) => node.key === SYNC_CARD)?.label ?? ""),
      peers: nodes.filter((node) => node.key === "s-presence-peers" && !ABSENT_PEERS.test(String(node.label ?? ""))).map((node) => String(node.label ?? "")),
      nodes,
      texts: await this.paintedTexts(),
    };
  }

  async waitFor<T>(probe: (view: Wasm32View) => T | null | false, budgetMs: number): Promise<{ value: T | null; view: Wasm32View; afterMs: number }> {
    const started = Date.now();
    let view = await this.view();
    let value = probe(view);
    while (!value && Date.now() - started < budgetMs) {
      await this.page.waitForTimeout(1_000);
      view = await this.view();
      value = probe(view);
    }
    return { value: value || null, view, afterMs: Date.now() - started };
  }

  /** 🫀️ The frame worker's own stats answer and how long it took (a frozen worker never answers). */
  async beat(): Promise<{ answered: boolean; latencyMs: number }> {
    const started = Date.now();
    const answer = await Promise.race([this.page.evaluate(async () => (await (globalThis as { semioWgpuIntrospection?: { dumpFrameStats?: () => Promise<string> } }).semioWgpuIntrospection?.dumpFrameStats?.()) ?? ""), new Promise<null>((resolveLate) => setTimeout(() => resolveLate(null), 5_000))]);
    return { answered: answer !== null, latencyMs: Date.now() - started };
  }

  /** 🔗️ The Sync card's link line (`framework.sync.link.<code>`) and its spoken text, read with the card open. */
  async link(): Promise<{ codes: string[]; texts: string[]; sync: string }> {
    await this.openSyncCard();
    const view = await this.view();
    const card = view.nodes.filter((node) => node.windowId === SYNC_CARD);
    return {
      codes: card.flatMap((node) => LINK_KEY.exec(String(node.key))?.slice(1, 2) ?? []),
      texts: card.filter((node) => node.role === "paragraph" && /verbindung|connection|zugriff|access/iu.test(String(node.label ?? ""))).map((node) => String(node.label)),
      sync: view.sync,
    };
  }

  /** 🪟️ The page rectangle of one document window's body, from its accessibility root node and the mirror's own placement. */
  async windowClip(windowId: string): Promise<{ x: number; y: number; width: number; height: number } | null> {
    return this.page.evaluate(({ selector, id }) => {
      const element = document.querySelector(`${selector} [data-window-id="${id}"], ${selector} [data-node-window="${id}"]`) as HTMLElement | null;
      const rect = element?.getBoundingClientRect();
      if (rect && rect.width > 20 && rect.height > 20) return { x: Math.round(rect.x), y: Math.round(rect.y), width: Math.round(rect.width), height: Math.round(rect.height) };
      const canvas = document.querySelector("canvas")?.getBoundingClientRect();
      return canvas ? { x: Math.round(canvas.x), y: Math.round(canvas.y + 40), width: Math.round(canvas.width), height: Math.round(canvas.height - 80) } : null;
    }, { selector: MIRROR, id: windowId });
  }
}
//#endregion 🔖️Wasm32Shell

//#region 🔖️ReactShell
/** ⚛️ What the React `s` shell publishes about the open document, its sync binding, the roster and the canvas presence. */
type ReactReading = Readonly<{
  ready: string | null;
  syncPill: string;
  hub: string | null;
  peers: readonly Readonly<{ id: string; label: string }>[];
  cursors: readonly Readonly<{ actor: string | null }>[];
  boards: readonly Readonly<{ x: number; y: number; w: number; h: number }>[];
  handleKinds: number;
  opening: boolean;
  home: boolean;
  spaceIndex: boolean;
}>;

/** ⚛️ One human in one isolated browser profile on the React `s` shell (block2d: the document observable is the Handle Kinds
 * count the block editor prints; edits and undo are window actions). */
export class ReactShell {
  readonly lines: string[] = [];
  readonly sockets: { url: string; openedAt: number; closedAt: number | null }[] = [];
  private constructor(
    readonly human: HubCollaborationHuman,
    readonly context: BrowserContext,
    readonly page: Page,
    readonly serve: string,
    private readonly clock: () => number,
  ) {}

  static async boot(browser: Browser, human: HubCollaborationHuman, serve: string, locale: "en" | "de", clock: () => number): Promise<ReactShell> {
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: locale === "de" ? "de-DE" : "en-US" });
    const shell = new ReactShell(human, context, await context.newPage(), serve, clock);
    const { page } = shell;
    page.on("console", (message) => {
      if (!/Download the React DevTools|\[vite\]|status of 404|\[stale\]/u.test(message.text())) shell.lines.push(`${clock()} ${message.type()} ${message.text().slice(0, 600)}`);
    });
    page.on("pageerror", (error) => shell.lines.push(`${clock()} pageerror ${String(error).slice(0, 600)}`));
    page.on("websocket", (socket) => {
      const row = { url: socket.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 160), openedAt: clock(), closedAt: null as number | null };
      shell.sockets.push(row);
      socket.on("close", () => (row.closedAt = clock()));
    });
    await page.goto(serve, { waitUntil: "domcontentloaded", timeout: HUB_COLLABORATION_BOUNDS.bootMs });
    await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 300_000 });
    return shell;
  }

  read(): Promise<ReactReading> {
    return this.page.evaluate(() => {
      const text = (element: Element | null): string => ((element as HTMLElement | null)?.innerText ?? "").replace(/\s+/gu, " ").trim();
      return {
        ready: document.documentElement.getAttribute("data-semio-os-ready"),
        syncPill: text(document.querySelector('[id="s-sync-status"]')),
        hub: document.querySelector("[data-semio-hub-connection]")?.getAttribute("data-semio-hub-connection") ?? null,
        peers: [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')].map((element) => ({ id: element.getAttribute("data-row-id") ?? "", label: text(element).slice(0, 64) })),
        cursors: [...document.querySelectorAll("[data-peer-cursor]")].map((element) => ({ actor: element.getAttribute("data-peer-actor") })),
        boards: [...document.querySelectorAll("canvas")].map((element) => {
          const rect = element.getBoundingClientRect();
          return { x: rect.x, y: rect.y, w: rect.width, h: rect.height };
        }),
        handleKinds: Number((/(\d+) (?:Handle Kinds?|Griffarten?)/u.exec(document.body.innerText) ?? [])[1] ?? Number.NaN),
        opening: /Verifying document component|Restoring|Dokumentkomponente wird geprüft|Wird wiederhergestellt/u.test(document.body.innerText),
        home: document.querySelectorAll('[data-ui-node-key="s-home-create-space"]').length > 0,
        spaceIndex: document.querySelectorAll('[data-ui-node-key="s-space-create-artifact"]').length > 0,
      };
    });
  }

  async until(budgetMs: number, test: (reading: ReactReading) => boolean): Promise<{ afterMs: number | null; reading: ReactReading }> {
    const started = Date.now();
    let reading = await this.read();
    while (!test(reading) && Date.now() - started < budgetMs) {
      await this.page.waitForTimeout(400);
      reading = await this.read();
    }
    return { afterMs: test(reading) ? Date.now() - started : null, reading };
  }

  async signIn(): Promise<boolean> {
    const { page } = this;
    await page.locator('[data-semio-hub-sign-in=""]').first().click();
    const form = page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 30_000 });
    await form.locator('input[type="email"]').fill(this.human.email);
    await form.locator('input[type="password"]').fill(this.human.password);
    await form.locator('[id="os.hub.signIn.submit"]').click();
    await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
    await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
    await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
    return true;
  }

  private async rowOpen(prefix: string, id: string): Promise<string> {
    const buttons = this.page.locator(`[data-ui-node-key="${prefix}:${id}"] button`);
    const count = await buttons.count();
    for (let index = 0; index < count; index += 1) {
      const button = buttons.nth(index);
      const name = `${(await button.getAttribute("aria-label")) ?? ""} ${(await button.getAttribute("title")) ?? ""} ${(await button.textContent()) ?? ""}`;
      if (/open|öffnen/iu.test(name)) {
        await button.focus();
        await button.press("Enter");
        return name.trim();
      }
    }
    return `no open among ${count} buttons`;
  }

  /** 📂️ Opens one hub document from its space's index and waits for it to be live and mounted. */
  async open(spaceId: string, documentId: string, mounted: (reading: ReactReading) => boolean): Promise<{ liveAfterMs: number | null; reading: ReactReading }> {
    const artifactRow = this.page.locator(`[data-ui-node-key="artifact:${documentId}"]`).first();
    const spaceRow = this.page.locator(`[data-ui-node-key="space:${spaceId}"]`).first();
    const landed = await Promise.race([artifactRow.waitFor({ state: "attached", timeout: 20_000 }).then(() => "artifact"), spaceRow.waitFor({ state: "attached", timeout: 20_000 }).then(() => "space")]).catch(() => "neither");
    if (landed === "space") await this.rowOpen("space", spaceId);
    else if (landed === "neither") await this.page.goto(new URL(`/spaces/${spaceId}`, this.serve).href, { waitUntil: "domcontentloaded", timeout: HUB_COLLABORATION_BOUNDS.bootMs });
    await artifactRow.waitFor({ state: "attached", timeout: HUB_COLLABORATION_BOUNDS.bootMs });
    await this.rowOpen("artifact", documentId);
    const live = await this.until(240_000, (reading) => Boolean(reading.syncPill) && !/detached|getrennt|connecting|verbindet|backoff/iu.test(reading.syncPill) && this.sockets.some((row) => row.url.includes("/document/ws") && row.closedAt === null));
    const settled = await this.until(240_000, (reading) => !reading.opening && mounted(reading));
    return { liveAfterMs: live.afterMs === null || settled.afterMs === null ? null : live.afterMs + settled.afterMs, reading: settled.reading };
  }

  /** 🎛️ Runs one window action, unfolding the window's Actions pane first when its row is not rendered yet. */
  async runAction(actionId: string): Promise<string> {
    if (!(await this.page.locator(`[id="action.${actionId}"]`).count())) {
      await this.page.getByText(/^(?:Actions|Aktionen)$/u).first().click({ force: true, timeout: 8_000 }).catch(() => undefined);
      await this.page.waitForTimeout(800);
    }
    if (!(await this.page.locator(`[id="action.${actionId}"]`).count())) return "absent";
    return this.page.locator(`[id="action.${actionId}"]`).first().click({ timeout: 8_000, force: true }).then(() => "ok").catch((error: unknown) => String(error).split("\n")[0]!.slice(0, 120));
  }
}
//#endregion 🔖️ReactShell

//#region 🔖️Kinds
/** 🧩️ How the wasm32 shell reads and edits one kind's document (controls through its accessibility projection). */
type Wasm32Kind = Readonly<{
  prepare: (shell: Wasm32Shell) => Promise<void>;
  count: (view: Wasm32View) => number;
  edit: (shell: Wasm32Shell) => Promise<string>;
  undo: (shell: Wasm32Shell) => Promise<string>;
}>;

const HANDLE_KINDS = /(\d+) (?:Handle Kinds?|Griffarten?)/u;

/** ↩️ The shell's own History panel undo (opened first when it is closed). */
async function historyUndo(shell: Wasm32Shell): Promise<string> {
  if (!(await shell.keyEndingWith("framework.history.undo.run"))) await shell.activate("framework.panel.history", 2_500);
  return shell.activate(await shell.awaitKey("framework.history.undo.run", 10_000), 4_000);
}

/** 🧩️ note: the Artifact panel lists one `note-play-block:` row per block; Add Text authors one. block2d: the board paints its
 * Handle Kinds count; the window's Actions pane runs `addHandleKind`. Undo is the shell's own History verb on both. */
const WASM32_KINDS: Readonly<Record<string, Wasm32Kind>> = {
  note: {
    prepare: async (shell) => {
      for (let attempt = 0; attempt < 3 && !(await shell.keyEndingWith("note-play-blocks.add.text")); attempt += 1) {
        if ((await shell.projection()).find((node) => node.key === "framework.panel.artifact")?.checked !== true) await shell.activate("framework.panel.artifact", 3_000);
        await shell.awaitKey("note-play-blocks.add.text", 10_000);
      }
    },
    count: (view) => view.nodes.filter((node) => String(node.key).startsWith("note-play-block:")).length,
    edit: async (shell) => shell.activate(await shell.keyEndingWith("note-play-blocks.add.text"), 4_000),
    undo: historyUndo,
  },
  block2d: {
    prepare: async (shell) => {
      if (!(await shell.keyEndingWith("action.addHandleKind"))) await shell.activate(await shell.awaitKey("framework.window.block2dBoard.engagement.toggle", 30_000), 3_000);
      await shell.awaitKey("action.addHandleKind", 30_000);
    },
    count: (view) => Number(view.texts.map((text) => HANDLE_KINDS.exec(text)?.[1]).find((value) => value !== undefined) ?? Number.NaN),
    edit: async (shell) => shell.activate(await shell.keyEndingWith("action.addHandleKind"), 4_000),
    undo: historyUndo,
  },
};

function wasm32Kind(variant: string): Wasm32Kind {
  const kind = WASM32_KINDS[variant];
  if (!kind) throw new Error(`no wasm32 document observable for ${variant}`);
  return kind;
}
//#endregion 🔖️Kinds

//#region 🔖️Hub
/** 🏘️ The journey's document: the given one, or a fresh private space of human A with B seated as an author and one document of
 * `schema` created by the hub's own creation saga. */
async function prepareDocument(options: HubCollaborationOptionsV1, schema: string, token: string): Promise<{ spaceId: string; documentId: string; createdMs: number }> {
  if (options.documentId) {
    if (!options.spaceId) throw new HubCollaborationBlocked("--document needs --space");
    return { spaceId: options.spaceId, documentId: options.documentId, createdMs: 0 };
  }
  const spaceId = options.spaceId ?? (await hubProbeCreateSpace(options.hub, token, `wgpu collaboration ${options.journey} ${options.tag}`));
  const member = await hubProbeCall(options.hub, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomRequestId(), { kind: "upsert-member", spaceId, email: options.humans[1].email, role: "author" })));
  if (member.status !== 202 && member.status !== 200) throw new Error(`seating ${options.humans[1].label} as author answered ${member.status} ${member.text.slice(0, 200)}`);
  const catalog = await hubProbeCreationCatalog(options.hub, token, spaceId);
  const kind = catalog.kinds.find((row) => row.schema === schema);
  if (!kind) throw new HubCollaborationBlocked(`the hub's creation catalog has no ${schema} kind`);
  const created = await hubProbeCreateArtifact(options.hub, token, spaceId, catalog.generationId, kind.kindId, `wgpu ${options.journey} ${options.tag}`);
  return { spaceId, documentId: created.artifactId, createdMs: created.ms };
}

function randomRequestId(): string {
  return Array.from(crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 📏️ The hub's committed head of one document, read over the hub's REST surface. */
async function documentHead(hub: string, token: string, spaceId: string, documentId: string): Promise<number> {
  const answer = await hubProbeCall(hub, "GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, token);
  return Number(answer.json?.head_seq ?? -1);
}
//#endregion 🔖️Hub

//#region 🔖️NativePeer
/** 🦀️ The native wgpu user: one ignored live law of the renderer's test binary (`--native-binary`) or of `cargo test`, seated as
 * human A on the hub with human B as its peer, stepping through the file handshake in `dir`. Output → `<out>/native.txt`. */
function spawnNativeLaw(options: HubCollaborationOptionsV1, law: string, dir: string, out: string): { child: ChildProcess; exit: Promise<number> } {
  if (!options.nativeModules) throw new HubCollaborationBlocked("the native block2d runtime directory is not staged (--native-modules)");
  const env: NodeJS.ProcessEnv = {
    ...process.env,
    SEMIO_HUB_LIVE_ORIGIN: options.hub,
    SEMIO_HUB_LIVE_EMAIL: options.humans[0].email,
    SEMIO_HUB_LIVE_PASSWORD: options.humans[0].password,
    SEMIO_HUB_LIVE_PEER_EMAIL: options.humans[1].email,
    SEMIO_CROSS_SHELL_DIR: dir,
    SEMIO_PLUGIN: "block2d",
    SEMIO_PLUGIN_MODULES: options.nativeModules,
  };
  const path = `shell::hub_projection_workspace_tests::${law}`;
  const [command, args] = options.nativeBinary
    ? [options.nativeBinary, [path, "--exact", "--ignored", "--nocapture", "--test-threads=1"]]
    : ["cargo", ["test", "-p", "semio-framework-os-renderer-wgpu", "--lib", "--", path, "--exact", "--ignored", "--nocapture", "--test-threads=1"]];
  mkdirSync(out, { recursive: true });
  const log: string[] = [];
  const child = spawn(command, args, { cwd: options.repoRoot, env, stdio: ["ignore", "pipe", "pipe"] });
  const keep = (chunk: Buffer): void => {
    log.push(chunk.toString("utf8"));
    writeFileSync(join(out, "native.txt"), log.join(""));
  };
  child.stdout!.on("data", keep);
  child.stderr!.on("data", keep);
  const exit = new Promise<number>((resolveExit) => child.on("close", (code) => resolveExit(code ?? 1)));
  options.signal.addEventListener("abort", () => child.kill("SIGTERM"), { once: true });
  return { child, exit };
}

/** 🤝️ The browser half of the native law's file handshake (`native-<step>.json` ← native, `react-<step>.json` ← browser). */
class Handshake {
  constructor(
    readonly dir: string,
    private readonly record: (name: string, pass: boolean, detail: unknown) => void,
  ) {
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
  }

  publish(step: string, value: Record<string, unknown>): void {
    const staged = join(this.dir, `.react-${step}.json`);
    writeFileSync(staged, JSON.stringify(value, null, 2));
    renameSync(staged, join(this.dir, `react-${step}.json`));
  }

  async awaitNative(step: string, budgetMs: number, alive: () => boolean): Promise<Record<string, unknown> | null> {
    const path = join(this.dir, `native-${step}.json`);
    const started = Date.now();
    while (Date.now() - started < budgetMs && alive()) {
      if (existsSync(path)) return JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 200));
    }
    this.record(`native step ${step} arrived`, false, { budgetMs, nativeAlive: alive() });
    return null;
  }
}

/** 👣️ What a browser user must do to follow the native edits law: open, see a peer, read the document count, edit, undo, reload. */
type Follower = Readonly<{
  open: (spaceId: string, documentId: string) => Promise<{ liveAfterMs: number | null; detail: unknown }>;
  seesPeer: (actor: string, userId: string) => Promise<{ afterMs: number | null; peers: unknown }>;
  count: () => Promise<number>;
  edit: () => Promise<string>;
  undo: () => Promise<string>;
  reload: (spaceId: string, documentId: string) => Promise<{ liveAfterMs: number | null }>;
}>;

async function countUntil(follower: Follower, budgetMs: number, test: (count: number) => boolean): Promise<{ afterMs: number | null; count: number }> {
  const started = Date.now();
  let count = await follower.count();
  while (!test(count) && Date.now() - started < budgetMs) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 400));
    count = await follower.count();
  }
  return { afterMs: test(count) ? Date.now() - started : null, count };
}

/** 🤝️ Follows `a_native_and_a_react_user_collaborate_on_one_hub_document` step by step; the native law judges the ledger, this
 * side records what the browser user saw. */
async function followNativeEdits(handshake: Handshake, follower: Follower, alive: () => boolean, record: (name: string, pass: boolean, detail: unknown) => void): Promise<void> {
  const nativeOpen = await handshake.awaitNative("open", HUB_COLLABORATION_BOUNDS.nativeStepMs, alive);
  if (!nativeOpen) return;
  const [spaceId, documentId] = [String(nativeOpen.spaceId ?? ""), String(nativeOpen.documentId ?? "")];
  const opened = await follower.open(spaceId, documentId);
  handshake.publish("open", { live: opened.liveAfterMs !== null, liveAfterMs: opened.liveAfterMs });
  record("browser opens the native user's document and is live", opened.liveAfterMs !== null, { spaceId, documentId, ...(opened.detail as object) });
  const nativePresence = await handshake.awaitNative("presence", 300_000, alive);
  const seen = await follower.seesPeer(String(nativeOpen.actor ?? ""), String(nativeOpen.userId ?? ""));
  handshake.publish("presence", { seesNative: seen.afterMs !== null, afterMs: seen.afterMs });
  record("presence both ways", seen.afterMs !== null && nativePresence?.seesReact === true, { browserSeesNativeAfterMs: seen.afterMs, nativeSeesBrowser: nativePresence?.seesReact, peers: seen.peers });
  const beforeNative = await follower.count();
  await handshake.awaitNative("edit", 300_000, alive);
  const ingested = await countUntil(follower, HUB_COLLABORATION_BOUNDS.editSeenMs, (count) => count > beforeNative);
  handshake.publish("ingested", { ingested: ingested.afterMs !== null, afterMs: ingested.afterMs });
  record("the native edit reaches the browser", ingested.afterMs !== null, { before: beforeNative, after: ingested.count, afterMs: ingested.afterMs });
  const beforeOwn = await follower.count();
  const executed = await follower.edit();
  const own = await countUntil(follower, HUB_COLLABORATION_BOUNDS.editSeenMs, (count) => count > beforeOwn);
  handshake.publish("edit", { ok: own.afterMs !== null, executed });
  const nativeIngested = await handshake.awaitNative("ingested", 300_000, alive);
  record("the browser edit reaches the native user", own.afterMs !== null && nativeIngested?.ingested === true, { executed, before: beforeOwn, after: own.count, nativeIngested: nativeIngested?.ingested });
  const nativeUndo = await handshake.awaitNative("undo", 300_000, alive);
  const sawNativeUndo = await countUntil(follower, 30_000, (count) => count < own.count);
  const beforeUndo = await follower.count();
  const undo = await follower.undo();
  const undone = await countUntil(follower, 30_000, (count) => count < beforeUndo);
  handshake.publish("undo", { ok: undo === "ok" || undo === "activated", ownReverted: undone.afterMs !== null, sawNativeUndo: sawNativeUndo.afterMs !== null });
  record("each user undoes only their own edit", nativeUndo?.ownReverted === true && sawNativeUndo.afterMs !== null && undone.afterMs !== null, { nativeUndo, sawNativeUndoAfterMs: sawNativeUndo.afterMs, undo, counts: [own.count, sawNativeUndo.count, beforeUndo, undone.count] });
  const reloaded = await follower.reload(spaceId, documentId);
  const settled = await countUntil(follower, 120_000, (count) => count === undone.count);
  handshake.publish("reloaded", { live: reloaded.liveAfterMs !== null && settled.afterMs !== null, liveAfterMs: reloaded.liveAfterMs });
  const beforeLast = await follower.count();
  await handshake.awaitNative("after-reload-edit", 300_000, alive);
  const converged = await countUntil(follower, HUB_COLLABORATION_BOUNDS.editSeenMs, (count) => count > beforeLast);
  handshake.publish("converged", { converged: converged.afterMs !== null, afterMs: converged.afterMs });
  record("a reload converges and the next native edit arrives", reloaded.liveAfterMs !== null && settled.afterMs !== null && converged.afterMs !== null, { reloadLiveAfterMs: reloaded.liveAfterMs, settled: settled.count, converged: converged.count });
  await handshake.awaitNative("done", 300_000, alive);
}

/** 🖱️ Follows `a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board` in the React shell. */
async function followNativeCursors(handshake: Handshake, react: ReactShell, alive: () => boolean, record: (name: string, pass: boolean, detail: unknown) => void): Promise<void> {
  const nativeOpen = await handshake.awaitNative("open", HUB_COLLABORATION_BOUNDS.nativeStepMs, alive);
  if (!nativeOpen) return;
  const opened = await react.open(String(nativeOpen.spaceId), String(nativeOpen.documentId), (reading) => reading.boards.some((box) => box.w > 100 && box.h > 100));
  handshake.publish("open", { live: opened.liveAfterMs !== null });
  record("React opens the native user's board and is live", opened.liveAfterMs !== null, { liveAfterMs: opened.liveAfterMs, sync: opened.reading.syncPill });
  await handshake.awaitNative("presence", 300_000, alive);
  const seen = await react.until(30_000, (reading) => reading.peers.some((row) => row.id.includes(String(nativeOpen.actor)) || row.id.includes(String(nativeOpen.userId))));
  handshake.publish("presence", { seesNative: seen.afterMs !== null });
  await handshake.awaitNative("cursor", 300_000, alive);
  const nativeCursor = await react.until(20_000, (reading) => reading.cursors.some((cursor) => cursor.actor === nativeOpen.actor));
  const board = [...nativeCursor.reading.boards].filter((box) => box.w > 100 && box.h > 100).sort((left, right) => right.w * right.h - left.w * left.h)[0];
  handshake.publish("cursor", { seesNativeCursor: nativeCursor.afterMs !== null });
  for (let step = 0; step < 40 && !existsSync(join(handshake.dir, "native-cursor-seen.json")); step += 1) {
    await react.page.mouse.move(board ? board.x + board.w * (0.35 + (step % 6) * 0.05) : 700, board ? board.y + board.h * 0.5 : 450);
    await react.page.waitForTimeout(250);
  }
  const nativeSeen = await handshake.awaitNative("cursor-seen", 60_000, alive);
  record("cursors both ways over one board", nativeCursor.afterMs !== null && nativeSeen?.seesReactCursor === true, { reactSeesNativeAfterMs: nativeCursor.afterMs, nativeSeesReact: nativeSeen?.seesReactCursor });
  await handshake.awaitNative("done", 300_000, alive);
}
//#endregion 🔖️NativePeer

//#region 🔖️Journeys
/** 🧰️ What every journey body receives: the options, the browser, the step recorder, the report's measured map, a clock,
 * the document it runs on and cleanup registration. */
type JourneyRun = Readonly<{
  options: HubCollaborationOptionsV1;
  browser: Browser;
  record: (name: string, pass: boolean, detail: unknown) => void;
  measured: Record<string, number | string | boolean>;
  clock: () => number;
  out: (name: string) => string;
  document: { spaceId: string; documentId: string };
  onCleanup: (cleanup: () => Promise<void>) => void;
  token: string;
}>;

const rosterNames = (label: string): string[] => label.split(/\s*·\s*/u).filter(Boolean);
const GERMAN_LINK_LINE = /Verbindung|Zugriff/u;
const ENGLISH_LINK_LINE = /connection|access/iu;

function requireServe(url: string | null, flag: string): string {
  if (!url) throw new HubCollaborationBlocked(`this run holds no ${flag} serve`);
  return url;
}

async function signedIn(run: JourneyRun, shell: Wasm32Shell): Promise<void> {
  const outcome = await shell.signIn();
  run.record(`${shell.human.label} signs in through the wgpu hub workspace`, outcome.minted, outcome);
}

async function attached(run: JourneyRun, shell: Wasm32Shell): Promise<boolean> {
  const outcome = await shell.attach(run.document.spaceId, run.document.documentId);
  const live = await shell.waitFor((view) => LIVE_SYNC.test(view.sync) && view.sync, HUB_COLLABORATION_BOUNDS.liveMs);
  writeFileSync(run.out(`projection-${shell.human.label}-${run.clock()}.json`), JSON.stringify(live.view.nodes, null, 1));
  run.record(`${shell.human.label} attaches the hub document and is live`, live.value !== null, { ...outcome, sync: live.view.sync, afterMs: live.afterMs });
  return live.value !== null;
}

/** 👥️ Two wasm32 shells, each behind its own severable relay: attach, presence, edits both ways, short / medium / long cuts,
 * a late joiner. */
async function wasm32Journey(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const serve = requireServe(options.wgpuServe, "--serve");
  const kind = wasm32Kind(HUB_COLLABORATION_KINDS.wasm32.variant);
  const relays = [await startSeverableRelay(options.hub), await startSeverableRelay(options.hub)] as const;
  run.onCleanup(async () => {
    for (const relay of relays) await relay.close();
  });
  const documentId = (): string => run.document.documentId;
  const [a, b] = [await Wasm32Shell.boot(browser, options.humans[0], serve, relays[0].origin, options.locale, run.clock, documentId), await Wasm32Shell.boot(browser, options.humans[1], serve, relays[1].origin, options.locale, run.clock, documentId)];
  run.onCleanup(async () => {
    for (const shell of [a, b]) {
      writeFileSync(run.out(`console-${shell.human.label}.txt`), shell.lines.join("\n"));
      writeFileSync(run.out(`hub-${shell.human.label}.txt`), shell.hubLines.join("\n"));
    }
  });
  for (const shell of [a, b]) record(`${shell.human.label} paints a frame`, (await distinctColours(shell.page, run.out(`boot-${shell.human.label}.png`))) > 4, {});
  for (const shell of [a, b]) await signedIn(run, shell);
  for (const shell of [a, b]) await attached(run, shell);
  const presence = await Promise.all([a, b].map((shell) => shell.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs)));
  record("presence both ways (each roster names both humans)", presence.every((seen) => seen.value !== null), { a: presence[0]!.view.peers, b: presence[1]!.view.peers });
  for (const shell of [a, b]) await kind.prepare(shell);
  const head = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  for (const shell of [a, b]) {
    const shown = await shell.waitFor((view) => kind.count(view) >= head && view, 30_000);
    record(`${shell.human.label} shows the document's committed content after attach (hub head ${head})`, shown.value !== null, { count: kind.count(shown.view), head });
  }
  const crossing = async (author: Wasm32Shell, reader: Wasm32Shell): Promise<void> => {
    const before = kind.count(await reader.view());
    const started = Date.now();
    const edited = await kind.edit(author);
    const seen = await reader.waitFor((view) => kind.count(view) > before && view, HUB_COLLABORATION_BOUNDS.editSeenMs);
    const latencyMs = Date.now() - started;
    run.measured[`edit${author.human.label}to${reader.human.label}Ms`] = seen.value === null ? -1 : latencyMs;
    record(`${author.human.label}'s edit reaches ${reader.human.label} without a reload`, edited === "activated" && seen.value !== null, { edited, before, after: kind.count(seen.view), latencyMs });
  };
  await crossing(a, b);
  await crossing(b, a);
  for (const shell of [a, b]) {
    const sent = shell.documentFrames.filter((frame) => frame.direction === "sent");
    record(`${shell.human.label}'s document frames name the document`, sent.some((frame) => frame.namesDocument), { sent: sent.length, received: shell.documentFrames.length - sent.length });
  }
  const onlineCount = kind.count(await a.view());
  const bBefore = kind.count(await b.view());
  const cut = relays[0].sever();
  const severedAt = Date.now();
  await a.page.waitForTimeout(3_000);
  const beats = [await a.beat(), await a.beat()];
  record("a short cut never freezes A (the frame worker keeps answering)", beats.every((beat) => beat.answered && beat.latencyMs < HUB_COLLABORATION_BOUNDS.frozenBeatMs), { beats, connectionsCut: cut });
  const offline = await kind.edit(a);
  const queued = await a.view();
  const keptSince = Date.now();
  let kept = await a.link();
  while (!/pending|ausstehend/iu.test(kept.sync) && !kept.codes.includes("reconnecting") && Date.now() - keptSince < HUB_COLLABORATION_BOUNDS.keptLineMs) {
    await a.page.waitForTimeout(500);
    kept = await a.link();
  }
  record("A edits during the cut: admitted locally, and the shell says the edit is kept", offline === "activated" && kind.count(queued) > onlineCount && (/pending|ausstehend/iu.test(kept.sync) || kept.codes.includes("reconnecting")), { offline, count: kind.count(queued), saidAfterMs: Date.now() - keptSince, ...kept });
  await a.page.waitForTimeout(Math.max(0, HUB_COLLABORATION_BOUNDS.shortCutMs - (Date.now() - severedAt)));
  relays[0].heal();
  const healedAt = Date.now();
  const relinked = await a.waitFor((view) => LIVE_SYNC.test(view.sync) && view, HUB_COLLABORATION_BOUNDS.relinkMs);
  const delivered = await b.waitFor((view) => kind.count(view) > bBefore && view, HUB_COLLABORATION_BOUNDS.relinkMs);
  run.measured.shortCutRelinkMs = relinked.value === null ? -1 : Date.now() - healedAt;
  record("after the short cut A relinks in place and B receives the offline edit", relinked.value !== null && delivered.value !== null, { cutMs: healedAt - severedAt, relinkAfterMs: relinked.afterMs, syncA: relinked.view.sync, countB: kind.count(delivered.view) });
  relays[0].sever();
  const mediumAt = Date.now();
  let spoken = await a.link();
  while (!spoken.codes.includes("reconnecting") && Date.now() - mediumAt < HUB_COLLABORATION_BOUNDS.mediumCutMs) {
    await a.page.waitForTimeout(1_000);
    spoken = await a.link();
  }
  await a.page.screenshot({ path: run.out("medium-cut-A.png") });
  record("a medium cut is spoken as a short shortage", spoken.codes.includes("reconnecting"), { afterMs: Date.now() - mediumAt, ...spoken });
  await a.page.waitForTimeout(Math.max(0, HUB_COLLABORATION_BOUNDS.mediumCutMs - (Date.now() - mediumAt)));
  relays[0].heal();
  const cleared = await a.waitFor((view) => LIVE_SYNC.test(view.sync) && view, HUB_COLLABORATION_BOUNDS.relinkMs);
  const line = await a.link();
  record("after the medium cut A relinks and the line clears", cleared.value !== null && line.codes.length === 0, { ...line, relinkAfterMs: cleared.afterMs });
  relays[1].sever();
  const longAt = Date.now();
  let expired = await b.link();
  while (!expired.codes.includes("link-expired") && Date.now() - longAt < HUB_COLLABORATION_BOUNDS.expiryMs) {
    await b.page.waitForTimeout(2_000);
    expired = await b.link();
  }
  await b.page.screenshot({ path: run.out("long-cut-B.png") });
  run.measured.linkExpiredAfterMs = expired.codes.includes("link-expired") ? Date.now() - longAt : -1;
  record("a long cut expires B's link and says so", expired.codes.includes("link-expired"), { afterMs: Date.now() - longAt, ...expired });
  const tongue = options.locale === "de" ? GERMAN_LINK_LINE : ENGLISH_LINK_LINE;
  record(`B's expiry line speaks the session's tongue (${options.locale})`, expired.texts.some((text) => tongue.test(text)) && (options.locale === "en" || !expired.texts.some((text) => /connection/iu.test(text))), { texts: expired.texts });
  relays[1].heal();
  await b.page.waitForTimeout(15_000);
  const after = await b.link();
  record("an expired link never relinks by itself", !LIVE_SYNC.test(after.sync) && (after.codes.includes("link-expired") || /detached|getrennt/iu.test(after.sync)), after);
  const late = await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId);
  run.onCleanup(async () => {
    writeFileSync(run.out("console-late.txt"), late.lines.join("\n"));
    writeFileSync(run.out("hub-late.txt"), late.hubLines.join("\n"));
  });
  await signedIn(run, late);
  await attached(run, late);
  await kind.prepare(late);
  const converged = kind.count(await a.view());
  const lateHead = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  const joined = await late.waitFor((view) => kind.count(view) === converged && view, 60_000);
  record("a late joiner attaching afterwards shows every committed edit", joined.value !== null, { lateCount: kind.count(joined.view), converged, hubHead: lateHead });
  await late.context.close();
}

/** ⚛️ A wasm32 shell and a React shell on one block2d document: open, presence, edits and undo both ways, late joiners of both
 * shell types. */
async function wasm32ReactJourney(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const [serve, reactServe] = [requireServe(options.wgpuServe, "--serve"), requireServe(options.reactServe, "--react-serve")];
  const kind = wasm32Kind(HUB_COLLABORATION_KINDS["wasm32-react"].variant);
  const documentId = (): string => run.document.documentId;
  const a = await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId);
  const b = await ReactShell.boot(browser, options.humans[1], reactServe, options.locale, run.clock);
  run.onCleanup(async () => {
    writeFileSync(run.out("console-A-wasm32.txt"), a.lines.join("\n"));
    writeFileSync(run.out("console-B-react.txt"), b.lines.join("\n"));
  });
  await signedIn(run, a);
  record("B signs in through the React hub workspace", await b.signIn(), {});
  await attached(run, a);
  await kind.prepare(a);
  const painted = await a.view();
  const counts = painted.nodes.find((node) => String(node.key).endsWith("block2d-play-board.counts"));
  record("the board's painted counts are named for assistive technology", HANDLE_KINDS.test(String(counts?.label ?? "")) && String(counts?.label) === painted.texts.find((text) => HANDLE_KINDS.test(text)), { accessibleName: counts?.label ?? null, painted: painted.texts });
  const opened = await b.open(run.document.spaceId, run.document.documentId, (reading) => Number.isFinite(reading.handleKinds));
  record("B opens the document in React and is live", opened.liveAfterMs !== null, { liveAfterMs: opened.liveAfterMs, sync: opened.reading.syncPill, handleKinds: opened.reading.handleKinds });
  const aSees = await a.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs);
  const bSees = await b.until(HUB_COLLABORATION_BOUNDS.presenceMs, (reading) => reading.peers.length >= 2);
  record("presence both ways across shell types", aSees.value !== null && bSees.afterMs !== null, { wasm32: aSees.view.peers, react: bSees.reading.peers.map((row) => row.label) });
  const wasmCount = async (): Promise<number> => kind.count(await a.view());
  const reactBefore = (await b.read()).handleKinds;
  const aEdit = await kind.edit(a);
  const reactSees = await b.until(HUB_COLLABORATION_BOUNDS.editSeenMs, (reading) => reading.handleKinds > reactBefore);
  run.measured.editWasm32ToReactMs = reactSees.afterMs ?? -1;
  record("the wasm32 edit reaches React without a reload", aEdit === "activated" && reactSees.afterMs !== null, { aEdit, handleKinds: [reactBefore, reactSees.reading.handleKinds], afterMs: reactSees.afterMs });
  const wasmBefore = await wasmCount();
  const bEdit = await b.runAction("addHandleKind");
  const wasmSees = await a.waitFor((view) => kind.count(view) > wasmBefore && view, HUB_COLLABORATION_BOUNDS.editSeenMs);
  run.measured.editReactToWasm32Ms = wasmSees.value === null ? -1 : wasmSees.afterMs;
  record("the React edit reaches wasm32 without a reload", bEdit === "ok" && wasmSees.value !== null, { bEdit, counts: [wasmBefore, kind.count(wasmSees.view)], afterMs: wasmSees.afterMs });
  const beforeAUndo = await wasmCount();
  const aUndo = await kind.undo(a);
  const aUndone = await a.waitFor((view) => kind.count(view) < beforeAUndo && view, 30_000);
  const reactSeesUndo = await b.until(30_000, (reading) => reading.handleKinds === kind.count(aUndone.view));
  record("wasm32 undoes its own edit and React follows", aUndone.value !== null && reactSeesUndo.afterMs !== null, { aUndo, wasm32: [beforeAUndo, kind.count(aUndone.view)], react: reactSeesUndo.reading.handleKinds });
  const beforeBUndo = (await b.read()).handleKinds;
  const bUndo = await b.runAction("undo");
  const bUndone = await b.until(30_000, (reading) => reading.handleKinds < beforeBUndo);
  const wasmSeesUndo = await a.waitFor((view) => kind.count(view) === bUndone.reading.handleKinds && view, 30_000);
  record("React undoes its own edit and wasm32 follows", bUndone.afterMs !== null && wasmSeesUndo.value !== null, { bUndo, react: [beforeBUndo, bUndone.reading.handleKinds], wasm32: kind.count(wasmSeesUndo.view) });
  const converged = (await b.read()).handleKinds;
  const lateReact = await ReactShell.boot(browser, options.humans[0], reactServe, options.locale, run.clock);
  await lateReact.signIn();
  const lateOpened = await lateReact.open(run.document.spaceId, run.document.documentId, (reading) => reading.handleKinds === converged);
  record("a late React joiner converges", lateOpened.liveAfterMs !== null, { converged, late: lateOpened.reading.handleKinds });
  await lateReact.context.close();
  const lateWasm = await Wasm32Shell.boot(browser, options.humans[1], serve, options.hub, options.locale, run.clock, documentId);
  await signedIn(run, lateWasm);
  await attached(run, lateWasm);
  await kind.prepare(lateWasm);
  const lateSeen = await lateWasm.waitFor((view) => kind.count(view) === converged && view, 60_000);
  record("a late wasm32 joiner converges", lateSeen.value !== null, { converged, late: kind.count(lateSeen.view) });
  await lateWasm.context.close();
}

/** 🦀️ The native wgpu user (the live law) with a wasm32 or React browser user following its handshake. */
async function nativeJourney(run: JourneyRun, browserShell: "wasm32" | "react", cursors = false): Promise<void> {
  const { options, browser, record } = run;
  const handshake = new Handshake(join(options.outDir, options.tag, "handshake"), record);
  const law = cursors ? "a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board" : "a_native_and_a_react_user_collaborate_on_one_hub_document";
  const native = spawnNativeLaw(options, law, handshake.dir, join(options.outDir, options.tag));
  let alive = true;
  const nativeExit = native.exit.then((code) => {
    alive = false;
    return code;
  });
  run.onCleanup(async () => {
    if (alive) native.child.kill("SIGTERM");
  });
  if (browserShell === "react") {
    const react = await ReactShell.boot(browser, options.humans[1], requireServe(options.reactServe, "--react-serve"), options.locale, run.clock);
    run.onCleanup(async () => writeFileSync(run.out("console-B-react.txt"), react.lines.join("\n")));
    record("B signs in through the React hub workspace", await react.signIn(), {});
    if (cursors) await followNativeCursors(handshake, react, () => alive, record);
    else
      await followNativeEdits(
        handshake,
        {
          open: async (spaceId, documentId) => {
            const opened = await react.open(spaceId, documentId, (reading) => Number.isFinite(reading.handleKinds));
            return { liveAfterMs: opened.liveAfterMs, detail: { sync: opened.reading.syncPill } };
          },
          seesPeer: async (actor, userId) => {
            const seen = await react.until(HUB_COLLABORATION_BOUNDS.presenceMs, (reading) => reading.peers.some((row) => row.id.includes(actor) || (userId !== "" && row.id.includes(userId))));
            return { afterMs: seen.afterMs, peers: seen.reading.peers };
          },
          count: async () => (await react.read()).handleKinds,
          edit: () => react.runAction("addHandleKind"),
          undo: () => react.runAction("undo"),
          reload: async (spaceId, documentId) => {
            await react.page.reload({ waitUntil: "domcontentloaded", timeout: HUB_COLLABORATION_BOUNDS.bootMs });
            const restored = await react.until(120_000, (reading) => Number.isFinite(reading.handleKinds) || (reading.ready === "s" && (reading.home || reading.spaceIndex)));
            if (Number.isFinite(restored.reading.handleKinds)) return { liveAfterMs: restored.afterMs };
            return { liveAfterMs: (await react.open(spaceId, documentId, (reading) => Number.isFinite(reading.handleKinds))).liveAfterMs };
          },
        },
        () => alive,
        record,
      );
  } else {
    const kind = wasm32Kind("block2d");
    let shell = await Wasm32Shell.boot(browser, options.humans[1], requireServe(options.wgpuServe, "--serve"), options.hub, options.locale, run.clock, () => run.document.documentId);
    run.onCleanup(async () => writeFileSync(run.out("console-B-wasm32.txt"), shell.lines.join("\n")));
    await signedIn(run, shell);
    const attach = async (spaceId: string, documentId: string): Promise<number | null> => {
      run.document.spaceId = spaceId;
      run.document.documentId = documentId;
      const started = Date.now();
      const live = await attached(run, shell);
      await kind.prepare(shell);
      return live ? Date.now() - started : null;
    };
    await followNativeEdits(
      handshake,
      {
        open: async (spaceId, documentId) => ({ liveAfterMs: await attach(spaceId, documentId), detail: {} }),
        seesPeer: async () => {
          const seen = await shell.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs);
          return { afterMs: seen.value === null ? null : seen.afterMs, peers: seen.view.peers };
        },
        count: async () => kind.count(await shell.view()),
        edit: () => kind.edit(shell),
        undo: () => kind.undo(shell),
        reload: async (spaceId, documentId) => {
          await shell.context.close();
          shell = await Wasm32Shell.boot(browser, options.humans[1], requireServe(options.wgpuServe, "--serve"), options.hub, options.locale, run.clock, () => run.document.documentId);
          await signedIn(run, shell);
          return { liveAfterMs: await attach(spaceId, documentId) };
        },
      },
      () => alive,
      record,
    );
  }
  const code = await Promise.race([nativeExit, new Promise<number>((resolveLate) => setTimeout(() => resolveLate(-1), 300_000))]);
  run.measured.nativeExit = code;
  record("the native law's own ledger passes (exit 0)", code === 0, { exit: code, log: join(options.outDir, options.tag, "native.txt") });
}

/** 🖱️ Two wasm32 shells on one puzzle2d board: each pointer over its own board is painted as a peer cursor on the other's. */
async function cursorsJourney(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const serve = requireServe(options.wgpuServe, "--serve");
  const documentId = (): string => run.document.documentId;
  const [a, b] = [await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId), await Wasm32Shell.boot(browser, options.humans[1], serve, options.hub, options.locale, run.clock, documentId)];
  for (const shell of [a, b]) await signedIn(run, shell);
  for (const shell of [a, b]) await attached(run, shell);
  const presence = await Promise.all([a, b].map((shell) => shell.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs)));
  record("presence both ways", presence.every((seen) => seen.value !== null), { a: presence[0]!.view.peers, b: presence[1]!.view.peers });
  const boardClip = async (shell: Wasm32Shell): Promise<{ x: number; y: number; width: number; height: number } | null> => {
    const box = await shell.page.evaluate(() => {
      const rect = document.querySelector("canvas")?.getBoundingClientRect();
      return rect ? { x: rect.x, y: rect.y, width: rect.width, height: rect.height } : null;
    });
    return box ? { x: Math.round(box.x + box.width * 0.3), y: Math.round(box.y + box.height * 0.25), width: Math.round(box.width * 0.4), height: Math.round(box.height * 0.5) } : null;
  };
  for (const [mover, watcher] of [[a, b], [b, a]] as const) {
    const label = `${mover.human.label} moves, ${watcher.human.label} sees`;
    const [moverClip, watcherClip] = [await boardClip(mover), await boardClip(watcher)];
    if (!moverClip || !watcherClip) {
      record(`${label}: peer cursor painted on the watcher's board`, false, { reason: "no canvas" });
      continue;
    }
    await mover.page.mouse.move(moverClip.x + moverClip.width * 0.2, moverClip.y + moverClip.height * 0.2);
    await watcher.page.waitForTimeout(4_000);
    const control = [await regionPixels(watcher.page, watcherClip)];
    await watcher.page.waitForTimeout(2_500);
    control.push(await regionPixels(watcher.page, watcherClip));
    const noise = changedPixelCount(control[0]!, control[1]!);
    const deltas: number[] = [];
    for (const [fx, fy] of [[0.8, 0.8], [0.5, 0.3], [0.2, 0.7]] as const) {
      await mover.page.mouse.move(moverClip.x + moverClip.width * fx, moverClip.y + moverClip.height * fy, { steps: 8 });
      await watcher.page.waitForTimeout(HUB_COLLABORATION_BOUNDS.cursorSettleMs);
      deltas.push(changedPixelCount(control[1]!, await regionPixels(watcher.page, watcherClip)));
    }
    await watcher.page.screenshot({ path: run.out(`cursor-${mover.human.label}-${watcher.human.label}.png`), clip: watcherClip });
    record(`${label}: peer cursor painted on the watcher's board`, deltas.every((delta) => delta > Math.max(40, noise * 3)), { noise, deltas });
  }
}

/** 🤖️ A human in the wasm32 shell and a delegated agent over the stdio semio MCP on one hub note: the roster names the agent AS an
 * agent, its block is decoded into the Artifact panel without a reload, and the live frame equals a cold reference render. */
async function agentPixelsJourney(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const serve = requireServe(options.wgpuServe, "--serve");
  const kind = wasm32Kind("note");
  const documentId = (): string => run.document.documentId;
  const human = await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId);
  run.onCleanup(async () => writeFileSync(run.out("console-human.txt"), human.lines.join("\n")));
  await signedIn(run, human);
  await attached(run, human);
  await kind.prepare(human);
  if (!(await human.closeSyncCard())) throw new Error("the human's Sync card does not close — the frames would differ in chrome");
  const parked = async (shell: Wasm32Shell): Promise<void> => {
    await shell.page.mouse.move(2, 996);
    await shell.page.waitForTimeout(1_500);
  };
  await parked(human);
  const body = { x: 0, y: 36, width: 1600, height: 1000 - 36 - 36 };
  const before = await regionPixels(human.page, body, run.out("agent-before.png"));
  await human.page.waitForTimeout(2_500);
  const noise = changedPixelCount(before, await regionPixels(human.page, body));
  const countBefore = kind.count(await human.view());
  const headBefore = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  const agentWord = options.locale === "de" ? "KI-Agent" : "AI agent";
  const agentLabel = `wgpu acceptance agent ${options.tag}`;
  const delegation = await hubProbeCall(options.hub, "POST", "/auth/agent-delegations", run.token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: run.document.spaceId, agentLabel, audience: "edit", ttlSecs: 1800 }));
  const delegationId = String(delegation.json?.delegationId ?? "");
  if (!delegationId) throw new Error(`agent delegation answered ${delegation.status} ${delegation.text.slice(0, 200)}`);
  run.onCleanup(async () => {
    await hubProbeCall(options.hub, "DELETE", `/auth/agent-delegations/${encodeURIComponent(delegationId)}`, run.token).catch(() => undefined);
  });
  const privateDir = join(options.outDir, options.tag, "agent");
  mkdirSync(privateDir, { recursive: true, mode: 0o700 });
  const credentialPath = join(privateDir, "credential.json");
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: options.hub, spaceId: run.document.spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  run.onCleanup(async () => rmSync(credentialPath, { force: true }));
  const agent = spawnRawMcp(requireMcpBinary(options.repoRoot), ["stdio", "--hub", options.hub, "--space", run.document.spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"]);
  run.onCleanup(async () => {
    await agent.close().catch(() => undefined);
  });
  await agent.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wgpu-hub-collaboration", version: "1" } }, 600_000);
  agent.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  const call = async (name: string, args: Record<string, unknown>): Promise<{ isError?: boolean; structuredContent?: any }> => ((await agent.request("tools/call", { name, arguments: args }, 600_000)).result ?? {}) as { isError?: boolean; structuredContent?: any };
  const context = await call("context_resolve", {});
  const opened = await call("artifact_open", { artifactId: run.document.documentId });
  record("the agent is its own delegated principal and opens the document", opened.isError !== true && String(context.structuredContent?.principal ?? "") === `agent:${delegationId}`, { principal: context.structuredContent?.principal });
  const roster = await human.waitFor((view) => view.peers.some((label) => label.includes(agentWord) && label.includes(agentLabel)) && view.peers, 30_000);
  record(`the human's wgpu roster names the agent AS an agent (${options.locale})`, roster.value !== null, { peers: roster.view.peers });
  const search = await call("capabilities_search", { query: "add a block", kind: ["mutation"] });
  const capabilityId = ((search.structuredContent?.results ?? []) as { capabilityId?: string; id?: string }[]).map((hit) => String(hit.capabilityId ?? hit.id)).find((id) => id.endsWith(".addBlock")) ?? "";
  const prepared = await call("action_prepare", { capabilityId, input: { kind: "text" } });
  const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
  const committedAt = Date.now();
  record("the agent commits an edit through the semio MCP", invoked.structuredContent?.status === "SUCCEEDED", { capabilityId, status: invoked.structuredContent?.status });
  const decoded = await human.waitFor((view) => kind.count(view) > countBefore && view, HUB_COLLABORATION_BOUNDS.agentSeenMs);
  run.measured.agentBlockSeenMs = decoded.value === null ? -1 : Date.now() - committedAt;
  record("the human's shell decodes the agent's block without a reload", decoded.value !== null, { count: [countBefore, kind.count(decoded.view)], afterMs: Date.now() - committedAt });
  const headAfter = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  record("the hub ledger advanced with the agent's commit", headAfter > headBefore, { head: [headBefore, headAfter] });
  if (!(await human.closeSyncCard())) throw new Error("the human's Sync card reopened — the frames would differ in chrome");
  await parked(human);
  let live = await regionPixels(human.page, body, run.out("agent-live.png"));
  for (let settle = 0; settle < 5; settle += 1) {
    await human.page.waitForTimeout(1_500);
    const next = await regionPixels(human.page, body);
    const moving = changedPixelCount(live, next) > noise;
    live = next;
    if (!moving) break;
  }
  await agent.close().catch(() => undefined);
  await human.context.close();
  const reference = await Wasm32Shell.boot(browser, { ...options.humans[0], label: `${options.humans[0].label} (cold reference)` }, serve, options.hub, options.locale, run.clock, documentId);
  run.onCleanup(async () => writeFileSync(run.out("console-reference.txt"), reference.lines.join("\n")));
  await signedIn(run, reference);
  await attached(run, reference);
  await kind.prepare(reference);
  if (!(await reference.closeSyncCard())) throw new Error("the reference's Sync card does not close — the frames would differ in chrome");
  const referenceDecoded = await reference.waitFor((view) => kind.count(view) === kind.count(decoded.view) && view, 60_000);
  record("a fresh session decodes the same committed state (cold reference)", referenceDecoded.value !== null, { count: [kind.count(decoded.view), kind.count(referenceDecoded.view)] });
  await parked(reference);
  const cold = await regionPixels(reference.page, body, run.out("agent-reference.png"));
  await reference.context.close();
  const differsFromBefore = changedPixelCount(before, live);
  const differsFromReference = changedPixelCount(live, cold);
  const edited = Math.max(4 * noise, HUB_COLLABORATION_BOUNDS.agentEditMinPixels);
  const same = Math.max(2 * noise, HUB_COLLABORATION_BOUNDS.agentSameMaxPixels);
  Object.assign(run.measured, { agentFrameNoise: noise, agentFrameChangedFromBefore: differsFromBefore, agentFrameDiffFromReference: differsFromReference, agentFrameEditedMin: edited, agentFrameSameMax: same });
  record("the agent's edit changes the rendered frame", differsFromBefore >= edited, { differsFromBefore, edited, noise });
  record("the live frame equals a cold reference render of the same committed state", referenceDecoded.value !== null && differsFromReference <= same, { differsFromReference, same, noise, evidence: [run.out("agent-before.png"), run.out("agent-live.png"), run.out("agent-reference.png")] });
}
//#endregion 🔖️Journeys

//#region 🔖️Run
const JOURNEY_BODIES: Readonly<Record<HubCollaborationJourneyV1, (run: JourneyRun) => Promise<void>>> = {
  wasm32: wasm32Journey,
  "wasm32-react": wasm32ReactJourney,
  "wasm32-native": (run) => nativeJourney(run, "wasm32"),
  "native-react": (run) => nativeJourney(run, "react"),
  "native-react-cursors": (run) => nativeJourney(run, "react", true),
  cursors: cursorsJourney,
  "agent-pixels": agentPixelsJourney,
};

/** 🔌️ A free loopback port for a serve this run starts itself. */
async function freeLoopbackPort(): Promise<number> {
  const probe = createServer();
  await new Promise<void>((ready, fail) => probe.once("error", fail).listen(0, "127.0.0.1", ready));
  const { port } = probe.address() as AddressInfo;
  await new Promise<void>((closed) => probe.close(() => closed()));
  return port;
}

/** 🛎️ The serve a journey drives, through S18's shared `ensureDevServe`: the one `--serve` / `--react-serve` names (reused when it
 * answers, started on that port otherwise), or — flag omitted — one started on a free loopback port (the wgpu release serve of
 * the journey's variant, the React `s` dev serve); only what this run started is stopped at its end. A serve that cannot be had
 * is a missing precondition (`blocked`). */
async function provisionServe(options: HubCollaborationOptionsV1, url: string | null, shell: Readonly<{ variant: string; renderer: "wgpu" | "react"; profile: "release" | "dev" }>, cleanups: (() => Promise<void>)[]): Promise<string> {
  const port = url ? devServePortV1(url) : await freeLoopbackPort();
  const fixture = await ensureDevServe({ repoRoot: options.repoRoot, port, variant: shell.variant, renderer: shell.renderer, profile: shell.profile, hubUrl: options.hub, locale: options.locale, signal: options.signal, onProgress: (_status, line) => console.log(line) }).catch((error: unknown) => {
    throw new HubCollaborationBlocked(`${shell.renderer} serve on port ${port}: ${error instanceof Error ? error.message : String(error)}`);
  });
  if (!fixture.reused) cleanups.push(() => fixture.stop());
  return url ?? (shell.renderer === "wgpu" ? `${fixture.url}?plugin=${shell.variant}` : fixture.url);
}

/** 🌐️ Refuses a hub or serve that does not answer: a missing precondition, never a failed journey. */
async function requireAnswering(url: string, what: string): Promise<void> {
  const answers = await fetch(url, { signal: AbortSignal.timeout(10_000) }).then((response) => response.status < 500, () => false);
  if (!answers) throw new HubCollaborationBlocked(`${what} ${url} does not answer`);
}

/** 🚦️ Runs one journey and writes `<out>/<tag>/report.json` (+ console, hub lines, screenshots, native log). */
export async function runHubCollaboration(options: HubCollaborationOptionsV1): Promise<HubCollaborationReportV1> {
  const directory = join(options.outDir, options.tag);
  rmSync(directory, { recursive: true, force: true });
  mkdirSync(directory, { recursive: true });
  const started = Date.now();
  const clock = (): number => Date.now() - started;
  const report: HubCollaborationReportV1 = { schema: "semio.wgpu.hub-collaboration-report/v1", journey: options.journey, locale: options.locale, hub: options.hub, spaceId: null, documentId: null, startedAt: new Date(started).toISOString(), finishedAt: null, steps: [], measured: {}, fatal: null };
  const flush = (): void => writeFileSync(join(directory, "report.json"), JSON.stringify(report, null, 2));
  const record = (name: string, pass: boolean, detail: unknown): void => {
    report.steps.push({ name, pass, atMs: clock(), detail });
    console.log(`[hub-collaboration] ${pass ? "PASS" : "FAIL"} ${name} +${(clock() / 1000).toFixed(1)}s ${JSON.stringify(detail).slice(0, 400)}`);
    flush();
  };
  await requireAnswering(`${options.hub}/readyz`, "hub");
  const kind = HUB_COLLABORATION_KINDS[options.journey];
  const cleanups: (() => Promise<void>)[] = [];
  const document = { spaceId: "", documentId: "" };
  try {
    const serving: HubCollaborationOptionsV1 = {
      ...options,
      wgpuServe: kind.wgpu ? await provisionServe(options, options.wgpuServe, { variant: kind.variant, renderer: "wgpu", profile: "release" }, cleanups) : null,
      reactServe: kind.react ? await provisionServe(options, options.reactServe, { variant: "s", renderer: "react", profile: "dev" }, cleanups) : null,
    };
    const token = await hubProbeSignIn(options.hub, options.humans[0].email, options.humans[0].password, "wgpucollab");
    if (!kind.native) {
      const prepared = await prepareDocument(options, kind.schema, token);
      Object.assign(document, { spaceId: prepared.spaceId, documentId: prepared.documentId });
      report.measured.documentCreatedMs = prepared.createdMs;
    }
    ensureParityPlaywrightBrowsersPath();
    const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
    cleanups.push(() => browser.close());
    await JOURNEY_BODIES[options.journey]({ options: serving, browser, record, measured: report.measured, clock, out: (name) => join(directory, name), document, onCleanup: (cleanup) => cleanups.push(cleanup), token });
  } catch (error) {
    if (error instanceof HubCollaborationBlocked) throw error;
    report.fatal = String(error instanceof Error ? (error.stack ?? error.message) : error).slice(0, 1_500);
    console.log(`[hub-collaboration] FATAL ${report.fatal}`);
  } finally {
    for (const cleanup of cleanups.reverse()) await cleanup().catch(() => undefined);
    report.spaceId = document.spaceId || null;
    report.documentId = document.documentId || null;
    report.finishedAt = new Date().toISOString();
    flush();
  }
  return report;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `hub-collaboration-acceptance --journey <wasm32|wasm32-react|wasm32-native|native-react|native-react-cursors|cursors|agent-pixels>
 * --hub <url> [--serve <wgpu serve url>] [--react-serve <url>] [--locale en|de] [--space <id> --document <id>] (an omitted serve is
 * started by the run itself on a free loopback port and stopped at its end)
 * [--native-binary <path>] [--native-modules <dir>] [--tag <t>] [--out <dir>]` — runs one journey, publishes its acceptance
 * record (`<check>-<locale>`), exits non-zero unless every step passes. */
export async function runHubCollaborationCli(repoRoot: string, defaultOutDir: string, segments: readonly string[], defaults: Readonly<{ nativeModules?: () => string }> = {}): Promise<void> {
  const journey = flagValue(segments, "--journey") as HubCollaborationJourneyV1 | undefined;
  if (!journey || !(journey in HUB_COLLABORATION_CHECKS)) throw new Error(`usage: hub-collaboration-acceptance --journey <${Object.keys(HUB_COLLABORATION_CHECKS).join("|")}> --hub <url> [--serve <url>] [--react-serve <url>] [--locale en|de]`);
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const check = `${HUB_COLLABORATION_CHECKS[journey]}-${locale}`;
  const tag = flagValue(segments, "--tag") ?? check;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  await withAcceptanceRecord(
    repoRoot,
    check,
    async () => {
      const hub = flagValue(segments, "--hub");
      if (!hub) throw new HubCollaborationBlocked("--hub is required");
      const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
      const native = HUB_COLLABORATION_KINDS[journey].native;
      const report = await runHubCollaboration({
        repoRoot,
        journey,
        hub: hub.replace(/\/$/u, ""),
        wgpuServe: flagValue(segments, "--serve") ?? null,
        reactServe: flagValue(segments, "--react-serve") ?? null,
        locale,
        humans: hubCollaborationHumans(),
        spaceId: flagValue(segments, "--space") ?? null,
        documentId: flagValue(segments, "--document") ?? null,
        nativeBinary: flagValue(segments, "--native-binary") ?? null,
        nativeModules: flagValue(segments, "--native-modules") ?? (native ? (defaults.nativeModules?.() ?? null) : null),
        outDir,
        tag,
        signal: controller.signal,
      });
      const passed = report.steps.filter((step) => step.pass).length;
      const failing = report.steps.filter((step) => !step.pass).map((step) => step.name);
      const status = !report.fatal && report.steps.length > 0 && failing.length === 0 ? "pass" : "fail";
      publishAcceptanceCheckResult(
        repoRoot,
        acceptanceCheckResult({
          check,
          status,
          startedAt,
          measured: { ...report.measured, locale, steps: report.steps.length, passed, failed: failing.length, fatal: Boolean(report.fatal) },
          summary: {
            en: `${passed}/${report.steps.length} steps of the wgpu ${journey} collaboration journey pass in ${locale}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; stopped: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
            de: `${passed}/${report.steps.length} Schritte des wgpu-Zusammenarbeitswegs ${journey} bestehen in ${locale}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; abgebrochen: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
          },
          evidence: [join(outDir, tag, "report.json")],
        }),
      );
      if (status !== "pass") process.exitCode = 1;
    },
    (error) => error instanceof HubCollaborationBlocked,
  );
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
//#endregion 🔖️Run
