/**
 * ♨️ Law: a source edit reaches a live page as a Vite HMR update through exactly the dev serve's own delivery path —
 * `server.watch: null` + `semioSourceFreshnessVitePlugins` (the `node:fs` source watcher replayed onto Vite's emitter) and
 * Vite's HMR socket, under the runtime this suite runs on (Bun: Vite's built-in-`ws` branch; Node: its bundled `ws`) — and
 * the serve survives its own restart under a connected page.
 *
 * A sandbox module graph (`🧰️framework/🟦️.ts` accepting `./🍃️leaf.ts`) is served by a real Vite dev server and loaded by
 * Chromium; every write style a developer's editor uses (in place, atomic temp + rename) must re-execute the leaf in the
 * page within the bound EXACTLY once (counted over a quiet window, so a doubled replay is a failure, not a pass), and a
 * server WITHOUT the watcher plugins must deliver nothing (so the law measures the replacement watcher, not a Vite watcher
 * that should not exist). The sandbox root is `mkdtemp` under the OS temp directory, a symlink on macOS: Vite keys its
 * module graph by real path, so the replayed events must name real paths too (the defect this root pins).
 *
 * `server.restart()` is what every edit of a serve's config entry runs. Under a connected page it must finish within the
 * bound, emit the old HTTP server's `close` exactly once (every source watcher and stream channel is released on it), answer
 * again, and the page Vite's client reloads on reconnect must keep receiving exactly one update per write — the contract of
 * `semioServeCloseVitePlugin`.
 * Expectations live in the language-agnostic fixture `🧫️fixtures/♨️hot-update.json`. Oracles: Vite's own client runtime
 * (`import.meta.hot.accept`, its reload-on-reconnect) and Chromium. Launches a browser, so it runs from the `long` level on.
 *
 * @vitest-environment node
 */
import { mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { createServer as createNetServer } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020.js";
import workerFreshness from "../../🧫️fixtures/👷️worker-freshness/🔣️.json" with { type: "json" };
import workerFreshnessSchema from "../../🧬️schema/👷️worker-freshness/🔣️.json" with { type: "json" };
import { createServer, type ViteDevServer } from "vite";
import { semioServeCloseVitePlugin } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { semioSourceFreshnessVitePlugins } from "../../🔌️vite-plugins/🟦️.ts";

type HotUpdateFixture = {
  readonly boundMs: number;
  readonly quietWindowMs: number;
  readonly writeStyles: readonly ("in-place" | "atomic-rename")[];
  readonly updatesPerWrite: number;
  readonly unwatchedUpdates: number;
  readonly restart: { readonly boundMs: number; readonly status: number; readonly closeEvents: number; readonly updatesPerWrite: number };
};
type HotState = { readonly value: string; readonly updates: number };
type Page = import("playwright").Page;

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♨️hot-update.json", import.meta.url), "utf8")) as HotUpdateFixture;
const longLevel = ["long", "exhaustive"].includes(process.env.SEMIO_TEST_LEVEL ?? "");
const runtime = process.versions.bun ? `bun ${process.versions.bun}` : `node ${process.versions.node}`;

const WRITES: Readonly<Record<HotUpdateFixture["writeStyles"][number], (path: string, content: string) => void>> = {
  "in-place": (path, content) => writeFileSync(path, content),
  "atomic-rename": (path, content) => {
    writeFileSync(`${path}.tmp`, content);
    renameSync(`${path}.tmp`, path);
  },
};

async function loopbackPort(): Promise<number> {
  return new Promise((resolvePort, reject) => {
    const server = createNetServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => (address && typeof address === "object" ? resolvePort(address.port) : reject(new Error("no port"))));
    });
  });
}

function sandboxGraph(): { readonly root: string; readonly leaf: string } {
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "semio-hot-update-"));
  mkdirSync(join(root, "🧰️framework"), { recursive: true });
  const leaf = join(root, "🧰️framework/🍃️leaf.ts");
  writeFileSync(leaf, 'export const value: string = "0";\n');
  writeFileSync(join(root, "🧰️framework/🟦️.ts"), [
    'import { value } from "./🍃️leaf.ts";',
    "const state = { value, updates: 0 };",
    "Object.defineProperty(window, \"__hot\", { value: state });",
    'if (import.meta.hot) import.meta.hot.accept("./🍃️leaf.ts", (next) => { if (next) { state.value = next.value; state.updates += 1; } });',
    "",
  ].join("\n"));
  writeFileSync(join(root, "index.html"), '<!doctype html><html><head><meta charset="utf-8"></head><body><script type="module" src="/🧰️framework/🟦️.ts"></script></body></html>');
  return { root, leaf };
}

async function serve(root: string, withWatcher: boolean) {
  const port = await loopbackPort();
  const server = await createServer({
    configFile: false,
    root,
    logLevel: "silent",
    cacheDir: join(root, ".vite"),
    optimizeDeps: { noDiscovery: true, include: [] },
    server: { host: "127.0.0.1", port, strictPort: true, watch: null },
    plugins: [semioServeCloseVitePlugin(), ...(withWatcher ? semioSourceFreshnessVitePlugins({ repoRoot: root }) : [])],
  });
  await server.listen();
  return { server, url: `http://127.0.0.1:${port}/` };
}

/** 🔇️ The page's update count once no update has arrived for a whole quiet window. Before the first write this absorbs the
 * filesystem's late report of the sandbox's own creation; after a write it is what makes a second, doubled update visible. */
async function settledUpdates(page: Page): Promise<number> {
  const read = () => page.evaluate(() => (window as unknown as { __hot: HotState }).__hot.updates);
  let last = await read();
  let since = Date.now();
  const deadline = Date.now() + 4 * fixture.boundMs;
  while (Date.now() - since < fixture.quietWindowMs && Date.now() < deadline) {
    await page.waitForTimeout(50);
    const now = await read();
    if (now !== last) [last, since] = [now, Date.now()];
  }
  return last;
}

/** ✍️ Writes the leaf once per fixture write style and returns, per style, whether the new value arrived within the bound,
 * how long it took, and how many updates the page counted for it. */
async function writeEveryStyle(page: Page, leaf: string, prefix: string) {
  const rows: { readonly style: string; readonly arrived: boolean; readonly ms: number; readonly updates: number }[] = [];
  for (const style of fixture.writeStyles) {
    const next = `${prefix}${style}`;
    const before = await settledUpdates(page);
    const started = Date.now();
    WRITES[style](leaf, `export const value: string = ${JSON.stringify(next)};\n`);
    const arrived = await page.waitForFunction((wanted) => (window as unknown as { __hot: HotState }).__hot.value === wanted, next, { timeout: fixture.boundMs, polling: 20 }).then(() => true, () => false);
    const ms = Date.now() - started;
    rows.push({ style, arrived, ms, updates: (await settledUpdates(page)) - before });
  }
  return rows;
}

async function bounded<T>(work: Promise<T>, ms: number): Promise<T | "timeout"> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const timeout = new Promise<"timeout">((resolve) => (timer = setTimeout(() => resolve("timeout"), ms)));
  try {
    return await Promise.race([work, timeout]);
  } finally {
    clearTimeout(timer);
  }
}

describe("dev serve hot update delivery", () => {
  it.runIf(longLevel)("reloads the current module-worker export graph after an atomic replacement with HMR disabled", async () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(workerFreshnessSchema);
    expect(validate(workerFreshness), JSON.stringify(validate.errors)).toBe(true);
    const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "semio-worker-freshness-"));
    mkdirSync(join(root, "🧰️framework"));
    const leaf = join(root, "🧰️framework/🍃️leaf.ts");
    const worker = join(root, "🧰️framework/👷️worker.ts");
    const write = (sample: typeof workerFreshness.before, atomic: boolean) => {
      const save = atomic ? WRITES["atomic-rename"] : WRITES["in-place"];
      save(leaf, `export const ${sample.name}: string = ${JSON.stringify(sample.value)};\n`);
      save(worker, `import { ${sample.name} } from "./🍃️leaf.ts";\npostMessage(${sample.name});\n`);
    };
    write(workerFreshness.before, false);
    writeFileSync(join(root, "🧰️framework/🟦️.ts"), 'const worker = new Worker(new URL("./👷️worker.ts", import.meta.url), { type: "module" });\nworker.onmessage = event => document.querySelector("output")!.textContent = event.data;\nworker.onerror = event => document.querySelector("output")!.textContent = `ERROR: ${event.message} at ${event.filename}:${event.lineno}`;\n');
    writeFileSync(join(root, "index.html"), '<!doctype html><html><head><meta charset="utf-8"></head><body><output>waiting</output><script type="module" src="/🧰️framework/🟦️.ts"></script></body></html>');
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    const port = await loopbackPort();
    const server = await createServer({ configFile: false, root, logLevel: "silent", cacheDir: join(root, ".vite"), optimizeDeps: { noDiscovery: true, include: [] }, server: { host: "127.0.0.1", port, strictPort: true, hmr: false, watch: null }, plugins: [semioServeCloseVitePlugin(), ...semioSourceFreshnessVitePlugins({ repoRoot: root })] });
    try {
      await server.listen();
      const page = await browser.newPage();
      const errors: string[] = [];
      page.on("pageerror", error => errors.push(error.message));
      page.on("console", message => { if (message.type() === "error") errors.push(message.text()); });
      page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
      page.on("response", response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
      const expectReceipt = async (value: string) => {
        await page.waitForFunction(() => document.querySelector("output")?.textContent !== "waiting", undefined, { timeout: 30_000 }).catch(() => undefined);
        expect({ receipt: await page.locator("output").textContent(), errors }).toEqual({ receipt: value, errors: [] });
      };
      await page.goto(`http://127.0.0.1:${port}/`);
      await expectReceipt(workerFreshness.before.value);
      write(workerFreshness.after, true);
      await page.reload();
      await expectReceipt(workerFreshness.after.value);
      expect(await page.locator("output").textContent()).toBe(workerFreshness.after.value);
      expect(errors).toEqual([]);
    } finally {
      await browser.close();
      await server.close();
      rmSync(root, { recursive: true, force: true });
    }
  }, 90_000);

  it.runIf(longLevel)("re-executes an edited module in the live page once per write style, and only through the source watcher", async () => {
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    const graph = sandboxGraph();
    const watched = await serve(graph.root, true);
    let rows: Awaited<ReturnType<typeof writeEveryStyle>> = [];
    try {
      const page = await browser.newPage();
      await page.goto(watched.url);
      await page.waitForFunction(() => (window as unknown as { __hot?: HotState }).__hot?.value === "0", undefined, { timeout: 60_000 });
      rows = await writeEveryStyle(page, graph.leaf, "");
      expect(rows.map(({ style, arrived, updates }) => ({ style, arrived, updates }))).toEqual(fixture.writeStyles.map((style) => ({ style, arrived: true, updates: fixture.updatesPerWrite })));
      await page.close();
    } finally {
      await watched.server.close();
    }
    const unwatched = await serve(graph.root, false);
    try {
      const page = await browser.newPage();
      await page.goto(unwatched.url);
      await page.waitForFunction(() => (window as unknown as { __hot?: HotState }).__hot !== undefined, undefined, { timeout: 60_000 });
      const before = await page.evaluate(() => (window as unknown as { __hot: HotState }).__hot.value);
      writeFileSync(graph.leaf, 'export const value: string = "unwatched";\n');
      await page.waitForTimeout(fixture.boundMs);
      expect(await page.evaluate(() => (window as unknown as { __hot: HotState }).__hot)).toEqual({ value: before, updates: fixture.unwatchedUpdates });
    } finally {
      await bounded(unwatched.server.close(), fixture.restart.boundMs);
      await browser.close();
      rmSync(graph.root, { recursive: true, force: true });
    }
    console.log(`hot update delivery (${runtime}): ${JSON.stringify(Object.fromEntries(rows.map(({ style, ms }) => [style, ms])))} ms`);
  }, 180_000);

  it.runIf(longLevel)("restarts under a connected page, releases the old server once, serves again and keeps delivering one update per write", async () => {
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    const graph = sandboxGraph();
    const watched = await serve(graph.root, true);
    const server: ViteDevServer = watched.server;
    let closeEvents = 0;
    server.httpServer?.on("close", () => (closeEvents += 1));
    try {
      const page = await browser.newPage();
      await page.goto(watched.url);
      await page.waitForFunction(() => (window as unknown as { __hot?: HotState }).__hot?.value === "0", undefined, { timeout: 60_000 });
      await settledUpdates(page);
      await page.evaluate(() => Object.defineProperty(window, "__beforeRestart", { value: true }));
      const started = Date.now();
      const restarted = await bounded(server.restart().then(() => "restarted" as const), fixture.restart.boundMs);
      const restartMs = Date.now() - started;
      const status = await fetch(watched.url, { signal: AbortSignal.timeout(fixture.boundMs) }).then((response) => response.status, () => 0);
      expect({ restarted, status, closeEvents }).toEqual({ restarted: "restarted", status: fixture.restart.status, closeEvents: fixture.restart.closeEvents });
      await page.waitForFunction(() => !("__beforeRestart" in window) && (window as unknown as { __hot?: HotState }).__hot !== undefined, undefined, { timeout: 4 * fixture.boundMs });
      const rows = await writeEveryStyle(page, graph.leaf, "restarted ");
      expect(rows.map(({ style, arrived, updates }) => ({ style, arrived, updates }))).toEqual(fixture.writeStyles.map((style) => ({ style, arrived: true, updates: fixture.restart.updatesPerWrite })));
      console.log(`dev serve restart under a connected page (${runtime}): ${restartMs} ms, close events ${closeEvents}, then ${JSON.stringify(Object.fromEntries(rows.map(({ style, ms }) => [style, ms])))} ms`);
    } finally {
      await bounded(server.close(), fixture.restart.boundMs);
      await browser.close();
      rmSync(graph.root, { recursive: true, force: true });
    }
  }, 180_000);
});
