/** 🧭️ Document open-plan fixtures and the open-plan gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { parseDocumentOpenIntentV1, parseDocumentOpenPlanV1, parseDocumentPlanSocketGrantIntentV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { componentReceiptBindsV1, componentReceiptValueV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { runCargo } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts";
import { runCmd, orchestratorBudgetOpts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runProbe } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔍️probe/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { documentOpenNeutralObject, documentOpenNeutralText } from "../../🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims/🟦️.ts";
import { readHubBinSource, readTrustedCatalogSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

export class CanonicalPairCheckScript extends BundleScript {
  run(): void {
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-hub-lag-rebootstrap", "--lib", "canonical_pair", "--", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-hub-boot-laws", "--lib", "canonical_pair_route", "--", "--test-threads=1"], this.root, env);
    runCmd("bun", ["repo", "run", "os-hub-ts:test", "--", "quick", "--run", "-t", "canonical checkpoint pair neutral contract"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root, env);
  }
}

function documentOpenLengthPrefix(bytes: Uint8Array): Buffer {
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(bytes.byteLength));
  return Buffer.concat([length, bytes]);
}

function documentOpenDescriptorEncoding(descriptor: Record<string, any>): Buffer {
  const text = (value: unknown): Buffer => {
    if (typeof value !== "string" || value.length === 0) throw new Error("document-open oracle invalid descriptor text");
    return Buffer.from(value, "utf8");
  };
  const hash = (value: unknown): Buffer => {
    if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || /^0{64}$/u.test(value)) throw new Error("document-open oracle invalid descriptor hash");
    return Buffer.from(value, "hex");
  };
  const integer = (value: unknown, width: 4 | 8): Buffer => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("document-open oracle invalid descriptor integer");
    const output = Buffer.alloc(width);
    if (width === 4) output.writeUInt32BE(value);
    else output.writeBigUInt64BE(BigInt(value));
    return output;
  };
  const fields = [
    text(descriptor.spaceId),
    text(descriptor.documentId),
    text(descriptor.artifactKind),
    text(descriptor.artifactSchema),
    text(descriptor.owner?.pluginId),
    text(descriptor.owner?.packageId),
    text(descriptor.owner?.version),
    hash(descriptor.owner?.packageHash),
    hash(descriptor.packSchemaHash),
    integer(descriptor.bootstrapVersion, 4),
    integer(descriptor.bootstrapFrontier?.headSeq, 8),
    integer(descriptor.bootstrapFrontier?.commitSeq, 8),
    integer(descriptor.bootstrapFrontier?.epoch, 8),
    hash(descriptor.bootstrapSnapshotHash),
  ];
  return Buffer.concat([Buffer.from("semio.document-descriptor.digest.v1\0"), ...fields.map(documentOpenLengthPrefix)]);
}

function documentOpenCatalogEncoding(rows: readonly Record<string, any>[]): Buffer {
  if (rows.length === 0 || rows.length > 1_024) throw new Error("document-open oracle invalid catalog size");
  const rowCount = Buffer.alloc(4);
  rowCount.writeUInt32BE(rows.length);
  const encodedRows = rows.map((row) => {
    documentOpenNeutralParentDialect(row);
    const fields = [
      Buffer.from(row.package.pluginId, "utf8"),
      Buffer.from(row.package.packageId, "utf8"),
      Buffer.from(row.package.version, "utf8"),
      Buffer.from(row.package.componentSha256, "hex"),
      Buffer.from(row.package.componentBlake3, "hex"),
      Buffer.from(row.package.descriptorByteSha256, "hex"),
      (() => {
        const version = row.package.executionProtocol?.appChannelVersion;
        if (version !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error("document-open oracle unsupported execution protocol");
        const encoded = Buffer.alloc(4);
        encoded.writeUInt32BE(version);
        return encoded;
      })(),
      Buffer.from(row.artifact.kind, "utf8"),
      Buffer.from(row.artifact.schema, "utf8"),
      Buffer.from(row.artifact.packSchemaHash, "hex"),
      Buffer.from(row.parentDialect.artifactKind, "utf8"),
      Buffer.from(row.parentDialect.standard, "utf8"),
      Buffer.from(row.parentDialect.subset, "utf8"),
      Buffer.from(row.surface.surfaceId, "utf8"),
      Buffer.from(row.surface.appId, "utf8"),
      Buffer.from(row.surface.windowKindId, "utf8"),
      Buffer.from(row.surface.role, "utf8"),
      Buffer.from(row.surface.rendererTarget, "utf8"),
      Buffer.from([row.grant.read ? 1 : 0, row.grant.write ? 1 : 0, row.grant.observe ? 1 : 0]),
    ];
    return Buffer.concat(fields.map(documentOpenLengthPrefix));
  });
  return Buffer.concat([Buffer.from("semio/hub/openable-document-catalog/v1\0"), rowCount, ...encodedRows]);
}

export function documentOpenMutation(value: Record<string, any>, path: string, replacement: unknown): Record<string, any> {
  const mutated = structuredClone(value);
  const components = path.split(".");
  let target = mutated;
  for (const component of components.slice(0, -1)) {
    if (target[component] === null || typeof target[component] !== "object") throw new Error(`document-open oracle invalid mutation path ${path}`);
    target = target[component];
  }
  target[components.at(-1)!] = replacement;
  return mutated;
}

function documentOpenRemoval(value: Record<string, any>, path: string): Record<string, any> {
  const mutated = structuredClone(value);
  const components = path.split(".");
  let target = mutated;
  for (const component of components.slice(0, -1)) {
    if (target[component] === null || typeof target[component] !== "object") throw new Error(`document-open oracle invalid removal path ${path}`);
    target = target[component];
  }
  delete target[components.at(-1)!];
  return mutated;
}

function documentOpenNeutralDialect(value: unknown): Record<string, string> {
  const dialect = documentOpenNeutralObject(value, ["artifactKind", "standard", "subset"]);
  for (const value of Object.values(dialect)) {
    const text = documentOpenNeutralText(value);
    if (text.trim() !== text) throw new Error("dialect-text");
  }
  return dialect as Record<string, string>;
}

function documentOpenNeutralParentDialect(value: unknown): Record<string, string> {
  const row = documentOpenNeutralObject(value, ["package", "artifact", "parentDialect", "surface", "browserActor", "grant"]);
  documentOpenNeutralObject(row.artifact, ["kind", "schema", "packSchemaHash"]);
  return documentOpenNeutralDialect(row.parentDialect);
}

/** 🌌️ Independent actor admission for complete plan and lease corpus consumers. */
export function documentOpenNeutralBrowserActor(actor: unknown, packageValue: Record<string, any>, renderer: unknown, lease: boolean): void {
  const row = documentOpenNeutralObject(actor, ["kind"], ["schema", "codegenPolicy", "sha256", "sourceComponentSha256", "sourceDescriptorByteSha256", "policySha256", "importInterfaces", ...(lease ? ["byteLength"] : [])]);
  if (row.kind === "none") {
    if (Object.keys(row).length !== 1 || !["react", "wgpu"].includes(String(renderer))) throw new Error("actor-none");
    return;
  }
  const keys = ["kind", "schema", "codegenPolicy", "sha256", "sourceComponentSha256", "sourceDescriptorByteSha256", "policySha256", "importInterfaces", ...(lease ? ["byteLength"] : [])];
  documentOpenNeutralObject(row, keys);
  const allowed = [
    "semio:framework/host-async@1.0.0",
    "semio:framework/pure@1.0.0",
    ...[
      "cli/environment",
      "cli/exit",
      "cli/stderr",
      "cli/stdin",
      "cli/stdout",
      "cli/terminal-input",
      "cli/terminal-output",
      "cli/terminal-stderr",
      "cli/terminal-stdin",
      "cli/terminal-stdout",
      "clocks/monotonic-clock",
      "clocks/wall-clock",
      "io/error",
      "io/poll",
      "io/streams",
    ].map((name) => `wasi:${name}@0.2.0`),
    "wasi:random/insecure-seed@0.2.9",
    "wasi:random/random@0.2.9",
  ];
  if (row.kind !== "closed-browser-actor" || renderer !== "wasm" || row.schema !== "semio.os.closed-browser-actor.v1" || row.codegenPolicy !== "semio.os.browser-jco-1.34.0-jspi.v1") throw new Error("actor-policy");
  if (![row.sha256, row.sourceComponentSha256, row.sourceDescriptorByteSha256, row.policySha256].every((value) => typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value))) throw new Error("actor-digest");
  if (row.sourceComponentSha256 !== packageValue.componentSha256 || row.sourceDescriptorByteSha256 !== packageValue.descriptorByteSha256) throw new Error("actor-source");
  if (
    !Array.isArray(row.importInterfaces) ||
    row.importInterfaces.length > 19 ||
    row.importInterfaces.some((value: unknown, index: number) => typeof value !== "string" || !allowed.includes(value) || (index > 0 && row.importInterfaces[index - 1] >= value))
  )
    throw new Error("actor-import");
  if (lease && (!Number.isSafeInteger(row.byteLength) || row.byteLength < 1 || row.byteLength > 67_108_864)) throw new Error("actor-length");
}

function documentOpenNeutralReceipt(value: unknown): string {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
  if (typeof value !== "string" || !/^open\.v1\.[A-Za-z0-9_-]{43}$/u.test(value) || (alphabet.indexOf(value.at(-1)!) & 0b11) !== 0) throw new Error("receipt");
  return value;
}

function documentOpenNeutralIntent(candidate: unknown): void {
  const root = documentOpenNeutralObject(candidate, ["schema", "version", "scope", "clientInstanceId"], ["requestedSurfaceId"]);
  if (root.schema !== "semio.hub.document-open-intent/v1" || root.version !== 1) throw new Error("intent-version");
  const scope = documentOpenNeutralObject(root.scope, ["spaceId", "documentId"]);
  documentOpenNeutralText(scope.spaceId);
  documentOpenNeutralText(scope.documentId);
  documentOpenNeutralText(root.clientInstanceId, 128);
  if (root.requestedSurfaceId !== undefined) documentOpenNeutralText(root.requestedSurfaceId);
}

function documentOpenNeutralExchange(candidate: unknown): void {
  const root = documentOpenNeutralObject(candidate, ["schema", "version", "planReceipt"]);
  if (root.schema !== "semio.hub.document-plan-socket-grant-intent/v1" || root.version !== 1) throw new Error("exchange-version");
  documentOpenNeutralReceipt(root.planReceipt);
}

/** 📦️ The neutral oracle's document-open package: exact members, the current execution protocol, and a shipped component
 * receipt binding the package's identity, component bytes and semantic descriptor bytes. */
function documentOpenNeutralPackage(value: unknown): Record<string, any> {
  const packageValue = documentOpenNeutralObject(value, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256", "executionProtocol", "componentReceipt"]);
  const executionProtocol = documentOpenNeutralObject(packageValue.executionProtocol, ["appChannelVersion"]);
  if (executionProtocol.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error("execution-protocol");
  documentOpenNeutralText(packageValue.pluginId);
  documentOpenNeutralText(packageValue.packageId);
  if (!componentReceiptBindsV1(componentReceiptValueV1(packageValue.componentReceipt), { profile: "release", pluginId: packageValue.pluginId, packageId: packageValue.packageId, wasmSha256: packageValue.componentSha256, semanticDescriptorSha256: packageValue.descriptorByteSha256 })) throw new Error("component-receipt");
  return packageValue;
}

function documentOpenNeutralIssueOutcome(candidate: Record<string, any>, subjectKind: unknown, role: unknown, fixture: Record<string, any>): { code: string; surfaceId?: string; write?: boolean } {
  try {
    documentOpenNeutralIntent(candidate);
  } catch {
    return { code: "denied" };
  }
  if (candidate.scope.spaceId !== fixture.descriptor.spaceId || candidate.scope.documentId !== fixture.descriptor.documentId) return { code: "denied" };
  if (subjectKind !== "share" && (subjectKind !== "session" || !new Set<unknown>(["author", "spectator"]).has(role))) return { code: "denied" };
  const writable = subjectKind === "session" && role === "author";
  const matches = fixture.catalogRows.filter((row: Record<string, any>) => {
    try {
      documentOpenNeutralParentDialect(row);
      const packageValue = documentOpenNeutralPackage(row.package);
      const artifact = documentOpenNeutralObject(row.artifact, ["kind", "schema", "packSchemaHash"]);
      const surface = documentOpenNeutralObject(row.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
      documentOpenNeutralBrowserActor(row.browserActor, packageValue, surface.rendererTarget, false);
      const grant = documentOpenNeutralObject(row.grant, ["read", "write", "observe"]);
      for (const text of [packageValue.pluginId, packageValue.packageId, packageValue.version, artifact.kind, artifact.schema, surface.surfaceId, surface.appId, surface.windowKindId]) documentOpenNeutralText(text);
      for (const digest of [packageValue.componentSha256, packageValue.componentBlake3, packageValue.descriptorByteSha256, artifact.packSchemaHash]) {
        if (typeof digest !== "string" || !/^[0-9a-f]{64}$/u.test(digest) || /^0{64}$/u.test(digest)) throw new Error("digest");
      }
      if (!new Set(["viewer", "editor"]).has(surface.role) || !new Set(["react", "wgpu", "wasm"]).has(surface.rendererTarget)) throw new Error("surface");
      if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean" || grant.write !== (surface.role === "editor")) throw new Error("grant");
      return (
        packageValue.pluginId === fixture.descriptor.owner.pluginId &&
        packageValue.packageId === fixture.descriptor.owner.packageId &&
        packageValue.version === fixture.descriptor.owner.version &&
        packageValue.componentSha256 === fixture.descriptor.owner.packageHash &&
        artifact.kind === fixture.descriptor.artifactKind &&
        artifact.schema === fixture.descriptor.artifactSchema &&
        artifact.packSchemaHash === fixture.descriptor.packSchemaHash &&
        grant.write === writable &&
        (candidate.requestedSurfaceId === undefined || candidate.requestedSurfaceId === surface.surfaceId)
      );
    } catch {
      return false;
    }
  });
  if (matches.length !== 1) return { code: "component-unavailable" };
  return { code: "accepted", surfaceId: matches[0].surface.surfaceId, write: matches[0].grant.write };
}

export function documentOpenNeutralStructure(candidate: Record<string, any>, nowMs: number): void {
  const hash = (value: unknown): string => {
    if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || /^0{64}$/u.test(value)) throw new Error("hash");
    return value;
  };
  const integer = (value: unknown, positive = false): number => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("integer");
    return value;
  };
  const root = documentOpenNeutralObject(
    candidate,
    ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "parentDialect", "surface", "browserActor", "grant", "checkpoint", "revalidation"],
  );
  if (root.schema !== "semio.hub.document-open-plan/v1" || root.version !== 1) throw new Error("version");
  documentOpenNeutralReceipt(root.receipt);
  const expiry = integer(root.expiresAtUnixMs, true);
  if (expiry <= nowMs) throw new Error("expired");
  if (expiry - nowMs > 30_000) throw new Error("ttl");
  const scope = documentOpenNeutralObject(root.scope, ["spaceId", "documentId"]);
  documentOpenNeutralText(scope.spaceId);
  documentOpenNeutralText(scope.documentId);
  hash(root.descriptorDigestV1);
  const catalog = documentOpenNeutralObject(root.catalog, ["generationId"]);
  hash(catalog.generationId);
  const packageValue = documentOpenNeutralPackage(root.package);
  documentOpenNeutralText(packageValue.version);
  hash(packageValue.componentSha256);
  hash(packageValue.componentBlake3);
  hash(packageValue.descriptorByteSha256);
  const artifact = documentOpenNeutralObject(root.artifact, ["kind", "schema", "packSchemaHash"]);
  documentOpenNeutralText(artifact.kind);
  documentOpenNeutralText(artifact.schema);
  hash(artifact.packSchemaHash);
  documentOpenNeutralDialect(root.parentDialect);
  const surface = documentOpenNeutralObject(root.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  documentOpenNeutralText(surface.surfaceId);
  documentOpenNeutralText(surface.appId);
  documentOpenNeutralText(surface.windowKindId);
  if (!new Set(["viewer", "editor"]).has(surface.role) || !new Set(["react", "wgpu", "wasm"]).has(surface.rendererTarget)) throw new Error("surface");
  documentOpenNeutralBrowserActor(root.browserActor, packageValue, surface.rendererTarget, false);
  const grant = documentOpenNeutralObject(root.grant, ["read", "write", "observe"]);
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean" || grant.write !== (surface.role === "editor")) throw new Error("grant");
  const revalidation = documentOpenNeutralObject(root.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  integer(revalidation.directoryRevision, true);
  integer(revalidation.membershipGeneration, true);
  if ((revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("binding");
  if (revalidation.sessionGeneration !== undefined) integer(revalidation.sessionGeneration, true);
  if (revalidation.shareGeneration !== undefined) integer(revalidation.shareGeneration, true);
  const checkpoint = documentOpenNeutralObject(root.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  hash(checkpoint.checkpointId);
  hash(checkpoint.descriptorDigestV1);
  hash(checkpoint.aggregateSha256);
  if (checkpoint.descriptorDigestV1 !== root.descriptorDigestV1) throw new Error("checkpoint-digest");
  const frontier = documentOpenNeutralObject(checkpoint.baselineFrontier, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
  documentOpenNeutralText(frontier.documentId);
  if (typeof frontier.headEditId !== "string") throw new Error("frontier");
  if (frontier.headEditId.length > 0) documentOpenNeutralText(frontier.headEditId);
  integer(frontier.headEditOrdinal);
  integer(frontier.lastCommitSeq);
  const chainIsCanonical = Array.isArray(frontier.chainHash) && frontier.chainHash.length === 32
    && frontier.chainHash.every((byte: unknown) => typeof byte === "number" && Number.isInteger(byte) && byte >= 0 && byte <= 255);
  const chainIsZero = chainIsCanonical && frontier.chainHash.every((byte: number) => byte === 0);
  const scopeBound = frontier.documentId === scope.documentId;
  const genesis = scopeBound && frontier.headEditOrdinal === 0 && frontier.headEditId === "" && frontier.lastCommitSeq === 0 && chainIsZero;
  const edited = scopeBound && frontier.headEditOrdinal > 0 && frontier.headEditId.length > 0 && frontier.lastCommitSeq > 0 && frontier.lastCommitSeq <= frontier.headEditOrdinal && chainIsCanonical && !chainIsZero;
  if (!genesis && !edited) throw new Error("frontier");
}

function documentOpenNeutralOutcome(candidate: Record<string, any>, fixture: Record<string, any>): string {
  try {
    documentOpenNeutralStructure(candidate, fixture.nowMs);
  } catch {
    return typeof candidate.expiresAtUnixMs === "number" && candidate.expiresAtUnixMs <= fixture.nowMs ? "expired" : "denied";
  }
  const selected = fixture.validPlan;
  const exact =
    candidate.scope?.spaceId === selected.scope.spaceId &&
    candidate.scope?.documentId === selected.scope.documentId &&
    candidate.descriptorDigestV1 === selected.descriptorDigestV1 &&
    candidate.catalog?.generationId === selected.catalog.generationId &&
    JSON.stringify(candidate.package) === JSON.stringify(selected.package) &&
    JSON.stringify(candidate.artifact) === JSON.stringify(selected.artifact) &&
    JSON.stringify(candidate.parentDialect) === JSON.stringify(selected.parentDialect) &&
    JSON.stringify(candidate.surface) === JSON.stringify(selected.surface) &&
    JSON.stringify(candidate.browserActor) === JSON.stringify(selected.browserActor) &&
    JSON.stringify(candidate.grant) === JSON.stringify(selected.grant) &&
    JSON.stringify(candidate.checkpoint) === JSON.stringify(selected.checkpoint) &&
    JSON.stringify(candidate.revalidation) === JSON.stringify(selected.revalidation);
  return exact ? "accepted" : "stale";
}

function documentOpenNeutralSocketConsumeOutcome(candidate: Record<string, any>, dialSurfaceId: unknown, fixture: Record<string, any>): string {
  try {
    documentOpenNeutralStructure(candidate, fixture.nowMs);
    documentOpenNeutralText(dialSurfaceId);
  } catch {
    return "denied";
  }
  const current = fixture.validPlan;
  const matches = fixture.catalogRows.filter(
    (row: Record<string, any>) =>
      JSON.stringify(row.package) === JSON.stringify(candidate.package) &&
      JSON.stringify(row.artifact) === JSON.stringify(candidate.artifact) &&
      JSON.stringify(row.parentDialect) === JSON.stringify(candidate.parentDialect) &&
      JSON.stringify(row.surface) === JSON.stringify(candidate.surface) &&
      JSON.stringify(row.browserActor) === JSON.stringify(candidate.browserActor) &&
      JSON.stringify(row.grant) === JSON.stringify(candidate.grant),
  );
  const exact =
    candidate.scope.spaceId === fixture.descriptor.spaceId &&
    candidate.scope.documentId === fixture.descriptor.documentId &&
    candidate.descriptorDigestV1 === fixture.descriptorDigestV1 &&
    candidate.catalog.generationId === fixture.catalogEncoding.expectedGenerationId &&
    candidate.surface.surfaceId === dialSurfaceId &&
    matches.length === 1 &&
    JSON.stringify(candidate.revalidation) === JSON.stringify(current.revalidation) &&
    JSON.stringify(candidate.checkpoint) === JSON.stringify(current.checkpoint);
  return exact ? "accepted" : "denied";
}

function documentOpenPublicKeysAreRedacted(value: unknown): boolean {
  if (Array.isArray(value)) return value.every(documentOpenPublicKeysAreRedacted);
  if (value === null || typeof value !== "object") return true;
  const forbidden = new Set(["actor", "actorId", "token", "descriptor", "componentBytes", "descriptorBytes", "storageKey", "factory", "factorySymbol", "sessionId", "shareId", "catalogRows"]);
  return Object.entries(value).every(([key, child]) => !forbidden.has(key) && documentOpenPublicKeysAreRedacted(child));
}

export async function proveDocumentOpenPlanFixture(repoRoot: string): Promise<void> {
  const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Record<string, any>;
  const hubSource = readHubBinSource(repoRoot);
  const routePaths = [...hubSource.matchAll(/\.route\(\s*"([^"]+)"/g)].map((match) => match[1]!);
  const productionSource = hubSource.slice(0, hubSource.indexOf("\nmod tests {"));
  const catalogSource = readTrustedCatalogSource(repoRoot);
  if (
    !catalogSource.includes("pub parent_dialect: semio_framework::ArtifactDialect") ||
    !productionSource.includes("parent_dialect: selected.parent_dialect") ||
    !productionSource.includes("parent_dialect: DocumentOpenParentDialectV1 {") ||
    !productionSource.includes("selection.parent_dialect != authority.parent_dialect") ||
    !productionSource.includes("selected.parent_dialect != authority.parent_dialect")
  )
    throw new Error("document-open authority must retain and revalidate the verified full parent dialect");
  if (routePaths.filter((path) => path === "/spaces/{space_id}/documents/{id}/open-plan").length !== 1) throw new Error("document-open verified-catalog issuer route is not mounted exactly once");
  if (routePaths.filter((path) => path === "/spaces/{space_id}/documents/{id}/socket-grants").length !== 1) throw new Error("document-open plan exchange route is not mounted exactly once");
  if (
    !productionSource.includes("issue_document_open_plan") ||
    !productionSource.includes("DocumentOpenCatalogAuthorityV1") ||
    !productionSource.includes("resolve_document_open(&descriptor") ||
    !productionSource.includes("open_plan: open_plan_ready") ||
    !productionSource.includes("open_plan_exchange: open_plan_ready") ||
    !productionSource.includes("DocumentPlanSocketGrantIntentV1") ||
    !productionSource.includes("issue_document_plan_socket_grant") ||
    !productionSource.includes("authority_for_authenticated_exchange") ||
    !productionSource.includes("document_plan_socket_validity") ||
    !productionSource.match(/if checkpoint\.as_ref\(\) == Some\(&authority\.checkpoint\) \{\s*DocumentPlanBootstrapV1::Current\s*\} else \{\s*DocumentPlanBootstrapV1::Stale\s*\}/u) ||
    productionSource.includes("post(issue_document_socket_grant)")
  )
    throw new Error("document-open catalog-gated issuer and exchange activation boundary drifted");
  const descriptorEncoding = documentOpenDescriptorEncoding(fixture.descriptor);
  const catalogEncoding = documentOpenCatalogEncoding(fixture.catalogRows);
  if (createHash("sha256").update(descriptorEncoding).digest("hex") !== fixture.descriptorDigestV1) throw new Error("document-open descriptor digest oracle mismatch");
  if (catalogEncoding.toString("hex") !== fixture.catalogEncoding.expectedHex || createHash("sha256").update(catalogEncoding).digest("hex") !== fixture.catalogEncoding.expectedGenerationId)
    throw new Error("document-open catalog generation oracle mismatch");
  const dialectFields = ["parentDialect.artifactKind", "parentDialect.standard", "parentDialect.subset"];
  if (fixture.catalogEncoding.fieldOrder[6] !== "package.executionProtocol.appChannelVersion") throw new Error("document-open execution protocol framing drifted");
  if (JSON.stringify(fixture.catalogEncoding.fieldOrder.slice(10, 13)) !== JSON.stringify(dialectFields)) throw new Error("document-open parent dialect framing drifted");
  for (const field of dialectFields) {
    const rows = structuredClone(fixture.catalogRows);
    const key = field.split(".")[1]!;
    rows[0].parentDialect[key] += "-foreign";
    if (createHash("sha256").update(documentOpenCatalogEncoding(rows)).digest("hex") === fixture.catalogEncoding.expectedGenerationId) throw new Error(`document-open catalog lost ${field}`);
  }
  for (const mutation of fixture.parentDialectNegativeMutations) {
    const value = Object.hasOwn(mutation, "unit") ? mutation.unit.repeat(mutation.repetitions) : mutation.value;
    const row = documentOpenMutation(fixture.catalogRows[0], mutation.path, value);
    let rejected = false;
    try {
      documentOpenCatalogEncoding([row]);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error(`document-open parent dialect admitted hostile ${mutation.path}`);
    const candidateFixture = { ...fixture, catalogRows: [row] };
    if (documentOpenNeutralIssueOutcome(fixture.intent, "session", "author", candidateFixture).code !== "component-unavailable") throw new Error(`document-open issuer admitted hostile ${mutation.path}`);
  }
  console.log(`verified parent dialect source:18 fields,3 digest substitutions,${fixture.parentDialectNegativeMutations.length} hostile rows,public plan+private exchange+socket equality; native authority unverified`);
  const receiptSecret = Buffer.from(fixture.receiptDigest.receipt.slice("open.v1.".length), "base64url");
  try {
    if (receiptSecret.length !== 32 || `open.v1.${receiptSecret.toString("base64url")}` !== fixture.receiptDigest.receipt) throw new Error("document-open receipt secret grammar mismatch");
    if (createHash("sha256").update(Buffer.from(fixture.receiptDigest.domainUtf8)).update(receiptSecret).digest("hex") !== fixture.receiptDigest.expectedHex) throw new Error("document-open receipt digest oracle mismatch");
  } finally {
    receiptSecret.fill(0);
  }
  documentOpenNeutralIntent(fixture.intent);
  for (const issueCase of fixture.issueCases) {
    let candidate = structuredClone(fixture.intent);
    if (issueCase.replacePath !== undefined) candidate = documentOpenMutation(candidate, issueCase.replacePath, issueCase.value);
    if (issueCase.removePath !== undefined) candidate = documentOpenRemoval(candidate, issueCase.removePath);
    const outcome = documentOpenNeutralIssueOutcome(candidate, issueCase.subjectKind, issueCase.role, fixture);
    if (outcome.code !== issueCase.expected || outcome.surfaceId !== issueCase.expectedSurfaceId || outcome.write !== issueCase.expectedWrite)
      throw new Error(`document-open issuer vector ${issueCase.name} expected ${issueCase.expected}/${issueCase.expectedSurfaceId ?? "-"}/${issueCase.expectedWrite ?? "-"}, got ${outcome.code}/${outcome.surfaceId ?? "-"}/${outcome.write ?? "-"}`);
  }
  documentOpenNeutralStructure(fixture.validPlan, fixture.nowMs);
  documentOpenNeutralExchange(fixture.exchangeIntent);
  if (!documentOpenPublicKeysAreRedacted(fixture.validPlan)) throw new Error("document-open public plan includes a private authority field");
  for (const mutation of fixture.negativeMutations) {
    const mutated = documentOpenMutation(fixture.validPlan, mutation.path, mutation.value);
    const outcome = documentOpenNeutralOutcome(mutated, fixture);
    if (outcome !== mutation.code) throw new Error(`document-open negative vector ${mutation.name} expected ${mutation.code}, got ${outcome}`);
  }
  for (const mutation of fixture.exchangeNegativeMutations) {
    const mutated = documentOpenMutation(fixture.exchangeIntent, mutation.path, mutation.value);
    let rejected = false;
    try {
      documentOpenNeutralExchange(mutated);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error(`document-open independent exchange codec admitted ${mutation.name}`);
  }
  for (const consumeCase of fixture.socketConsumeCases) {
    let candidate = structuredClone(fixture.validPlan);
    if (consumeCase.replacePath !== undefined) candidate = documentOpenMutation(candidate, consumeCase.replacePath, consumeCase.value);
    const outcome = documentOpenNeutralSocketConsumeOutcome(candidate, consumeCase.dialSurfaceId ?? candidate.surface.surfaceId, fixture);
    if (outcome !== consumeCase.expected) throw new Error(`document-open socket-consume vector ${consumeCase.name} expected ${consumeCase.expected}, got ${outcome}`);
  }
  console.log(
    `document-open-plan-oracle: descriptor=1 catalog=${fixture.catalogRows.length} receipt=1 independent-codecs=3 issuer=${fixture.issueCases.length} consume=${fixture.socketConsumeCases.length} negative=${fixture.negativeMutations.length} exchange-negative=${fixture.exchangeNegativeMutations.length} redaction=1 activation=catalog-gated-issuer+exchange passed`,
  );

  if (JSON.stringify(parseDocumentOpenIntentV1(fixture.intent)) !== JSON.stringify(fixture.intent)) throw new Error("document-open intent codec mismatch");
  if (JSON.stringify(parseDocumentOpenPlanV1(fixture.validPlan, fixture.nowMs)) !== JSON.stringify(fixture.validPlan)) throw new Error("document-open plan codec mismatch");
  if (JSON.stringify(parseDocumentPlanSocketGrantIntentV1(fixture.exchangeIntent)) !== JSON.stringify(fixture.exchangeIntent)) throw new Error("document-open exchange codec mismatch");
  for (const mutation of fixture.negativeMutations) {
    const mutated = documentOpenMutation(fixture.validPlan, mutation.path, mutation.value);
    if (mutation.code === "denied" || mutation.code === "expired") {
      let rejected = false;
      try {
        parseDocumentOpenPlanV1(mutated, fixture.nowMs);
      } catch {
        rejected = true;
      }
      if (!rejected) throw new Error(`document-open production codec admitted ${mutation.name}`);
    }
  }
  for (const mutation of fixture.exchangeNegativeMutations) {
    const mutated = documentOpenMutation(fixture.exchangeIntent, mutation.path, mutation.value);
    let rejected = false;
    try {
      parseDocumentPlanSocketGrantIntentV1(mutated);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error(`document-open production exchange codec admitted ${mutation.name}`);
  }
  console.log(
    `document-open-plan-production-parity: codecs=3 rejected=${fixture.negativeMutations.filter((mutation: Record<string, any>) => mutation.code === "denied" || mutation.code === "expired").length} exchange-rejected=${fixture.exchangeNegativeMutations.length} passed`,
  );
}

export class OpenPlanCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--source")) throw new Error("open-plan-check accepts only --source");
    await proveDocumentOpenPlanFixture(this.repoRoot);
    if (segments[0] === "--source") {
      console.log("open-plan-source-check: neutral fixture, independent framing and owned schema; no native or mounted browser acceptance");
      return;
    }
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--all-features", ...target, suffix, "--", "--list"], { cwd: this.root, env, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) {
        const related = listed.stdout
          .split("\n")
          .filter((line) => line.includes("document_open"))
          .join(",");
        const diagnostic = listed.stderr.trim().slice(-4_000);
        throw new Error(`open-plan-check expected exactly one ${suffix} law, selected ${matches.length}; status=${listed.status}; related=${related || "<none>"}; diagnostic=${diagnostic || "<none>"}`);
      }
      return matches[0]!;
    };
    const schemaTarget = ["-p", "semio-framework-os-kernel", "--lib"];
    const schemaLaw = exactLaw(schemaTarget, "document_open_plan_v1_matches_language_neutral_fixture");
    const catalogTarget = ["-p", "semio-hub-trusted-catalog-laws", "--lib"];
    const catalogLaw = exactLaw(catalogTarget, "verified_trusted_catalog_document_open_generation_and_resolution_are_exact");
    const ledgerLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped");
    const revocationLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_admin_revocation_invalidates_session_and_share_bindings");
    const exchangeLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_receipt_exchange_admits_one_exact_bounded_secret_free_socket_grant");
    const routeLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use");
    const wipeLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes");
    const issueLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable");
    const consumeLaw = exactLaw(["-p", "semio-hub-open-plan-laws", "--lib"], "document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint");
    console.log(`document-open-plan-laws: schema=${schemaLaw} catalog=${catalogLaw} ledger=${ledgerLaw} revocation=${revocationLaw} exchange=${exchangeLaw} route=${routeLaw} wipe=${wipeLaw} issue=${issueLaw} consume=${consumeLaw}`);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...schemaTarget, schemaLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...catalogTarget, catalogLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", ledgerLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", revocationLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", exchangeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", routeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", wipeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", issueLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-open-plan-laws", "--lib", consumeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
  }
}

export class OpenPlanServerCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("unsupported open-plan server argument");
    await proveDocumentOpenPlanFixture(this.repoRoot);
    if (segments.includes("--oracle-only")) return;
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", ...target, suffix, "--", "--list"], { cwd: this.root, env, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) {
        const diagnostic = listed.stderr.trim().slice(-4_000);
        throw new Error(`open-plan-server-check expected exactly one ${suffix} law, selected ${matches.length}; status=${listed.status}; diagnostic=${diagnostic || "<none>"}`);
      }
      return matches[0]!;
    };
    const laws = [
      { target: ["-p", "semio-hub-trusted-catalog-laws", "--lib"], suffix: "verified_trusted_catalog_document_open_generation_and_resolution_are_exact" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_admin_revocation_invalidates_session_and_share_bindings" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_receipt_exchange_admits_one_exact_bounded_secret_free_socket_grant" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint" },
    ].map((law) => ({ ...law, name: exactLaw(law.target, law.suffix) }));
    console.log(`document-open-plan-server-laws: qualification=default-feature-subset exact=${laws.length} laws=${laws.map((law) => law.name).join(",")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", ...law.target, law.name, "--", "--exact", "--test-threads=1"], this.root, env);
    console.log("document-open-plan-server-check: default-feature subset passed; kernel all-features schema qualification remains separate");
  }
}
