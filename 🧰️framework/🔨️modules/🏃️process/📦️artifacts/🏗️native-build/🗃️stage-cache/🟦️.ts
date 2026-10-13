import { constants, createReadStream } from "node:fs";
import { copyFile, lstat, mkdir, readdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { createHash, randomUUID } from "node:crypto";
import { dirname, join, relative, sep } from "node:path";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🗃️ Stage cache policy: warm and fresh store names, the entry manifest and the temporary prefix. */
export const STAGE_CACHE_POLICY_V1 = schema.definitions.Policy.const;

/** 🔑️ What a post-link stage is a pure function of: the digests of its inputs, the identities of its tools and its own schema version (`StageKeyV1`). */
export type StageKeyV1 = Readonly<{ stage: string; schemaVersion: number; inputs: Readonly<Record<string, string>>; tools: Readonly<Record<string, string>> }>;

/** 📄️ One stored output file. */
export type StageFileV1 = Readonly<{ path: string; sha256: string; bytes: number }>;

/** 🧾️ The manifest stored beside a stage's outputs (`StageEntryV1`). */
export type StageEntryV1 = Readonly<{ version: 1; key: StageKeyV1; digest: string; files: readonly StageFileV1[] }>;

/** 🎯️ How one stage request was served. */
export type StageResultV1 = Readonly<{ hit: boolean; digest: string; files: readonly StageFileV1[] }>;

/** 📊️ Hits, misses and bytes of one cache instance. */
export type StageCacheStatsV1 = Readonly<{ hits: number; misses: number; restoredBytes: number; storedBytes: number }>;

/** 🗃️ A content-addressed store of stage outputs. */
export interface StageCacheV1 {
  readonly root: string;
  run(key: StageKeyV1, output: string, produce: (directory: string) => Promise<void>, signal?: AbortSignal): Promise<StageResultV1>;
  stats(): StageCacheStatsV1;
}

/** 🛂️ Validates one value against a named stage cache schema definition. */
function admitDefinition<T>(name: keyof typeof schema.definitions, value: unknown): T {
  const errors = validateJsonSchemaSubset(schema.definitions[name], value, schema);
  if (errors.length) throw Error(`Invalid ${name}: ${errors.join("; ")}`);
  return value as T;
}

/** 🔤️ Object keys ordered by their UTF-8 bytes (Unicode code point order), recursively, so every language serializes one key to the same bytes. */
function canonical(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonical);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.keys(value).sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right))).map((key) => [key, canonical((value as Record<string, unknown>)[key])]));
}

/** 🧾️ The canonical bytes of a stage key: recursively ordered keys, no whitespace, UTF-8. */
export function stageKeyCanonicalV1(key: StageKeyV1): string {
  return JSON.stringify(canonical(admitDefinition<StageKeyV1>("StageKeyV1", key)));
}

/** #️⃣ The SHA-256 of a stage key's canonical bytes: the directory its outputs live under.
 * @see ./🧫️fixtures/🔣️.json */
export function stageKeyDigestV1(key: StageKeyV1): string {
  return createHash("sha256").update(stageKeyCanonicalV1(key)).digest("hex");
}

/** #️⃣ Streams one file through SHA-256. */
export function fileSha256V1(path: string): Promise<string> {
  return new Promise((accept, reject) => {
    const hash = createHash("sha256");
    createReadStream(path).on("error", reject).on("data", (chunk) => hash.update(chunk)).on("end", () => accept(hash.digest("hex")));
  });
}

/** 🌲️ Every regular file below `root` as `/`-separated relative paths in UTF-8 byte order; links and other kinds are refused. */
async function regularFiles(root: string): Promise<string[]> {
  const found: string[] = [];
  const walk = async (directory: string): Promise<void> => {
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) await walk(path);
      else if (entry.isFile()) found.push(relative(root, path).split(sep).join("/"));
      else throw Error(`Stage output must hold regular files only: ${path}`);
    }
  };
  await walk(root);
  return found.sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}

/** 📥️ Copies the stored files into a fresh output directory (copy-on-write clones where the filesystem offers them) and verifies every digest. */
async function restore(entry: string, files: readonly StageFileV1[], output: string): Promise<number> {
  await rm(output, { recursive: true, force: true });
  await mkdir(output, { recursive: true });
  let bytes = 0;
  for (const file of files) {
    const source = join(entry, ...file.path.split("/")), target = join(output, ...file.path.split("/"));
    await mkdir(dirname(target), { recursive: true });
    await copyFile(source, target, constants.COPYFILE_FICLONE);
    if ((await lstat(target)).size !== file.bytes || await fileSha256V1(target) !== file.sha256) throw Error(`Stage cache entry changed: ${source}`);
    bytes += file.bytes;
  }
  return bytes;
}

/** 📖️ The admitted entry manifest stored under `entry`, or `null` when none is complete. */
async function readEntry(entry: string, digest: string): Promise<StageEntryV1 | null> {
  const text = await readFile(join(entry, STAGE_CACHE_POLICY_V1.entryFile), "utf8").catch(() => null);
  if (text === null) return null;
  const value = admitDefinition<StageEntryV1>("StageEntryV1", JSON.parse(text));
  if (value.digest !== digest || stageKeyDigestV1(value.key) !== digest) throw Error(`Stage cache entry names another key: ${entry}`);
  return value;
}

/** 🗃️ Opens the stage cache at `root` (`⚡️cache/stages` for a warm build, `<generation>/s` for a fresh one). A request whose key is stored restores its files; a
 * miss runs `produce` into a private directory, records every file with its digest and publishes the entry by one atomic rename, so concurrent builders of
 * one key never see a half-written entry and the second publisher keeps the first entry. Outputs are restored, never linked, so a consumer cannot alter an entry.
 * @see ./🧬️schema/🔣️.json
 * @see ../🧩️component-pass/🟦️.ts */
export function stageCacheV1(root: string): StageCacheV1 {
  let hits = 0, misses = 0, restoredBytes = 0, storedBytes = 0;
  return {
    root,
    async run(key, output, produce, signal) {
      const digest = stageKeyDigestV1(key), entry = join(root, key.stage, digest);
      signal?.throwIfAborted();
      const stored = await readEntry(entry, digest);
      if (stored) {
        restoredBytes += await restore(entry, stored.files, output);
        hits++;
        return Object.freeze({ hit: true, digest, files: stored.files });
      }
      await mkdir(join(root, key.stage), { recursive: true });
      const temporary = join(root, key.stage, `${STAGE_CACHE_POLICY_V1.temporaryPrefix}${randomUUID()}`);
      await mkdir(temporary);
      try {
        await produce(temporary);
        signal?.throwIfAborted();
        const files: StageFileV1[] = [];
        for (const path of await regularFiles(temporary)) {
          const file = join(temporary, ...path.split("/"));
          files.push({ path, sha256: await fileSha256V1(file), bytes: (await lstat(file)).size });
        }
        const manifest: StageEntryV1 = admitDefinition<StageEntryV1>("StageEntryV1", { version: 1, key, digest, files });
        await writeFile(join(temporary, STAGE_CACHE_POLICY_V1.entryFile), `${JSON.stringify(manifest)}\n`, { flag: "wx" });
        try { await rename(temporary, entry); storedBytes += files.reduce((sum, file) => sum + file.bytes, 0); }
        catch (error) { if (!["EEXIST", "ENOTEMPTY", "EPERM"].includes((error as { code?: string }).code ?? "") || !await readEntry(entry, digest)) throw error; }
        misses++;
        const published = (await readEntry(entry, digest))!;
        restoredBytes += await restore(entry, published.files, output);
        return Object.freeze({ hit: false, digest, files: published.files });
      } finally {
        await rm(temporary, { recursive: true, force: true });
      }
    },
    stats: () => Object.freeze({ hits, misses, restoredBytes, storedBytes }),
  };
}
