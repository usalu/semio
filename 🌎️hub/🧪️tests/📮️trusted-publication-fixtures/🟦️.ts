/** 🧭️ Trusted bootstrap codec capture, generation stage and publication fixtures.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { createHash, webcrypto } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { canonicalJson } from "@semio-tech/repo-lib/authoring";
import { encodePackValue } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { assertObservationExpectation, type ObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { captureTrustedBootstrapCodecsV1, trustedBootstrapFixturePluginModule, trustedBootstrapGenerationReceipt, trustedBootstrapStageFixturePackages, trustedBootstrapVerifyGeneration, trustedBootstrapWriteFixturePluginModule } from "../../🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapProfileEncoding } from "../../🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapPublicationRevision, trustedBootstrapReadCurrentPointer, trustedBootstrapVerifyPublication } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🔄️rotation-and-current-pointer/🟦️.ts";
import { runTrustedPublicationChild, trustedBootstrapPublicationReceipt, TrustedBootstrapPublicationUnconfirmed, trustedPublicationChildReceipt } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/📮️receipts-and-publication-child/🟦️.ts";
import { webSha256Hex } from "../../🚀️local-relay/🛰️servers/🟦️.ts";

/** 🧪️ Compares fixed source admission with package schemas and independent generation hashes. */
export async function proveTrustedBootstrapCodecCaptureFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { writeFileSync, truncateSync } = await import("node:fs");
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧊️codec-source");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  assert.deepEqual(Object.keys(fixture), ["schema", "maximumSourceBytes", "maximumTotalBytes", "cases"]);
  assert.equal(fixture.schema, "semio.hub.codec-source.v1");
  assert.equal(fixture.maximumSourceBytes, 65_536);
  assert.equal(fixture.maximumTotalBytes, 131_072);
  assert.deepEqual(fixture.cases.map((row: any) => row.change), [
    "none", "permuted", "replaced-source", "invalid-hash", "zero-hash", "duplicate-stdio", "duplicate-gis", "unknown-root", "unknown-row",
    "foreign-package", "crossed-gis-schema", "zero-pack-schema-hash", "foreign-version", "invalid-utf8", "oversize", "cancelled",
  ]);
  for (const row of fixture.cases) assert.deepEqual(Object.keys(row), ["change", "accepted", "schemaAccepted"]);
  assert.equal(new Set(fixture.cases.map((row: any) => row.change)).size, fixture.cases.length);
  const sourcePaths = {
    stdio: "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json",
    gis: "🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🔣️.json",
  };
  const originals = { stdio: JSON.parse(readFileSync(join(repoRoot, sourcePaths.stdio), "utf8")), gis: JSON.parse(readFileSync(join(repoRoot, sourcePaths.gis), "utf8")) };
  const schemas = { stdio: hubSchemaExport(repoRoot, "schema://hub.compositions.stdio.catalog/NativeCodecFactories"), gis: hubSchemaExport(repoRoot, "schema://hub.compositions.gis/GisNativeCodecManifestV1") };
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"));
  mkdirSync(artifactRoot, { recursive: true });
  const evidence = mkdtempSync(join(artifactRoot, "codec-source-"));
  let expected: string | undefined, generation: string | undefined;
  const profile = JSON.parse(readFileSync(join(root, "../🧬️stdio-gis-bootstrap/🔣️.json"), "utf8")).profile;
  for (const test of fixture.cases) {
    const input = structuredClone(originals);
    if (test.change === "permuted") {
      input.stdio.receipts.reverse();
      input.gis.receipts.reverse();
    }
    if (test.change === "invalid-hash") input.gis.receipts[0].protocolSha256 = "zz";
    if (test.change === "zero-hash") input.gis.receipts[0].protocolSha256 = "00".repeat(32);
    if (test.change === "duplicate-stdio") input.stdio.receipts[1] = structuredClone(input.stdio.receipts[0]);
    if (test.change === "duplicate-gis") input.gis.receipts[1] = structuredClone(input.gis.receipts[0]);
    if (test.change === "unknown-root") input.stdio.extra = true;
    if (test.change === "unknown-row") input.gis.receipts[0].extra = true;
    if (test.change === "foreign-package") input.stdio.package_id = "semio:foreign";
    if (test.change === "crossed-gis-schema") input.gis.receipts[0].schema = "gis.terrain";
    if (test.change === "zero-pack-schema-hash") input.gis.receipts[0].packSchemaHash = "00".repeat(32);
    if (test.change === "foreign-version") input.gis.packageVersion = "99.0.0";
    assert.equal(Boolean(schemas.stdio(input.stdio) && schemas.gis(input.gis)), test.schemaAccepted, test.change);
    const testRoot = join(evidence, test.change);
    for (const plugin of ["stdio", "gis"] as const) {
      const path = join(testRoot, sourcePaths[plugin]);
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, JSON.stringify(input[plugin]));
    }
    if (test.change === "invalid-utf8") writeFileSync(join(testRoot, sourcePaths.gis), Buffer.from([0xff]));
    if (test.change === "oversize") truncateSync(join(testRoot, sourcePaths.gis), fixture.maximumSourceBytes + 1);
    let captured = false;
    const check = (stage?: string) => {
      if (test.change === "cancelled") throw new Error("codec capture cancelled");
      if (test.change === "replaced-source" && stage === "project-codecs" && !captured) {
        captured = true;
        for (const plugin of ["stdio", "gis"] as const) writeFileSync(join(testRoot, sourcePaths[plugin]), "replaced source");
      }
    };
    if (!test.accepted) {
      assert.throws(() => captureTrustedBootstrapCodecsV1(testRoot, check), /codec|build input|UTF-8|encoded data/);
      continue;
    }
    const result = captureTrustedBootstrapCodecsV1(testRoot, check);
    assert(Object.isFrozen(result) && Object.isFrozen(result.codecs));
    for (const rows of Object.values(result.codecs)) {
      assert(Object.isFrozen(rows));
      for (const row of rows) assert(Object.isFrozen(row));
    }
    const canonical = JSON.stringify(result);
    const bytes = trustedBootstrapProfileEncoding(profile, result.codecs);
    const digest = await webSha256Hex(bytes);
    assert.equal(createHash("sha256").update(bytes).digest("hex"), digest);
    if (expected === undefined) {
      expected = canonical;
      generation = digest;
    }
    assert.equal(canonical, expected, test.change);
    assert.equal(digest, generation, test.change);
    if (test.change === "replaced-source") assert(captured);
  }
  console.log(`trusted-codec-source: AJV=3 WebCrypto=1 cases=${fixture.cases.length} evidence=${evidence}`);
}

/** 🧪️ Exercises the actual pre-publication file fence against independent digest oracles. */
export async function proveTrustedGenerationStageFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { symlinkSync, truncateSync, writeFileSync } = await import("node:fs");
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧱️generation-stage");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateProtocol = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleExecutionProtocolV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "componentHex", "descriptors", "executionProtocol", "cases"]);
  assert.equal(fixture.schema, "semio.hub.generation-stage.v1");
  assert.equal(fixture.componentHex, "0061736d0d000100");
  assert.deepEqual(fixture.descriptors, {
    gis: { packageId: "semio:gis", manifest: { pluginId: "gis", version: "0.1.0", dependencies: [{ pluginId: "stdio", version: "=0.1.0" }] }, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } },
    stdio: { packageId: "semio:stdio", manifest: { pluginId: "stdio", version: "0.1.0", dependencies: [] }, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } },
  });
  assert.deepEqual(fixture.executionProtocol, { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 });
  for (const protocol of [fixture.executionProtocol, fixture.descriptors.gis.executionProtocol, fixture.descriptors.stdio.executionProtocol]) assert(validateProtocol(protocol), "generation-stage execution protocol must satisfy the scope-owned contract");
  assert.deepEqual(fixture.cases.map((row: any) => row.change), [
    "none", "component", "descriptor", "bundle", "extra-file", "missing-file", "symlink-file", "symlink-directory", "post-read-component",
    "oversize-component", "cancelled", "receipt-path", "actor", "actor-source", "actor-path", "missing-actor", "oversize-actor", "post-read-actor",
    "actor-symlink", "missing-protocol", "unsupported-protocol", "receipt-protocol", "missing-dependency", "receipt-dependency", "bundle-dependency",
    "wrong-package", "nonexact-dependency", "duplicate-dependency", "plugin-module-file", "plugin-module-manifest", "plugin-module-missing-file",
    "plugin-module-extra-file", "post-read-plugin-module-file",
  ]);
  assert.deepEqual(fixture.cases.filter((row: any) => row.accepted).map((row: any) => row.change), ["none", "receipt-path"]);
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"));
  mkdirSync(artifactRoot, { recursive: true });
  const evidence = mkdtempSync(join(artifactRoot, "generation-stage-"));
  const component = Buffer.from(fixture.componentHex, "hex");
  const identity = async (bytes: Uint8Array) => ({ byteLength: bytes.byteLength, sha256: await webSha256Hex(bytes) });
  const actorFixture = JSON.parse(readFileSync(join(root, "../🌐️browser-actor/🔣️.json"), "utf8"));
  const { packages, descriptors } = trustedBootstrapStageFixturePackages(fixture, actorFixture.closed, component);
  const receipts = new Map(packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)]));
  const moduleBlob = (stage: string, plugin: string, suffix: string) => {
    const manifest = JSON.parse(readFileSync(join(stage, "packages", plugin, "plugin-module.json"), "utf8"));
    return join(stage, "plugin-modules", manifest.files.find((file: any) => file.path.endsWith(suffix)).sha256);
  };
  for (const record of packages) {
    assert.deepEqual(record.component, await identity(component));
    assert.deepEqual(record.descriptor, await identity(descriptors.get(record.pluginId)!));
  }
  for (const test of fixture.cases) {
    const caseReceipts = structuredClone(receipts);
    const casePackages = structuredClone(packages);
    const caseDescriptors = new Map(descriptors);
    const stage = join(evidence, test.change);
    for (const plugin of ["gis", "stdio"]) {
      mkdirSync(join(stage, "packages", plugin), { recursive: true });
      writeFileSync(join(stage, "packages", plugin, "component.wasm"), component);
      writeFileSync(join(stage, "packages", plugin, "descriptor.semio"), descriptors.get(plugin)!);
    }
    mkdirSync(join(stage, "packages/gis/browser"));
    const actorPath = join(stage, "packages/gis/browser/closed-actor.mjs");
    writeFileSync(actorPath, Buffer.from(actorFixture.bodyHex, "hex"));
    const componentPath = join(stage, "packages/gis/component.wasm");
    if (test.change === "component") writeFileSync(componentPath, "xbc");
    if (test.change === "descriptor") writeFileSync(join(stage, "packages/stdio/descriptor.semio"), "xef");
    if (test.change === "extra-file") writeFileSync(join(stage, "extra"), "extra");
    if (test.change === "missing-file") rmSync(componentPath);
    if (test.change === "symlink-file") {
      rmSync(componentPath);
      symlinkSync(join(stage, process.platform === "win32" ? "packages/stdio" : "packages/stdio/component.wasm"), componentPath, process.platform === "win32" ? "junction" : "file");
    }
    if (test.change === "symlink-directory") {
      renameSync(join(stage, "packages/gis"), join(evidence, "external-gis"));
      symlinkSync(join(evidence, "external-gis"), join(stage, "packages/gis"), "junction");
    }
    if (test.change === "oversize-component") truncateSync(componentPath, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES + 1);
    if (test.change === "receipt-path") Object.assign(caseReceipts.get("gis")!.component, { path: "untrusted-receipt-cannot-select-path" });
    if (test.change === "receipt-protocol") Object.assign(caseReceipts.get("stdio")!, { executionProtocol: { appChannelVersion: 13 } });
    if (test.change === "receipt-dependency") Object.assign(caseReceipts.get("gis")!, { dependencies: [] });
    if (test.change === "bundle-dependency") casePackages[0].dependencies = [];
    if (["missing-dependency", "wrong-package", "nonexact-dependency", "duplicate-dependency"].includes(test.change)) {
      const changed = structuredClone(fixture.descriptors.gis);
      if (test.change === "missing-dependency") changed.manifest.dependencies = [];
      if (test.change === "wrong-package") changed.packageId = "semio:foreign";
      if (test.change === "nonexact-dependency") changed.manifest.dependencies[0].version = "^0.1.0";
      if (test.change === "duplicate-dependency") changed.manifest.dependencies.push({ ...changed.manifest.dependencies[0] });
      const bytes = Buffer.from(encodePackValue(changed));
      const digest = await identity(bytes);
      caseDescriptors.set("gis", bytes);
      writeFileSync(join(stage, "packages/gis/descriptor.semio"), bytes);
      Object.assign(casePackages[0], { descriptor: digest, receipt: { ...casePackages[0].receipt, semanticDescriptorSha256: digest.sha256 }, browserActor: { ...casePackages[0].browserActor, sourceDescriptorByteSha256: digest.sha256 } });
      caseReceipts.set("gis", trustedBootstrapGenerationReceipt(casePackages[0]));
    }
    if (test.change === "missing-protocol" || test.change === "unsupported-protocol") {
      const changed = Buffer.from(encodePackValue(test.change === "missing-protocol" ? {} : { executionProtocol: { appChannelVersion: 13 } }));
      writeFileSync(join(stage, "packages/stdio/descriptor.semio"), changed);
      caseDescriptors.set("stdio", changed);
      Object.assign(caseReceipts.get("stdio")!, { descriptor: await identity(changed) });
      casePackages[1].descriptor = await identity(changed);
    }
    if (test.change === "actor") writeFileSync(actorPath, "xbc");
    if (test.change === "actor-source") Object.assign(caseReceipts.get("gis")!.browserActor, { sourceDescriptorByteSha256: "42".repeat(32) });
    if (test.change === "actor-path") Object.assign(caseReceipts.get("gis")!.browserActor, { path: "packages/gis/component.wasm" });
    if (test.change === "missing-actor") rmSync(actorPath);
    if (test.change === "oversize-actor") truncateSync(actorPath, 67_108_865);
    if (test.change === "actor-symlink") {
      rmSync(actorPath);
      symlinkSync(join(stage, process.platform === "win32" ? "packages/stdio" : "packages/stdio/component.wasm"), actorPath, process.platform === "win32" ? "junction" : "file");
    }
    for (const [index, plugin] of ["gis", "stdio"].entries()) {
      if (!existsSync(join(stage, "packages", plugin))) continue;
      const module = trustedBootstrapFixturePluginModule(casePackages[index], casePackages[index].component.sha256, caseDescriptors.get(plugin)!, casePackages[index].receipt);
      trustedBootstrapWriteFixturePluginModule(stage, module);
      casePackages[index].pluginModule = module.record;
      caseReceipts.set(plugin, { ...caseReceipts.get(plugin)!, pluginModule: module.record });
    }
    if (test.change === "plugin-module-file") writeFileSync(moduleBlob(stage, "gis", "bridge.js"), "export {};\n");
    if (test.change === "plugin-module-manifest") writeFileSync(join(stage, "packages/stdio/plugin-module.json"), "{}\n");
    if (test.change === "plugin-module-missing-file") rmSync(moduleBlob(stage, "stdio", "io.js"));
    if (test.change === "plugin-module-extra-file") writeFileSync(join(stage, "plugin-modules", "ab".repeat(32)), "unlisted");
    const pluginModuleFile = moduleBlob(stage, "gis", ".json");
    const bundle = Buffer.from(JSON.stringify({ neutral: true, packages: casePackages }) + "\n");
    writeFileSync(join(stage, "trusted-catalog.json"), test.change === "bundle" ? '{"neutral":false}' : bundle);
    let replaced = false,
      identityChecks = 0;
    const check = (phase?: string) => {
      if (identityChecks) throw new Error("generation checkpoint after the final identity fence began");
      if (phase === "identity") identityChecks++;
      if (test.change === "cancelled") throw new Error("generation stage cancelled");
      if (test.change === "post-read-component" && phase === "identity" && !replaced) {
        writeFileSync(componentPath, "xbc");
        replaced = true;
      }
      if (test.change === "post-read-actor" && phase === "identity" && !replaced) {
        writeFileSync(actorPath, "xbc");
        replaced = true;
      }
      if (test.change === "post-read-plugin-module-file" && phase === "identity" && !replaced) {
        writeFileSync(pluginModuleFile, "{}");
        replaced = true;
      }
    };
    if (test.accepted) {
      trustedBootstrapVerifyGeneration(stage, bundle, caseReceipts, check);
      assert.equal(identityChecks, 1);
    } else assert.throws(() => trustedBootstrapVerifyGeneration(stage, bundle, caseReceipts, check), /generation|build input/);
  }
  console.log(`trusted-generation-stage: scope-export=TrustedBundleExecutionProtocolV1 WebCrypto=1 cases=${fixture.cases.length} evidence=${evidence}`);
}

/** 📤️ Proves the publisher rechecks each receipt-bound leaf after candidate validation. */
export async function proveTrustedPublicationFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { writeFileSync } = await import("node:fs");
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/📤️publication");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  assert.deepEqual(Object.keys(fixture), ["schema", "generationId", "profileId", "cases", "commandCases", "receiptCases"]);
  assert.equal(fixture.schema, "semio.hub.trusted-publication/v1");
  assert.equal(fixture.generationId, "aa".repeat(32));
  assert.equal(fixture.profileId, "publication-fixture");
  assert.deepEqual(fixture.cases, [
    { change: "none", accepted: true }, { change: "component", accepted: false }, { change: "descriptor", accepted: false },
    { change: "actor", accepted: false }, { change: "plugin-module", accepted: false }, { change: "bundle", accepted: false },
  ]);
  const command = JSON.parse(readFileSync(join(root, "📬️command.json"), "utf8"));
  const validateCommand = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogPublicationCommandV1");
  const validateReceipt = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogPublicationReceiptV1");
  const pointerShape = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogCurrentPointerV1");
  assert(validateCommand(command), "publication command fixture must satisfy the scope-owned contract");
  for (const law of fixture.commandCases) {
    const candidate: Record<string, unknown> = { ...command, ...law.set };
    for (const key of law.remove) delete candidate[key];
    assertObservationExpectation(`command ${law.id}`, law.expectation as ObservationExpectation, Boolean(validateCommand(candidate)));
  }
  assert.deepEqual(JSON.parse(canonicalJson(command)), command);
  const transport = JSON.parse(readFileSync(join(root, "📡️transport.json"), "utf8"));
  assert.deepEqual(transport, {
    schema: "semio.hub.trusted-publication-transport/v1",
    command: ["trusted-catalog", "publish"],
    commandBytes: 4096,
    receiptBytes: 4096,
    inputDeadlineMs: 5000,
    publicationDeadlineMs: 30_000,
    outcomes: [
      { outcome: "durable", exit: "success", activate: true },
      { outcome: "replaced-unconfirmed", exit: "failure", activate: false },
      { outcome: "missing-or-invalid", exit: "failure", activate: false },
    ],
    rejectedArguments: [["trusted-catalog"], ["publish"], ["trusted-catalog", "publish", "--force"], ["trusted-catalog", "activate"]],
    processCases: [
      { mode: "durable", result: "durable" }, { mode: "unconfirmed", result: "unconfirmed" }, { mode: "unconfirmed-zero", result: "indeterminate" },
      { mode: "unconfirmed-two", result: "indeterminate" }, { mode: "overflow", result: "indeterminate" }, { mode: "signal", result: "indeterminate" },
      { mode: "empty", result: "indeterminate" }, { mode: "identity", result: "indeterminate" }, { mode: "durable-one", result: "indeterminate" },
      { mode: "timeout", result: "indeterminate" }, { mode: "cancel", result: "indeterminate" }, { mode: "durable-after-cancel", result: "durable" },
    ],
  });
  const pointerBytes = new TextEncoder().encode(JSON.stringify({ profileId: command.profileId, generationId: command.generationId, bundleSha256: command.bundleSha256, publicationRevision: "1" }) + "\n");
  const currentSha256 = Buffer.from(await webcrypto.subtle.digest("SHA-256", pointerBytes)).toString("hex");
  const receiptFixture = { schema: "semio.hub.trusted-catalog-publication-receipt/v1", requestId: command.requestId, profileId: command.profileId, generationId: command.generationId, bundleSha256: command.bundleSha256, publicationRevision: "1", currentSha256, outcome: "durable" };
  for (const row of transport.outcomes) {
    if (row.outcome === "missing-or-invalid") { assert.throws(() => trustedBootstrapPublicationReceipt(new Uint8Array(), command, "1")); continue; }
    const receipt = { ...receiptFixture, outcome: row.outcome };
    assert(validateReceipt(receipt), `${row.outcome} receipt must satisfy the scope-owned contract`);
    const result = trustedBootstrapPublicationReceipt(Buffer.from(JSON.stringify(receipt) + "\n"), command, "1");
    assert.equal(result.outcome === "durable", row.activate);
    assert.equal(result.currentSha256, currentSha256);
  }
  for (const law of fixture.receiptCases) {
    const candidate = { ...receiptFixture, ...law.set };
    assertObservationExpectation(`receipt ${law.id}`, law.expectation as ObservationExpectation, Boolean(validateReceipt(candidate)));
    const decode = (): unknown => trustedBootstrapPublicationReceipt(Buffer.from(JSON.stringify(candidate) + "\n"), command, "1");
    if (law.expectation.result === "accepted") decode();
    else assert.throws(decode, `receipt ${law.id} must be refused by the production decoder`);
  }
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"));
  const evidence = mkdtempSync(join(artifactRoot, "publication-fence-"));
  const fixtureChild = `
    const fs = require("node:fs"), crypto = require("node:crypto"), path = require("node:path");
    const command = JSON.parse(fs.readFileSync(0, "utf8"));
    const mode = process.argv[1];
    fs.writeFileSync(path.join(process.env.OS_HUB_DATA, "child-ready.json"), JSON.stringify({ pid: process.pid, mode }));
    if (mode === "timeout" || mode === "cancel") { setInterval(() => {}, 1000); }
    else if (mode !== "empty") {
      const pointer = { profileId: command.profileId, generationId: command.generationId, bundleSha256: command.bundleSha256, publicationRevision: "1" };
      const outcome = ["unconfirmed", "unconfirmed-zero", "unconfirmed-two", "overflow", "signal"].includes(mode) ? "replaced-unconfirmed" : "durable";
      const receipt = { schema: "semio.hub.trusted-catalog-publication-receipt/v1", requestId: command.requestId, ...pointer, currentSha256: crypto.createHash("sha256").update(JSON.stringify(pointer) + "\\n").digest("hex"), outcome };
      if (mode === "identity") receipt.requestId = "ff".repeat(16);
      process.exitCode = mode === "unconfirmed-two" ? 2 : ["unconfirmed", "durable-one", "overflow", "signal"].includes(mode) ? 1 : 0;
      process.stdout.write(JSON.stringify(receipt) + "\\n", () => {
        if (mode === "overflow") process.stdout.write(" ".repeat(4097));
        if (mode === "signal") process.kill(process.pid, "SIGKILL");
      });
    }
  `;
  for (const row of transport.processCases) {
    const caseRoot = join(evidence, `child-${row.mode}`);
    mkdirSync(caseRoot);
    const started = Date.now(), input = Buffer.from(JSON.stringify(command) + "\n");
    const result = await runTrustedPublicationChild(process.execPath, ["-e", fixtureChild, row.mode], input, caseRoot, caseRoot, {
      cancelled: () => row.mode === "cancel" && existsSync(join(caseRoot, "child-ready.json")),
      remainingMs: () => Math.max(0, (row.mode === "timeout" ? 500 : 5000) - (Date.now() - started)),
    });
    let actual = "indeterminate";
    try { trustedPublicationChildReceipt(result, command, "1", caseRoot); actual = "durable"; }
    catch (error) { if (error instanceof TrustedBootstrapPublicationUnconfirmed) actual = "unconfirmed"; }
    assert.equal(actual, row.result, row.mode);
    assert(result.status !== null || result.signal !== null, `${row.mode}: child not reaped`);
    assert(statSync(join(caseRoot, "publication.stdout.json")).size <= 4096);
    assert(statSync(join(caseRoot, "publication.stderr.txt")).size <= 65536);
    if (row.mode === "cancel") { assert(existsSync(join(caseRoot, "child-ready.json"))); assert.equal(result.failed, "publication cancelled"); }
    if (row.mode === "timeout") assert.equal(result.failed, "publication timed out");
    if (row.mode === "overflow") assert.equal(result.failed, "publication stdout exceeded its receipt bound");
    result.bytes.fill(0); input.fill(0);
  }
  const ownerFixture = JSON.parse(readFileSync(join(root, "🔒️owner.json"), "utf8"));
  assert.deepEqual(ownerFixture, {
    schema: "semio.hub.trusted-publication-owner/v1",
    before: "previous-selection\n",
    after: "next-selection\n",
    nonce: "0123456789abcdef0123456789abcdef",
    lock: { first: "acquired", competing: "contended", afterDrop: "acquired" },
    replacement: { selected: "next-selection\n", foreignRoot: "foreign-selection\n" },
    process: { holder: "acquired", competing: "contended", afterCrash: "acquired", current: "unchanged" },
    cases: ["same-directory-replace", "cancel-before-replace", "duplicate-nonce-refused", "root-replacement-isolated", "lock-link-refused", "lock-directory-refused", "process-crash-releases-lock"],
  });
  const ownerOracle = join(evidence, "owner-oracle");
  mkdirSync(ownerOracle);
  writeFileSync(join(ownerOracle, "current.json"), ownerFixture.before);
  writeFileSync(join(ownerOracle, "next.json"), ownerFixture.after, { flag: "wx" });
  renameSync(join(ownerOracle, "next.json"), join(ownerOracle, "current.json"));
  assert.equal(readFileSync(join(ownerOracle, "current.json"), "utf8"), ownerFixture.replacement.selected);
  const cas = JSON.parse(readFileSync(join(root, "🔄️cas.json"), "utf8"));
  assert.deepEqual(cas, {
    schema: "semio.hub.trusted-publication-cas/v1",
    cases: [
      { id: "initial", currentRevision: null, nextRevision: "1", accepted: true },
      { id: "next", currentRevision: "1", nextRevision: "2", accepted: true },
      { id: "stale", currentRevision: "2", expectedRevision: "1", accepted: false },
      { id: "aba", currentRevision: "3", expectedRevision: "1", sameGeneration: true, accepted: false },
      { id: "maximum", currentRevision: "18446744073709551615", accepted: false },
      { id: "zero", currentRevision: "0", accepted: false },
      { id: "leading-zero", currentRevision: "01", accepted: false },
      { id: "overflow", currentRevision: "18446744073709551616", accepted: false },
    ],
    publicationOutcomes: ["durable", "replaced-unconfirmed"],
  });
  const pointerFor = (publicationRevision: string) => ({ profileId: fixture.profileId, generationId: fixture.generationId, bundleSha256: "42".repeat(32), publicationRevision });
  for (const row of cas.cases) {
    const casRoot = join(evidence, `cas-${row.id}`);
    mkdirSync(join(casRoot, "trusted-catalog"), { recursive: true });
    if (row.currentRevision !== null) writeFileSync(join(casRoot, "trusted-catalog/current.json"), JSON.stringify(pointerFor(row.currentRevision)) + "\n");
    let accepted = false;
    try {
      if (row.currentRevision !== null) assert(pointerShape(pointerFor(row.currentRevision)));
      const current = trustedBootstrapReadCurrentPointer(casRoot);
      if (row.expectedRevision !== undefined) {
        const bytes = Buffer.from(JSON.stringify(pointerFor(row.expectedRevision)) + "\n");
        assert.equal(current?.sha256, await webSha256Hex(bytes));
      }
      const next = trustedBootstrapPublicationRevision((BigInt(current?.pointer.publicationRevision ?? "0") + 1n).toString());
      if (row.nextRevision !== undefined) assert.equal(next, row.nextRevision);
      accepted = true;
    } catch {}
    assert.equal(accepted, row.accepted, row.id);
  }
  const stageFixture = JSON.parse(readFileSync(join(root, "../🧱️generation-stage/🔣️.json"), "utf8"));
  const component = Buffer.from("0061736d0d000100", "hex");
  const digest = async (bytes: Uint8Array) => await webSha256Hex(bytes);
  const actorFixture = JSON.parse(readFileSync(join(root, "../🌐️browser-actor/🔣️.json"), "utf8"));
  const { packages, descriptors, modules } = trustedBootstrapStageFixturePackages(stageFixture, actorFixture.closed, component);
  for (const record of packages) {
    assert.equal(record.component.sha256, await digest(component));
    assert.equal(record.descriptor.sha256, await digest(descriptors.get(record.pluginId)!));
    assert.equal(record.pluginModule.sha256, await digest(modules.get(record.pluginId)!.manifest));
  }
  const bundle = Buffer.from(JSON.stringify({ schemaVersion: 4, profiles: [{ id: fixture.profileId, generationId: fixture.generationId }], packages }) + "\n");
  for (const row of fixture.cases) {
    const dataRoot = join(evidence, row.change), trustedRoot = join(dataRoot, "trusted-catalog"), generationRoot = join(trustedRoot, "generations", fixture.generationId);
    mkdirSync(join(generationRoot, "packages/gis/browser"), { recursive: true });
    mkdirSync(join(generationRoot, "packages/stdio"));
    for (const plugin of ["gis", "stdio"]) {
      writeFileSync(join(generationRoot, "packages", plugin, "component.wasm"), component);
      writeFileSync(join(generationRoot, "packages", plugin, "descriptor.semio"), descriptors.get(plugin)!);
      trustedBootstrapWriteFixturePluginModule(generationRoot, modules.get(plugin)!);
    }
    const actorPath = join(generationRoot, "packages/gis/browser/closed-actor.mjs"), bundlePath = join(generationRoot, "trusted-catalog.json");
    writeFileSync(actorPath, Buffer.from(actorFixture.bodyHex, "hex")); writeFileSync(bundlePath, bundle);
    const receipt = { profileId: fixture.profileId, generationId: fixture.generationId, bundlePath, bundleSha256: await digest(bundle) };
    trustedBootstrapVerifyGeneration(generationRoot, bundle, new Map(packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), () => {});
    const pointerPath = join(trustedRoot, "current.json"), previous = Buffer.from(JSON.stringify({ profileId: fixture.profileId, generationId: "dd".repeat(32), bundleSha256: "ee".repeat(32), publicationRevision: "1" }) + "\n");
    writeFileSync(pointerPath, previous);
    const changes: Readonly<Record<string, string>> = { component: join(generationRoot, "packages/gis/component.wasm"), descriptor: join(generationRoot, "packages/stdio/descriptor.semio"), actor: actorPath, "plugin-module": join(generationRoot, "packages/stdio/plugin-module.json"), bundle: bundlePath };
    const changed = changes[row.change];
    if (changed) writeFileSync(changed, "substituted-after-candidate-proof");
    if (row.accepted) {
      trustedBootstrapVerifyPublication(dataRoot, receipt);
    } else {
      assert.throws(() => trustedBootstrapVerifyPublication(dataRoot, receipt), /generation|bundle|rotation/);
    }
    assert.deepEqual(readFileSync(pointerPath), previous);
  }
  console.log(`trusted publication final-byte fence: scope-exports=3 WebCrypto=1 Node-replace-oracle=1 cases=${fixture.cases.length} command-cases=${fixture.commandCases.length} receipt-cases=${fixture.receiptCases.length} revision-cases=${cas.cases.length} transport-outcomes=3 real-child-cases=${transport.processCases.length}; native-owner/CAS remain unqualified; evidence=${evidence}`);
}
