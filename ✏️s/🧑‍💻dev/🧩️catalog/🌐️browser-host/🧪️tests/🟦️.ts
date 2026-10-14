import {describe,it,expect} from "bun:test";
import {createHash} from "node:crypto";
import {dirname,join,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,writeFileSync,rmSync} from "node:fs";
import {exactSpaceCreateArtifactArgs} from "../🏗️staging/🟦️.ts";
import {TEST_BROWSER_ACTIVATION_ROOT_ENV,TEST_BROWSER_HOST_RECEIPT_ENV,TEST_BROWSER_MODULE_ROOT_ENV,closeTestBrowserHostStagingV1,parseTestBrowserGisMaterializationReceiptV1,parseTestBrowserHostStagingReceiptV1,prepareTestBrowserHostRootsV1,resolveTestBrowserHostRootsV1,writeTestBrowserGisMaterializationReceiptV1} from "../🟦️.ts";
import {readActivationReceipt,ACTIVATION_RECEIPT_FILE} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
  const ownedRelativeFiles = (root: string): string[] => {
    const files: string[] = [];
    const visit = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) visit(path);
        else if (entry.isFile()) files.push(relative(root, path).replaceAll("\\", "/"));
      }
    };
    visit(root);
    return files.sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  };

  describe("ticket-owned browser host staging", () => {
    it("matches the neutral schema and closes only the exact Space, GIS and support module set", async () => {
      const contractRoot = fileURLToPath(new URL("../",import.meta.url));
      const document = JSON.parse(readFileSync(join(contractRoot, "🧬️schema/🔣️.json"), "utf8"));
      const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8"));
      const { default: AjvDraft7 } = await import("ajv");
      const stagingAjv = new AjvDraft7({ strict: true, allErrors: true });
      stagingAjv.addSchema(document);
      const validate = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserHostStagingV1`)!;
      const validateMaterialization = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserGisMaterializationV1`)!;
      
      const provenanceFixture = JSON.parse(readFileSync(join(contractRoot, "🧪️tests/🧫️fixtures/🧬️selected-gis-byte-provenance-v1/🔣️.json"), "utf8"));
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      
      expect(parseTestBrowserHostStagingReceiptV1(fixture)).toEqual(fixture);
      for (const specimen of provenanceFixture.specimens) {
        const bytes = new TextEncoder().encode(specimen.utf8);
        expect(createHash("sha256").update(bytes).digest("hex")).toBe(specimen.sha256);
        expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(specimen.sha256);
      }
      expect(exactSpaceCreateArtifactArgs({ dialogs: [{ id: "createArtifact", args: [{ id: "name" }, { id: "kindChoice" }] }] })).toBe(true);
      expect(exactSpaceCreateArtifactArgs({ dialogs: [{ id: "createArtifact", args: [{ id: "name" }, { id: "kindId" }] }] })).toBe(false);
      const scriptSource = readFileSync(fileURLToPath(new URL("../🏗️staging/🟦️.ts", import.meta.url)), "utf8");
      // 🔒️ `buildPluginCargo` now always builds into the ONE shared, fine-grain-locked `cargoTargetDirectory`
      // (no private per-caller target dirs) — this asserts the ticket-owned COPY step that still makes the
      // fresh Space artifact eligible for `ownedTestBrowserHostInput`'s bounded-regular-file check survives.
      expect(scriptSource).toContain("const builtSpace = await buildPluginCargo(space, input.control);");
      expect(scriptSource).toContain('const stagedSpacePath = join(artifactRoot, "browser-host-wasi-target", basename(builtSpace.artifact));');
      expect(scriptSource).toContain("writeFileSync(stagedSpacePath, readFileSync(builtSpace.artifact), { mode: 0o600 });");
      const materializerStart = scriptSource.indexOf("async function materializeTestBrowserPluginV1(");
      const materializerEnd = scriptSource.indexOf("export async function stageTestBrowserHostV1", materializerStart);
      const materializerOwner = scriptSource.slice(materializerStart, materializerEnd);
      expect(materializerOwner.indexOf("writeFileSync(join(outDir, MODULE_BRIDGE_FILE)")).toBeLessThan(materializerOwner.indexOf("writeTestBrowserGisMaterializationReceiptV1(outDir, input.selectedSource)"));
      expect(scriptSource).toContain("selectedSource: { componentSha256: input.selectedGis.componentSha256, descriptorSha256: input.selectedGis.descriptorSha256 }");
      expect(scriptSource.indexOf("await materializeTestBrowserPluginV1({ target: gis")).toBeLessThan(scriptSource.indexOf("Selected GIS bytes changed during browser staging"));

      const owner = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? (()=>{throw Error("Ticket artifacts required");})(), "browser-host-law-"));
      const artifactRoot = join(owner, "🗑️generated");
      mkdirSync(artifactRoot, { recursive: true });
      try {
        const roots = prepareTestBrowserHostRootsV1(artifactRoot);
        const hostComponent = Buffer.from("space-cargo-component");
        const hostComponentSha256 = createHash("sha256").update(hostComponent).digest("hex");
        const hostCoreSha256 = createHash("sha256").update("space-component").digest("hex");
        const specimens = Object.fromEntries(provenanceFixture.specimens.map((row: any) => [row.id, row]));
        const files = new Map([
          ["🪞️vendor/.nx-artifact.json", "vendor"],
          ["🧵️shard/🟨️shard-worker.js", "shard"],
          ["🪐️space/semio_s_plugin_space_component.core.wasm", "space-component"],
          ["🪐️space/🛂️.descriptor.semio", "space-descriptor"],
          ["🪐️space/🔣️.json", JSON.stringify({ packageId: "semio:space", manifest: { pluginId: "space" } })],
          ["🪐️space/🧾️receipt.json", `${JSON.stringify({ schema: "semio.plugin.component-receipt/v1", pluginId: "space", packageId: "semio:space", profile: "wasm-release", target: "wasm32-wasip2", wasmSha256: hostComponentSha256, wasmBytes: hostComponent.byteLength, semanticDescriptorSha256: createHash("sha256").update("space-descriptor").digest("hex"), toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "3".repeat(64), provenanceSha256: "4".repeat(64) }, null, 2)}\n`],
          ["🪐️space/🌉️bridge.js", "space-bridge"],
          ["🌍️gis/semio_hub_gis_component.core.wasm", specimens["staged-core"].utf8],
          ["🌍️gis/🛂️.descriptor.semio", specimens["selected-descriptor"].utf8],
          ["🌍️gis/🌉️bridge.js", specimens["staged-bridge"].utf8],
        ]);
        for (const [name, body] of files) {
          const path = join(roots.moduleRoot, name);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, body);
        }
        const materialization = writeTestBrowserGisMaterializationReceiptV1(join(roots.moduleRoot, "🌍️gis"), { componentSha256: specimens["selected-component"].sha256, descriptorSha256: specimens["selected-descriptor"].sha256 });
        expect(validateMaterialization(materialization), JSON.stringify(validateMaterialization.errors)).toBe(true);
        expect(parseTestBrowserGisMaterializationReceiptV1(materialization)).toEqual(materialization);
        const current = { generationId: "a".repeat(64), currentSha256: "b".repeat(64), componentSha256: specimens["selected-component"].sha256, descriptorSha256: specimens["selected-descriptor"].sha256 };
        expect(() => closeTestBrowserHostStagingV1(artifactRoot, current, { byteLength: hostComponent.byteLength, sha256: "c".repeat(64) })).toThrow("receipt differs from its materialized owner");
        const closed = closeTestBrowserHostStagingV1(artifactRoot, current, { byteLength: hostComponent.byteLength, sha256: hostComponentSha256 });
        expect(validate(closed.receipt), JSON.stringify(validate.errors)).toBe(true);
        expect(closed.receipt.selectedGis).toMatchObject(current);
        expect(closed.receipt.selectedGis).toMatchObject({ stagedDescriptorSha256: specimens["selected-descriptor"].sha256, bridgeSha256: specimens["staged-bridge"].sha256, generatedCores: [{ relativePath: "semio_hub_gis_component.core.wasm", byteLength: specimens["staged-core"].utf8.length, sha256: specimens["staged-core"].sha256 }] });
        expect(closed.receipt.host).toMatchObject({ pluginId: "space", componentByteLength: hostComponent.byteLength, componentSha256: hostComponentSha256, coreSha256: hostCoreSha256 });
        expect(readActivationReceipt(closed.activationRoot).plugins.map((row) => row.pluginId)).toEqual(["gis", "space"]);
        const environment = {
          SEMIO_TEST_ARTIFACT_DIR: artifactRoot,
          [TEST_BROWSER_MODULE_ROOT_ENV]: closed.moduleRoot,
          [TEST_BROWSER_ACTIVATION_ROOT_ENV]: closed.activationRoot,
          [TEST_BROWSER_HOST_RECEIPT_ENV]: closed.receiptPath,
        };
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);
        expect(resolveTestBrowserHostRootsV1({})).toBeNull();
        expect(() => resolveTestBrowserHostRootsV1({ SEMIO_TEST_ARTIFACT_DIR: artifactRoot, [TEST_BROWSER_MODULE_ROOT_ENV]: closed.moduleRoot })).toThrow("complete ticket authority");

        const descriptor = join(closed.moduleRoot, "🪐️space/🛂️.descriptor.semio");
        writeFileSync(descriptor, "substituted-space-descriptor");
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("module set changed");
        writeFileSync(descriptor, "space-descriptor");
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);

        const bridge = join(closed.moduleRoot, "🪐️space/🌉️bridge.js");
        writeFileSync(bridge, "substituted-space-module");
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("module set changed");
        writeFileSync(bridge, "space-bridge");
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);

        const receiptBytes = readFileSync(closed.receiptPath);
        const moduleSetSha256 = (): string => {
          const paths = ownedRelativeFiles(closed.moduleRoot);
          const hash = createHash("sha256");
          for (const path of paths) {
            const bytes = readFileSync(join(closed.moduleRoot, path));
            hash.update(`${JSON.stringify([path.replaceAll("\\", "/"), bytes.byteLength, createHash("sha256").update(bytes).digest("hex")])}\n`);
          }
          return hash.digest("hex");
        };
        for (const hostile of provenanceFixture.hostiles) {
          if (hostile.target === "host-receipt") {
            const changed: any = JSON.parse(JSON.stringify(closed.receipt));
            changed.selectedGis.componentSha256 = hostile.replacement;
            writeFileSync(closed.receiptPath, `${JSON.stringify(changed)}\n`);
          } else {
            const target = join(closed.moduleRoot, hostile.target === "staged-descriptor" ? "🌍️gis/🛂️.descriptor.semio" : "🌍️gis/🌉️bridge.js");
            const original = readFileSync(target);
            writeFileSync(target, hostile.replacement);
            const changed: any = JSON.parse(JSON.stringify(closed.receipt));
            changed.moduleSetSha256 = moduleSetSha256();
            writeFileSync(closed.receiptPath, `${JSON.stringify(changed)}\n`);
            expect(() => resolveTestBrowserHostRootsV1(environment), hostile.id).toThrow("GIS materialization differs");
            writeFileSync(target, original);
            writeFileSync(closed.receiptPath, receiptBytes);
            continue;
          }
          expect(() => resolveTestBrowserHostRootsV1(environment), hostile.id).toThrow("GIS identity changed");
          writeFileSync(closed.receiptPath, receiptBytes);
        }

        const activationPath = join(closed.activationRoot, ACTIVATION_RECEIPT_FILE);
        const activation = readFileSync(activationPath);
        writeFileSync(activationPath, `${activation.toString("utf8").trim()} `);
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("activation changed");
        writeFileSync(activationPath, activation);
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);
      } finally {
        rmSync(owner, { recursive: true, force: true });
      }
    });
  });

