/** 🚀️ The trusted stdio gis bootstrap fixture proof and the trusted compiled dependencies fixture proof. */
import { type ScriptInvocation } from "@semio-tech/process/routing";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { blake3Hex } from "@semio-tech/framework/hash";
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { componentReceiptBindsV1, componentReceiptValueV1 } from "@semio-tech/framework-plugin/describe/component-receipt";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { hubSchemaExport } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { readHubScriptModule } from "../../../🧪️testing/📜️script-sources/🟦️.ts";
import { projectTrustedBootstrapCodecsV1 } from "../../../🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapClosureEncoding, type TrustedBootstrapCodec, trustedBootstrapDependencies, trustedBootstrapDependencyEncoding, trustedBootstrapDescriptorClaims, trustedBootstrapIdentity, trustedBootstrapPlanGenerationOutcome, trustedBootstrapProfileEncoding, trustedBootstrapResolveClosure, trustedBootstrapResolveDependencies } from "../../../🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims/🟦️.ts";
import { proveTrustedRotationSourceFixture } from "../../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🗺️gis-proofs-and-staging/🟦️.ts";
import { webSha256Hex } from "../../../🚀️local-relay/🛰️servers/🟦️.ts";
import { moduleRustSource } from "../../../🧪️testing/📖️source-readers/🟦️.ts";
import { proveTrustedBootstrapCodecCaptureFixture, proveTrustedGenerationStageFixture, proveTrustedPublicationFixture } from "../../📮️trusted-publication-fixtures/🟦️.ts";
import { proveDocumentBrowserActorIdentityFixture } from "../🪪️browser-actor-and-opened-root-fixtures/🟦️.ts";
import { proveTrustedGisMapCollaborationContractFixture, proveTrustedGisPublicationFixture, proveTrustedGisRetainedBrowserFixture } from "../🗺️gis-retained-collaboration-and-publication-fixtures/🟦️.ts";

export async function proveTrustedStdioGisBootstrapFixture(invocation: ScriptInvocation, repoRoot: string): Promise<void> {
  await proveTrustedCompiledDependenciesFixture(repoRoot);
  await proveDocumentBrowserActorIdentityFixture(repoRoot);
  const { createFreshComponentTests } = await import("@semio-tech/framework-plugin/describe/testing/fresh-component");
  const { testFreshComponentStagingV1, testFreshComponentSourceEpochV1, testFreshComponentProcessV1 } = createFreshComponentTests(invocation);
  await testFreshComponentStagingV1(repoRoot);
  await testFreshComponentSourceEpochV1(repoRoot);
  await testFreshComponentProcessV1(repoRoot);
  await proveTrustedGenerationStageFixture(repoRoot);
  await proveTrustedPublicationFixture(repoRoot);
  await proveTrustedBootstrapCodecCaptureFixture(repoRoot);
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const bootstrapAssert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const validateIdentity = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleIdentityV1");
  const validateActor = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleBrowserActorV1");
  const validateProtocol = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleExecutionProtocolV1");
  const validateDialect = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleParentDialectV1");
  const validateGrant = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleGrantV1");
  bootstrapAssert.deepEqual(Object.keys(fixture), ["schema", "profile", "sources", "limits", "framing", "cancellationStages", "hostile", "publication", "rotation"]);
  bootstrapAssert.equal(fixture.schema, "semio.hub.trusted-stdio-gis-bootstrap/v1");
  bootstrapAssert.equal(fixture.profile.id, "local-stdio-gis-open-v1");
  bootstrapAssert.deepEqual(fixture.limits, { descriptorBytes: 4_194_304, descriptorBytesPlusOne: 4_194_305, componentBytes: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, ioChunkBytes: 65_536, packageCount: 2, codecCount: fixture.profile.packages.reduce((total: number, row: { codecCount: number }) => total + row.codecCount, 0), openTargetCount: 1 });
  bootstrapAssert.deepEqual(fixture.framing, { closureDomain: "semio/hub/trusted-profile-selected-closure/v1\u0000", generationDomain: "semio/hub/trusted-profile-generation/v2\u0000", integerEndian: "big", lengthBytes: 8 });
  bootstrapAssert.deepEqual(fixture.cancellationStages, ["build", "inspect-wit", "emit-descriptor", "stage-component", "stage-descriptor", "hash", "pre-publication"]);
  bootstrapAssert.deepEqual(fixture.hostile, [
    "stale-stdio-component", "stale-stdio-descriptor", "stale-stdio-codec", "wrong-stdio-version", "wrong-gis-version", "missing-map", "missing-terrain",
    "duplicate-terrain", "terrain-target", "non-wasm", "readonly-editor", "mixed-package", "closure-order", "path-escape", "path-collision",
    "descriptor-max-plus-one", "cancel-before-publication", "candidate-crash", "stale-plan-generation",
  ]);
  bootstrapAssert.deepEqual(Object.keys(fixture.sources), ["stdioReceipts", "gisReceipts"]);
  bootstrapAssert.deepEqual(fixture.rotation.sourceCases.map((row: any) => row.change), ["none", "replaced-bundle", "wrong-generation", "wrong-profile", "leaf-replaced-during-read", "cancel-before-read"]);
  bootstrapAssert.equal(fixture.rotation.writeCases.length, 7);
  bootstrapAssert.equal(new Set(fixture.rotation.writeCases.map((row: any) => row.id)).size, 7);
  for (const row of fixture.rotation.writeCases) bootstrapAssert(["new", "old", "absent"].includes(row.retained) && Number.isInteger(row.byteLength) && row.byteLength >= 1 && row.byteLength <= 67_108_864, row.id);
  for (const identity of fixture.profile.selectedClosure) bootstrapAssert(validateIdentity(identity), "bootstrap closure identity must satisfy the scope-owned contract");
  const validateReceipt = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleReceiptV1");
  for (const record of fixture.profile.packages) {
    bootstrapAssert(validateReceipt(record.receipt) && componentReceiptBindsV1(componentReceiptValueV1(record.receipt), { profile: "release", pluginId: record.pluginId, packageId: record.packageId, wasmSha256: record.componentSha256, semanticDescriptorSha256: record.descriptorSha256 }), `${record.pluginId} receipt must be a shipped receipt binding its component to its descriptor`);
    bootstrapAssert(validateActor(record.browserActor), `${record.pluginId} browser actor must satisfy the scope-owned contract`);
    bootstrapAssert(validateProtocol(record.executionProtocol), `${record.pluginId} execution protocol must satisfy the scope-owned contract`);
    for (const dependency of record.dependencies) bootstrapAssert(validateIdentity(dependency), `${record.pluginId} dependency must satisfy the scope-owned contract`);
  }
  bootstrapAssert.equal(fixture.profile.openTargets.length, fixture.limits.openTargetCount);
  for (const declared of fixture.profile.openTargets) bootstrapAssert(validateDialect(declared.parentDialect) && validateGrant(declared.grant), "bootstrap open target dialect/grant must satisfy the scope-owned contract");
  await proveTrustedGisPublicationFixture(repoRoot, fixture.publication);
  await proveTrustedGisRetainedBrowserFixture(repoRoot);
  await proveTrustedGisMapCollaborationContractFixture(repoRoot);
  const stdio = JSON.parse(readFileSync(join(repoRoot, fixture.sources.stdioReceipts), "utf8"));
  const gis = JSON.parse(readFileSync(join(repoRoot, fixture.sources.gisReceipts), "utf8"));
  const codecs = projectTrustedBootstrapCodecsV1(stdio, gis).codecs;
  const profile = fixture.profile;
  const unique = (rows: readonly TrustedBootstrapCodec[]): boolean =>
    rows.length === new Set(rows.map((row) => JSON.stringify([row.artifactKind, row.artifactSchema, row.packSchemaHash]))).size && rows.every((row) => /^(?!0{64}$)[0-9a-f]{64}$/u.test(row.packSchemaHash));
  if (stdio.plugin_id !== "stdio" || stdio.package_id !== "semio:stdio" || codecs.stdio.length !== profile.packages[1]?.codecCount || !unique(codecs.stdio)) throw new Error(`stdio bootstrap closure is not the profile's exact ${profile.packages[1]?.codecCount} codecs`);
  if (
    gis.pluginId !== "gis" ||
    gis.packageId !== "semio:gis" ||
    gis.packageVersion !== profile.selectedClosure[0].version ||
    codecs.gis.length !== 2 ||
    !unique(codecs.gis) ||
    !codecs.gis.some((row) => row.artifactKind === "s.gis.gismap") ||
    !codecs.gis.some((row) => row.artifactKind === "s.gis.gisterrain")
  )
    throw new Error("GIS bootstrap closure is not exact Map plus Terrain");
  if (
    profile.packages.length !== 2 ||
    profile.packages[0].pluginId !== "gis" ||
    profile.packages[0].codecCount !== 2 ||
    profile.packages[0].targetCount !== 1 ||
    profile.packages[1].pluginId !== "stdio" ||
    profile.packages[1].codecCount !== codecs.stdio.length ||
    profile.packages[1].targetCount !== 0
  )
    throw new Error("trusted bootstrap package/target cardinality drifted");
  const closure = trustedBootstrapClosureEncoding(profile);
  const closureNode = createHash("sha256").update(closure).digest("hex");
  const closureWeb = await webSha256Hex(closure);
  if (closureNode !== profile.selectedClosureSha256 || closureWeb !== closureNode) throw new Error("trusted bootstrap selected closure digest mismatch");
  const generationBytes = trustedBootstrapProfileEncoding(profile, codecs);
  const generationNode = createHash("sha256").update(generationBytes).digest("hex");
  const generationWeb = await webSha256Hex(generationBytes);
  if (generationNode !== profile.generationId || generationWeb !== generationNode) throw new Error(`trusted bootstrap full generation mismatch: ${generationNode}`);
  const changed = structuredClone(profile);
  changed.packages[1].componentSha256 = "31".repeat(32);
  const changedComponent = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  changed.packages[1].componentSha256 = profile.packages[1].componentSha256;
  changed.packages[1].descriptorSha256 = "32".repeat(32);
  const changedDescriptor = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  const changedCodecs = { ...codecs, stdio: codecs.stdio.map((row, index) => (index === 0 ? { ...row, packSchemaHash: "33".repeat(32) } : row)) };
  const changedCodec = createHash("sha256").update(trustedBootstrapProfileEncoding(profile, changedCodecs)).digest("hex");
  changed.packages[1].descriptorSha256 = profile.packages[1].descriptorSha256;
  changed.packages[1].pluginModule = { ...profile.packages[1].pluginModule, blake3: "38".repeat(32) };
  const changedPluginModule = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  changed.packages[1].pluginModule = profile.packages[1].pluginModule;
  changed.packages[1].receipt = { ...profile.packages[1].receipt, provenanceSha256: "39".repeat(32) };
  const changedReceipt = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  if ([changedComponent, changedDescriptor, changedCodec, changedPluginModule, changedReceipt].some((digest) => digest === generationNode)) throw new Error("trusted bootstrap generation omits zero-target stdio authority");
  const target = profile.openTargets[0];
  if (
    target.pluginId !== "gis" ||
    target.artifactKind !== "s.gis.gismap" ||
    target.surfaceId !== "s.gis.gismap@1/*#editor" ||
    target.appId !== target.surfaceId ||
    target.windowKindId !== "gis2d-main" ||
    target.role !== "editor" ||
    target.rendererTarget !== "wasm" ||
    !target.grant.read ||
    !target.grant.write ||
    !target.grant.observe ||
    codecs.gis.some((row: TrustedBootstrapCodec) => row.artifactKind === "s.gis.gisterrain" && row.artifactKind === target.artifactKind)
  )
    throw new Error("trusted bootstrap target is not the sole writable GIS Map editor surface");
  const relative = (value: string): boolean => !/^[/\\]|^[A-Za-z]:[/\\]|(?:^|[/\\])\.\.(?:[/\\]|$)/u.test(value);
  if (!relative("packages/gis/component.wasm") || relative("../component.wasm") || relative("C:\\component.wasm")) throw new Error("trusted bootstrap path fence drifted");
  const rotation = fixture.rotation;
  if (
    fixture.limits.descriptorBytes + 1 !== fixture.limits.descriptorBytesPlusOne ||
    rotation.initialGenerationId !== generationNode ||
    rotation.currentAfterFailedCandidate !== rotation.initialGenerationId ||
    rotation.failedCandidateGenerationId === rotation.initialGenerationId ||
    rotation.nextGenerationId === rotation.initialGenerationId ||
    trustedBootstrapPlanGenerationOutcome(rotation.stalePlan.issuedGenerationId, rotation.stalePlan.observedGenerationId) !== rotation.stalePlan.expected ||
    trustedBootstrapPlanGenerationOutcome(rotation.freshPlan.issuedGenerationId, rotation.freshPlan.observedGenerationId) !== rotation.freshPlan.expected ||
    rotation.stalePlan.issuedGenerationId !== rotation.initialGenerationId ||
    rotation.stalePlan.observedGenerationId !== rotation.nextGenerationId ||
    rotation.freshPlan.issuedGenerationId !== rotation.nextGenerationId ||
    fixture.cancellationStages.length !== 7 ||
    fixture.hostile.length !== 19 ||
    !fixture.hostile.includes("stale-plan-generation")
  )
    throw new Error("trusted bootstrap bounds/cancellation/cross-generation rotation corpus drifted");
  if (blake3Hex(Buffer.from("abc")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("trusted bootstrap first-party BLAKE3 known answer mismatch");
  const descriptor: Record<string, any> = {
    descriptorVersion: 1,
    packageId: "semio:gis",
    role: "plugin",
    manifest: { pluginId: "gis", label: "GIS", version: "0.1.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [], dependencies: [{ pluginId: "stdio", version: "=0.1.0" }], contributions: [] },
    activationEvents: [],
    capabilityRequests: [],
    extensionPoints: [],
    execution: "isolated",
    executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 },
    quotas: {},
    contributions: {},
    assets: [],
  };
  const jsonBytes = Buffer.from(JSON.stringify(descriptor));
  const packBytes = Buffer.from(encodePackValue(descriptor));
  const binding = { profile: "release" as const, pluginId: "gis", packageId: "semio:gis", wasmSha256: "41".repeat(32), semanticDescriptorSha256: createHash("sha256").update(packBytes).digest("hex") };
  const freshPair = (json: Uint8Array, pack: Uint8Array) => {
    const pair = verifySemanticDescriptorPairBytesV1(json, pack);
    if (pair.pluginId !== "gis" || pair.packageId !== "semio:gis" || pair.version !== "0.1.0" || pair.role !== "plugin" || pair.execution !== "isolated") throw new Error("fresh descriptor identity differs");
    return pair;
  };
  const receipt = componentReceiptValueV1({ schema: "semio.plugin.component-receipt/v1", ...binding, profile: "release", target: "wasm32-wasip2", wasmBytes: 4096, toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "43".repeat(32), provenanceSha256: "44".repeat(32) });
  if (freshPair(jsonBytes, packBytes).semanticSha256 !== binding.semanticDescriptorSha256 || !componentReceiptBindsV1(receipt, binding)) throw new Error("fresh semantic descriptor and its receipt do not bind");
  if (componentReceiptBindsV1({ ...receipt, profile: "prototype" }, binding) || componentReceiptBindsV1(receipt, { ...binding, semanticDescriptorSha256: "45".repeat(32) })) throw new Error("fresh receipt law admitted a non-shipped or foreign binding");
  const hostilePairs = [
    [Buffer.from(JSON.stringify({ ...descriptor, manifest: { ...descriptor.manifest, version: "9.9.9" } })), packBytes],
    [jsonBytes, Buffer.concat([packBytes, Buffer.from([0])])],
    [Buffer.from([0xff]), packBytes],
    [Buffer.from(JSON.stringify({ ...descriptor, hashes: { wasmSha256: "41".repeat(32) } })), Buffer.from(encodePackValue({ ...descriptor, hashes: { wasmSha256: "41".repeat(32) } }))],
  ] as const;
  for (const [json, pack] of hostilePairs) {
    let rejected = false;
    try {
      freshPair(json, pack);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error("fresh catalog package verifier admitted a hostile JSON/pack pair");
  }
  await proveTrustedRotationSourceFixture(fixture);
  console.log(
    `trusted-stdio-gis-bootstrap-oracle: packages=2 codecs=${codecs.stdio.length + codecs.gis.length} targets=1 hostile=${fixture.hostile.length} cancellation=${fixture.cancellationStages.length} descriptor-pairs=5 receipt-laws=3 stale-plan=1 scope-exports=5+node+webcrypto+first-party-pack+blake3=1; no materialization or hub activation claim`,
  );
}

/** 🔗️ Exercises receipt-derived dependency claims with Pack, AJV and independent byte framing. */
export async function proveTrustedCompiledDependenciesFixture(repoRoot: string): Promise<void> {
  await (await import("@semio-tech/framework-pack-record-ts/script")).proveWireValueMaterializationFixture(repoRoot);
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateIdentity = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleIdentityV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "publicationFiles", "publicationDirectories", "descriptorPreviewCases", "selectedClosure", "cases", "consumerCatalogCases", "atomicCases", "nativeCases", "rawCases", "encodingCases", "ordering"]);
  assert.equal(fixture.schema, "semio.hub.compiled-stdio-dependencies/v1");
  assert.deepEqual(fixture.publicationFiles, ["component.wasm", "descriptor.semio", "closed-actor.mjs", "plugin-module-gis.json", "stdio-component.wasm", "stdio-descriptor.semio", "plugin-module-stdio.json", "trusted-catalog.json"]);
  assert.deepEqual(fixture.publicationDirectories, ["plugin-modules"]);
  assert.deepEqual(fixture.descriptorPreviewCases, [
    { id: "first-selected-descriptor-invalid", packageIndex: 1, previews: [] },
    { id: "second-selected-descriptor-invalid", packageIndex: 0, previews: [] },
  ]);
  assert.deepEqual(fixture.consumerCatalogCases, [
    { change: "exact", accepted: true }, { change: "missing", accepted: false }, { change: "duplicate", accepted: false },
    { change: "wrong-version", accepted: false }, { change: "foreign-topic-version", accepted: false },
  ]);
  assert.deepEqual(fixture.atomicCases, [
    { change: "stdio-catalog", accepted: false, previews: [] }, { change: "gis-catalog", accepted: false, previews: ["stdio"] },
    { change: "gis-dependency", accepted: false, previews: [] }, { change: "gis-provider", accepted: false, previews: ["stdio"] },
    { change: "gis-trailing-byte", accepted: false, previews: [] }, { change: "gis-duplicate-field", accepted: false, previews: [] },
    { change: "exact", accepted: true, previews: ["stdio", "gis"] },
  ]);
  assert.equal(fixture.selectedClosure.length, 2);
  for (const identity of [...fixture.selectedClosure, ...fixture.ordering.input, ...fixture.encodingCases.flatMap((row: any) => row.identities)]) assert(validateIdentity(identity), "compiled-dependency identity must satisfy the scope-owned contract");
  assert.equal(fixture.cases.length, 25);
  assert.deepEqual([...new Set(fixture.cases.map((row: any) => row.change))], [
    "exact", "selected-order", "missing", "duplicate", "self", "unknown", "any", "caret", "wrong-version", "noncanonical-version", "overflow-version",
    "foreign-package", "duplicate-selected", "missing-selected", "extra-field", "missing-manifest", "oversized-claim", "stdio-dependent", "owner-plugin",
    "owner-package", "owner-version", "selected-version", "missing-protocol", "unsupported-protocol",
  ]);
  assert.equal(new Set(fixture.cases.map((row: any) => `${row.change}/${row.owner}`)).size, fixture.cases.length);
  for (const row of fixture.cases) assert.deepEqual(Object.keys(row).sort(), ["accepted", "change", "owner"]);
  for (const row of fixture.cases) assert(["gis", "stdio"].includes(row.owner) && typeof row.accepted === "boolean", row.change);
  assert.deepEqual(fixture.encodingCases.map((row: any) => row.name), ["gis-direct-dependency-stdio", "stdio-direct-dependencies-empty"]);
  assert.equal(fixture.rawCases.length, 4);
  for (const row of [...fixture.rawCases, ...fixture.encodingCases]) assert.match(row.hex, /^(?:[0-9a-f]{2}){4,1024}$/u);
  assert.equal(fixture.ordering.input.length, 3);
  assert.equal(fixture.ordering.expected.length, 3);
  const kindRoot = join(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures");
  const kind = JSON.parse(readFileSync(join(kindRoot, "🗄️artifact-kind-formats.json"), "utf8"));
  assert.deepEqual(packValueToExactJson(decodePackValue(encodePackValue(kind))), JSON.parse(JSON.stringify(kind)));
  assert.deepEqual(fixture.nativeCases.map((row: any) => row.id), ["exact", "missing", "duplicate", "foreign", "any", "caret", "tilde", "at-least", "wrong-version"]);
  const hubSource = moduleRustSource(join(repoRoot, "🌎️hub", "🧪️tests"));
  const readinessFixture = hubSource.slice(hubSource.indexOf("fn native_openable_stdio_bundle("), hubSource.indexOf("async fn native_openable_stdio_provider_is_the_only_atomic_readiness_transition("));
  assert(readinessFixture.includes('"executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version }'), "readiness fixture must carry its actual descriptor protocol");
  const runnerSource = readHubScriptModule(repoRoot, "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication");
  const fixtureCopy = hubSource.slice(hubSource.indexOf("async fn check_in_process_fixture_emits_verified_gis_ledger_and_catalog"));
  const cliCopy = runnerSource.slice(runnerSource.indexOf("\nasync function proveTrustedPublicationCli("));
  assert.deepEqual(JSON.parse(fixtureCopy.match(/for name in (\[[^\]\n]+\])/u)?.[1] ?? "null"), fixture.publicationFiles, "native process fixture must retain every dependency file");
  assert(cliCopy.includes("withCheckInProcessFixtureV1")&&cliCopy.includes('purpose:"publication"'),"publication CLI must retain the actual complete native fixture through its owned seed reader");
  assert.deepEqual(JSON.parse(fixtureCopy.match(/for directory in (\[[^\]\n]+\])/u)?.[1] ?? "null"), fixture.publicationDirectories, "native process fixture must retain every plugin module file");
  assert.deepEqual(JSON.parse(cliCopy.match(/for \(const directory of (\[[^\]\n]+\])/u)?.[1] ?? "null"), fixture.publicationDirectories, "publication CLI must retain every plugin module file");
  for (const row of fixture.nativeCases) assert.equal(row.dependencies.length === 1 && row.dependencies[0].pluginId === "stdio" && row.dependencies[0].version === "=0.1.0", row.accepted, row.id);
  assert.deepEqual(fixture.cases.map((row: any) => `${row.owner}:${row.change}`), ["gis:exact", "stdio:exact", "gis:selected-order", "gis:missing", "gis:duplicate", "gis:self", "gis:unknown", "gis:any", "gis:caret", "gis:wrong-version", "gis:noncanonical-version", "gis:overflow-version", "gis:foreign-package", "gis:duplicate-selected", "gis:missing-selected", "gis:extra-field", "gis:missing-manifest", "gis:oversized-claim", "stdio:stdio-dependent", "gis:owner-plugin", "gis:owner-package", "gis:owner-version", "gis:selected-version", "gis:missing-protocol", "gis:unsupported-protocol"]);
  assert.deepEqual(fixture.rawCases.map((row: any) => row.name), ["canonical-gis-descriptor", "duplicate-top-level-packageId-earlier-null-overwritten", "duplicate-manifest-earlier-null-overwritten", "canonical-descriptor-with-trailing-zero"]);
  const canonicalRaw = decodePackValue(Buffer.from(fixture.rawCases[0].hex, "hex"));
  for (const row of fixture.rawCases) {
    const bytes = Buffer.from(row.hex, "hex");
    assert.deepEqual(decodePackValue(bytes), canonicalRaw);
    if (row.accepted) assert.deepEqual(trustedBootstrapResolveDependencies(trustedBootstrapDescriptorClaims(bytes), fixture.selectedClosure), [fixture.selectedClosure[1]]);
    else assert.throws(() => trustedBootstrapDescriptorClaims(bytes), /Pack is not canonical/);
  }
  assert.deepEqual(fixture.encodingCases.map((row: any) => row.name), ["gis-direct-dependency-stdio", "stdio-direct-dependencies-empty"]);
  for (const row of fixture.encodingCases) assert.equal(trustedBootstrapDependencyEncoding(row.identities).toString("hex"), row.hex);
  for (const pluginId of [" stdio", "stdio ", String.fromCharCode(0xd800)]) assert.throws(() => trustedBootstrapIdentity({ pluginId, packageId: "semio:stdio", version: "0.1.0" }), /identity/);
  let accepted = 0;
  for (const row of fixture.cases) {
    const selected = structuredClone(fixture.selectedClosure);
    const owner = selected.find((identity: any) => identity.pluginId === row.owner);
    const dependencies: any[] = row.owner === "gis" ? [{ pluginId: "stdio", version: "=0.1.0" }] : [];
    const descriptor: any = { packageId: owner.packageId, manifest: { pluginId: row.owner, version: owner.version, dependencies }, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } };
    if (row.change === "selected-order") selected.reverse();
    if (row.change === "missing") dependencies.length = 0;
    if (row.change === "duplicate") dependencies.push({ ...dependencies[0] });
    if (row.change === "self") dependencies[0].pluginId = "gis";
    if (row.change === "unknown") dependencies[0].pluginId = "unknown";
    if (row.change === "any") dependencies[0].version = "*";
    if (row.change === "caret") dependencies[0].version = "^0.1.0";
    if (row.change === "wrong-version") dependencies[0].version = "=0.2.0";
    if (row.change === "noncanonical-version") dependencies[0].version = "=00.1.0";
    if (row.change === "overflow-version") dependencies[0].version = "=18446744073709551616.1.0";
    if (row.change === "foreign-package") selected[1].packageId = "semio:foreign";
    if (row.change === "duplicate-selected") selected.push({ ...selected[1] });
    if (row.change === "missing-selected") selected.pop();
    if (row.change === "extra-field") dependencies[0].extra = true;
    if (row.change === "missing-manifest") delete descriptor.manifest;
    if (row.change === "oversized-claim") dependencies[0].pluginId = "x".repeat(257);
    if (row.change === "stdio-dependent") dependencies.push({ pluginId: "gis", version: "=0.1.0" });
    if (row.change === "owner-plugin") descriptor.manifest.pluginId = "foreign";
    if (row.change === "owner-package") descriptor.packageId = "semio:foreign";
    if (row.change === "owner-version") descriptor.manifest.version = "0.2.0";
    if (row.change === "selected-version") selected[1].version = "0.2.0";
    if (row.change === "missing-protocol") delete descriptor.executionProtocol;
    if (row.change === "unsupported-protocol") descriptor.executionProtocol.appChannelVersion = DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 + 1;
    const counterpart = fixture.selectedClosure.find((identity: any) => identity.pluginId !== row.owner);
    const counterpartDescriptor = { packageId: counterpart.packageId, manifest: { pluginId: counterpart.pluginId, version: counterpart.version, dependencies: counterpart.pluginId === "gis" ? [{ pluginId: "stdio", version: `=${counterpart.version}` }] : [] }, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } };
    const operation = () => {
      const claims = new Map([[row.owner, trustedBootstrapDescriptorClaims(encodePackValue(descriptor))], [counterpart.pluginId, trustedBootstrapDescriptorClaims(encodePackValue(counterpartDescriptor))]]);
      return trustedBootstrapResolveClosure("local-stdio-gis-open-v1", claims, selected).get(row.owner)!;
    };
    if (!row.accepted) { assert.throws(operation, row.change === "extra-field" ? /keys/ : /dependenc|descriptor|identity|version|manifest|protocol|object/); continue; }
    const actual = operation();
    assert.deepEqual(actual, row.owner === "gis" ? [fixture.selectedClosure[1]] : []);
    assert(Object.isFrozen(actual) && actual.every(Object.isFrozen));
    const independent = new Uint8Array(4 + actual.reduce((size, identity) => size + [identity.pluginId, identity.packageId, identity.version].reduce((sum, field) => sum + 8 + new TextEncoder().encode(field).length, 0), 0));
    const view = new DataView(independent.buffer);
    view.setUint32(0, actual.length, false);
    let offset = 4;
    for (const identity of actual) for (const field of [identity.pluginId, identity.packageId, identity.version]) {
      const bytes = new TextEncoder().encode(field);
      view.setBigUint64(offset, BigInt(bytes.length), false); offset += 8;
      independent.set(bytes, offset); offset += bytes.length;
    }
    assert.deepEqual(trustedBootstrapDependencyEncoding(actual), Buffer.from(independent));
    accepted++;
  }
  assert.deepEqual(trustedBootstrapDependencies(fixture.ordering.input).map((identity) => identity.pluginId), fixture.ordering.expected);
  assert.equal(accepted, 3);
  console.log(`compiled dependency claims: AJV=1 Pack=1 DataView=1 cases=${fixture.cases.length} accepted=${accepted} native-vectors=${fixture.nativeCases.length} raw-Pack=${fixture.rawCases.length} source-derived-goldens=${fixture.encodingCases.length} publication-files=${fixture.publicationFiles.length} UTF8-order=1; native guest agreement remains separate`);
}
