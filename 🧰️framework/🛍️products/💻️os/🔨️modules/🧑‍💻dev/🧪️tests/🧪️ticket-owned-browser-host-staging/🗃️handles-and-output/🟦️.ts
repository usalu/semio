/** 🗃️ The per-path sqlite handle cache and the deployment output preservation of the ticket-owned browser host staging suite. */
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerHandleAndOutputTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { PLUGIN_HOST_SHIM_FILE, assertExtensionOutputsFresh, assertNoStalePublicPluginOutputs, assertPluginOutputChildren, createHash, dirname, existsSync, hostShimSource, join, mkdirSync, mkdtempSync, readFileSync, repoRoot, rmSync, tmpdir, writeFileSync } = dependencies;
  const { describe, expect, it, beforeEach, afterEach } = vitest;
  const { underBun } = createTestKit(vitest, dependencies, source);
  const handleCache = (...paths: readonly string[]): { readonly same: boolean[] } => underBun(`const { backboneDbHandleFor } = await import(process.argv[1]); const paths = JSON.parse(process.argv[2]); const handles = []; for (const path of paths) handles.push(await backboneDbHandleFor(path)); console.log(JSON.stringify({ same: handles.slice(1).map((handle, index) => handle === handles[index]) }));`, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🗄️backbone-storage/🟦️.ts"), JSON.stringify(paths));

  //#region 🔖️T-P8-sqlite-handle-cache-tests
  describe("backboneDbHandleFor (T-P8 per-path sqlite handle cache)", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-backbone-db-cache-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("returns the SAME handle for the same path across repeated calls", () => {
      const dbPath = join(root, "a", "documents.db");
      mkdirSync(dirname(dbPath), { recursive: true });
      expect(handleCache(dbPath, dbPath).same).toEqual([true]);
    });

    it("returns DISTINCT handles for distinct paths", () => {
      const dbPathA = join(root, "a", "documents.db");
      const dbPathB = join(root, "b", "documents.db");
      mkdirSync(dirname(dbPathA), { recursive: true });
      mkdirSync(dirname(dbPathB), { recursive: true });
      expect(handleCache(dbPathA, dbPathB).same).toEqual([false]);
    });
  });

  //#endregion 🔖️T-P8-sqlite-handle-cache-tests
  //#region 🔖️T-P8-extension-sweep-tests
  describe("deployment output preservation", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-extension-sweep-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("reports a retained worker without deleting it", () => {
      const dir = join(root, "📝️flow-extension-text");
      mkdirSync(dir, { recursive: true });
      const staleWorker = join(dir, "🧵️plugin-worker.js");
      writeFileSync(staleWorker, "/** pre-H2 leftover */");
      expect(() => assertExtensionOutputsFresh(root)).toThrow(/retained worker/);
      expect(readFileSync(staleWorker, "utf8")).toBe("/** pre-H2 leftover */");
    });

    it("reports a stale host shim without deleting it", () => {
      const dir = join(root, "📝️flow-extension-text");
      mkdirSync(dir, { recursive: true });
      const staleShim = join(dir, PLUGIN_HOST_SHIM_FILE);
      writeFileSync(staleShim, "/** @generated semio plugin host shim */\nexport function readDocument(handle) { throw `unsupported: ${handle}`; }\n");
      expect(() => assertExtensionOutputsFresh(root)).toThrow(/stale host shim/);
      expect(readFileSync(staleShim, "utf8")).toContain("export function readDocument");
    });

    it("keeps a 🟨️.js whose content already matches the current hostShimSource()", () => {
      const dir = join(root, "🗒️note");
      mkdirSync(dir, { recursive: true });
      const freshShim = join(dir, PLUGIN_HOST_SHIM_FILE);
      writeFileSync(freshShim, hostShimSource());
      assertExtensionOutputsFresh(root);
      expect(existsSync(freshShim)).toBe(true);
      expect(readFileSync(freshShim, "utf8")).toBe(hostShimSource());
    });

    it("is a no-op against a missing root", () => {
      expect(() => assertExtensionOutputsFresh(join(root, "does-not-exist"))).not.toThrow();
    });

    it("does not refuse the very build that republishes the stale install", () => {
      const dir = join(root, "📝️flow-extension-text");
      mkdirSync(dir, { recursive: true });
      writeFileSync(join(dir, PLUGIN_HOST_SHIM_FILE), "/** @generated semio plugin host shim */\n");
      writeFileSync(join(dir, "🧵️plugin-worker.js"), "/** pre-H2 leftover */");
      const inScope = [{ pluginId: "flow-extension-text", directoryName: "📝️flow-extension-text", role: "extension" }] as never;
      const otherScope = [{ pluginId: "note", directoryName: "🗒️note", role: "plugin" }] as never;
      expect(() => assertExtensionOutputsFresh(root, inScope)).not.toThrow();
      expect(() => assertExtensionOutputsFresh(root, otherScope)).toThrow(/preserved/);
      expect(() => assertExtensionOutputsFresh(root)).toThrow(/preserved/);
      expect(readFileSync(join(dir, "🧵️plugin-worker.js"), "utf8")).toBe("/** pre-H2 leftover */");
    });

    it("preserves an unexpected public output tree and accepts its absence", async () => {
      const { webcrypto } = await import("node:crypto");
      const specimen = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🛡️deployment-preservation.json"), "utf8")).stalePublicOutput;
      const directory = join(root, specimen.directory), file = join(directory, specimen.filename);
      expect(() => assertNoStalePublicPluginOutputs(directory)).not.toThrow();
      mkdirSync(directory, { recursive: true });
      writeFileSync(file, specimen.content);
      expect(() => assertNoStalePublicPluginOutputs(directory)).toThrow(/preserved/);
      expect(createHash("sha256").update(readFileSync(file)).digest("hex")).toBe(Buffer.from(await webcrypto.subtle.digest("SHA-256", new TextEncoder().encode(specimen.content))).toString("hex"));
    });

    it("preserves every neutral hostile specimen byte for byte", async () => {
      const { webcrypto } = await import("node:crypto");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🛡️deployment-preservation.json"), "utf8"));
      for (const row of fixture.cases) {
        const parent = join(root, row.name), dir = join(parent, "📝️flow-extension-text");
        mkdirSync(dir, { recursive: true });
        const path = join(dir, row.filename);
        writeFileSync(path, row.content);
        const expected = Buffer.from(await webcrypto.subtle.digest("SHA-256", new TextEncoder().encode(row.content))).toString("hex");
        if (row.filename !== PLUGIN_HOST_SHIM_FILE) expect(() => assertPluginOutputChildren(dir, "fixture_component")).toThrow(/preserved/);
        if (row.extensionRejected) expect(() => assertExtensionOutputsFresh(parent)).toThrow(/preserved/);
        else expect(() => assertExtensionOutputsFresh(parent)).not.toThrow();
        expect(createHash("sha256").update(readFileSync(path)).digest("hex")).toBe(expected);
      }
    });
  });
  //#endregion 🔖️T-P8-extension-sweep-tests
}
