/** 🏗️ Fresh build verifier of the catalog verification: the fresh catalog commit marker, the exact catalog row requirement and the verifier of one fresh catalog build root. */
import { cargoTargetDirectory, cargoBuildDirectory } from "@semio-tech/process/execution";
import { existsSync, lstatSync, readdirSync, realpathSync } from "node:fs";
import { isAbsolute, join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { decodePackValue, encodePackValue } from "@semio-tech/framework-os";
import type { PackValue } from "@semio-tech/framework-os";
import { COMPONENT_RECEIPT_FILE, COMPONENT_RECEIPT_MAX_BYTES, componentReceiptBindsV1, parseComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt/schema";
import type { ComponentReceiptV1 } from "@semio-tech/framework/component-receipt";
import { CATALOG_ARTIFACT_MAX_BYTES, CATALOG_COMMIT_MARKER_FILENAME, CATALOG_COMMIT_MARKER_MAX_BYTES, CATALOG_DESCRIPTOR_MAX_BYTES, CATALOG_DESCRIPTOR_PACK_FILENAME, CATALOG_PACKAGE_ID, type CatalogArtifactControl, type CatalogFileReceipt, type FreshCatalogBuildVerifier, type FreshCatalogCommitMarker, type StrictCatalogDescriptor } from "../🟦️.ts";
import { boundedCatalogDiagnostic, isStrictJsonValue, pathIsWithin, rejectDuplicateJsonObjectNames } from "../🛡️guards/🟦️.ts";
import { catalogArtifactReceipt, readCatalogFile, readVerifiedCatalogArtifact } from "../📥️artifact-io/🟦️.ts";

//#region 🔖️CatalogCompleteness
/** 🧾️ Computes the exact immutable marker a producer must publish last for one fresh row: the staged descriptor must be the
 * committed semantic descriptor itself, and the staged receipt a canonical shipped receipt binding it to the staged raw component. */
export function createFreshCatalogCommitMarker(source: StrictCatalogDescriptor, buildRoot: string, control: CatalogArtifactControl = {}): FreshCatalogCommitMarker {
  const exactRoot = realpathSync(resolve(buildRoot));
  const rowRoot = join(exactRoot, source.entry.pluginId);
  const rawRelative = join("raw", source.entry.wasmOut);
  const descriptorRelative = join("descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME);
  const receiptRelative = join("receipt", COMPONENT_RECEIPT_FILE);
  const packageId = source.entry.packageId;
  if (!CATALOG_PACKAGE_ID.test(packageId) || source.descriptor.packageId !== packageId) throw new Error(`${source.entry.pluginId}: descriptor packageId cannot identify a commit marker`);
  const staged = readCatalogFile(join(rowRoot, descriptorRelative), exactRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  if (!Buffer.from(staged).equals(Buffer.from(source.packBytes))) throw new Error(`${source.entry.pluginId}: staged descriptor is not the committed semantic descriptor`);
  const raw = catalogArtifactReceipt(join(rowRoot, rawRelative), exactRoot, rawRelative, source.entry.pluginId, "raw", control);
  const receipt = parseComponentReceiptV1(Buffer.from(readCatalogFile(join(rowRoot, receiptRelative), exactRoot, COMPONENT_RECEIPT_MAX_BYTES)).toString("utf8"));
  if (!componentReceiptBindsV1(receipt, { profile: "release", pluginId: source.entry.pluginId, packageId, wasmSha256: raw.sha256, semanticDescriptorSha256: source.semanticSha256 }) || receipt.wasmBytes !== raw.bytes) throw new Error(`${source.entry.pluginId}: staged receipt does not bind the staged component to the committed semantic descriptor`);
  return {
    schemaVersion: 2,
    packageId,
    pluginId: source.entry.pluginId,
    packageName: source.entry.packageName,
    wasmOut: source.entry.wasmOut,
    raw,
    descriptor: catalogArtifactReceipt(join(rowRoot, descriptorRelative), exactRoot, descriptorRelative, source.entry.pluginId, "descriptor", control),
    receipt: catalogArtifactReceipt(join(rowRoot, receiptRelative), exactRoot, receiptRelative, source.entry.pluginId, "receipt", control),
  };
}

export function requireExactCatalogRow(rowRoot: string, wasmOut: string): void {
  const expected = [CATALOG_COMMIT_MARKER_FILENAME, "descriptor", "raw", "receipt"].sort();
  if (!isDeepStrictEqual(readdirSync(rowRoot).sort(), expected)) throw new Error(`${rowRoot}: catalog row is not the exact committed triplet`);
  for (const [directory, filename] of [["raw", wasmOut], ["descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME], ["receipt", COMPONENT_RECEIPT_FILE]] as const) {
    const root = join(rowRoot, directory);
    const info = lstatSync(root);
    if (info.isSymbolicLink() || !info.isDirectory() || !isDeepStrictEqual(readdirSync(root), [filename])) throw new Error(`${root}: catalog artifact directory is not exact`);
  }
}

/** 🏗️ Binds completion evidence to a caller-owned isolated root and rejects ambient target/cache authority. */
export function createFreshCatalogBuildVerifier(repoRoot: string, buildRoot: string): FreshCatalogBuildVerifier {
  if (!isAbsolute(buildRoot)) throw new Error("fresh catalog build root must be absolute");
  const resolvedRepo = resolve(repoRoot);
  const resolvedBuild = resolve(buildRoot);
  const sharedTarget = cargoTargetDirectory(resolvedRepo);
  const sharedBuild = cargoBuildDirectory(resolvedRepo);
  const developmentCache = resolve(resolvedRepo, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "📦️packages", "🟦️typescript", "dist");
  if (pathIsWithin(sharedTarget, resolvedBuild)) throw new Error("fresh catalog verification cannot use the ambient shared target");
  if (pathIsWithin(sharedBuild, resolvedBuild)) throw new Error("fresh catalog verification cannot use the ambient shared build directory");
  if (pathIsWithin(developmentCache, resolvedBuild)) throw new Error("fresh catalog verification cannot use the development cache");
  if (resolvedBuild === resolvedRepo) throw new Error("fresh catalog build root must be a dedicated directory");
  const rootInfo = lstatSync(resolvedBuild);
  if (rootInfo.isSymbolicLink() || !rootInfo.isDirectory()) throw new Error("fresh catalog build root must be a regular non-symlink directory");
  const exactRoot = realpathSync(resolvedBuild);
  return {
    root: exactRoot,
    verify(entry, control = {}) {
      const rowRoot = join(exactRoot, entry.pluginId);
      const markerPath = join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME);
      if (!existsSync(markerPath)) throw new Error(`${entry.pluginId}: catalog commit marker is missing`);
      const markerBytes = readCatalogFile(markerPath, exactRoot, CATALOG_COMMIT_MARKER_MAX_BYTES);
      let marker: unknown;
      try {
        const json = Buffer.from(markerBytes).toString("utf8");
        rejectDuplicateJsonObjectNames(json);
        marker = JSON.parse(json);
      } catch (error) {
        throw new Error(`${entry.pluginId}: catalog commit marker does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (marker === null || typeof marker !== "object" || Array.isArray(marker) || !isStrictJsonValue(marker)) throw new Error(`${entry.pluginId}: catalog commit marker is not an object`);
      const markerRecord = marker as Record<string, unknown>;
      const markerFields = ["descriptor", "packageId", "packageName", "pluginId", "raw", "receipt", "schemaVersion", "wasmOut"];
      if (!isDeepStrictEqual(Object.keys(markerRecord).sort(), markerFields)) throw new Error(`${entry.pluginId}: catalog commit marker fields are not exact`);
      if (markerRecord.schemaVersion !== 2 || markerRecord.pluginId !== entry.pluginId || markerRecord.packageId !== entry.packageId || markerRecord.packageName !== entry.packageName || markerRecord.wasmOut !== entry.wasmOut) throw new Error(`${entry.pluginId}: catalog commit marker identity is not the carried Cargo identity`);
      if (!Buffer.from(markerBytes).equals(Buffer.from(`${JSON.stringify(marker)}\n`))) throw new Error(`${entry.pluginId}: catalog commit marker is not canonical JSON`);
      requireExactCatalogRow(rowRoot, entry.wasmOut);
      const raw = readVerifiedCatalogArtifact(join(rowRoot, "raw", entry.wasmOut), exactRoot, CATALOG_ARTIFACT_MAX_BYTES, entry.pluginId, "raw", control);
      const descriptor = readVerifiedCatalogArtifact(join(rowRoot, "descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME), exactRoot, CATALOG_DESCRIPTOR_MAX_BYTES, entry.pluginId, "descriptor", control);
      const receiptFile = readVerifiedCatalogArtifact(join(rowRoot, "receipt", COMPONENT_RECEIPT_FILE), exactRoot, COMPONENT_RECEIPT_MAX_BYTES, entry.pluginId, "receipt", control);
      const fileReceipt = (path: string, bytes: Uint8Array, sha256: string): CatalogFileReceipt => ({ path, bytes: bytes.byteLength, sha256 });
      if (!isDeepStrictEqual(markerRecord.raw, fileReceipt(join("raw", entry.wasmOut), raw.bytes, raw.sha256)) || !isDeepStrictEqual(markerRecord.descriptor, fileReceipt(join("descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME), descriptor.bytes, descriptor.sha256)) || !isDeepStrictEqual(markerRecord.receipt, fileReceipt(join("receipt", COMPONENT_RECEIPT_FILE), receiptFile.bytes, receiptFile.sha256))) throw new Error(`${entry.pluginId}: catalog commit marker artifact receipts disagree with the staged row`);
      let packed: PackValue;
      try {
        packed = decodePackValue(descriptor.bytes);
      } catch (error) {
        throw new Error(`${entry.pluginId}: staged descriptor pack does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(descriptor.bytes))) throw new Error(`${entry.pluginId}: staged descriptor pack is not canonical`);
      if (packed === null || typeof packed !== "object" || Array.isArray(packed) || "hashes" in (packed as Record<string, unknown>)) throw new Error(`${entry.pluginId}: staged descriptor is not a semantic descriptor`);
      let receipt: ComponentReceiptV1;
      try {
        receipt = parseComponentReceiptV1(Buffer.from(receiptFile.bytes).toString("utf8"));
      } catch (error) {
        throw new Error(`${entry.pluginId}: staged receipt does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (!componentReceiptBindsV1(receipt, { profile: "release", pluginId: entry.pluginId, packageId: entry.packageId, wasmSha256: raw.sha256, semanticDescriptorSha256: descriptor.sha256 }) || receipt.wasmBytes !== raw.bytes.byteLength) throw new Error(`${entry.pluginId}: staged receipt does not bind the staged component to its semantic descriptor`);
      if (!Buffer.from(readCatalogFile(markerPath, exactRoot, CATALOG_COMMIT_MARKER_MAX_BYTES)).equals(Buffer.from(markerBytes))) throw new Error(`${entry.pluginId}: catalog commit marker changed during verification`);
      return {
        pluginId: entry.pluginId,
        rawSha256: raw.sha256,
        descriptorSha256: descriptor.sha256,
        receipt,
        rawBytes: raw.bytes,
        descriptorBytes: descriptor.bytes,
      };
    },
  };
}

//#endregion 🔖️CatalogCompleteness
