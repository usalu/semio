import type { Browser, BrowserContext, Page } from "playwright";
import { HUB_COLLABORATION_BOUNDS, type HubCollaborationHuman } from "../📋️contract/🟦️.ts";

//#region 🔖️Wasm32Shell
/** 🔍️ One node of the wasm32 shell's accessibility projection. */
export type MirrorNode = Readonly<{ key: string; label?: string; role?: string; disabled?: boolean; checked?: boolean; valueText?: string; windowId: string; rect?: readonly number[] }>;

/** 👁️ One reading of a wasm32 shell: its sync label, roster lines, accessibility projection and painted document texts. */
export type Wasm32View = Readonly<{ sync: string; peers: string[]; nodes: MirrorNode[]; texts: string[] }>;

const MIRROR = "#semio-wgpu-accessibility";

export const LIVE_SYNC = /live|connected|persisted|verbunden|gespeichert/iu;

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
