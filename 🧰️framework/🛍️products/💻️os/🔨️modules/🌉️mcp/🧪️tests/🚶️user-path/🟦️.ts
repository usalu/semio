/** 🚶️ The user path for an AI client, zero-touch and timed, from a CLEAN browser profile.
 *
 * A human signs in to the `s` shell, lands in their space holding one note (set up through the hub's own directory command
 * and server-owned creation transaction), turns an agent delegation into an MCP client configuration with the in-product
 * "Set up MCP client" action, and a REAL MCP client — the official `@modelcontextprotocol/sdk` `Client` over
 * `StdioClientTransport`, started from exactly the shown `command`/`args` (the third-party oracle) — connects, lists the
 * tools, edits the note, and asks for a destructive change the human approves in the shell. The human then withdraws the
 * delegation in the same pane: the installed credential is removed, a client holding a copy is refused, and the client
 * that was already connected is refused on its next request. Every row is required.
 *
 * Configuration by environment (the siblings' names): `OS_MCP_HUB_ORIGIN` (default `http://127.0.0.1:8787`),
 * `OS_MCP_HUB_EMAIL` / `OS_MCP_HUB_PASSWORD` (default the `dev s` local user), `S_OS_MCP_LIVE_SHELL_URL` (default
 * `http://127.0.0.1:6080`, a serve joined to that hub), `S_OS_MCP_LIVE_LOCALE` (`en` | `de`), `S_OS_MCP_USER_PATH_OUT`
 * (captures, default `🌉️mcp/🤖️generated/🚶️user-path`). Promoted from the ticket harness `wp-g10/g10-user-path.ts` →
 * `wp-g11/g11-user-path.ts` (ticket 26/09/23, G10/G11 S4).
 */
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import type { Page } from "playwright";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { sealSpaceArtifactCreateV1 } from "../../../📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "../../../📇️directory/🏘️spaces/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findRepoRoot(start: string): string {
  for (let current = start, depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the user-path gate could not locate the repository root above ${start}`);
}
const repoRoot = findRepoRoot(here);
const HUB = (process.env.OS_MCP_HUB_ORIGIN ?? "http://127.0.0.1:8787").replace(/\/$/u, "");
const SHELL = (process.env.S_OS_MCP_LIVE_SHELL_URL ?? "http://127.0.0.1:6080").replace(/\/$/u, "");
const EMAIL = process.env.OS_MCP_HUB_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.OS_MCP_HUB_PASSWORD ?? "gm1-local-dev-pass-1";
const LOCALE = process.env.S_OS_MCP_LIVE_LOCALE === "de" ? "de" : "en";
const OUT = process.env.S_OS_MCP_USER_PATH_OUT ?? join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️generated/🚶️user-path");
const WORDS = { en: { install: "Set up MCP client", once: "Approve Once" }, de: { install: "MCP-Client einrichten", once: "Einmal genehmigen" } }[LOCALE];
mkdirSync(OUT, { recursive: true });
const startedAt = new Date();
const seconds = (): string => ((Date.now() - startedAt.getTime()) / 1000).toFixed(1);
const rows: { step: string; ok: boolean; detail: string; at: string }[] = [];
const row = (step: string, ok: boolean, detail: string): void => {
  rows.push({ step, ok, detail, at: seconds() });
  console.log(`${ok ? "PASS" : "FAIL"}  [${seconds()} s] ${step} — ${detail}`);
};
const pause = (ms: number): Promise<void> => new Promise((resolveDelay) => setTimeout(resolveDelay, ms));

async function hub(method: string, path: string, token?: string, body?: string): Promise<{ status: number; json: any }> {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try {
    json = JSON.parse(text);
  } catch {
    json = null;
  }
  return { status: response.status, json };
}

/** 🧹️ Dismisses every first-run introduction tour over the page, the way a user skips it. */
async function skipTours(page: Page): Promise<void> {
  for (let attempt = 0; attempt < 20; attempt += 1) {
    const skip = page.getByRole("button", { name: /^\s*(skip|überspringen)\s*$/iu }).first();
    if ((await skip.count()) === 0) return;
    await skip.click({ force: true }).catch(() => undefined);
    await pause(400);
  }
}

const setup = await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `userpath${randomBytes(10).toString("hex")}`, clientClass: "browser" }));
const token = String(setup.json?.token ?? "");
const spaceName = `User path ${LOCALE} ${randomBytes(3).toString("hex")}`;
await hub("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
const spaceId = String((await hub("GET", "/directory/spaces", token)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, token);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
const requestId = randomBytes(16).toString("hex");
let creation = (await hub("POST", creations, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: `User path ${LOCALE}` })))).json;
for (const deadline = Date.now() + 1_800_000; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
  await pause(1_000);
  creation = (await hub("GET", `${creations}/${requestId}`, token)).json;
}
const documentId = String(creation?.ready?.artifactId ?? "");
row("0 setup: the user owns a fresh space with one note (hub authorities)", setup.status < 300 && spaceId.length > 0 && documentId.length > 0, `sign-in=${setup.status} space=${spaceId} note=${documentId} kind=${kind?.kindId}`);
const pathStarted = Date.now();

const { chromium }: typeof import("playwright") = await import("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" });
const page = await context.newPage();
const lines: string[] = [];
page.on("console", (message) => lines.push(`${message.type()} ${message.text().slice(0, 400)}`));
let client: Client | undefined;
try {
  if (!documentId) throw new Error("setup produced no note; nothing to drive");
  await page.goto(`${SHELL}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") === "s", undefined, { timeout: 300_000 });
  await page.locator("[data-semio-hub-sign-in]").first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 30_000 });
  await workspace.locator('input[type="email"]').fill(EMAIL);
  await workspace.locator('input[type="password"]').fill(PASSWORD);
  await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
  await page.waitForFunction((id) => document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") === id, spaceId, { timeout: 90_000 });
  row("1 a clean profile signs in through the shell and lands in the space", true, `space=${spaceId}`);

  const label = `Client ${LOCALE} ${randomBytes(2).toString("hex")}`;
  await page.locator("input[data-element-alias='os.hub.agent.name']").fill(label);
  await page.locator('[id="os.hub.agent.create"]').click();
  await page.locator('[data-semio-hub-agent-mcp="idle"]').waitFor({ state: "visible", timeout: 60_000 });
  const installName = await page.locator('[id="os.hub.agent.mcpInstall"]').getAttribute("aria-label");
  await page.locator('[id="os.hub.agent.mcpInstall"]').click();
  await page.locator('[data-semio-hub-agent-mcp="ready"]').waitFor({ state: "visible", timeout: 35_000 });
  const config = JSON.parse(await page.locator("[data-semio-hub-agent-mcp-config]").innerText()) as { mcpServers: Record<string, { command: string; args: string[] }> };
  const [serverName, entry] = Object.entries(config.mcpServers)[0]!;
  const credentialPath = entry.args[entry.args.indexOf("--credential-file") + 1] ?? "";
  row(`2 "${WORDS.install}" hands out one stdio entry bound to the hub space (${LOCALE})`, installName === WORDS.install && existsSync(credentialPath) && entry.args.includes(spaceId), `server=${serverName} command=${entry.command}`);
  await skipTours(page);

  const connectStarted = Date.now();
  client = new Client({ name: "semio-user-path", version: "1" }, { capabilities: {} });
  await client.connect(new StdioClientTransport({ command: entry.command, args: entry.args, env: { ...process.env } as Record<string, string>, stderr: "pipe" }), { timeout: 900_000 });
  const tools = await client.listTools(undefined, { timeout: 120_000 });
  const resolved: any = await client.callTool({ name: "context_resolve", arguments: {} }, undefined, { timeout: 120_000 });
  const principal = String(resolved.structuredContent?.principal ?? "");
  row("3 the official MCP SDK client connects from that entry and lists the tools", tools.tools.length > 0 && principal.startsWith("agent:"), `tools=${tools.tools.length} principal=${principal} connect=${((Date.now() - connectStarted) / 1000).toFixed(1)} s`);

  const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: documentId } }, undefined, { timeout: 600_000 });
  const search: any = await client.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } }, undefined, { timeout: 120_000 });
  const addBlock = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((id) => id.endsWith(".addBlock")) ?? "");
  const edited: any = await client.callTool({ name: "action_invoke", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
  row("4 the client edits the hub note", opened.isError !== true && edited.structuredContent?.status === "SUCCEEDED", `${addBlock} status=${edited.structuredContent?.status ?? JSON.stringify(edited.structuredContent).slice(0, 200)}`);

  const destructive = addBlock.replace(/\.addBlock$/u, ".deleteSelection");
  const before = new Set(await page.locator("[data-semio-agent-approval-id]").evaluateAll((elements) => elements.map((element) => element.getAttribute("data-semio-agent-approval-id"))));
  const pending = client.callTool({ name: "action_invoke", arguments: { capabilityId: destructive, input: {} } }, undefined, { timeout: 600_000 });
  let approvalId = "";
  for (let tick = 0; tick < 120 && !approvalId; tick += 1) {
    approvalId = (await page.locator("[data-semio-agent-approval-id]").evaluateAll((elements) => elements.map((element) => element.getAttribute("data-semio-agent-approval-id") ?? ""))).find((id) => id && !before.has(id)) ?? "";
    if (!approvalId) await pause(500);
  }
  const onceWord = approvalId ? ((await page.locator(`[id="framework.approvals.once.${approvalId}"]`).first().innerText().catch(() => "")) ?? "").trim() : "";
  await page.screenshot({ path: join(OUT, `approval-${LOCALE}.png`) });
  if (approvalId) {
    await skipTours(page);
    const once = page.locator(`[id="framework.approvals.once.${approvalId}"]`).first();
    await once.waitFor({ state: "visible", timeout: 30_000 });
    await once.click({ timeout: 10_000 });
  }
  const decided: any = await pending;
  const approved = approvalId.length > 0 && onceWord === WORDS.once && decided.isError !== true;
  row(`5 the destructive request waits for the human, who approves it in the shell (${LOCALE})`, approved, `approval=${approvalId || "<none>"} button="${onceWord}"`);
  row("6 time from the clean profile's first page load to an approved agent edit", approved, approved ? `${((Date.now() - pathStarted) / 1000).toFixed(1)} s` : "no approved agent edit to time");

  const copy = join(mkdtempSync(join(tmpdir(), "semio-user-path-cred-")), "credential.json");
  copyFileSync(credentialPath, copy);
  chmodSync(copy, 0o600);
  await skipTours(page);
  if ((await page.locator("[data-semio-hub-agent-space]").first().getAttribute("data-semio-hub-agent-space").catch(() => null)) !== spaceId) {
    console.log(`INFO [${seconds()} s] the agent pane is bound to another space at withdrawal time; the user reopens the space from the hub list to manage its agents`);
    await page.locator(`[data-semio-hub-workspace] li[data-space-id="${spaceId}"] button`).first().click({ force: true, timeout: 30_000 });
    await page.waitForFunction((id) => window.location.pathname.includes(`/spaces/${id}`), spaceId, { timeout: 120_000 });
    await skipTours(page);
    await page.locator('[data-slot="navbar"]').first().focus().catch(() => undefined);
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const palette = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await palette.waitFor({ state: "visible", timeout: 15_000 });
    await palette.fill("hub");
    await page.locator('[data-slot="command-item"]').filter({ hasText: /open hub|hub.*öffnen/iu }).first().click({ timeout: 10_000 });
    await page.waitForFunction((id) => document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") === id, spaceId, { timeout: 60_000 }).catch(() => undefined);
    await skipTours(page);
  }
  await page.locator("li[data-delegation-id]", { hasText: label }).locator("button").first().click({ timeout: 60_000 });
  await page.locator("[data-semio-hub-agent-revoke-confirm]").waitFor({ state: "visible", timeout: 10_000 });
  await page.locator("[data-semio-hub-agent-revoke-confirm] button").first().click();
  for (let tick = 0; tick < 60 && existsSync(credentialPath); tick += 1) await pause(250);
  const stale = new Client({ name: "semio-user-path-revoked", version: "1" }, { capabilities: {} });
  const refused = await stale.connect(new StdioClientTransport({ command: entry.command, args: entry.args.map((arg) => (arg === credentialPath ? copy : arg)), env: { ...process.env } as Record<string, string>, stderr: "pipe" }), { timeout: 120_000 }).then(
    () => "connected",
    (error: unknown) => `refused: ${String(error).slice(0, 200)}`,
  );
  await stale.close().catch(() => undefined);
  row("7 withdrawing in the pane removes the credential and the hub refuses a client holding a copy", !existsSync(credentialPath) && refused.startsWith("refused"), `installed file present=${existsSync(credentialPath)} copy=${refused}`);
  const afterRevoke: any = await client.callTool({ name: "action_invoke", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 120_000 }).catch((error: unknown) => ({ isError: true, structuredContent: { message: String(error) } }));
  row("8 the client that was already connected is refused on its next request after the withdrawal", afterRevoke.isError === true, afterRevoke.isError ? `refused ${JSON.stringify(afterRevoke.structuredContent).slice(0, 240)}` : `still edits: status=${afterRevoke.structuredContent?.status}`);
} catch (error) {
  row("run", false, error instanceof Error ? (error.stack ?? error.message) : String(error));
  await page.screenshot({ path: join(OUT, `error-${LOCALE}.png`) }).catch(() => undefined);
} finally {
  await client?.close().catch(() => undefined);
  writeFileSync(join(OUT, `console-${LOCALE}.txt`), lines.slice(-400).join("\n"));
  writeFileSync(join(OUT, `rows-${LOCALE}.json`), JSON.stringify(rows, null, 1));
  await browser.close();
}
const green = rows.filter((entry) => entry.ok).length;
const red = rows.filter((entry) => !entry.ok);
console.log(`user-path (${LOCALE}): ${green}/${rows.length} rows green in ${seconds()} s`);
publishAcceptanceCheckResult(
  repoRoot,
  acceptanceCheckResult({
    check: "mcp-user-path",
    status: red.length === 0 && rows.length >= 9 ? "pass" : "fail",
    startedAt,
    measured: { locale: LOCALE, rows: rows.length, green, hub: HUB, shell: SHELL },
    summary: {
      en: `${green}/${rows.length} user-path rows green in ${LOCALE}${red.length ? `; red: ${red.map((entry) => entry.step.split(" ")[0]).join(",")}` : ""}`,
      de: `${green}/${rows.length} Zeilen des Nutzerwegs grün in ${LOCALE}${red.length ? `; rot: ${red.map((entry) => entry.step.split(" ")[0]).join(",")}` : ""}`,
    },
    evidence: [join(OUT, `rows-${LOCALE}.json`), join(OUT, `console-${LOCALE}.txt`)],
  }),
);
process.exit(red.length === 0 ? 0 : 1);
