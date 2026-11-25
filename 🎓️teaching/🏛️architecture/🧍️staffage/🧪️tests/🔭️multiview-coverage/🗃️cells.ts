/** 🗃️ The per-asset records of a matrix run: one file `<out>/cells/<asset>.json` written when the asset ends, so a long run survives a cut, a rerun skips what is finished and shards produced by separate invocations merge.
 *
 * A record is `done` with the cells of the asset, or `failed` with a named failure (`asset-crashed`, `asset-timeout`) and its message. It carries the digest of the asset document it was produced from: a record of an asset that changed since is stale and the asset is run again.
 * The generator works on {@link AssetHead}s (id, category, digest, scale entries; {@link catalogHeads} reads them one document at a time) and merges asset by asset ({@link mergeStream}, {@link writeMatrix}), so neither a run nor a merge over the catalog holds the documents or all the cells.
 *
 * @see ./📊️matrix.ts — the generator that writes and merges the records
 * @see ./🧰️.ts — the engine
 */
import { createHash } from "node:crypto";
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, readSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { dirname, join } from "node:path";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { cellsByKey, declaredOf, failedCells, mapCatalog, placeCells, summariser, type Cell, type Declared, type Summary } from "./🧰️.ts";

/** 🏷️ The version of a record file. */
export const RECORD_VERSION = "semio.staffage.coverage.asset/v1";

/** 🧯️ The named failures of an asset whose run did not end. */
export const FAILURE_CODES = ["asset-crashed", "asset-timeout"] as const;

/** 🚑️ A named failure of one asset. */
export type AssetFailure = { readonly code: (typeof FAILURE_CODES)[number]; readonly message: string };

/** 📄️ The record of one asset: its cells when the run ended, or the named failure. */
export type AssetRecord = { readonly version: typeof RECORD_VERSION; readonly asset: string; readonly digest: string; readonly seconds: number } & ({ readonly outcome: "done"; readonly cells: readonly Cell[] } | { readonly outcome: "failed"; readonly failure: AssetFailure });

/** 🔏️ The digest of an asset document, the identity of the input of a record. */
export const assetDigest = (asset: Asset): string => createHash("sha256").update(JSON.stringify(asset)).digest("hex");

/** 🪪️ What the generator keeps of an asset document: its {@link Declared} (id, category, scale entries) and its digest. */
export type AssetHead = Declared & { readonly digest: string };

/** 👤️ The {@link AssetHead} of an asset document; a head is returned as it is. */
export const headOf = (subject: Asset | AssetHead): AssetHead => ("digest" in subject ? subject : { ...declaredOf(subject), digest: assetDigest(subject) });

/** 🗂️ The heads of the catalog in id order, of the named `ids` or of all assets, read one document at a time: no document is held. */
export const catalogHeads = (ids?: readonly string[], root?: string): AssetHead[] => mapCatalog(headOf, ids, root);

const digestOf = (subject: Asset | AssetHead): string => ("digest" in subject ? subject.digest : assetDigest(subject));

/** 📍️ The path of the record of an asset. */
export const recordPath = (out: string, assetId: string): string => join(out, "cells", `${assetId}.json`);

/** 💾️ Writes a record atomically (a temporary file renamed into place), so a cut never leaves half a record. */
export function writeRecord(out: string, record: AssetRecord): void {
  const path = recordPath(out, record.asset);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(`${path}.part`, JSON.stringify(record));
  renameSync(`${path}.part`, path);
}

/** 📖️ The record of an asset in a run folder, or undefined when there is none, it is unreadable, of another version or stale (the asset document changed since). */
export function readRecord(out: string, asset: Asset | AssetHead): AssetRecord | undefined {
  const path = recordPath(out, asset.id);
  if (!existsSync(path)) return undefined;
  try {
    const record = JSON.parse(readFileSync(path, "utf8")) as AssetRecord;
    return record.version === RECORD_VERSION && record.asset === asset.id && record.digest === digestOf(asset) ? record : undefined;
  } catch {
    return undefined;
  }
}

/** ⏭️ The assets a rerun still has to run: those without a current record, and with `retryFailed` those whose record is a named failure. */
export function pendingAssets<T extends Asset | AssetHead>(out: string, assets: readonly T[], retryFailed = false): T[] {
  return assets.filter((asset) => {
    const record = readRecord(out, asset);
    return record === undefined || (retryFailed && record.outcome === "failed");
  });
}

/** 🛟️ Runs one asset and returns its record: the cells, or `asset-crashed` with the message of the error thrown inside it. */
export function runGuarded(asset: Asset, run: (asset: Asset) => readonly Cell[]): AssetRecord {
  const started = Date.now();
  const head = { version: RECORD_VERSION, asset: asset.id, digest: assetDigest(asset) } as const;
  try {
    const cells = run(asset);
    return { ...head, seconds: (Date.now() - started) / 1000, outcome: "done", cells };
  } catch (error) {
    return failedRecord(asset, "asset-crashed", (error instanceof Error ? error.message : String(error)).split("\n")[0]!.slice(0, 400), (Date.now() - started) / 1000);
  }
}

/** 🪦️ The record of an asset whose run did not end. */
export const failedRecord = (asset: Asset | AssetHead, code: AssetFailure["code"], message: string, seconds: number): AssetRecord => ({ version: RECORD_VERSION, asset: asset.id, digest: digestOf(asset), seconds, outcome: "failed", failure: { code, message } });

/** 🧵️ One asset of a merge: its cells in the order of the matrix (every universe cell of it exactly once) and how its record ended; `missing` when no folder holds a current record. */
export type MergedAsset = { readonly asset: string; readonly cells: Cell[] } & ({ readonly outcome: "done"; readonly seconds: number } | { readonly outcome: "failed"; readonly seconds: number; readonly failure: AssetFailure } | { readonly outcome: "missing" });

/** 🚿️ The merge of the records of several run folders, one asset at a time in the order given: the cells of a `done` record, the cells of a named failure failing with that name and message, or `cell-missing` cells when no folder holds the asset. A `done` record wins over a `failed` one of another folder. Nothing of an asset is kept after its turn, so a merge over the catalog needs the memory of one record. */
export function* mergeStream(assets: Iterable<Asset | AssetHead>, folders: readonly string[]): Generator<MergedAsset> {
  for (const subject of assets) {
    const declared: Declared = "entries" in subject ? subject : declaredOf(subject);
    const records = folders.flatMap((folder) => readRecord(folder, subject) ?? []);
    const record = records.find((candidate) => candidate.outcome === "done") ?? records[0];
    if (record === undefined) yield { asset: subject.id, outcome: "missing", cells: placeCells(declared, new Map()) };
    else if (record.outcome === "done") yield { asset: subject.id, outcome: "done", seconds: record.seconds, cells: placeCells(declared, cellsByKey(record.cells)) };
    else yield { asset: subject.id, outcome: "failed", seconds: record.seconds, failure: record.failure, cells: failedCells(declared, record.failure.code, record.failure.message) };
  }
}

/** 🧩️ The merge of the records of several run folders (shards of separate invocations): the matrix of the assets with every cell exactly once, the assets that are done, those with a named failure (their cells fail with that name and message) and those no folder holds (their cells are `cell-missing`). A `done` record wins over a `failed` one of another folder. */
export function mergeRecords(assets: readonly (Asset | AssetHead)[], folders: readonly string[]): { readonly cells: Cell[]; readonly done: string[]; readonly failed: { readonly asset: string; readonly code: string; readonly message: string }[]; readonly missing: string[]; readonly seconds: Readonly<Record<string, number>> } {
  const run = mergeRun();
  const cells: Cell[] = [];
  for (const merged of mergeStream(assets, folders)) {
    run.add(merged);
    for (const cell of merged.cells) cells.push(cell);
  }
  return { cells, ...run.result() };
}

/** 🧮️ Collects how the assets of a merge ended, without their cells: the assets that are done, those with a named failure, those no folder holds, and the seconds each took. */
export function mergeRun(): { readonly add: (merged: MergedAsset) => void; readonly result: () => { readonly done: string[]; readonly failed: { readonly asset: string; readonly code: string; readonly message: string }[]; readonly missing: string[]; readonly seconds: Readonly<Record<string, number>> } } {
  const done: string[] = [];
  const failed: { asset: string; code: string; message: string }[] = [];
  const missing: string[] = [];
  const seconds: Record<string, number> = {};
  const add = (merged: MergedAsset): void => {
    if (merged.outcome === "missing") return void missing.push(merged.asset);
    seconds[merged.asset] = merged.seconds;
    if (merged.outcome === "done") done.push(merged.asset);
    else failed.push({ asset: merged.asset, ...merged.failure });
  };
  return { add, result: () => ({ done, failed, missing, seconds }) };
}

/** 🔖️ The version of the merged matrix file. */
export const MATRIX_VERSION = "semio.staffage.coverage/v1";

/** 🏁️ How the assets of a merge ended: the count of the done ones, the named failures, the missing ones and the seconds each took. */
export type MatrixRun = { readonly done: number; readonly failed: readonly { readonly asset: string; readonly code: string; readonly message: string }[]; readonly missing: readonly string[]; readonly seconds: Readonly<Record<string, number>> };

/** 🖋️ Merges the run folders into `<out>/matrix.json` (`{ version, assets, coverage, summary, run, cells }`, every cell of the assets exactly once) and `<out>/summary.json` (`{ coverage, summary, run }`), asset by asset: the cells go to a part file as they are merged and are copied behind the head once the summary is known, so the merge of the whole catalog holds one record at a time. `matrix.json` appears complete or not at all (a temporary file renamed into place). Returns the summary, the run and the count of cells. */
export function writeMatrix(out: string, assets: readonly (Asset | AssetHead)[], folders: readonly string[], coverage: unknown): { readonly summary: Summary; readonly run: MatrixRun; readonly cells: number } {
  const counted = summariser();
  const outcomes = mergeRun();
  const cellsPart = join(out, "matrix.json.cells.part");
  const matrixPart = join(out, "matrix.json.part");
  mkdirSync(out, { recursive: true });
  let cells = 0;
  const cellsFile = openSync(cellsPart, "w");
  try {
    for (const merged of mergeStream(assets, folders)) {
      counted.add(merged.cells);
      outcomes.add(merged);
      if (merged.cells.length > 0) writeSync(cellsFile, `${cells > 0 ? "," : ""}${merged.cells.map((cell) => JSON.stringify(cell)).join(",")}`);
      cells += merged.cells.length;
    }
  } finally {
    closeSync(cellsFile);
  }
  const summary = counted.result();
  const ended = outcomes.result();
  const run: MatrixRun = { done: ended.done.length, failed: ended.failed, missing: ended.missing, seconds: ended.seconds };
  const matrixFile = openSync(matrixPart, "w");
  const cellsRead = openSync(cellsPart, "r");
  try {
    writeSync(matrixFile, `${JSON.stringify({ version: MATRIX_VERSION, assets: assets.length, coverage, summary, run }).slice(0, -1)},"cells":[`);
    const chunk = Buffer.allocUnsafe(1 << 20);
    for (let read = readSync(cellsRead, chunk, 0, chunk.length, null); read > 0; read = readSync(cellsRead, chunk, 0, chunk.length, null)) writeSync(matrixFile, chunk, 0, read);
    writeSync(matrixFile, "]}");
  } finally {
    closeSync(cellsRead);
    closeSync(matrixFile);
  }
  renameSync(matrixPart, join(out, "matrix.json"));
  rmSync(cellsPart);
  writeFileSync(join(out, "summary.json"), JSON.stringify({ coverage, summary, run }, null, 2));
  return { summary, run, cells };
}
