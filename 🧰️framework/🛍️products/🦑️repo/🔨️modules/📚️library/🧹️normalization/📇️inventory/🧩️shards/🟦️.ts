import { canonicalJson } from "../../🟦️.ts";
import { taxonomyCliByteCompare, taxonomyCliCanonicalArrayDigest, taxonomyCliCanonicalJson, taxonomyCliEntryViolations, taxonomyCliExactKeys, taxonomyCliInventoryEntries, taxonomyCliInventoryMetadata, taxonomyCliRecord, taxonomyCliSha256, taxonomyCliStableViolations, taxonomyInventoryIncrementalCanonicalDigest } from "../🧾️serialization/🟦️.ts";

//#region 📊️TaxonomyInventoryShards
export const TAXONOMY_INVENTORY_SHARD_MAX_BYTES = 5 * 1024 * 1024;


export interface TaxonomyInventoryShardDescriptor {
  readonly path: string;
  readonly digest: string;
  readonly ownerId: string;
  readonly part: number;
  readonly entryCount: number;
  readonly bytes: number;
  readonly firstSourcePath: string;
  readonly lastSourcePath: string;
}


export interface TaxonomyInventoryShardManifest {
  readonly schemaVersion: 1;
  readonly inventoryMetadata: Readonly<Record<string, unknown>>;
  readonly inventoryCanonicalDigest: string;
  readonly entryCount: number;
  readonly violationCount: number;
  readonly violationsDigest: string;
  readonly shardLedgerDigest: string;
  readonly shards: readonly TaxonomyInventoryShardDescriptor[];
}


export interface TaxonomyInventoryArtifactShard {
  readonly descriptor: TaxonomyInventoryShardDescriptor;
  readonly content: string;
}


export interface TaxonomyInventoryArtifactShards {
  readonly manifest: TaxonomyInventoryShardManifest;
  readonly manifestContent: string;
  readonly shards: readonly TaxonomyInventoryArtifactShard[];
}


export interface TaxonomyInventoryShardValidation {
  readonly inventory: Readonly<Record<string, unknown>>;
  readonly entryCount: number;
  readonly violationCount: number;
}


export interface TaxonomyInventoryShardProgress {
  readonly phase: "write-shards";
  readonly current: number;
  readonly total: number;
  readonly path?: string;
}


/** 🧩️ Builds deterministic lossless owner shards without filesystem access. */
export function buildTaxonomyInventoryArtifactShards(value: unknown): TaxonomyInventoryArtifactShards {
  const inventory = taxonomyCliRecord(value, "Inventory");
  const entries = taxonomyCliInventoryEntries(inventory);
  if (!Array.isArray(inventory.violations)) throw new Error("Inventory violations must be an array.");
  const sourcePaths = new Set<string>();
  const owners = new Map<string, Record<string, unknown>[]>();
  for (const entry of entries) {
    const sourcePath = String(entry.sourcePath);
    if (sourcePaths.has(sourcePath)) throw new Error(`Duplicate inventory sourcePath ${JSON.stringify(sourcePath)}.`);
    sourcePaths.add(sourcePath);
    const ownerId = String(entry.ownerId);
    const rows = owners.get(ownerId) ?? [];
    rows.push(entry);
    owners.set(ownerId, rows);
  }
  const stableViolations = taxonomyCliStableViolations(taxonomyCliEntryViolations(entries));
  const declaredViolations = taxonomyCliStableViolations(inventory.violations);
  if (stableViolations.length !== inventory.violations.length || taxonomyCliCanonicalArrayDigest(stableViolations) !== taxonomyCliCanonicalArrayDigest(declaredViolations)) throw new Error("Inventory top-level violations must equal the canonical violations derived from entries.");
  const shards: TaxonomyInventoryArtifactShard[] = [];
  for (const [ownerId, ownerEntries] of [...owners].sort(([left], [right]) => taxonomyCliByteCompare(left, right))) {
    ownerEntries.sort((left, right) => taxonomyCliByteCompare(String(left.sourcePath), String(right.sourcePath)));
    const prefixBytes = Buffer.byteLength('{"entries":[');
    const suffix = `],"ownerId":${taxonomyCliCanonicalJson(ownerId)},"schemaVersion":1}\n`;
    const suffixBytes = Buffer.byteLength(suffix);
    let part = 0;
    let rows: Record<string, unknown>[] = [];
    let rowJson: string[] = [];
    let bytes = prefixBytes + suffixBytes;
    const flush = (): void => {
      if (rows.length === 0) return;
      const content = `{"entries":[${rowJson.join(",")}],"ownerId":${taxonomyCliCanonicalJson(ownerId)},"schemaVersion":1}\n`;
      const payloadBytes = Buffer.byteLength(content);
      if (payloadBytes >= TAXONOMY_INVENTORY_SHARD_MAX_BYTES) throw new Error(`Inventory shard for ${JSON.stringify(ownerId)} is not strictly below ${TAXONOMY_INVENTORY_SHARD_MAX_BYTES} bytes.`);
      const digest = taxonomyCliSha256(content);
      const descriptor: TaxonomyInventoryShardDescriptor = {
        path: `📊️shards/🔖️${digest}/🔣️.json`,
        digest,
        ownerId,
        part,
        entryCount: rows.length,
        bytes: payloadBytes,
        firstSourcePath: String(rows[0]!.sourcePath),
        lastSourcePath: String(rows.at(-1)!.sourcePath),
      };
      shards.push({ descriptor, content });
      part += 1;
      rows = [];
      rowJson = [];
      bytes = prefixBytes + suffixBytes;
    };
    for (const entry of ownerEntries) {
      const json = taxonomyCliCanonicalJson(entry);
      const entryBytes = Buffer.byteLength(json);
      const separatorBytes = rows.length === 0 ? 0 : 1;
      if (rows.length > 0 && bytes + separatorBytes + entryBytes >= TAXONOMY_INVENTORY_SHARD_MAX_BYTES) flush();
      if (bytes + entryBytes >= TAXONOMY_INVENTORY_SHARD_MAX_BYTES) throw new Error(`Inventory entry exceeds the shard byte limit: ${entry.sourcePath}`);
      rows.push(entry);
      rowJson.push(json);
      bytes += (rows.length === 1 ? 0 : 1) + entryBytes;
    }
    flush();
  }
  const descriptors = shards.map((shard) => shard.descriptor);
  const manifest: TaxonomyInventoryShardManifest = {
    schemaVersion: 1,
    inventoryMetadata: taxonomyCliInventoryMetadata(inventory),
    inventoryCanonicalDigest: taxonomyInventoryIncrementalCanonicalDigest({ ...inventory, entries: [...entries].sort((left, right) => taxonomyCliByteCompare(String(left.sourcePath), String(right.sourcePath))), violations: stableViolations }),
    entryCount: entries.length,
    violationCount: stableViolations.length,
    violationsDigest: taxonomyCliCanonicalArrayDigest(stableViolations),
    shardLedgerDigest: taxonomyCliCanonicalArrayDigest(descriptors),
    shards: descriptors,
  };
  const result: TaxonomyInventoryArtifactShards = { manifest, manifestContent: `${taxonomyCliCanonicalJson(manifest)}\n`, shards };
  validateTaxonomyInventoryArtifactShards(result);
  return result;
}


/** 🛡️ Validates canonical bytes, ordering, digests, counts, uniqueness and available shard closure. */
export function validateTaxonomyInventoryArtifactShards(value: TaxonomyInventoryArtifactShards, availableShardPaths: readonly string[] = value.shards.map((shard) => shard.descriptor.path)): TaxonomyInventoryShardValidation {
  const manifest = taxonomyCliRecord(value.manifest, "Inventory shard manifest") as unknown as TaxonomyInventoryShardManifest;
  taxonomyCliExactKeys(manifest as unknown as Record<string, unknown>, ["schemaVersion", "inventoryMetadata", "inventoryCanonicalDigest", "entryCount", "violationCount", "violationsDigest", "shardLedgerDigest", "shards"], "Inventory shard manifest");
  if (manifest.schemaVersion !== 1) throw new Error("Inventory shard manifest schemaVersion must be 1.");
  const metadata = taxonomyCliRecord(manifest.inventoryMetadata, "Inventory shard metadata");
  if ("entries" in metadata || "violations" in metadata) throw new Error("Inventory shard metadata cannot contain entries or violations.");
  if (!Array.isArray(manifest.shards)) throw new Error("Inventory shard descriptors must be an array.");
  if (!Number.isSafeInteger(manifest.entryCount) || manifest.entryCount < 0 || !Number.isSafeInteger(manifest.violationCount) || manifest.violationCount < 0) throw new Error("Inventory shard counts must be non-negative safe integers.");
  if (!/^[a-f0-9]{64}$/u.test(manifest.inventoryCanonicalDigest) || !/^[a-f0-9]{64}$/u.test(manifest.violationsDigest) || !/^[a-f0-9]{64}$/u.test(manifest.shardLedgerDigest)) throw new Error("Inventory shard manifest digests must be lowercase SHA-256.");
  const manifestContent = `${taxonomyCliCanonicalJson(manifest)}\n`;
  if (value.manifestContent !== manifestContent) throw new Error("Inventory shard manifest bytes are not canonical.");
  if (Buffer.byteLength(manifestContent) >= TAXONOMY_INVENTORY_SHARD_MAX_BYTES) throw new Error("Inventory shard manifest is not strictly below the shard byte limit.");
  if (taxonomyCliCanonicalArrayDigest(manifest.shards) !== manifest.shardLedgerDigest) throw new Error("Inventory shard ledger digest mismatch.");
  if (!Array.isArray(value.shards) || value.shards.length !== manifest.shards.length) throw new Error("Inventory shard payload count does not match the manifest.");
  const expectedOrder = [...manifest.shards].sort((left, right) => taxonomyCliByteCompare(left.ownerId, right.ownerId) || left.part - right.part);
  if (expectedOrder.some((descriptor, index) => descriptor !== manifest.shards[index])) throw new Error("Inventory shard descriptors are not in byte-sorted owner/part order.");
  const available = [...availableShardPaths].sort(taxonomyCliByteCompare);
  if (available.length !== new Set(available).size) throw new Error("Available inventory shard paths contain duplicates.");
  const declaredPaths = manifest.shards.map((descriptor) => descriptor.path).sort(taxonomyCliByteCompare);
  if (canonicalJson(available) !== canonicalJson(declaredPaths)) throw new Error("Available inventory shard paths contain missing or unreferenced shards.");
  const entries: Record<string, unknown>[] = [];
  const sourcePaths = new Set<string>();
  const descriptorPaths = new Set<string>();
  const descriptorDigests = new Set<string>();
  const nextPart = new Map<string, number>();
  for (let index = 0; index < manifest.shards.length; index += 1) {
    const descriptor = taxonomyCliRecord(manifest.shards[index], `Inventory shard descriptor ${index}`) as unknown as TaxonomyInventoryShardDescriptor;
    taxonomyCliExactKeys(descriptor as unknown as Record<string, unknown>, ["path", "digest", "ownerId", "part", "entryCount", "bytes", "firstSourcePath", "lastSourcePath"], `Inventory shard descriptor ${index}`);
    if (typeof descriptor.ownerId !== "string" || descriptor.ownerId.length === 0 || descriptor.ownerId !== descriptor.ownerId.normalize("NFC")) throw new Error(`Inventory shard descriptor ${index} ownerId must be non-empty NFC.`);
    if (!Number.isSafeInteger(descriptor.part) || descriptor.part < 0 || descriptor.part !== (nextPart.get(descriptor.ownerId) ?? 0)) throw new Error(`Inventory shard parts for ${JSON.stringify(descriptor.ownerId)} must be contiguous from zero.`);
    nextPart.set(descriptor.ownerId, descriptor.part + 1);
    if (!Number.isSafeInteger(descriptor.entryCount) || descriptor.entryCount < 1 || !Number.isSafeInteger(descriptor.bytes) || descriptor.bytes < 1 || descriptor.bytes >= TAXONOMY_INVENTORY_SHARD_MAX_BYTES) throw new Error(`Inventory shard descriptor ${index} has invalid counts or bytes.`);
    if (!/^[a-f0-9]{64}$/u.test(descriptor.digest) || descriptor.path !== `📊️shards/🔖️${descriptor.digest}/🔣️.json`) throw new Error(`Inventory shard descriptor ${index} path/digest identity is invalid.`);
    if (descriptorPaths.has(descriptor.path) || descriptorDigests.has(descriptor.digest)) throw new Error(`Inventory shard descriptor ${index} duplicates a path or digest.`);
    descriptorPaths.add(descriptor.path);
    descriptorDigests.add(descriptor.digest);
    const shard = value.shards[index];
    if (!shard || taxonomyCliCanonicalJson(shard.descriptor) !== taxonomyCliCanonicalJson(descriptor)) throw new Error(`Inventory shard payload ${index} descriptor mismatch.`);
    if (Buffer.byteLength(shard.content) !== descriptor.bytes || taxonomyCliSha256(shard.content) !== descriptor.digest) throw new Error(`Inventory shard payload ${index} byte/digest mismatch.`);
    const envelope = taxonomyCliRecord(JSON.parse(shard.content) as unknown, `Inventory shard envelope ${index}`);
    taxonomyCliExactKeys(envelope, ["entries", "ownerId", "schemaVersion"], `Inventory shard envelope ${index}`);
    if (envelope.schemaVersion !== 1 || envelope.ownerId !== descriptor.ownerId || !Array.isArray(envelope.entries)) throw new Error(`Inventory shard envelope ${index} identity is invalid.`);
    if (`${taxonomyCliCanonicalJson(envelope)}\n` !== shard.content) throw new Error(`Inventory shard payload ${index} is not canonical JSON.`);
    const shardEntries = envelope.entries.map((entry, entryIndex) => taxonomyCliRecord(entry, `Inventory shard ${index} entry ${entryIndex}`));
    if (shardEntries.length !== descriptor.entryCount) throw new Error(`Inventory shard payload ${index} entry count mismatch.`);
    for (let entryIndex = 0; entryIndex < shardEntries.length; entryIndex += 1) {
      const entry = shardEntries[entryIndex]!;
      if (entry.ownerId !== descriptor.ownerId || typeof entry.sourcePath !== "string" || entry.sourcePath.length === 0 || entry.sourcePath !== entry.sourcePath.normalize("NFC")) throw new Error(`Inventory shard ${index} entry ${entryIndex} identity is invalid.`);
      if (entryIndex > 0 && taxonomyCliByteCompare(String(shardEntries[entryIndex - 1]!.sourcePath), entry.sourcePath) >= 0) throw new Error(`Inventory shard ${index} entries are not strictly sourcePath byte-sorted.`);
      if (sourcePaths.has(entry.sourcePath)) throw new Error(`Duplicate inventory sourcePath ${JSON.stringify(entry.sourcePath)} across shards.`);
      sourcePaths.add(entry.sourcePath);
      entries.push(entry);
    }
    if (descriptor.firstSourcePath !== shardEntries[0]!.sourcePath || descriptor.lastSourcePath !== shardEntries.at(-1)!.sourcePath) throw new Error(`Inventory shard payload ${index} boundary mismatch.`);
  }
  if (entries.length !== manifest.entryCount) throw new Error("Inventory shard manifest entry count mismatch.");
  entries.sort((left, right) => taxonomyCliByteCompare(String(left.sourcePath), String(right.sourcePath)));
  const violations = taxonomyCliStableViolations(taxonomyCliEntryViolations(entries));
  if (violations.length !== manifest.violationCount || taxonomyCliCanonicalArrayDigest(violations) !== manifest.violationsDigest) throw new Error("Inventory shard violation count/digest mismatch.");
  const inventory = { ...metadata, entries, violations };
  if (taxonomyInventoryIncrementalCanonicalDigest(inventory) !== manifest.inventoryCanonicalDigest) throw new Error("Reconstructed inventory canonical digest mismatch.");
  return { inventory, entryCount: entries.length, violationCount: violations.length };
}
