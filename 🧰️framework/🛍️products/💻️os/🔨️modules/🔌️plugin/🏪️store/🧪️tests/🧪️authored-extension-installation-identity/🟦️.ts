type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../../🧩️extension/🟦️.ts"), "installationDirectoryCollision" | "installationDirectoryEmoji"> & Pick<typeof import("../../../📇️registry/📦️deployment/🟦️.ts"), "MODULE_EXTENSION_ROUTE"> & Pick<typeof import("../../📥️installation/🟦️.ts"), "EXTENSION_COMPONENT_FILE" | "EXTENSION_INSTALL_PATH" | "EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI" | "EXTENSION_PACKAGE_ENVELOPE_TOKEN" | "createExtensionStore" | "semioExtensionStoreVitePlugin" | "extensionPackageContentHash" | "packExtensionPackage" | "unpackExtensionPackage" | "wrapExtensionPackageEnvelope"> & Pick<typeof import("../../🟦️.ts"), "decodeOwnedZip"> & Pick<typeof import("@semio-tech/framework-os"), "decodePackValue"> & Pick<typeof import("node:fs"), "existsSync" | "mkdtempSync" | "readFileSync" | "rmSync"> & Pick<typeof import("node:os"), "tmpdir"> & Pick<typeof import("node:path"), "join">, source: TestSource): Promise<void> {
  const { EXTENSION_COMPONENT_FILE, EXTENSION_INSTALL_PATH, EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, EXTENSION_PACKAGE_ENVELOPE_TOKEN, MODULE_EXTENSION_ROUTE, createExtensionStore, semioExtensionStoreVitePlugin, decodeOwnedZip, decodePackValue, existsSync, extensionPackageContentHash, installationDirectoryCollision, installationDirectoryEmoji, join, mkdtempSync, packExtensionPackage, readFileSync, rmSync, tmpdir, unpackExtensionPackage, wrapExtensionPackageEnvelope } = dependencies;
  type ExtensionPackageManifestRecord = import("../../📥️installation/🟦️.ts").ExtensionPackageManifestRecord;

  const { describe, expect, it } = vitest;
  const legacyFflateZip = Uint8Array.from(
    Buffer.from(
      "UEsDBBQAAAgIAAAAIQAAHXf+kQAAAJwAAAAVAAAA8J+bgu+4j21hbmlmZXN0LnNlbWlvY2U11DPSM+ZPy6woKS1K1Tu8JzcnsbSEpSg1MYWxmP/D/OUr3+/oV3CDSDMyCgqws/MkJxYkJmXmZJZkphbzMLIxsfMm5+eVFGUmlZZk5ucV8zCws6dWlKTmpRSzMbNzg5nFQAnPFDZGdtacxKTUHDYWdt6CxOTsxPRUt/yi3MQSVgYw+GDPzl6WWgRSzcYAAFBLAwQUAAAACAAAACEAzjNLHAoAAAAIAAAADgAAAGNvbXBvbmVudC53YXNtY0gszmVkYGAAAFBLAwQUAAAICAAAACEA2f/rzhIAAAAQAAAAGAAAAGFzc2V0cy9pY29ucy/wn6ep77iPLnR4dHMvOrwntSpT4cP8nt73O/q5AFBLAQIUABQAAAgIAAAAIQAAHXf+kQAAAJwAAAAVAAAAAAAAAAAAAAAAAAAAAADwn5uC77iPbWFuaWZlc3Quc2VtaW9QSwECFAAUAAAACAAAACEAzjNLHAoAAAAIAAAADgAAAAAAAAAAAAAAAADEAAAAY29tcG9uZW50Lndhc21QSwECFAAUAAAICAAAACEA2f/rzhIAAAAQAAAAGAAAAAAAAAAAAAAAAAD6AAAAYXNzZXRzL2ljb25zL/Cfp6nvuI8udHh0UEsFBgAAAAADAAMAxQAAAEIBAAAAAA==",
      "base64",
    ),
  );
  const fixtureManifest: ExtensionPackageManifestRecord = {
    extensionId: "fixture.ümlaut",
    directoryName: "🧩️fixture-umlaut",
    label: "🧩️ Fixture",
    version: "1.2.3",
    extends: "s",
    capabilities: ["read"],
    contributions: [],
    packageFormat: 1,
  };
  const fixtureWasm = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]);
  const fixtureAsset = new TextEncoder().encode("Grüezi 🌍️\n");

  describe("authored extension installation identity", () => {
    it("retains the declared physical name independently of the public extension ID", async () => {
      const root = mkdtempSync(join(tmpdir(), "semio-authored-install-"));
      const writes: string[] = [];
      try {
        const store = createExtensionStore({ installRoot: root, repoRoot: root, materializer: async (input) => {
          writes.push(input.outDir);
          return { moduleUrl: `${MODULE_EXTENSION_ROUTE}/${input.directoryName}/module.js` };
        } });
        const packed = packExtensionPackage({ manifest: fixtureManifest, componentWasm: fixtureWasm });
        const installed = await store.installFromBytes(packed);
        expect(installed).toEqual((await store.listInstalled())[0]);
        expect(installed).toMatchObject({ extensionId: fixtureManifest.extensionId, directoryName: fixtureManifest.directoryName, label: fixtureManifest.label, version: fixtureManifest.version, extends: fixtureManifest.extends, packageHash: extensionPackageContentHash(packed) });
        expect(writes).toEqual([join(root, fixtureManifest.directoryName)]);
        expect((await store.listInstalled())[0].directoryName).toBe(fixtureManifest.directoryName);
        expect(existsSync(join(root, fixtureManifest.extensionId))).toBe(false);
        const collision = { ...fixtureManifest, extensionId: "other-id", directoryName: "🧩️another" };
        await expect(store.installFromBytes(packExtensionPackage({ manifest: collision, componentWasm: fixtureWasm }))).rejects.toThrow(/sibling emoji/);
        expect(writes).toHaveLength(1);
        await store.uninstall(fixtureManifest.extensionId);
        expect(existsSync(join(root, fixtureManifest.directoryName))).toBe(false);
      } finally { rmSync(root, { recursive: true, force: true }); }
    });

    it("matches the language-neutral installed-record schema", async () => {
      const { default: Ajv2020 } = await import("ajv/dist/2020.js");
      const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
      expect(new Ajv2020({ strict: true }).compile(schema)(fixture)).toBe(true);
    });

    it("agrees with JSON Schema and independent emoji identity checks", async () => {
      const { default: Ajv } = await import("ajv");
      const emojiRegex = (await import("emoji-regex")).default;
      const { installationDirectoryEmoji, installationDirectoryCollision } = await import("../../../../🧩️extension/🟦️.ts");
      const validate = new Ajv({ strict: true }).compile({ type: "string", pattern: "^(?=.*[^\\x00-\\x7F]).+$" });
      const cases = { validDirectories: ["🧩️fixture", "🧪️probe"], invalidDirectories: ["fixture", ""] };
      for (const name of [...cases.validDirectories, ...cases.invalidDirectories]) {
        const valid = cases.validDirectories.includes(name);
        expect(validate(name), name).toBe(valid);
        if (valid) expect(installationDirectoryEmoji(name)).toBe([...name.replaceAll("\uFE0F", "").matchAll(emojiRegex())][0][0]);
        else expect(() => installationDirectoryEmoji(name)).toThrow();
      }
      expect(installationDirectoryCollision("🧩️another", ["🧩️fixture"])).toBe("🧩️fixture");
    });
  });

  describe("extension package ZIP ownership", () => {
    it("decodes the pinned fflate UTF-8/DEFLATE fixture and preserves its hash", () => {
      expect(extensionPackageContentHash(legacyFflateZip)).toBe("43675c79f03ba52f45cc57eecabee2a9334e93957128e7975a2979528d14efa9");
      const decoded = decodeOwnedZip(legacyFflateZip);
      expect(decodePackValue(decoded.get(EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI)!)).toMatchObject({ extensionId: fixtureManifest.extensionId, label: fixtureManifest.label, version: fixtureManifest.version });
      expect(decoded.get(EXTENSION_COMPONENT_FILE)).toEqual(fixtureWasm);
      expect(decoded.get("assets/icons/🧩️.txt")).toEqual(fixtureAsset);
      expect(() => unpackExtensionPackage(wrapExtensionPackageEnvelope(legacyFflateZip))).toThrow(/Installation directory/);
    });

    it("encodes deterministic synchronous packages with UTF-8 asset names", () => {
      const input = { manifest: fixtureManifest, componentWasm: fixtureWasm, assets: new Map([["icons/🧩️.txt", fixtureAsset]]) };
      const first = packExtensionPackage(input);
      const second = packExtensionPackage(input);
      expect(first).toEqual(second);
      const unpacked = unpackExtensionPackage(first);
      expect(unpacked.packageHash).toBe(extensionPackageContentHash(first));
      expect(unpacked.manifest).toMatchObject({ extensionId: fixtureManifest.extensionId, label: fixtureManifest.label, version: fixtureManifest.version });
      expect(unpacked.wasmBytes).toEqual(fixtureWasm);
      expect(unpacked.assets.get("icons/🧩️.txt")).toEqual(fixtureAsset);
    });

    it("rejects an entry whose declared expansion exceeds the owned bound", () => {
      const packed = packExtensionPackage({ manifest: fixtureManifest, componentWasm: fixtureWasm });
      const zipStart = 12 + new TextEncoder().encode(EXTENSION_PACKAGE_ENVELOPE_TOKEN).length;
      const corrupted = packed.slice(zipStart);
      const data = new DataView(corrupted.buffer, corrupted.byteOffset, corrupted.byteLength);
      const end = corrupted.length - 22;
      const central = data.getUint32(end + 16, true);
      data.setUint32(central + 24, 256 * 1024 * 1024 + 1, true);
      expect(() => unpackExtensionPackage(wrapExtensionPackageEnvelope(corrupted))).toThrow("decoded size limit");
    });
  });

  type RouteMiddleware = (req: RouteRequest, res: RouteResponse, next: () => void) => void | Promise<void>;
  type RouteRequest = { readonly url: string; readonly method: string; readonly headers: Record<string, string>; on: (event: string, listener: () => void) => void };
  type RouteResponse = { statusCode: number; readonly headers: Map<string, string>; readonly chunks: string[]; ended: boolean; setHeader: (name: string, value: string) => void; write: (chunk: string) => boolean; end: (chunk?: string) => void; on: () => void; once: () => void; emit: () => boolean };
  type RoutePlugin = { readonly name: string; readonly enforce?: "pre" | "post"; readonly apply?: string; configureServer?: (server: { middlewares: { use: (handler: RouteMiddleware) => void } }) => void };

  /** 🧭️ Vite's own plugin order (`sortUserPlugins`): `pre`, then unmarked, then `post`, each in declaration order. */
  const viteServeOrder = (plugins: readonly RoutePlugin[]): RoutePlugin[] => [...plugins.filter((plugin) => plugin.enforce === "pre"), ...plugins.filter((plugin) => plugin.enforce === undefined), ...plugins.filter((plugin) => plugin.enforce === "post")].filter((plugin) => plugin.apply !== "build");

  const answer = async (plugins: readonly RoutePlugin[], url: string, method = "GET"): Promise<RouteResponse & { readonly passedThrough: boolean }> => {
    const handlers: RouteMiddleware[] = [];
    for (const plugin of viteServeOrder(plugins)) plugin.configureServer?.({ middlewares: { use: (handler) => handlers.push(handler) } });
    const headers = new Map<string, string>();
    const res: RouteResponse = { statusCode: 200, headers, chunks: [], ended: false, setHeader: (name, value) => void headers.set(name.toLowerCase(), value), write: (chunk) => (res.chunks.push(chunk), true), end: (chunk) => { if (chunk) res.chunks.push(chunk); res.ended = true; }, on: () => {}, once: () => {}, emit: () => false };
    const req: RouteRequest = { url, method, headers: {}, on: () => {} };
    let index = 0;
    let passedThrough = false;
    const next = (): void => {
      const handler = handlers[index++];
      if (handler === undefined) {
        passedThrough = true;
        return;
      }
      void handler(req, res, next);
    };
    next();
    for (let tick = 0; tick < 50 && !res.ended && !res.chunks.some((chunk) => chunk.startsWith("data:")) && !passedThrough; tick += 1) await new Promise((resolve) => setTimeout(resolve, 10));
    return Object.assign(res, { passedThrough });
  };

  /** 🔌️ LAW: the dev serve mounts the install root statically on the store's own route and lists the store first. The
   * static mount answers 404 for every path its root lacks, so the store must be `pre` too — unmarked, vite put it after the
   * mount and every request to the store's own endpoint was a 404 (ticket 26/09/23 U5, measured on :6580). Real plugins,
   * vite's ordering rule, one request through the resulting middleware chain. */
  describe("extension store route precedence", () => {
    it("answers its install endpoint before the static extension mount, which still 404s a missing module file", async () => {
      const { staticDirVitePlugin } = (await import(new URL("../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts", source.url).href)) as { staticDirVitePlugin: (repoRoot: string, spec: { readonly kind: "static-dir"; readonly route: string; readonly root: string }) => RoutePlugin[] };
      const root = mkdtempSync(join(tmpdir(), "semio-extension-route-"));
      try {
        const store = semioExtensionStoreVitePlugin({ installRoot: root, repoRoot: root }) as RoutePlugin;
        expect(store.enforce).toBe("pre");
        const mount = staticDirVitePlugin(root, { kind: "static-dir", route: MODULE_EXTENSION_ROUTE, root: "." });
        const listed = await answer([store, ...mount], EXTENSION_INSTALL_PATH);
        expect(listed.statusCode).toBe(200);
        expect(JSON.parse(listed.chunks.join(""))).toEqual([]);
        const missing = await answer([store, ...mount], `${MODULE_EXTENSION_ROUTE}/🧩️absent/module.js`);
        expect(missing.statusCode).toBe(404);
        const unmarked = await answer([{ ...store, enforce: undefined }, ...mount], EXTENSION_INSTALL_PATH);
        expect(unmarked.statusCode, "the defect this law pins: an unmarked store is shadowed by the pre static mount").toBe(404);
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });

    it("publishes installed extensions on the dev stream channel, not as an HTTP response held open", async () => {
      const { devStreamMuxServer } = (await import(new URL("../../../🧑‍💻dev/🔌️vite-plugins/🟦️.ts", source.url).href)) as typeof import("../../../../🧑‍💻dev/🔌️vite-plugins/🟦️.ts");
      const { DEV_STREAM_ROUTES } = (await import("@semio-tech/framework-os")) as typeof import("@semio-tech/framework-os");
      const root = mkdtempSync(join(tmpdir(), "semio-extension-stream-"));
      try {
        const httpServer = { on: () => undefined, once: () => undefined };
        const store = semioExtensionStoreVitePlugin({ installRoot: root, repoRoot: root }) as RoutePlugin & { configureServer: (server: { middlewares: { use: (handler: RouteMiddleware) => void }; httpServer: typeof httpServer }) => void };
        const handlers: RouteMiddleware[] = [];
        store.configureServer({ middlewares: { use: (handler) => handlers.push(handler) }, httpServer });
        const sent: string[] = [];
        const connection = devStreamMuxServer(httpServer).connect({ send: (frame) => sent.push(frame), bufferedAmount: () => 0, close: () => undefined });
        connection.receive(JSON.stringify({ kind: "open", stream: 1, route: DEV_STREAM_ROUTES.extensionModules, key: "", resume: null, credit: 4 }));
        for (let tick = 0; tick < 20 && sent.length < 3; tick += 1) await new Promise((resolve) => setTimeout(resolve, 5));
        expect(sent.map((frame) => JSON.parse(frame).kind)).toEqual(["hello", "opened", "data"]);
        expect(JSON.parse(sent[2]!).data).toEqual({ kind: "snapshot", extensions: [] });
        const watched = await answer([store], `${MODULE_EXTENSION_ROUTE}/watch`);
        expect(watched.passedThrough).toBe(true);
        connection.closed();
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });

    it("lists the complete record and uninstalls through the same canonical endpoint", async () => {
      const root = mkdtempSync(join(tmpdir(), "semio-extension-route-ledger-"));
      try {
        const materializer = async (input: { readonly directoryName: string }) => ({ moduleUrl: `${MODULE_EXTENSION_ROUTE}/${input.directoryName}/module.js` });
        const direct = createExtensionStore({ installRoot: root, repoRoot: root, materializer });
        const installed = await direct.installFromBytes(packExtensionPackage({ manifest: fixtureManifest, componentWasm: fixtureWasm }));
        const plugin = semioExtensionStoreVitePlugin({ installRoot: root, repoRoot: root, materializer }) as RoutePlugin;
        const listed = await answer([plugin], EXTENSION_INSTALL_PATH);
        expect(JSON.parse(listed.chunks.join(""))).toEqual([installed]);
        const removed = await answer([plugin], `${EXTENSION_INSTALL_PATH}?extensionId=${encodeURIComponent(installed.extensionId)}`, "DELETE");
        expect(JSON.parse(removed.chunks.join(""))).toEqual({ extensionId: installed.extensionId });
        expect(await direct.listInstalled()).toEqual([]);
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  });

}
