/** 📁️ Validates committed descriptor files against their discovered component owner. */
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { DESCRIPTOR_JSON_REL_PATH } from "../../../🔎️discovery/🟦️.ts";
import type { CompiledComponentOwnerV1 } from "../../../🔎️discovery/🧬️schema/🟦️.ts";
import { CATALOG_DESCRIPTOR_MAX_BYTES, CATALOG_DESCRIPTOR_PACK_FILENAME, type StrictCatalogDescriptor } from "../../🟦️.ts";
import { boundedCatalogDiagnostic, pathIsWithin } from "../../🛡️guards/🟦️.ts";
import { readCatalogFile } from "../../📥️artifact-io/🟦️.ts";
import { decodeDescriptorPairBytesV1, validateSemanticCatalogDescriptorValue } from "../🟦️.ts";

/** 🔐️ Strict-decodes and cross-checks one owner-root SEMANTIC JSON/pack `PackageDescriptor` pair against the discovered
 * component identity: strict equal forms, a canonical pack and no byte-identity binding. */
export function validateCatalogDescriptorPair(entry: CompiledComponentOwnerV1, repoRoot: string): StrictCatalogDescriptor {
  const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
  const ownerRoot = dirname(jsonPath);
  const packPath = join(ownerRoot, CATALOG_DESCRIPTOR_PACK_FILENAME);
  if (!pathIsWithin(resolve(repoRoot), jsonPath) || !pathIsWithin(resolve(repoRoot), packPath)) throw new Error(`${entry.pluginId}: descriptor owner escapes the repository`);
  const jsonBytes = readCatalogFile(jsonPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  const packBytes = readCatalogFile(packPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  let decoded: ReturnType<typeof decodeDescriptorPairBytesV1>;
  try {
    decoded = decodeDescriptorPairBytesV1(jsonBytes, packBytes);
  } catch (error) {
    throw new Error(`${entry.pluginId}: committed descriptor pair refused: ${boundedCatalogDiagnostic(error)}`);
  }
  validateSemanticCatalogDescriptorValue(entry, decoded.record);
  return { entry, jsonPath, jsonBytes, packPath, packBytes, descriptor: decoded.packed as Record<string, unknown>, semanticSha256: createHash("sha256").update(packBytes).digest("hex") };
}
