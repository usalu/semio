/** 🧭️ Trusted bootstrap descriptor claims, dependency closure, encodings and selection preflight.
 * @see ../../../📦️packages/🦀️rust/📜️script.ts
 */
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, type DocumentExecutionProtocolV1 } from "@semio-tech/framework/directory/schema";
import { parseDocumentOpenBrowserActorV1, documentBrowserActorLeaseFromPlanV1, type DocumentBrowserActorSourceV1, type DocumentClosedBrowserActorV1 } from "@semio-tech/framework/directory/schema/browser-actor";
import { componentReceiptValueV1, encodeComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt";
import { TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES } from "@semio-tech/plugin-registry/deployment/schema";
import { requireHubDatum } from "../../../🧪️testing/📖️source-readers/🟦️.ts";

export function documentOpenNeutralObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, any> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("object");
  const object = value as Record<string, any>;
  const keys = Object.keys(object);
  if (required.some((key) => !(key in object)) || keys.some((key) => !required.includes(key) && !optional.includes(key))) throw new Error("keys");
  return object;
}

export function documentOpenNeutralText(value: unknown, maximum = 256): string {
  if (typeof value !== "string" || value.length === 0 || Buffer.byteLength(value, "utf8") > maximum || /\p{Cc}/u.test(value)) throw new Error("text");
  return value;
}

export type TrustedBootstrapCodec = { readonly artifactKind: string; readonly artifactSchema: string; readonly packSchemaHash: string };
export type TrustedBootstrapIdentityV1 = Readonly<{ pluginId: string; packageId: string; version: string }>;
export type TrustedBootstrapDescriptorClaimsV1 = Readonly<{
  identity: TrustedBootstrapIdentityV1;
  executionProtocol: Readonly<DocumentExecutionProtocolV1>;
  directDependencies: readonly Readonly<{ pluginId: string; version: string }>[];
}>;

/** 🔢️ Requires canonical unsigned 64-bit version spelling for the strict local profile. */
function trustedBootstrapVersion(value: unknown): string {
  if (typeof value !== "string" || value.length > 62 || !/^(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)$/u.test(value) || value.split(".").some((part) => BigInt(part) > 18_446_744_073_709_551_615n)) throw new Error("trusted dependency version is not canonical u64");
  return value;
}

/** 🪪️ Copies one closed bounded concrete package identity. */
export function trustedBootstrapIdentity(value: unknown): TrustedBootstrapIdentityV1 {
  const row = documentOpenNeutralObject(value, ["pluginId", "packageId", "version"]);
  for (const key of ["pluginId", "packageId"] as const) if (typeof row[key] !== "string" || Buffer.byteLength(row[key], "utf8") > 256 || row[key].length === 0 || row[key].trim() !== row[key] || Buffer.from(row[key], "utf8").toString("utf8") !== row[key] || /\p{Cc}/u.test(row[key])) throw new Error("trusted dependency identity is not bounded");
  return Object.freeze({ pluginId: row.pluginId as string, packageId: row.packageId as string, version: trustedBootstrapVersion(row.version) });
}

/** 🔠️ Mirrors Rust string tuple ordering, including non-BMP UTF-8 identities. */
export function trustedBootstrapTupleOrder(left: readonly string[], right: readonly string[]): number {
  for (let index = 0; index < left.length; index++) {
    const order = Buffer.compare(Buffer.from(requireHubDatum(left[index],"left tuple component"), "utf8"), Buffer.from(requireHubDatum(right[index],"right tuple component"), "utf8"));
    if (order) return order;
  }
  return 0;
}

/** 🔗️ Owns a bounded unique dependency vector in native identity order. */
export function trustedBootstrapDependencies(value: unknown): readonly TrustedBootstrapIdentityV1[] {
  if (!Array.isArray(value) || value.length > 128) throw new Error("trusted dependency vector is not bounded");
  const rows = value.map(trustedBootstrapIdentity);
  if (new Set(rows.map((row) => row.pluginId)).size !== rows.length) throw new Error("trusted dependency identity is duplicated");
  return Object.freeze(rows.sort((left, right) => trustedBootstrapTupleOrder([left.pluginId, left.packageId, left.version], [right.pluginId, right.packageId, right.version])));
}

/** 🧾️ Retains dependency claims from the same bounded descriptor bytes as its protocol. */
export function trustedBootstrapDescriptorClaims(bytes: Uint8Array): TrustedBootstrapDescriptorClaimsV1 {
  if (bytes.byteLength === 0 || bytes.byteLength > DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES) throw new Error("trusted descriptor dependency bytes are not bounded");
  const decoded = decodePackValue(bytes);
  if (!Buffer.from(encodePackValue(decoded)).equals(Buffer.from(bytes))) throw new Error("trusted descriptor dependency Pack is not canonical or has duplicate fields");
  const descriptor = packValueToExactJson(decoded) as Record<string, unknown>;
  const manifest = descriptor?.manifest as Record<string, unknown>;
  if (!manifest || typeof manifest !== "object" || Array.isArray(manifest)) throw new Error("trusted descriptor manifest is missing");
  // 🔗️ `PluginManifest.dependencies` is `#[value(default, skip_serializing_if = "Vec::is_empty")]`
  // (`🛂️manifest/🦀️.rs:4418`), so a plugin that depends on nothing — `semio-hub-stdio` is exactly
  // that — emits a descriptor with NO `dependencies` key at all. Requiring the key present rejected
  // every dependency-free package, which is the whole trusted bootstrap set. Absent means the empty
  // vector, which is what the encoder's own contract says; the 128 bound still applies to a present one.
  const declaredDependencies = manifest.dependencies === undefined ? [] : manifest.dependencies;
  if (!Array.isArray(declaredDependencies) || declaredDependencies.length > 128) throw new Error("trusted descriptor manifest dependency vector is missing or unbounded");
  const identity = trustedBootstrapIdentity({ pluginId: manifest.pluginId, packageId: descriptor.packageId, version: manifest.version });
  const directDependencies = declaredDependencies.map((value) => {
    const row = documentOpenNeutralObject(value, ["pluginId", "version"]);
    if (typeof row.pluginId !== "string" || row.pluginId.length === 0 || Buffer.byteLength(row.pluginId, "utf8") > 256 || row.pluginId.trim() !== row.pluginId || Buffer.from(row.pluginId, "utf8").toString("utf8") !== row.pluginId || /\p{Cc}/u.test(row.pluginId) || typeof row.version !== "string" || !row.version.startsWith("=")) throw new Error("trusted descriptor dependency is not exact and bounded");
    trustedBootstrapVersion(row.version.slice(1));
    return Object.freeze({ pluginId: row.pluginId, version: row.version });
  });
  if (new Set(directDependencies.map((row) => row.pluginId)).size !== directDependencies.length) throw new Error("trusted descriptor dependency is duplicated");
  return Object.freeze({ identity, executionProtocol: trustedBootstrapExecutionProtocol(descriptor.executionProtocol), directDependencies: Object.freeze(directDependencies) });
}

/** 🧬️ Resolves one package's descriptor-attested dependencies against the selected closure. Every
 * dependency a package's own compiled manifest claims must be IN the closure at the exact compiled
 * version and must be another package; nothing outside it is admitted, and no package's edges are named
 * by this builder. The closure as a whole is held by [`trustedBootstrapResolveClosure`]. */
export function trustedBootstrapResolveDependencies(claims: TrustedBootstrapDescriptorClaimsV1, candidates: unknown): readonly TrustedBootstrapIdentityV1[] {
  const selected = trustedBootstrapDependencies(candidates);
  if (selected.length === 0) throw new Error("trusted dependency closure is empty");
  const owner = selected.find((row) => row.pluginId === claims.identity.pluginId);
  if (!owner || owner.packageId !== claims.identity.packageId || owner.version !== claims.identity.version) throw new Error("trusted descriptor dependency owner differs from its selected identity");
  const expected = claims.directDependencies.map((claimed) => {
    if (claimed.pluginId === claims.identity.pluginId) throw new Error(`trusted descriptor dependency ${claimed.pluginId} names its own package`);
    const resolved = selected.find((row) => row.pluginId === claimed.pluginId);
    if (!resolved || claimed.version !== `=${resolved.version}`) throw new Error(`trusted descriptor dependency ${claimed.pluginId} is outside the selected closure or pinned to another version`);
    return resolved;
  });
  return Object.freeze([...expected].sort((left, right) => trustedBootstrapTupleOrder([left.pluginId, left.packageId, left.version], [right.pluginId, right.packageId, right.version])));
}

/** 🔗️ Resolves a whole selected closure the way the hub's `validate_bundle` admits it (`🔏️trusted-catalog/🦀️.rs`): each
 * selected identity is exactly one package's own descriptor identity (a closure row naming another package id is foreign),
 * each package's dependencies resolve by [`trustedBootstrapResolveDependencies`], and the dependency graph is acyclic. The
 * linked profile `local-stdio-gis-open-v1` is also held to its exact edge set — GIS depends on stdio, stdio on nothing — as
 * the hub's profile fence is, so a publication fails where the hub would refuse to load it. Answers every package's
 * resolved dependencies by plugin id. */
export function trustedBootstrapResolveClosure(profileId: string, claims: ReadonlyMap<string, TrustedBootstrapDescriptorClaimsV1>, candidates: unknown): ReadonlyMap<string, readonly TrustedBootstrapIdentityV1[]> {
  const selected = trustedBootstrapDependencies(candidates);
  if (claims.size !== selected.length) throw new Error("trusted dependency closure and its package descriptors differ in count");
  const resolved = new Map<string, readonly TrustedBootstrapIdentityV1[]>();
  for (const identity of selected) {
    const own = claims.get(identity.pluginId);
    if (!own || own.identity.pluginId !== identity.pluginId || own.identity.packageId !== identity.packageId || own.identity.version !== identity.version) throw new Error(`trusted dependency closure names ${identity.pluginId} as another package than its own descriptor`);
    resolved.set(identity.pluginId, trustedBootstrapResolveDependencies(own, selected));
  }
  const pending = new Map([...resolved].map(([plugin, dependencies]) => [plugin, dependencies.length]));
  const ready = [...pending].filter(([, count]) => count === 0).map(([plugin]) => plugin);
  let ordered = 0;
  for (let plugin = ready.pop(); plugin !== undefined; plugin = ready.pop()) {
    ordered += 1;
    for (const [dependent, dependencies] of resolved) {
      if (!dependencies.some((dependency) => dependency.pluginId === plugin)) continue;
      const left = pending.get(dependent)! - 1;
      pending.set(dependent, left);
      if (left === 0) ready.push(dependent);
    }
  }
  if (ordered !== resolved.size) throw new Error("trusted dependency closure contains a cycle");
  if (profileId === "local-stdio-gis-open-v1" && (resolved.get("gis")?.map((dependency) => dependency.pluginId).join(",") !== "stdio" || resolved.get("stdio")?.length !== 0))
    throw new Error("local stdio plus GIS profile dependency closure is not exactly gis → stdio");
  return resolved;
}

/** 🧮️ Frames all resolved dependency identities in the native generation format. */
export function trustedBootstrapDependencyEncoding(value: unknown): Buffer {
  const dependencies = trustedBootstrapDependencies(value);
  return Buffer.concat([trustedBootstrapCount(dependencies.length), ...dependencies.flatMap((row) => [row.pluginId, row.packageId, row.version].map(trustedBootstrapField))]);
}

export function trustedBootstrapCodecOrder(left: TrustedBootstrapCodec, right: TrustedBootstrapCodec): number {
  return trustedBootstrapTupleOrder([left.artifactKind, left.artifactSchema, left.packSchemaHash], [right.artifactKind, right.artifactSchema, right.packSchemaHash]);
}

function trustedBootstrapField(value: Uint8Array | string): Buffer {
  const bytes = typeof value === "string" ? Buffer.from(value, "utf8") : Buffer.from(value);
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(bytes.byteLength));
  return Buffer.concat([length, bytes]);
}

function trustedBootstrapCount(value: number): Buffer {
  const bytes = Buffer.alloc(4);
  bytes.writeUInt32BE(value);
  return bytes;
}

/** 🔖️ Retains only the supported protocol attested by a compiled descriptor. */
export function trustedBootstrapExecutionProtocol(value: unknown): Readonly<DocumentExecutionProtocolV1> {
  const protocol = documentOpenNeutralObject(value, ["appChannelVersion"]);
  if (protocol.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error("trusted generation execution protocol is unsupported");
  return Object.freeze({ appChannelVersion: protocol.appChannelVersion });
}

export type TrustedBootstrapBrowserActorV1 = Readonly<{ kind: "none" }> | (DocumentClosedBrowserActorV1 & Readonly<{ path: string; byteLength: number }>);

/** 📦️ Admits private location metadata while preserving the public source-bound identity. */
export function trustedBootstrapBrowserActorV1(candidate: unknown, source: DocumentBrowserActorSourceV1, renderer: string): TrustedBootstrapBrowserActorV1 {
  const row = documentOpenNeutralObject(candidate, ["kind"], ["schema", "codegenPolicy", "sha256", "sourceComponentSha256", "sourceDescriptorByteSha256", "policySha256", "importInterfaces", "path", "byteLength"]);
  if (row.kind === "none") return parseDocumentOpenBrowserActorV1(row, source, renderer) as Readonly<{ kind: "none" }>;
  const { path, byteLength, ...identity } = row;
  const plan = parseDocumentOpenBrowserActorV1(identity, source, renderer);
  const lease = documentBrowserActorLeaseFromPlanV1(plan, source, renderer, byteLength);
  if (lease.kind !== "closed-browser-actor" || typeof path !== "string" || Buffer.byteLength(path, "utf8") > 1024 || /[\\:\p{Cc}]/u.test(path) || path.split("/").some((part) => part === "" || part === "." || part === ".."))
    throw new Error("trusted browser actor path or identity is invalid");
  return Object.freeze({ ...lease, path });
}

/** 🧬️ Frames every actor field, including explicit absence, in the immutable generation. */
export function trustedBootstrapBrowserActorEncoding(candidate: unknown, source: DocumentBrowserActorSourceV1, renderer: string): Buffer {
  const actor = trustedBootstrapBrowserActorV1(candidate, source, renderer);
  const pieces = [trustedBootstrapField(actor.kind)];
  if (actor.kind === "none") return Buffer.concat(pieces);
  for (const value of [actor.schema, actor.codegenPolicy, actor.path]) pieces.push(trustedBootstrapField(value));
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(actor.byteLength));
  pieces.push(length);
  for (const value of [actor.sha256, actor.sourceComponentSha256, actor.sourceDescriptorByteSha256, actor.policySha256]) pieces.push(trustedBootstrapField(Buffer.from(value, "hex")));
  pieces.push(trustedBootstrapCount(actor.importInterfaces.length));
  for (const value of actor.importInterfaces) pieces.push(trustedBootstrapField(value));
  return Buffer.concat(pieces);
}

/** 📇️ Every selected package's codec rows, keyed by plugin id — open to an N-package closure. */
type TrustedBootstrapCodecSetV1 = Readonly<Record<string, readonly TrustedBootstrapCodec[]>>;

/** 🎯️ One creatable document kind a package's own descriptor declares, in generation order. */
export type TrustedBootstrapOpenTargetV1 = Readonly<{
  pluginId: string;
  artifactKind: string;
  artifactSchema: string;
  packSchemaHash: string;
  surfaceId: string;
  appId: string;
  windowKindId: string;
  role: "editor" | "viewer";
  rendererTarget: "wasm";
  parentDialect: Readonly<{ artifactKind: string; standard: string; subset: string }>;
  grant: Readonly<{ read: boolean; write: boolean; observe: boolean }>;
}>;

/** 🖥️ Mirrors `package_actor_renderer` (`🔏️trusted-catalog/🦀️.rs`): a package that opens any document
 * carries a wasm actor, every other selected package a React one. Derived from the package's own
 * target set so a fourth package needs no renderer table. */
function trustedBootstrapPackageRenderer(profile: any, pluginId: string): string {
  return profile.openTargets.some((target: TrustedBootstrapOpenTargetV1) => target.pluginId === pluginId) ? "wasm" : "react";
}

/** 🔢️ Mirrors `trusted_profile_generation`'s open-target ordering byte for byte. */
function trustedBootstrapOpenTargetOrder(left: TrustedBootstrapOpenTargetV1, right: TrustedBootstrapOpenTargetV1, packages: any[]): number {
  const identity = (target: TrustedBootstrapOpenTargetV1): readonly string[] => {
    const owner = packages.find((candidate: any) => candidate.pluginId === target.pluginId);
    if (!owner) throw new Error("trusted bootstrap open target has no selected package");
    return [owner.pluginId, owner.packageId, owner.version, target.artifactKind, target.artifactSchema, target.surfaceId, target.role === "viewer" ? "" : ""];
  };
  return trustedBootstrapTupleOrder(identity(left), identity(right));
}

export function trustedBootstrapProfileEncoding(profile: any, codecs: TrustedBootstrapCodecSetV1): Buffer {
  const pieces = [Buffer.from("semio/hub/trusted-profile-generation/v2\0"), trustedBootstrapField(profile.id), trustedBootstrapCount(profile.selectedClosure.length)];
  for (const identity of profile.selectedClosure) {
    const selected = profile.packages.find((candidate: any) => candidate.pluginId === identity.pluginId && candidate.packageId === identity.packageId && candidate.version === identity.version);
    if (!selected) throw new Error("trusted bootstrap package is outside selected closure");
    for (const value of [selected.pluginId, selected.packageId, selected.version, selected.role, selected.componentSha256, selected.componentBlake3, selected.descriptorSha256]) {
      pieces.push(trustedBootstrapField(/^[0-9a-f]{64}$/u.test(value) ? Buffer.from(value, "hex") : value));
    }
    pieces.push(trustedBootstrapField(Buffer.from(encodeComponentReceiptV1(componentReceiptValueV1(selected.receipt)), "utf8")));
    pieces.push(trustedBootstrapField(trustedBootstrapCount(trustedBootstrapExecutionProtocol(selected.executionProtocol).appChannelVersion)));
    pieces.push(
      trustedBootstrapBrowserActorEncoding(selected.browserActor, { componentSha256: selected.componentSha256, descriptorByteSha256: selected.descriptorSha256 }, trustedBootstrapPackageRenderer(profile, selected.pluginId)),
    );
    pieces.push(trustedBootstrapPluginModuleEncoding(trustedBootstrapPluginModuleRecordV1(selected.pluginModule, selected.pluginId)));
    pieces.push(trustedBootstrapDependencyEncoding(selected.dependencies));
    const declared = codecs[selected.pluginId as string];
    if (!declared) throw new Error("trusted bootstrap selected package declares no codec closure");
    const rows = [...declared].sort(trustedBootstrapCodecOrder);
    pieces.push(trustedBootstrapCount(rows.length));
    for (const row of rows) pieces.push(trustedBootstrapField(row.artifactKind), trustedBootstrapField(row.artifactSchema), trustedBootstrapField(Buffer.from(row.packSchemaHash, "hex")));
  }
  // 🎯️ The whole SET behind its own count, in `trusted_profile_generation`'s canonical order (ticket
  // 26/09/18 slice TC3b). A one-target profile frames byte-identically to the retired single-target
  // encoding, so generalising this does not by itself rotate an existing generation id.
  const selectedTargets = [...(profile.openTargets as TrustedBootstrapOpenTargetV1[])].sort((left, right) => trustedBootstrapOpenTargetOrder(left, right, profile.packages));
  pieces.push(trustedBootstrapCount(selectedTargets.length));
  for (const target of selectedTargets) {
    const targetPackage = profile.packages.find((candidate: any) => candidate.pluginId === target.pluginId);
    const targetCodec = (codecs[target.pluginId] ?? []).find((codec) => codec.artifactKind === target.artifactKind && codec.artifactSchema === target.artifactSchema);
    if (!targetPackage || !targetCodec) throw new Error("trusted bootstrap open target has no package codec");
    if (targetCodec.packSchemaHash !== target.packSchemaHash) throw new Error("trusted bootstrap open target pack schema hash differs from its package codec");
    for (const value of [
      targetPackage.pluginId,
      targetPackage.packageId,
      targetPackage.version,
      Buffer.from(targetPackage.componentSha256, "hex"),
      Buffer.from(targetPackage.componentBlake3, "hex"),
      Buffer.from(targetPackage.descriptorSha256, "hex"),
      trustedBootstrapCount(trustedBootstrapExecutionProtocol(targetPackage.executionProtocol).appChannelVersion),
      target.artifactKind,
      target.artifactSchema,
      Buffer.from(targetCodec.packSchemaHash, "hex"),
      target.parentDialect.artifactKind,
      target.parentDialect.standard,
      target.parentDialect.subset,
      target.surfaceId,
      target.appId,
      target.windowKindId,
      target.role,
      target.rendererTarget,
      Buffer.from([Number(target.grant.read), Number(target.grant.write), Number(target.grant.observe)]),
    ])
      pieces.push(trustedBootstrapField(value));
  }
  return Buffer.concat(pieces);
}

export function trustedBootstrapClosureEncoding(profile: any): Buffer {
  const pieces = [Buffer.from("semio/hub/trusted-profile-selected-closure/v1\0"), trustedBootstrapCount(profile.selectedClosure.length)];
  for (const identity of profile.selectedClosure) pieces.push(trustedBootstrapField(identity.pluginId), trustedBootstrapField(identity.packageId), trustedBootstrapField(identity.version));
  return Buffer.concat(pieces);
}

export function trustedBootstrapPlanGenerationOutcome(issuedGenerationId: string, observedGenerationId: string): "accepted" | "stale" {
  return issuedGenerationId === observedGenerationId ? "accepted" : "stale";
}

/** 🧩️ Where a package record finds its plugin module manifest: `TrustedBundlePluginModuleV1`. */
export type TrustedBootstrapPluginModuleRecordV1 = Readonly<{ path: string; byteLength: number; sha256: string; blake3: string }>;

/** 🧩️ Admits one `TrustedBundlePluginModuleV1` record at its own package's fixed manifest path. */
export function trustedBootstrapPluginModuleRecordV1(candidate: unknown, pluginId: string): TrustedBootstrapPluginModuleRecordV1 {
  const row = documentOpenNeutralObject(candidate, ["path", "byteLength", "sha256", "blake3"]);
  if (row.path !== trustedBootstrapPluginModuleManifestPath(pluginId) || !Number.isSafeInteger(row.byteLength) || (row.byteLength as number) <= 0 || (row.byteLength as number) > TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES || !/^[0-9a-f]{64}$/u.test(String(row.sha256)) || !/^[0-9a-f]{64}$/u.test(String(row.blake3)))
    throw new Error("trusted plugin module record is not its package's exact manifest");
  return Object.freeze({ path: row.path as string, byteLength: row.byteLength as number, sha256: row.sha256 as string, blake3: row.blake3 as string });
}

/** 🧩️ The generation-relative manifest path of one package's plugin module. */
export function trustedBootstrapPluginModuleManifestPath(pluginId: string): string {
  return `packages/${pluginId}/plugin-module.json`;
}

/** 🗃️ The generation-relative content-addressed path of one plugin module file. */
export function trustedBootstrapPluginModuleBlobPath(sha256: string): string {
  return `plugin-modules/${sha256}`;
}

/** 🧬️ Frames one plugin module record into the generation exactly as `trusted_profile_generation` does. */
function trustedBootstrapPluginModuleEncoding(record: TrustedBootstrapPluginModuleRecordV1): Buffer {
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(record.byteLength));
  return Buffer.concat([trustedBootstrapField(record.path), length, trustedBootstrapField(Buffer.from(record.sha256, "hex")), trustedBootstrapField(Buffer.from(record.blake3, "hex"))]);
}
