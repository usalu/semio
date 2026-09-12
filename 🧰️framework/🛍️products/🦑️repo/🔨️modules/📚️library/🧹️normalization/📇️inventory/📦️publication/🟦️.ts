import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { canonicalJson } from "../../🟦️.ts";
import { buildTaxonomyInventoryArtifactShards, type TaxonomyInventoryShardManifest, type TaxonomyInventoryShardProgress, TAXONOMY_INVENTORY_SHARD_MAX_BYTES } from "../🧩️shards/🟦️.ts";
import { taxonomyCliByteCompare, taxonomyCliCanonicalArrayDigest, taxonomyCliCanonicalJson, taxonomyCliRecord, taxonomyCliSha256 } from "../🧾️serialization/🟦️.ts";

export function taxonomyCliShardPayloadPaths(root: string): string[] {
  const paths: string[] = [];
  for (const entry of readdirSync(root, { withFileTypes: true }).sort((left, right) => taxonomyCliByteCompare(left.name, right.name))) {
    if (entry.name === "🔣️.json" && entry.isFile()) continue;
    if (!entry.isDirectory() || !/^🔖️[a-f0-9]{64}$/u.test(entry.name)) {
      paths.push(`📊️shards/${entry.name}`);
      continue;
    }
    const children = readdirSync(join(root, entry.name), { withFileTypes: true });
    for (const child of children) paths.push(`📊️shards/${entry.name}/${child.name}`);
  }
  return paths.sort(taxonomyCliByteCompare);
}


export function taxonomyCliValidatePublishedShardRoot(root: string): TaxonomyInventoryShardManifest | null {
  const manifestPath = join(root, "🔣️.json");
  const available = taxonomyCliShardPayloadPaths(root);
  if (!existsSync(manifestPath)) {
    if (available.length > 0) throw new Error("Inventory shard root has unreferenced payload or staging evidence without a manifest.");
    return null;
  }
  const manifestState = lstatSync(manifestPath);
  if (!manifestState.isFile() || manifestState.isSymbolicLink()) throw new Error("Inventory shard manifest must be a real file.");
  const content = readFileSync(manifestPath, "utf8");
  const manifest = taxonomyCliRecord(JSON.parse(content) as unknown, "Published inventory shard manifest") as unknown as TaxonomyInventoryShardManifest;
  if (`${taxonomyCliCanonicalJson(manifest)}\n` !== content || Buffer.byteLength(content) >= TAXONOMY_INVENTORY_SHARD_MAX_BYTES) throw new Error("Published inventory shard manifest is not canonical or exceeds the byte limit.");
  if (!Array.isArray(manifest.shards) || taxonomyCliCanonicalArrayDigest(manifest.shards) !== manifest.shardLedgerDigest) throw new Error("Published inventory shard manifest ledger is invalid.");
  const declared = manifest.shards.map((descriptor) => descriptor.path).sort(taxonomyCliByteCompare);
  if (canonicalJson(available) !== canonicalJson(declared)) throw new Error("Published inventory shard root contains missing or unreferenced shards.");
  for (const descriptor of manifest.shards) {
    if (!/^[a-f0-9]{64}$/u.test(descriptor.digest) || descriptor.path !== `📊️shards/🔖️${descriptor.digest}/🔣️.json`) throw new Error(`Published inventory shard descriptor is invalid: ${descriptor.path}`);
    const directory = join(root, `🔖️${descriptor.digest}`);
    const path = join(directory, "🔣️.json");
    const directoryState = lstatSync(directory);
    const fileState = lstatSync(path);
    if (!directoryState.isDirectory() || directoryState.isSymbolicLink() || !fileState.isFile() || fileState.isSymbolicLink()) throw new Error(`Published inventory shard must be a real directory/file pair: ${descriptor.path}`);
    const bytes = readFileSync(path);
    if (bytes.byteLength !== descriptor.bytes || taxonomyCliSha256(bytes) !== descriptor.digest) throw new Error(`Published inventory shard digest collision or corruption: ${descriptor.path}`);
  }
  return manifest;
}


/** 📦️ Publishes immutable verified payloads before one atomic manifest rename. */
export function publishTaxonomyInventoryArtifactShards(dataRoot: string, inventory: unknown, progress?: (event: TaxonomyInventoryShardProgress) => void): TaxonomyInventoryShardManifest {
  const build = buildTaxonomyInventoryArtifactShards(inventory);
  const total = build.shards.length + 1;
  progress?.({ phase: "write-shards", current: 0, total });
  if (existsSync(dataRoot)) {
    const state = lstatSync(dataRoot);
    if (!state.isDirectory() || state.isSymbolicLink()) throw new Error(`Inventory artifact root must be a real directory: ${dataRoot}`);
  } else mkdirSync(dataRoot, { recursive: true });
  const shardRoot = join(dataRoot, "📊️shards");
  if (existsSync(shardRoot)) {
    const state = lstatSync(shardRoot);
    if (!state.isDirectory() || state.isSymbolicLink()) throw new Error(`Inventory shard root must be a real directory: ${shardRoot}`);
  } else mkdirSync(shardRoot);
  taxonomyCliValidatePublishedShardRoot(shardRoot);
  for (let index = 0; index < build.shards.length; index += 1) {
    const shard = build.shards[index]!;
    const directory = join(shardRoot, `🔖️${shard.descriptor.digest}`);
    const path = join(directory, "🔣️.json");
    if (existsSync(directory)) {
      const directoryState = lstatSync(directory);
      if (!directoryState.isDirectory() || directoryState.isSymbolicLink()) throw new Error(`Inventory shard digest collision: ${shard.descriptor.path}`);
      const children = readdirSync(directory, { withFileTypes: true });
      if (children.length !== 1 || children[0]!.name !== "🔣️.json" || !children[0]!.isFile()) throw new Error(`Inventory shard digest directory is not exact: ${shard.descriptor.path}`);
      const fileState = lstatSync(path);
      const bytes = readFileSync(path);
      if (!fileState.isFile() || fileState.isSymbolicLink() || bytes.byteLength !== shard.descriptor.bytes || taxonomyCliSha256(bytes) !== shard.descriptor.digest || bytes.toString("utf8") !== shard.content) throw new Error(`Inventory shard digest collision: ${shard.descriptor.path}`);
    } else {
      const staging = join(shardRoot, `.inventory-shard-${shard.descriptor.digest}.staging`);
      if (existsSync(staging)) throw new Error(`Retained failed inventory shard staging evidence blocks publication: ${staging}`);
      mkdirSync(staging);
      const stagingPath = join(staging, "🔣️.json");
      writeFileSync(stagingPath, shard.content, { flag: "wx" });
      const written = readFileSync(stagingPath);
      if (written.byteLength !== shard.descriptor.bytes || taxonomyCliSha256(written) !== shard.descriptor.digest) throw new Error(`Written inventory shard failed verification: ${shard.descriptor.path}`);
      renameSync(staging, directory);
    }
    progress?.({ phase: "write-shards", current: index + 1, total, path: shard.descriptor.path });
  }
  for (const shard of build.shards) {
    const path = join(shardRoot, `🔖️${shard.descriptor.digest}`, "🔣️.json");
    const bytes = readFileSync(path);
    if (bytes.byteLength !== shard.descriptor.bytes || taxonomyCliSha256(bytes) !== shard.descriptor.digest) throw new Error(`Inventory shard failed pre-manifest verification: ${shard.descriptor.path}`);
  }
  const manifestDigest = taxonomyCliSha256(build.manifestContent);
  const stagingManifest = join(shardRoot, `.inventory-manifest-${manifestDigest}.staging`);
  const manifestPath = join(shardRoot, "🔣️.json");
  if (existsSync(stagingManifest)) throw new Error(`Retained failed inventory manifest staging evidence blocks publication: ${stagingManifest}`);
  writeFileSync(stagingManifest, build.manifestContent, { flag: "wx" });
  if (readFileSync(stagingManifest, "utf8") !== build.manifestContent) throw new Error("Written inventory shard manifest failed byte verification.");
  renameSync(stagingManifest, manifestPath);
  const retained = new Set(build.shards.map((shard) => `🔖️${shard.descriptor.digest}`));
  for (const entry of readdirSync(shardRoot, { withFileTypes: true })) {
    if (entry.isDirectory() && /^🔖️[a-f0-9]{64}$/u.test(entry.name) && !retained.has(entry.name)) rmSync(join(shardRoot, entry.name), { recursive: true, force: true });
  }
  taxonomyCliValidatePublishedShardRoot(shardRoot);
  progress?.({ phase: "write-shards", current: total, total, path: "📊️shards/🔣️.json" });
  return build.manifest;
}
