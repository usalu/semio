/** 🔑️ The browser sessions of the two-human harness: one human per isolated profile, boot and sign-in, dialogs, windowed table rows and the status line. */
import type { Browser, BrowserContext, Page } from "playwright";
import { FAULT, NOISE } from "../../🧮️program-matrix/🟦️.ts";

//#region 🔖️Sessions
/** 🔑️ One human: a hub credential. */
export type Human = Readonly<{ label: string; email: string; password: string }>;

/** 🌐️ One human's isolated browser profile on one serve, with its console and hub traffic lines. */
export type Session = { human: Human; url: string; context: BrowserContext; page: Page; lines: string[] };

export const started = Date.now();
const ms = (): number => Date.now() - started;
export const pause = (session: Session, duration: number): Promise<void> => session.page.waitForTimeout(duration);
export const originOf = (session: Session): string => new URL(session.url).origin;
export const faultsSince = (session: Session, cursor: number): string[] => session.lines.slice(cursor).filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => line.slice(0, 240));
export const short = (text: string): string => (text.length <= 160 ? text : `${text.slice(0, 80)}…${text.slice(-70)}`);

export async function openSessions(browser: Browser, urls: readonly string[], humans: readonly Human[], locale: string): Promise<Session[]> {
  const sessions: Session[] = [];
  for (const [index, human] of humans.entries()) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale });
    const page = await context.newPage();
    const lines: string[] = [];
    page.on("console", (message) => {
      const text = message.text();
      if (/Download the React DevTools|\[vite\]|agent-bridge|status of 404/u.test(text)) return;
      lines.push(`${ms()} ${message.type()} ${text.slice(0, 900)}`);
    });
    page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 900)}`));
    page.on("websocket", (socket) => socket.on("close", () => lines.push(`${ms()} ws-closed ${socket.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 160)}`)));
    page.on("request", (request) => {
      if (request.url().includes("/directory/event-page/v1?after=0")) lines.push(`${ms()} request /directory/event-page/v1?after=0`);
    });
    sessions.push({ human, url: urls[index] ?? urls[0]!, context, page, lines });
  }
  return sessions;
}

export async function until<T>(probe: () => Promise<T | null>, deadlineMs: number, stepMs = 500): Promise<T | null> {
  const deadline = Date.now() + deadlineMs;
  let value = await probe();
  while (!value && Date.now() < deadline) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, stepMs));
    value = await probe();
  }
  return value;
}

export async function boot(session: Session): Promise<void> {
  await session.page.goto(session.url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await session.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
}

export async function signIn(session: Session): Promise<void> {
  const page = session.page;
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(session.human.email);
  await form.locator('input[type="password"]').fill(session.human.password);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
}

/** ⌨️ Keyboard activation of a UI node by its stable `data-ui-node-key`. */
export async function activate(page: Page, key: string): Promise<void> {
  const node = page.locator(`[data-ui-node-key="${key}"]`).first();
  await node.waitFor({ state: "attached", timeout: 30_000 });
  await node.focus();
  await node.press("Enter");
}

export const dialog = (page: Page) => page.locator('[role="dialog"][data-slot="dialog-content"]');

export async function selectOption(page: Page, triggerId: string, option: RegExp): Promise<void> {
  await page.locator(`[id="${triggerId}"]`).click();
  await page.getByRole("option", { name: option }).first().click();
  await page.waitForTimeout(200);
}

export async function submitDialog(page: Page): Promise<void> {
  await page.locator('[id="ui.dialog.submit"]').click();
  await dialog(page).waitFor({ state: "hidden", timeout: 20_000 });
}

export async function rowIds(page: Page, prefix: string): Promise<Set<string>> {
  return new Set(await page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements) => elements.map((element) => element.getAttribute("data-ui-node-key") ?? "")));
}

export async function waitNewRow(page: Page, prefix: string, before: ReadonlySet<string>, deadlineMs: number): Promise<string> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    for (const id of await rowIds(page, prefix)) if (!before.has(id)) return id.slice(prefix.length + 1);
    await page.waitForTimeout(400);
  }
  throw new Error(`no new ${prefix} row within ${deadlineMs} ms`);
}

/** 🧭️ Pages every windowed table (`table-window-scroll`) top to bottom, as a human scrolls, until `found` answers. */
export async function pageWindowedTables<T>(page: Page, found: () => Promise<T | null>): Promise<T | null> {
  let hit = await found();
  const scrollers = page.locator('[data-slot="table-window-scroll"]');
  for (let index = 0, count = await scrollers.count(); index < count && hit === null; index += 1) {
    for (let top = 0; ; ) {
      const metrics = await scrollers
        .nth(index)
        .evaluate((element, y) => {
          element.scrollTop = y;
          return { top: element.scrollTop, scrollHeight: element.scrollHeight, clientHeight: element.clientHeight };
        }, top)
        .catch(() => null);
      await page.waitForTimeout(400);
      hit = await found();
      if (metrics === null || hit !== null || metrics.top + metrics.clientHeight >= metrics.scrollHeight) break;
      top = metrics.top + Math.max(1, Math.floor(metrics.clientHeight * 0.8));
    }
  }
  return hit;
}

export async function waitRow(page: Page, prefix: string, id: string, deadlineMs: number): Promise<void> {
  const row = page.locator(`[data-ui-node-key="${prefix}:${id}"]`).first();
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    if ((await pageWindowedTables(page, async () => ((await row.count()) > 0 ? true : null))) !== null) return;
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for [data-ui-node-key="${prefix}:${id}"] (${deadlineMs} ms, windowed tables paged)`);
}

export async function waitNamedRow(page: Page, prefix: string, name: string, deadlineMs: number): Promise<string> {
  const find = (): Promise<string | null> => page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements, wanted) => elements.find((element) => (element.textContent ?? "").includes(wanted))?.getAttribute("data-ui-node-key") ?? null, name);
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const key = await pageWindowedTables(page, find);
    if (key !== null) return key.slice(prefix.length + 1);
    await page.waitForTimeout(500);
  }
  throw new Error(`no ${prefix} row named ${JSON.stringify(name)} within ${deadlineMs} ms (windowed tables paged)`);
}

export async function clickRowAction(page: Page, prefix: string, id: string, pattern: RegExp): Promise<string> {
  const buttons = page.locator(`[data-ui-node-key="${prefix}:${id}"] button`);
  for (let index = 0, count = await buttons.count(); index < count; index += 1) {
    const button = buttons.nth(index);
    const name = `${(await button.getAttribute("aria-label")) ?? ""} ${(await button.getAttribute("title")) ?? ""} ${(await button.textContent()) ?? ""}`;
    if (pattern.test(name)) {
      await button.focus();
      await button.press("Enter");
      return name.trim();
    }
  }
  throw new Error(`row ${prefix}:${id} has no action matching ${pattern}`);
}

/** 🪞️ What the shell publishes about presence and the execution target of the open document. */
export const readStatus = (page: Page): Promise<{ peers: string[]; peerLabels: string[]; executionTarget: string[]; windows: string[]; error: string | null }> =>
  page.evaluate(() => {
    const text = (element: Element | null): string => ((element as HTMLElement | null)?.innerText ?? "").replace(/\s+/gu, " ").trim();
    const peerNodes = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      peers: peerNodes.map((element) => element.getAttribute("data-row-id") ?? ""),
      peerLabels: peerNodes.map((element) => text(element).slice(0, 64)),
      executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((element) => `${element.getAttribute("data-semio-execution-target-status")}:${text(element).slice(0, 80)}`),
      windows: [...document.querySelectorAll('[data-slot="window"]')].map((element) => element.id).slice(0, 12),
      error: document.documentElement.getAttribute("data-semio-os-error"),
    };
  });
//#endregion 🔖️Sessions
