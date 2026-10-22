/** 🎯️ Native openable catalog provider, headless stdio and catalog selection gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { withExactCargoLawOutputV1, type ExactCargoLawOutputTaskV1 } from "@semio-tech/process/testing/cargo/exact";
import { createHash } from "node:crypto";
import { closeSync, existsSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { repoCacheDirectory, runCmd, orchestratorBudgetOpts } from "@semio-tech/process/execution";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { readStableBuildFile } from "@semio-tech/repo-lib/build";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { assertObservationExpectation, type ObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { webSha256Hex } from "../../🚀️local-relay/🛰️servers/🟦️.ts";
import { requireHubDatum } from "../../🧪️testing/📖️source-readers/🟦️.ts";
import { proveTrustedCompiledDependenciesFixture } from "../🏛️trusted-catalog-fixtures/🚀️stdio-gis-bootstrap-and-compiled-dependencies-fixtures/🟦️.ts";

type NativeOpenableProjectionReceipt = {
  artifact: string;
  factory_id: string;
  descriptor_codec_id: string;
  runtime_capability_id: string;
  artifact_kind: string;
  artifact_schema: string;
  extension: string;
  pack_schema_hash: string;
  protocol_source_sha256: string;
  protocol_path: string;
  definition_path: string;
};

type NativeOpenableOwnerReceipt = Omit<NativeOpenableProjectionReceipt, "protocol_path" | "protocol_source_sha256" | "definition_path"> & { runtimeAuthorized: boolean };

/** 🧬 Proves the neutral provider projection independently from Rust codecs and loader parsers. */
async function proveNativeOpenableCatalogProviderFixture(repoRoot: string): Promise<void> {
  const fixtureRoot = join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️tests/🧫️fixtures/🪪️v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as any;
  const admitsOpenTarget = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.native-openable-provider/NativeOpenableOpenTargetV1");
  const admitsAttestation = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.native-openable-provider/NativeOpenableAttestationV1");
  if (fixture.schema !== "semio.hub.native-openable-catalog-provider-fixture/v1" || fixture.receiptAuthority !== "compiled-kind-schema-roster" || fixture.hostileCases.length !== 13
    || new Set(fixture.hostileCases.map((row: any) => row.mutation)).size !== 13 || typeof fixture.providerProjection !== "string" || typeof fixture.artifactDefinitionsRoot !== "string") throw new Error("native-openable catalog provider envelope differs");
  if (!admitsOpenTarget(fixture.openTarget) || !admitsAttestation(fixture.attestation)) throw new Error("native-openable open target or attestation violates its owning scope contract");
  const projectionPath = join(repoRoot, fixture.providerProjection);
  const projection = JSON.parse(readFileSync(projectionPath, "utf8")) as { schema: string; provider_id: string; plugin_id: string; package_id: string; receipts: NativeOpenableProjectionReceipt[] };
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ allErrors: true, strict: true });
  const claimRoot = join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/🧾️claim-authority");
  const surfaceRoot = join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/📇️native-catalog-surface");
  const builderRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder");
  const builderFixture = JSON.parse(readFileSync(join(builderRoot, "🧪️tests/🧫️fixtures/📇️topic-contributions/🔣️.json"), "utf8"));
  
  
  if (!readFileSync(join(builderRoot, "🦀️.rs"), "utf8").includes("pub fn contributes_topic(")) throw new Error("plugin builder does not retain domain-neutral topic contributions before assembly");
  hubSchemaExport(repoRoot, "schema://hub.compositions.stdio.catalog/NativeCatalogSurfaceCommitment");
  const catalogSource = readFileSync(join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/🦀️.rs"), "utf8");
  if (!catalogSource.includes("pub fn native_artifact_catalog_dependency(") || !catalogSource.includes("pub fn validate_native_artifact_catalog_dependency(")) throw new Error("native catalog compiled dependency contract is absent");
  const registryRoot = join(surfaceRoot, "../..");
  const artifactsRoot = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
  const compiledDefinition = /pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!\("([^"]+)"\);/gu;
  const compiledInventory = readdirSync(artifactsRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => {
      const module = join(artifactsRoot, entry.name, "🦀️.rs");
      const compiled = existsSync(module) ? [...readFileSync(module, "utf8").matchAll(compiledDefinition)] : [];
      if (compiled.length !== 1) throw new Error(`native Stdio artifact module does not compile exactly one artifact definition: ${entry.name}`);
      return resolve(dirname(module), requireHubDatum(requireHubDatum(compiled[0],"compiled[0]")[1],"compiled definition"));
    })
    .sort();
  if (compiledInventory.length !== 36 || new Set(compiledInventory).size !== 36 || compiledInventory.some((path) => !lstatSync(path).isFile())) throw new Error("native Stdio compiled artifact definition roster is not the complete set of 36 module-owned definitions");
  const budgetFixture = JSON.parse(readFileSync(join(surfaceRoot, "🧪️budget.json"), "utf8"));
  for (const row of budgetFixture.cases) if (Buffer.byteLength(JSON.stringify(row.text), "utf8") !== row.encodedBytes) throw new Error(`native catalog JSON budget oracle differs: ${row.id}`);
  for (const row of budgetFixture.aggregateCases) {
    const accepted = row.projectionCharges.reduce((sum: number, value: number) => sum + value, 0) <= budgetFixture.projectionLimitBytes && row.descriptorCharges.reduce((sum: number, value: number) => sum + value, 0) <= budgetFixture.descriptorLimitBytes;
    if (accepted !== row.accepted) throw new Error(`native catalog aggregate budget oracle differs: ${row.id}`);
  }
  if (!catalogSource.includes("preflight_native_catalog_projection(assemblies, &receipts)?") || !catalogSource.includes("struct NativeCatalogProjectionBudget")) throw new Error("native catalog projection is not aggregate-budgeted before allocation and hashing");
  const providerSource = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"), "utf8");
  if (!catalogSource.includes("pub fn validate_native_codec_artifact_kinds(") || !providerSource.includes("validate_native_codec_artifact_kinds(&descriptor.manifest.artifact_kinds)")) throw new Error("native Stdio catalog is not bound to the complete decoded descriptor surface");
  if (!catalogSource.includes("pub fn validate_native_artifact_catalog_contributions(") || !providerSource.includes("validate_native_artifact_catalog_contributions(&descriptor.manifest.topic_contributions)")) throw new Error("native Stdio catalog omits its compiled guest semantic commitment");
  const claimFixture = JSON.parse(readFileSync(join(claimRoot, "🔣️.json"), "utf8"));
  const uniqueClaims = ajv.compile({ type: "array", items: { type: "string" }, uniqueItems: true });
  for (const row of claimFixture.cases) {
    for (const claim of row.claims) {
      if (claim.category === "codec") {
        const utf8Length = new TextEncoder().encode(claim.codecSchema).length;
        if (utf8Length !== Buffer.byteLength(claim.codecSchema, "utf8") || claim.value !== `${utf8Length}:${claim.codecSchema}:${claim.extension}`) throw new Error("native-openable codec extension framing disagrees");
      }
    }
    const pairs = row.claims.map((claim: any) => `${claim.namespace}:${claim.value}`);
    const unique = new Set(pairs).size === pairs.length;
    if (uniqueClaims(pairs) !== unique) throw new Error("native-openable AJV and independent claim uniqueness differ");
    const code = unique ? "accepted" : "artifact-definition.duplicate-claim";
    if (code !== row.code) throw new Error(`native-openable claim oracle differs for ${row.id}`);
  }
  console.log(`native-openable-claim-oracle cases=${claimFixture.cases.length}`);
  const validateProjection = hubSchemaExport(repoRoot, "schema://hub.compositions.stdio.catalog/NativeCodecFactories");
  if (!validateProjection(projection)) throw new Error("native-openable projection violates its owning scope contract");
  const definitionRoot = join(repoRoot, fixture.artifactDefinitionsRoot);
  const definitionFiles = compiledInventory;
  const owners: NativeOpenableOwnerReceipt[] = [];
  for (const path of definitionFiles.sort()) {
    const definition = JSON.parse(readFileSync(path, "utf8")) as any;
    for (const standard of definition.standards) {
      if (!/^[a-z0-9_-]+$/.test(standard.revision) || standard.id !== `${definition.id}.standard.${standard.revision}`) throw new Error("native-openable standard identity is not a canonical source-owned segment");
    }
    const claimsSeen = new Set<string>();
    for (const capability of definition.runtime_capabilities) {
      const category = capability.category;
      const prefix = category === "codec" || category === "representation" ? `${definition.standards[0].id}.${category}.` : `${definition.id}.${category}.`;
      const leaf = capability.id.startsWith(prefix) ? capability.id.slice(prefix.length) : "";
      if (!(category === "representation" ? /^[a-z0-9_-]+$/ : /^[a-z0-9_-]+\.v[1-9][0-9]*$/).test(leaf)) throw new Error("native-openable runtime capability violates canonical category ownership");
      for (const claim of capability.claims) {
        const key = `${claim.namespace}:${claim.value}`;
        if (claimsSeen.has(key)) throw new Error(`native-openable duplicate owned claim ${key}`);
        if (category === "subset-validator" && claim.namespace !== "validated-dialect") throw new Error("native-openable validator claims must be distinct from composition authority");
        claimsSeen.add(key);
      }
    }
    for (const codec of Array.isArray(definition.codecs) ? definition.codecs : []) {
      if (codec.executable_registration !== true) continue;
      const native = codec.native_factory;
      const runtime = Array.isArray(definition.runtime_capabilities) ? definition.runtime_capabilities.find((candidate: any) => candidate.id === native?.runtime_capability_id) : undefined;
      const claims = new Map(Array.isArray(runtime?.claims) ? runtime.claims.map((claim: any) => [claim.namespace, claim.value]) : []);
      owners.push({
        artifact: definition.artifact,
        factory_id: native?.factory_id,
        descriptor_codec_id: codec.id,
        runtime_capability_id: native?.runtime_capability_id,
        artifact_kind: native?.artifact_kind,
        artifact_schema: native?.artifact_schema,
        extension: native?.extension,
        pack_schema_hash: native?.pack_schema_hash,
        runtimeAuthorized:
          runtime?.category === "codec" &&
          claims.size === 2 &&
          claims.get("codec") === native?.artifact_schema &&
          claims.get("codec-extension") === `${Buffer.byteLength(native?.artifact_schema ?? "", "utf8")}:${native?.artifact_schema}:${native?.extension}`,
      });
    }
  }
  const sha256 = async (bytes: Uint8Array): Promise<string> => await webSha256Hex(bytes);
  const protocolDigests = new Map<string, string>();
  const stdioRoot = dirname(definitionRoot);
  for (const receipt of projection.receipts) protocolDigests.set(receipt.protocol_path, await sha256(readFileSync(join(repoRoot, receipt.protocol_path))));
  const componentDigest = await sha256(Buffer.from(fixture.attestation.componentHex, "hex"));
  const descriptorDigest = await sha256(Buffer.from(fixture.attestation.descriptorProjectionHex, "hex"));
  type Candidate = {
    owners: NativeOpenableOwnerReceipt[];
    projected: NativeOpenableProjectionReceipt[];
    targets: any[];
    componentSha256: string;
    descriptorSha256: string;
  };
  const valid = (candidate: Candidate): boolean => {
    if (candidate.owners.length !== owners.length || candidate.projected.length !== owners.length || candidate.targets.length !== 1) return false;
    const unique = (values: string[]): boolean => values.length === new Set(values).size;
    if (!unique(candidate.owners.map((row) => row.factory_id)) || !unique(candidate.owners.map((row) => row.descriptor_codec_id)) || !unique(candidate.projected.map((row) => row.factory_id))) return false;
    if (candidate.projected.some((row, index) => index > 0 && candidate.projected[index - 1]!.factory_id.localeCompare(row.factory_id) >= 0)) return false;
    if (candidate.componentSha256 !== componentDigest || candidate.descriptorSha256 !== descriptorDigest) return false;
    const ownerByFactory = new Map(candidate.owners.map((row) => [row.factory_id, row]));
    for (const row of candidate.projected) {
      const owner = ownerByFactory.get(row.factory_id);
      if (!owner?.runtimeAuthorized || protocolDigests.get(row.protocol_path) !== row.protocol_source_sha256) return false;
      for (const key of ["artifact", "factory_id", "descriptor_codec_id", "runtime_capability_id", "artifact_kind", "artifact_schema", "extension", "pack_schema_hash"] as const) if (owner[key] !== row[key]) return false;
    }
    const target = candidate.targets[0];
    const json = candidate.projected.find((row) => row.factory_id === "stdio.native.json.v1");
    return Boolean(
      json &&
      target.artifactKind === json.artifact_kind &&
      target.artifactSchema === json.artifact_schema &&
      target.packSchemaHash === json.pack_schema_hash &&
      target.surfaceId === "s.stdio.json@rfc8259/*#viewer" &&
      target.appId === target.surfaceId &&
      target.windowKindId === "framework.window.tree" &&
      target.role === "viewer" &&
      target.rendererTarget === "wasm",
    );
  };
  const baseline = (): Candidate => ({
    owners: structuredClone(owners),
    projected: structuredClone(projection.receipts),
    targets: [structuredClone(fixture.openTarget)],
    componentSha256: fixture.attestation.componentSha256,
    descriptorSha256: fixture.attestation.descriptorProjectionSha256,
  });
  if (!valid(baseline())) throw new Error("native-openable positive owner/projection/target bijection was denied");
  for (const hostile of fixture.hostileCases as { name: string; mutation: string; stage: ObservationExpectation["stage"]; result: ObservationExpectation["result"]; code: string; publishedTargets: 0 }[]) {
    const candidate = baseline();
    switch (hostile.mutation) {
      case "missing-owner":
        candidate.owners.pop();
        break;
      case "extra-projection":
        candidate.projected.push({ ...candidate.projected[0]!, artifact: "foreign", factory_id: "stdio.native.foreign.v1" });
        break;
      case "duplicate-factory":
        candidate.owners[1]!.factory_id = candidate.owners[0]!.factory_id;
        break;
      case "duplicate-descriptor-codec":
        candidate.owners[1]!.descriptor_codec_id = candidate.owners[0]!.descriptor_codec_id;
        break;
      case "missing-runtime-capability":
        candidate.owners[0]!.runtimeAuthorized = false;
        break;
      case "wrong-protocol-hash":
        candidate.projected[0]!.protocol_source_sha256 = "11".repeat(32);
        break;
      case "zero-protocol-hash":
        candidate.projected[0]!.protocol_source_sha256 = "00".repeat(32);
        break;
      case "wrong-component-hash":
        candidate.componentSha256 = "11".repeat(32);
        break;
      case "wrong-descriptor-hash":
        candidate.descriptorSha256 = "11".repeat(32);
        break;
      case "wrong-surface":
        candidate.targets[0]!.surfaceId = "s.stdio.json@rfc8259/*#foreign";
        break;
      case "wrong-role":
        candidate.targets[0]!.role = "editor";
        break;
      case "wrong-renderer":
        candidate.targets[0]!.rendererTarget = "wgpu";
        break;
      case "duplicate-target":
        candidate.targets.push(structuredClone(candidate.targets[0]));
        break;
      default:
        throw new Error(`native-openable unknown hostile mutation ${hostile.mutation}`);
    }
    const contractAdmitted = hostile.mutation === "wrong-surface" ? admitsOpenTarget(candidate.targets[0]) : valid(candidate);
    assertObservationExpectation(`native-openable/${hostile.name}`, { stage: hostile.stage, result: hostile.result, code: hostile.code }, contractAdmitted);
    if (valid(candidate) || hostile.result !== "rejected" || hostile.publishedTargets !== 0) throw new Error(`native-openable hostile case admitted or partially published: ${hostile.name}`);
  }
  console.log(`native-openable-neutral-oracle: AJV=1 scope-exports=2 owner-receipts=${owners.length} protocol-webcrypto=${protocolDigests.size} targets=1 hostile-denied=${fixture.hostileCases.length} no-partial=${fixture.hostileCases.length}`);
}

/** 🌿 Independently evaluates the exact VCS selection, admission fences and unconsumed-binding profiles. */
async function proveVcsNativeProviderSelectionFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️tests/🧫️fixtures/🌿️vcs-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  if (fixture.schema !== "semio.hub.vcs-native-provider-selection/v1" || fixture.cases.length !== 8 || fixture.unconsumedProfiles.length !== 2 || new Set(fixture.cases.map((row: any) => row.name)).size !== fixture.cases.length) throw new Error("VCS provider selection envelope differs");
  const receipts = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🧩️compositions/🌿️vcs/📇️native-codecs/🔣️.json"), "utf8"));
  if (fixture.packageVersion !== receipts.packageVersion || fixture.codecCount !== receipts.receipts.length) throw new Error("VCS selection differs from the package-owned closure");
  let accepted = 0;
  for (const row of fixture.cases) {
    const result = row.pluginId === receipts.pluginId && row.packageId === receipts.packageId && row.version === receipts.packageVersion && !row.cancelled && row.nowMs < row.deadlineMs;
    assertObservationExpectation(`vcs-native-provider-selection/${row.name}`, { stage: row.stage, result: row.accepted ? "accepted" : "rejected", code: row.code }, result);
    if (result !== row.accepted) throw new Error(`VCS native selection mismatch: ${row.name}`);
    if (result) accepted++;
  }
  if (accepted !== 1) throw new Error("VCS provider corpus must admit exactly one selection");
  const linked = new Map<string, number>([
    ["semio:stdio", JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json"), "utf8")).receipts.length],
    ["semio:gis", 2],
    ["semio:vcs", receipts.receipts.length],
  ]);
  for (const profile of fixture.unconsumedProfiles) {
    const previewed = profile.selected.filter((packageId: string) => linked.has(packageId));
    if (JSON.stringify(previewed) !== JSON.stringify(profile.previews)) throw new Error(`VCS provider set previewed an unselected package: ${profile.name}`);
    if (previewed.reduce((sum: number, packageId: string) => sum + linked.get(packageId)!, 0) < 1) throw new Error(`VCS unconsumed profile is empty: ${profile.name}`);
  }
  const provider = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"), "utf8");
  if (!provider.includes('NativeCodecProviderEntryV1 { plugin_id: "vcs", package_id: "semio:vcs", preview: preview_vcs_bindings }') || !provider.includes('identity.factory_id != "vcs.vcs.v1"') || !provider.includes('identity.schema != "vcs.vcs"'))
    throw new Error("linked VCS provider entry or its exact identity fences are absent");
  if ([...linked.values()].some((count) => count < 1) || !provider.includes("pub fn native_openable_provider_receipt_count()")) throw new Error("linked provider closure is not the exact stdio+GIS+VCS sum");
  console.log(`vcs-native-provider-selection-oracle: cases=${fixture.cases.length} accepted=${accepted} unconsumed-profiles=${fixture.unconsumedProfiles.length} linked-receipts=${[...linked.values()].reduce((sum, count) => sum + count, 0)} scope-exports=2; no native or catalog activation claim`);
}

const HEADLESS_STDIO_METADATA_MAX_BYTES = 2 * 1024 * 1024 * 1024;
/** 🧾️ One dependency-closure member captured into the private deps directory. */
type HeadlessStdioMetadataArtifactV1 = Readonly<{ originalPath: string; capturedPath: string; byteLength: number; sha256: string }>;
type HeadlessStdioMetadataCaptureV1 = Readonly<{
  schema: "semio.hub.headless-stdio-metadata-capture/v2";
  sourcePath: string;
  features: readonly string[];
  capturedPath: string;
  depsDirectory: string;
  artifacts: readonly HeadlessStdioMetadataArtifactV1[];
  byteLength: number;
  sha256: string;
}>;
type StableBuildFileReaderV1 = typeof readStableBuildFile;

/** 🏠️ Requires one command's Cargo target to be its own generated artifact child. */
function headlessStdioOwnedCargoTarget(artifactRoot: string, cargoTargetDir: string): { artifactRoot: string; cargoTargetDir: string } {
  if (!isAbsolute(artifactRoot) || !isAbsolute(cargoTargetDir) || !artifactRoot.split(/[\\/]/u).includes("🗑️generated")) throw new Error("headless Stdio command roots must be absolute generated paths");
  const ownedArtifactRoot = resolve(artifactRoot), ownedCargoTargetDir = resolve(cargoTargetDir), child = relative(ownedArtifactRoot, ownedCargoTargetDir);
  if (!child || child.startsWith("..") || isAbsolute(child)) throw new Error("headless Stdio Cargo target must be a private child of its command artifact root");
  return { artifactRoot: ownedArtifactRoot, cargoTargetDir: ownedCargoTargetDir };
}

/** 🧭️ Resolves the current exact command's isolated artifact and Cargo roots against the workspace root and hands the absolute roots to every child. */
function headlessStdioCommandRoots(repoRoot: string): { artifactRoot: string; cargoTargetDir: string } {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR, cargoTargetDir = process.env.CARGO_TARGET_DIR;
  if (!artifactRoot || !cargoTargetDir) throw new Error("headless Stdio command requires explicit artifact and Cargo roots");
  const roots = headlessStdioOwnedCargoTarget(resolve(repoRoot, artifactRoot), resolve(repoRoot, cargoTargetDir));
  process.env.SEMIO_TEST_ARTIFACT_DIR = roots.artifactRoot;
  process.env.CARGO_TARGET_DIR = roots.cargoTargetDir;
  return roots;
}

/** 🧩️ Proves the three independently declared native-catalog commands of the owning Nx targets own distinct nested targets inside the repository cache. */
function proveHeadlessStdioCommandIsolation(repoRoot: string): void {
  type Declared = { command?: string; args?: string; env?: Record<string, string> };
  const targets = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📋️project.json"), "utf8")).targets as Record<string, { options?: Declared; configurations?: Record<string, Declared> }>;
  const provider = targets["native-openable-catalog-provider-check"], stdioOnly = provider?.configurations?.["stdio-only"], selection = targets["native-catalog-selection-check"], cache = repoCacheDirectory(repoRoot);
  const rows = ([
    ["os-hub:native-openable-catalog-provider-check", provider?.options?.command === "repo ./📜️script.ts native-openable-catalog-provider-check", provider?.options?.env],
    ["os-hub:native-openable-catalog-provider-check:stdio-only", stdioOnly?.command === undefined && stdioOnly?.args === "--stdio-only", stdioOnly?.env],
    ["os-hub:native-catalog-selection-check", selection?.options?.command === "repo ./📜️script.ts native-catalog-selection-check", selection?.options?.env],
  ] as const).map(([name, declared, env]) => {
    const artifactRoot = env?.SEMIO_TEST_ARTIFACT_DIR, cargoTargetDir = env?.CARGO_TARGET_DIR, cached = artifactRoot ? relative(cache, resolve(repoRoot, artifactRoot)) : "";
    if (!declared || !artifactRoot || !cargoTargetDir || isAbsolute(artifactRoot) || !artifactRoot.split("/").includes("🗑️generated") || !cached || cached.startsWith("..") || isAbsolute(cached) || cargoTargetDir !== `${artifactRoot}/cargo-target`) throw new Error(`headless Stdio command is not privately isolated: ${name}`);
    return headlessStdioOwnedCargoTarget(resolve(repoRoot, artifactRoot), resolve(repoRoot, cargoTargetDir));
  });
  if (new Set(rows.map((row) => row.artifactRoot)).size !== rows.length || new Set(rows.map((row) => row.cargoTargetDir)).size !== rows.length) throw new Error("headless Stdio independently declared commands share build ownership");
}

/**
 * 📦️ Captures one Cargo compiler-artifact's full dependency closure into one command-private, immutable deps
 * directory — layout-independent: every original path is taken from `build.stdout` compiler-artifact records,
 * never from a `<target>/debug/deps` convention (the new build-dir layout keeps no such directory).
 */
function captureHeadlessStdioMetadata(
  input: Readonly<{
    artifactRoot: string;
    cargoTargetDir: string;
    runRoot: string;
    sourcePath: string;
    stdioOriginalPath: string;
    dependencyOriginalPaths: readonly string[];
    features: readonly string[];
    check: () => void;
    progress?: (stage: "read" | "copy", completed: number, total: number) => void;
  }>,
  reader: StableBuildFileReaderV1 = readStableBuildFile,
): HeadlessStdioMetadataCaptureV1 {
  const roots = headlessStdioOwnedCargoTarget(input.artifactRoot, input.cargoTargetDir), runRoot = resolve(input.runRoot);
  if (!isAbsolute(input.sourcePath) || relative(roots.artifactRoot, runRoot).startsWith("..") || isAbsolute(relative(roots.artifactRoot, runRoot))) throw new Error("headless Stdio metadata paths are outside their exact command owner");
  input.check();
  const stdioOriginalPath = resolve(input.stdioOriginalPath);
  const originals = [stdioOriginalPath, ...input.dependencyOriginalPaths.map((path) => resolve(path))].filter((path, index, all) => all.indexOf(path) === index);
  if (originals.length < 1 || !originals.every((path) => isAbsolute(path))) throw new Error("headless Stdio dependency closure requires absolute original artifact paths");
  if (new Set(originals.map((path) => basename(path))).size !== originals.length) throw new Error("headless Stdio dependency closure has colliding artifact names");
  const depsDirectory = join(runRoot, "deps");
  mkdirSync(depsDirectory, { recursive: true, mode: 0o700 });
  const sizes = originals.map((path) => {
    const original = lstatSync(path);
    if (!original.isFile() || original.isSymbolicLink()) throw new Error("headless Stdio metadata is not a regular dependency artifact");
    return original.size;
  });
  const totalBytes = sizes.reduce((sum, size) => sum + size, 0);
  if (totalBytes > HEADLESS_STDIO_METADATA_MAX_BYTES) throw new Error("headless Stdio dependency closure exceeds its bounded byte budget");
  const artifacts: HeadlessStdioMetadataArtifactV1[] = [];
  let readProgress = 0, copyProgress = 0;
  for (const originalPath of originals) {
    input.check();
    const bytes = reader(originalPath, HEADLESS_STDIO_METADATA_MAX_BYTES, { remaining: HEADLESS_STDIO_METADATA_MAX_BYTES - readProgress }, () => {
      input.check();
      input.progress?.("read", readProgress, totalBytes);
    });
    readProgress += bytes.byteLength;
    input.progress?.("read", readProgress, totalBytes);
    const capturedPath = join(depsDirectory, basename(originalPath));
    const output = openSync(capturedPath, "wx", 0o600), digest = createHash("sha256");
    let offset = 0, complete = false;
    try {
      while (offset < bytes.byteLength) {
        input.check();
        const count = writeSync(output, bytes, offset, Math.min(64 * 1024, bytes.byteLength - offset), offset);
        if (!count) throw new Error("headless Stdio private metadata copy stopped making progress");
        digest.update(bytes.subarray(offset, offset + count));
        offset += count;
        copyProgress += count;
        input.progress?.("copy", copyProgress, totalBytes);
      }
      complete = true;
    } finally {
      closeSync(output);
      if (!complete) rmSync(capturedPath, { force: true });
    }
    const captured = lstatSync(capturedPath), sha256 = digest.digest("hex");
    if (!captured.isFile() || captured.isSymbolicLink() || captured.size !== bytes.byteLength) throw new Error("headless Stdio private metadata copy differs from its retained source descriptor");
    artifacts.push({ originalPath, capturedPath, byteLength: bytes.byteLength, sha256 });
  }
  const stdioArtifact = artifacts.find((artifact) => artifact.originalPath === stdioOriginalPath);
  if (!stdioArtifact) throw new Error("headless Stdio captured closure is missing its own compiled metadata");
  const features = [...input.features].sort();
  const receipt: HeadlessStdioMetadataCaptureV1 = { schema: "semio.hub.headless-stdio-metadata-capture/v2", sourcePath: resolve(input.sourcePath), features, capturedPath: stdioArtifact.capturedPath, depsDirectory, artifacts, byteLength: totalBytes, sha256: stdioArtifact.sha256 };
  writeFileSync(join(runRoot, "stdio.capture.json"), JSON.stringify(receipt), { flag: "wx", mode: 0o600 });
  return receipt;
}

/** 🔗️ Builds one exact metadata import command with no alternate dependency resolver. */
function headlessStdioImportArguments(capture: HeadlessStdioMetadataCaptureV1, dependencyRoots: readonly string[], input: string, output: string): string[] {
  if (dependencyRoots.length !== 1 || resolve(dependencyRoots[0]!) !== capture.depsDirectory) throw new Error("headless Stdio imports require exactly their captured private dependency root");
  const info = lstatSync(capture.capturedPath);
  if (!info.isFile() || info.isSymbolicLink() || dirname(capture.capturedPath) !== capture.depsDirectory) throw new Error("headless Stdio captured metadata is not a regular file inside its private dependency root");
  return ["--edition=2021", "--crate-name", "stdio_native_surface_probe", "--crate-type=lib", "--emit=metadata", "--error-format=json", "--extern", `semio_hub_stdio_catalog=${capture.capturedPath}`, "-L", `dependency=${capture.depsDirectory}`, "-o", output, input];
}

/**
 * 🧪️ Proves private capture stability, closure completeness, replacement refusal and command-root isolation
 * without Cargo — original artifacts are deliberately scattered across distinct directories (one per simulated
 * build-dir compilation unit) to prove the capture never assumes a `<target>/debug/deps` layout.
 */
function proveHeadlessStdioMetadataCaptureContract(artifactRoot: string): void {
  if (!isAbsolute(artifactRoot) || !artifactRoot.split(/[\\/]/u).includes("🗑️generated")) throw new Error("headless Stdio capture law requires one generated artifact root");
  mkdirSync(artifactRoot, { recursive: true });
  const lawRoot = mkdtempSync(join(artifactRoot, "headless-capture-law-"));
  try {
    const captures: HeadlessStdioMetadataCaptureV1[] = [];
    for (const [name, payload] of [["full", "full-metadata"], ["stdio-only", "stdio-only-metadata"]] as const) {
      const commandRoot = join(lawRoot, name), cargoTargetDir = join(commandRoot, "cargo-target"), runRoot = join(commandRoot, "headless-imports-proof"), sourcePath = join(commandRoot, "stdio.rs");
      const stdioUnit = join(commandRoot, "build-dir", "debug", "build", "semio-s-plugin-stdio-aaaa", "out"), depUnitOne = join(commandRoot, "build-dir", "debug", "build", "serde-bbbb", "out"), depUnitTwo = join(commandRoot, "build-dir", "debug", "build", "blake3-cccc", "out");
      const stdioOriginalPath = join(stdioUnit, "libsemio_hub_stdio_catalog.rmeta"), depOne = join(depUnitOne, "libserde.rmeta"), depTwo = join(depUnitTwo, "libblake3.rmeta");
      mkdirSync(stdioUnit, { recursive: true });
      mkdirSync(depUnitOne, { recursive: true });
      mkdirSync(depUnitTwo, { recursive: true });
      mkdirSync(cargoTargetDir, { recursive: true });
      mkdirSync(runRoot);
      writeFileSync(sourcePath, "pub const SOURCE: u8 = 1;\n", { flag: "wx", mode: 0o600 });
      writeFileSync(stdioOriginalPath, payload, { flag: "wx", mode: 0o600 });
      writeFileSync(depOne, `${payload}-serde`, { flag: "wx", mode: 0o600 });
      writeFileSync(depTwo, `${payload}-blake3`, { flag: "wx", mode: 0o600 });
      captures.push(captureHeadlessStdioMetadata({ artifactRoot: commandRoot, cargoTargetDir, runRoot, sourcePath, stdioOriginalPath, dependencyOriginalPaths: [depOne, depTwo, stdioOriginalPath], features: [], check() {} }));
    }
    if (captures[0]!.depsDirectory === captures[1]!.depsDirectory || captures[0]!.sha256 === captures[1]!.sha256) throw new Error("headless Stdio command captures are not isolated");
    if (captures[0]!.artifacts.length !== 3 || !captures[0]!.artifacts.every((artifact) => dirname(artifact.capturedPath) === captures[0]!.depsDirectory)) throw new Error("headless Stdio capture did not land its full dependency closure in one private directory");
    const firstBytes = readFileSync(captures[0]!.capturedPath);
    const firstStdioOriginal = captures[0]!.artifacts.find((artifact) => artifact.capturedPath === captures[0]!.capturedPath)!.originalPath;
    renameSync(firstStdioOriginal, `${firstStdioOriginal}.replaced`);
    writeFileSync(firstStdioOriginal, "replacement", { flag: "wx", mode: 0o600 });
    if (createHash("sha256").update(firstBytes).digest("hex") !== captures[0]!.sha256 || !readFileSync(captures[0]!.capturedPath).equals(firstBytes)) throw new Error("headless Stdio private capture followed a replaced live target");
    const args = headlessStdioImportArguments(captures[0]!, [captures[0]!.depsDirectory], join(lawRoot, "probe.rs"), join(lawRoot, "probe.rmeta"));
    if (args.filter((value) => value === "-L").length !== 1 || args.some((value) => value.includes("executable"))) throw new Error("headless Stdio import command admitted an alternate dependency directory");
    let rejectedSecondRoot = false;
    try { headlessStdioImportArguments(captures[0]!, [captures[0]!.depsDirectory, dirname(captures[0]!.capturedPath)], join(lawRoot, "probe.rs"), join(lawRoot, "probe.rmeta")); } catch { rejectedSecondRoot = true; }
    if (!rejectedSecondRoot) throw new Error("headless Stdio import command admitted a second dependency root");
    let rejectedOutside = false;
    try { headlessStdioOwnedCargoTarget(join(lawRoot, "full"), join(lawRoot, "foreign-target")); } catch { rejectedOutside = true; }
    if (!rejectedOutside) throw new Error("headless Stdio command admitted an externally owned Cargo target");
    const collisionRoot = join(lawRoot, "collision"), collisionTarget = join(collisionRoot, "cargo-target"), collisionRun = join(collisionRoot, "headless-imports-proof"), collisionSource = join(collisionRoot, "stdio.rs");
    const collisionStdioUnit = join(collisionRoot, "build-dir", "debug", "build", "stdio-aaaa", "out"), collisionDepUnit = join(collisionRoot, "build-dir", "debug", "build", "stdio-bbbb", "out");
    mkdirSync(collisionStdioUnit, { recursive: true });
    mkdirSync(collisionDepUnit, { recursive: true });
    mkdirSync(collisionTarget, { recursive: true });
    mkdirSync(collisionRun);
    writeFileSync(collisionSource, "pub const SOURCE: u8 = 1;\n", { flag: "wx", mode: 0o600 });
    const collisionStdio = join(collisionStdioUnit, "libsemio_hub_stdio_catalog.rmeta"), collisionDuplicateName = join(collisionDepUnit, "libsemio_hub_stdio_catalog.rmeta");
    writeFileSync(collisionStdio, "stdio", { flag: "wx", mode: 0o600 });
    writeFileSync(collisionDuplicateName, "stdio-again", { flag: "wx", mode: 0o600 });
    let rejectedCollision = false;
    try { captureHeadlessStdioMetadata({ artifactRoot: collisionRoot, cargoTargetDir: collisionTarget, runRoot: collisionRun, sourcePath: collisionSource, stdioOriginalPath: collisionStdio, dependencyOriginalPaths: [collisionDuplicateName], features: [], check() {} }); } catch { rejectedCollision = true; }
    if (!rejectedCollision) throw new Error("headless Stdio capture admitted two dependency-closure artifacts with the same file name");
    const raceRoot = join(lawRoot, "race"), raceTarget = join(raceRoot, "cargo-target"), raceRun = join(raceRoot, "headless-imports-proof"), raceSource = join(raceRoot, "stdio.rs");
    const raceStdioUnit = join(raceRoot, "build-dir", "debug", "build", "stdio-aaaa", "out"), raceDepUnit = join(raceRoot, "build-dir", "debug", "build", "serde-bbbb", "out");
    const raceStdio = join(raceStdioUnit, "libsemio_hub_stdio_catalog.rmeta"), raceDep = join(raceDepUnit, "libserde.rmeta");
    mkdirSync(raceStdioUnit, { recursive: true });
    mkdirSync(raceDepUnit, { recursive: true });
    mkdirSync(raceTarget, { recursive: true });
    mkdirSync(raceRun);
    writeFileSync(raceSource, "pub const SOURCE: u8 = 1;\n", { flag: "wx", mode: 0o600 });
    writeFileSync(raceStdio, Buffer.alloc(0), { flag: "wx", mode: 0o600 });
    writeFileSync(raceDep, Buffer.alloc(128 * 1024, 9), { flag: "wx", mode: 0o600 });
    let checks = 0, refusedRace = false;
    const racingReader: StableBuildFileReaderV1 = (path, maximum, admission, check) => readStableBuildFile(path, maximum, admission, () => {
      check();
      if (++checks === 3) {
        renameSync(raceDep, `${raceDep}.replaced`);
        writeFileSync(raceDep, "raced", { flag: "wx", mode: 0o600 });
      }
    });
    try { captureHeadlessStdioMetadata({ artifactRoot: raceRoot, cargoTargetDir: raceTarget, runRoot: raceRun, sourcePath: raceSource, stdioOriginalPath: raceStdio, dependencyOriginalPaths: [raceDep], features: [], check() {} }, racingReader); } catch { refusedRace = true; }
    if (!refusedRace) throw new Error("headless Stdio metadata replacement during capture was accepted");
    console.log("headless-stdio-metadata-capture-oracle: private-commands=2 closure-complete=1 replacement-stable=1 replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1 colliding-artifact-name-denied=1");
  } finally {
    rmSync(lawRoot, { recursive: true, force: true });
  }
}

/** 🧪️ Compiles bounded positive/negative imports against one private metadata capture. */
async function proveHeadlessStdioImports(invocation:ScriptInvocation,repoRoot:string,task:ExactCargoLawOutputTaskV1):Promise<void>{
 const {receipt,control:phaseControl}=task;
  const fixtureRoot = join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/📇️native-catalog-surface");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🧪️imports.json"), "utf8")) as { features: string[]; cases: { id: string; source: string; accepted: boolean }[] };
  const reportPath = join(receipt.artifactDir, "build.stdout");
  const report = lstatSync(reportPath);
  if (!report.isFile() || report.isSymbolicLink() || report.size > 256 * 1024 * 1024) throw new Error("headless Stdio Cargo report is not bounded regular output");
  const sourcePath = join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/🦀️.rs");
  const records = readFileSync(reportPath, "utf8").split("\n").filter(Boolean).map((line) => JSON.parse(line)).filter((row) => row.reason === "compiler-artifact");
  const stdio = records.filter((row) => row.target?.name === "semio_hub_stdio_catalog" && resolve(row.target.src_path) === sourcePath);
  if (stdio.length !== 1 || JSON.stringify([...stdio[0].features].sort()) !== JSON.stringify([...fixture.features].sort())) throw new Error("headless Stdio build did not report the exact catalog-only feature closure");
  const libraries = stdio[0].filenames.filter((path: unknown) => typeof path === "string" && path.endsWith(".rmeta")) as string[];
  if (libraries.length !== 1) throw new Error("headless Stdio build did not report one full metadata artifact");
  const dependencyArtifacts = new Set<string>();
  for (const row of records) {
    const filenames = Array.isArray(row.filenames) ? row.filenames : [];
    const isProcMacro = Array.isArray(row.target?.kind) && row.target.kind.includes("proc-macro");
    for (const filePath of filenames) {
      if (typeof filePath !== "string") continue;
      if (filePath.endsWith(".rmeta") || filePath.endsWith(".rlib") || (isProcMacro && /\.(so|dylib|dll)$/u.test(filePath))) dependencyArtifacts.add(resolve(filePath));
    }
  }
  if (dependencyArtifacts.size < 1) throw new Error("headless Stdio build reported no dependency-closure artifacts");
  const runRoot = mkdtempSync(join(receipt.artifactDir, "headless-imports-"));
  let cancelled = false;
  const cancel = () => { cancelled = true; };
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  try {
    let progressStage: "read" | "copy" | undefined, progressReported = -64 * 1024 * 1024;
    const capture = captureHeadlessStdioMetadata({
      artifactRoot: task.directory, cargoTargetDir: receipt.cargoTargetDir, runRoot, sourcePath, stdioOriginalPath: libraries[0]!, dependencyOriginalPaths: [...dependencyArtifacts], features: stdio[0].features,
      check() { checkResourceLeaseControl(phaseControl);if (cancelled) throw new Error("headless Stdio imports cancelled"); },
      progress(stage, completed, total) {
        if (stage === progressStage && completed !== total && completed - progressReported < 64 * 1024 * 1024) return;
        progressStage = stage;
        progressReported = completed;
        console.log(`headless Stdio metadata ${stage}: bytes=${completed}/${total}`);
      },
    });
    Bun.gc(true);
    const capturedBefore = lstatSync(capture.capturedPath);
    for (const row of fixture.cases) {
      if (cancelled) throw new Error("headless Stdio imports cancelled");
      const input = join(runRoot, `${row.id}.rs`);
      writeFileSync(input, row.source + "\n", { flag: "wx", mode: 0o600 });
      const result = await captureOwnedCommandResultV1("rustc", headlessStdioImportArguments(capture, [capture.depsDirectory], input, join(runRoot, `${row.id}.rmeta`)),repoRoot,{control:phaseControl,env:exactCargoStageEnvironments().env,output:{maximumBytes:1048576,maximumLines:16384}});
      const errors = result.stderr.split("\n").filter(Boolean).map((line) => JSON.parse(line)).filter((diagnostic) => diagnostic.level === "error" && diagnostic.code);
      if (result.signal || (row.accepted ? result.status !== 0 || errors.length !== 0 : result.status !== 1 || errors.length !== 1 || !["E0432", "E0433"].includes(errors[0].code.code))) throw new Error(`headless Stdio import law failed: ${row.id}; evidence=${runRoot}`);
      const after = lstatSync(capture.capturedPath);
      if (after.dev !== capturedBefore.dev || after.ino !== capturedBefore.ino || after.size !== capturedBefore.size || after.mtimeMs !== capturedBefore.mtimeMs || after.ctimeMs !== capturedBefore.ctimeMs) throw new Error("headless Stdio private metadata changed during import proof");
    }
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
  console.log(`headless Stdio imports: features=${fixture.features.join(",")} cases=${fixture.cases.length}; evidence=${runRoot}; private-metadata-capture=1; native metadata only, no guest execution claim`);
}

async function proveNativeStdioCommitmentSchema(repoRoot: string, receipt: Awaited<ReturnType<typeof runAdmittedExactCargoLawsV1>>[number]): Promise<void> {
  const law = receipt.laws.findIndex((name) => name.endsWith("native_catalog_commitment_covers_all_definition_semantics_and_codec_authorities"));
  if (law < 0) throw new Error("native Stdio commitment law receipt is absent");
  const output = join(receipt.artifactDir, `law-${law}.stdout`);
  const stat = lstatSync(output);
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 8 * 1024 * 1024) throw new Error("native Stdio commitment evidence is not a bounded regular file");
  const prefix = "native-catalog-payload=";
  const lines = readFileSync(output, "utf8").split("\n").filter((line) => line.startsWith(prefix));
  if (lines.length !== 1 || Buffer.byteLength(lines[0]!.slice(prefix.length), "utf8") > 2 * 1024 * 1024) throw new Error("native Stdio commitment payload must be unique and at most 2 MiB");
  const validate = hubSchemaExport(repoRoot, "schema://hub.compositions.stdio.catalog/NativeCatalogSurfaceCommitment");
  if (!validate(JSON.parse(lines[0]!.slice(prefix.length)))) throw new Error("actual native Stdio commitment violates its owning scope contract");
  console.log("actual native Stdio commitment: AJV schema validated; no guest descriptor equality claim");
}

export class NativeOpenableCatalogProviderCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (new Set(segments).size !== segments.length || segments.some((segment) => segment !== "--oracle-only" && segment !== "--stdio-only")) throw new Error("unsupported native catalog provider argument");
    const commandRoots = headlessStdioCommandRoots(this.repoRoot);
    proveHeadlessStdioCommandIsolation(this.repoRoot);
    proveHeadlessStdioMetadataCaptureContract(commandRoots.artifactRoot);
    await (await import("../../🧩️compositions/🌿️vcs/🧪️tests/📇️native-codecs/🟦️.ts")).proveVcsNativeCodecReceipts(this.repoRoot);
    await proveVcsNativeProviderSelectionFixture(this.repoRoot);
    await proveNativeOpenableCatalogProviderFixture(this.repoRoot);
    if (segments.includes("--oracle-only")) return;
    const stdioOnly = segments.includes("--stdio-only");
    const options: Parameters<typeof runAdmittedExactCargoLawsV1>[0] = { control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, ...exactCargoStageEnvironments(), groups: [
        {
          package: "semio-hub-stdio",
          target: { kind: "test", name: "native_openable_provider" },
          laws: ["native_codec_projection_pack_schema_hashes_equal_live_receipts", "las_native_receipt_binds_canonical_graph_and_independent_blake3", "native_composition_and_validation_claims_are_disjoint_but_each_exclusive", "artifact_owned_native_codec_receipts_form_one_complete_static_bijection", "native_catalog_matches_every_decoded_descriptor_kind_without_guest_app_assembly", "native_catalog_commitment_covers_all_definition_semantics_and_codec_authorities", "generic_plugin_builder_preserves_domain_owned_topic_contributions", "native_catalog_dependency_is_exactly_its_compiled_owner"],
        },
        {
          package: "semio-hub-stdio-catalog",
          target: { kind: "lib", name: "semio_hub_stdio_catalog" },
          cargoArgs: [],
          laws: ["catalog_projection_budget_matches_serde_and_refuses_before_overdraw", "catalog_projection_preflight_matches_actual_serde_payload_bytes"],
        },
        {
          package: "semio-hub-native-openable",
          target: { kind: "lib" },
          laws: ["native_openable_provider_consumes_exact_complete_stdio_factory_closure", "native_openable_provider_rejects_missing_extra_and_duplicate_receipts_without_publication", "native_openable_provider_rejects_identity_hash_schema_and_factory_substitution", "vcs_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication", "linked_provider_set_previews_only_the_selected_packages_of_a_stdio_gis_or_stdio_gis_vcs_profile", "linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog"],
        },
        {
          package: "semio-hub-trusted-catalog-laws",
          target: { kind: "lib" },
          laws: ["descriptor_owned_surface_is_required_before_any_catalog_or_codec_publication", "linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure", "trusted_descriptor_wire_materialization_matches_neutral_boundaries", "local_stdio_gis_profile_is_exact_two_packages_thirty_one_codecs_and_opens_every_package_target", "trusted_profile_generation_binds_zero_target_package_and_every_codec_row"],
        },
        { package: "semio-hub-hostile-input-laws", target: { kind: "lib" }, laws: ["native_openable_stdio_provider_is_the_only_atomic_readiness_transition"] },
      ], progress(event) {
        console.log(`native-openable-provider ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
      } };
    const [integration, ...remaining] = stdioOnly ? options.groups.filter((group) => group.package.startsWith("semio-hub-stdio")) : options.groups;
    if (!integration) throw new Error("native Stdio integration target is absent");
    const receipts = [...(await runAdmittedExactCargoLawsV1({...options,groups:[integration],consume:outputs=>withExactCargoLawOutputV1(outputs,{version:1,receiptIndex:0},async task=>{
      await proveNativeStdioCommitmentSchema(this.repoRoot,task.receipt);
      await proveHeadlessStdioImports(this.invocation,this.repoRoot,task);
    })}))];
    receipts.push(...await runAdmittedExactCargoLawsV1({ ...options, groups: remaining }));
    for (const receipt of receipts) console.log(`native-openable-provider-receipt: ${JSON.stringify(receipt)}`);
    console.log(`native-openable-catalog-provider-laws: passed=${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)}`);
    console.log(stdioOnly
      ? "native-openable-catalog-provider-check: headless full Stdio catalog only; no Hub, component or client-mount claim"
      : "native-openable-catalog-provider-check: complete stdio+GIS+VCS linked closure, synthetic descriptor-owned JSON viewer and isolated readiness journey; no all-plugin, immutable-bundle or client-mount claim");
  }
}

export class NativeCatalogSelectionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("unsupported native catalog selection argument");
    headlessStdioCommandRoots(this.repoRoot);
    proveHeadlessStdioCommandIsolation(this.repoRoot);
    await proveTrustedCompiledDependenciesFixture(this.repoRoot);
    runCmd("bun", ["repo", "run", "@semio-tech/plugin-registry:native-catalog-selection-check", "--skip-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    if (segments.includes("--oracle-only")) return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, ...exactCargoStageEnvironments(), groups: [
        {
          package: "semio-hub-trusted-catalog-laws",
          target: { kind: "lib" },
          laws: ["selected_native_providers_are_descriptor_verified_dependency_first_and_only_selected", "selected_native_provider_failure_substitution_and_conflict_publish_no_partial_closure", "selected_native_provider_descriptor_and_cancellation_fences_precede_publication", "linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure", "trusted_descriptor_wire_materialization_matches_neutral_boundaries", "local_stdio_gis_profile_is_exact_two_packages_thirty_one_codecs_and_opens_every_package_target", "trusted_profile_generation_binds_zero_target_package_and_every_codec_row"],
        },
        {
          package: "semio-hub-native-openable",
          target: { kind: "lib" },
          laws: ["linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog"],
        },
      ], progress(event) {
        console.log(`native-catalog-selection ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      } });
    console.log(`native-catalog-selection-laws: ${JSON.stringify(receipts)}`);
    console.log("native-catalog-selection-check: selected-only loader admission, compiled dependency and catalog commitments, complete two-package publication; synthetic component bytes only, no client activation claim");
  }
}
