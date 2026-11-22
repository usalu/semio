import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, relative, isAbsolute, sep } from "node:path";
import { brotliCompressSync, brotliDecompressSync, constants as zlibConstants } from "node:zlib";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export const ANALYSIS_STORE_POLICY_V1 = Object.freeze({ ...schema.definitions.Policy.const });
const P = ANALYSIS_STORE_POLICY_V1;

/** 🔑️ The key of the analysis of one source (only a source of at least `minimumSourceBytes`: the store, its read, decompression and row check, costs about what the parse of a small file does; measured 7275 analyses in a cold Rust owner, 2035 below 2 KB for 0.4 s and 1130 above 16 KB for 2.4 s of parsing): the digest of the domain, the provider id and revision, the path RELATIVE to the workspace and the digest of the content; no machine path, so the key is the same on every machine that holds the same tree. Undefined for a source outside the workspace.
 * @see ./🧪️tests/🟦️.ts */
export function analysisStoreKeyV1(root, providerId, providerRevision, path, contentSha256, contentBytes = Infinity) {
  const local = relative(root, path);
  if (contentBytes < P.minimumSourceBytes) return undefined;
  if (local === "" || local.startsWith("..") || isAbsolute(local)) return undefined;
  return createHash("sha256").update(JSON.stringify([P.domain, providerId, providerRevision, local.split(sep).join("/"), contentSha256])).digest("hex");
}

/** 🪙️ The text with every spelling of the workspace root replaced by the token, and back: reversible while the token does not occur in the text, which `neutralText` checks. */
export const neutralAnalysisTextV1 = (text, root) => text.includes(P.rootToken) ? undefined : text.replaceAll(root, P.rootToken);
export const localAnalysisTextV1 = (text, root) => text.replaceAll(P.rootToken, root);

const pathOf = (directory, key) => join(directory, key.slice(0, P.shardCharacters), key + ".json.br");

/** 📖️ The analysis stored under `key` for the workspace `root` (`{image, nestedObservations}` with the root of this tree), or undefined when none is stored or it is damaged. The caller verifies the rows before it uses it. */
export function readStoredAnalysisV1(directory, root, key) {
  try {
    const entry = JSON.parse(localAnalysisTextV1(brotliDecompressSync(readFileSync(pathOf(directory, key))).toString("utf8"), root));
    return entry?.version === 1 && entry.key === key && typeof entry.image === "string" && Array.isArray(entry.nestedObservations) ? { image: entry.image, nestedObservations: entry.nestedObservations } : undefined;
  } catch { return undefined; }
}

let writes = 0;

/** 💾️ Stores one analysis atomically (a temporary file, then a rename: two processes that store the same key write the same bytes), never above `entryMaximumBytes`, and every `collectEveryWrites` writes brings the directory under its budget. */
export function writeStoredAnalysisV1(directory, root, key, result) {
  try {
    const text = neutralAnalysisTextV1(JSON.stringify({ version: 1, key, image: result.image, nestedObservations: result.nestedObservations }), root);
    if (text === undefined) return false;
    const bytes = brotliCompressSync(Buffer.from(text), { params: { [zlibConstants.BROTLI_PARAM_QUALITY]: P.brotliQuality } });
    if (bytes.length > P.entryMaximumBytes) return false;
    const path = pathOf(directory, key), temporary = path + "." + process.pid + ".tmp";
    mkdirSync(join(directory, key.slice(0, P.shardCharacters)), { recursive: true });
    writeFileSync(temporary, bytes);
    renameSync(temporary, path);
    if (++writes % P.collectEveryWrites === 0) collectAnalysisStoreV1(directory);
    return true;
  } catch { return false; }
}

/** 🧹️ Removes the oldest entries (by modification time) until the directory holds at most `lowWaterPercent` of `maximumBytes`, when it holds more than `maximumBytes`; answers the bytes it removed. Temporary files of dead writers older than a minute go too. */
export function collectAnalysisStoreV1(directory, maximumBytes = P.maximumBytes, now = Date.now()) {
  const rows = [];
  let total = 0, removed = 0;
  try {
    for (const shard of readdirSync(directory)) {
      let names; try { names = readdirSync(join(directory, shard)); } catch { continue; }
      for (const name of names) {
        const path = join(directory, shard, name);
        try {
          const status = statSync(path);
          if (name.endsWith(".tmp")) { if (now - status.mtimeMs > 60000) rmSync(path, { force: true }); continue; }
          rows.push({ path, size: status.size, at: status.mtimeMs }); total += status.size;
        } catch {}
      }
    }
    if (total <= maximumBytes) return 0;
    const target = Math.floor(maximumBytes * P.lowWaterPercent / 100);
    for (const row of rows.sort((left, right) => left.at - right.at)) {
      if (total <= target) break;
      try { rmSync(row.path, { force: true }); total -= row.size; removed += row.size; } catch {}
    }
  } catch {}
  return removed;
}
