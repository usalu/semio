type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { EXTENSION_COMPONENT_FILE, EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, EXTENSION_PACKAGE_ENVELOPE_TOKEN, MODULE_EXTENSION_ROUTE, createExtensionStore, decodeOwnedZip, decodePackValue, existsSync, extensionPackageContentHash, installationDirectoryCollision, installationDirectoryEmoji, join, mkdtempSync, packExtensionPackage, readFileSync, rmSync, tmpdir, unpackExtensionPackage, wrapExtensionPackageEnvelope } = dependencies;
  type ExtensionPackageManifestRecord = any;

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
      const vector = JSON.parse(readFileSync(new URL("../../🧩️extension/🧪️installation.json", source.url), "utf8"));
      const root = mkdtempSync(join(tmpdir(), "semio-authored-install-"));
      const writes: string[] = [];
      try {
        const store = createExtensionStore({ installRoot: root, repoRoot: root, materializer: async (input) => {
          writes.push(input.outDir);
          return { moduleUrl: `${MODULE_EXTENSION_ROUTE}/${input.directoryName}/module.js` };
        } });
        const packed = packExtensionPackage({ manifest: vector.manifest, componentWasm: fixtureWasm });
        const installed = await store.installFromBytes(packed);
        expect(installed.extensionId).toBe(vector.manifest.extensionId);
        expect(writes).toEqual([join(root, vector.manifest.directoryName)]);
        expect((await store.listInstalled())[0].directoryName).toBe(vector.manifest.directoryName);
        expect(existsSync(join(root, vector.manifest.extensionId))).toBe(false);
        const collision = { ...vector.manifest, extensionId: "other-id", directoryName: "🧩️another" };
        await expect(store.installFromBytes(packExtensionPackage({ manifest: collision, componentWasm: fixtureWasm }))).rejects.toThrow(/sibling emoji/);
        expect(writes).toHaveLength(1);
        await store.uninstall(vector.manifest.extensionId);
        expect(existsSync(join(root, vector.manifest.directoryName))).toBe(false);
      } finally { rmSync(root, { recursive: true, force: true }); }
    });

    it("agrees with JSON Schema and independent emoji identity checks", async () => {
      const { default: Ajv } = await import("ajv");
      const emojiRegex = (await import("emoji-regex")).default;
      const { installationDirectoryEmoji, installationDirectoryCollision } = await import("../../../../🧩️extension/🟦️.ts");
      const schema = JSON.parse(readFileSync(new URL("../../🧩️extension/📐️directory.schema.json", source.url), "utf8"));
      const vector = JSON.parse(readFileSync(new URL("../../🧩️extension/🧪️installation.json", source.url), "utf8"));
      const cases = JSON.parse(readFileSync(new URL("../📇️registry/📦️deployment/🧪️cases.json", source.url), "utf8"));
      const validate = new Ajv({ strict: true }).compile(schema);
      for (const name of [...cases.validDirectories, ...cases.invalidDirectories]) {
        const valid = cases.validDirectories.includes(name);
        expect(validate(name), name).toBe(valid);
        if (valid) expect(installationDirectoryEmoji(name)).toBe([...name.replaceAll("\uFE0F", "").matchAll(emojiRegex())][0][0]);
        else expect(() => installationDirectoryEmoji(name)).toThrow();
      }
      for (const row of vector.collisions) expect(installationDirectoryCollision(row.directoryName, row.siblings) ?? null).toBe(row.conflict);
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

}
