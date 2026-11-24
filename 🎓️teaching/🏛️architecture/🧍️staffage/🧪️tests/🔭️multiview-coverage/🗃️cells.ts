/** 🗃️ The per-asset records of a matrix run: one file `<out>/cells/<asset>.json` written when the asset ends, so a long run survives a cut, a rerun skips what is finished and shards produced by separate invocations merge.
 *
 * A record is `done` with the cells of the asset, or `failed` with a named failure (`asset-crashed`, `asset-timeout`) and its message. It carries the digest of the asset document it was produced from: a record of an asset that changed since is stale and the asset is run again.
 *
 * @see ./📊️matrix.ts — the generator that writes and merges the records
 * @see ./🧰️.ts — the engine
 */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { assemble, failedAsset, type Cell } from "./🧰️.ts";

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
export function readRecord(out: string, asset: Asset): AssetRecord | undefined {
  const path = recordPath(out, asset.id);
  if (!existsSync(path)) return undefined;
  try {
    const record = JSON.parse(readFileSync(path, "utf8")) as AssetRecord;
    return record.version === RECORD_VERSION && record.asset === asset.id && record.digest === assetDigest(asset) ? record : undefined;
  } catch {
    return undefined;
  }
}

/** ⏭️ The assets a rerun still has to run: those without a current record, and with `retryFailed` those whose record is a named failure. */
export function pendingAssets(out: string, assets: readonly Asset[], retryFailed = false): Asset[] {
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
export const failedRecord = (asset: Asset, code: AssetFailure["code"], message: string, seconds: number): AssetRecord => ({ version: RECORD_VERSION, asset: asset.id, digest: assetDigest(asset), seconds, outcome: "failed", failure: { code, message } });

/** 🧩️ The merge of the records of several run folders (shards of separate invocations): the matrix of the assets with every cell exactly once, the assets that are done, those with a named failure (their cells fail with that name and message) and those no folder holds (their cells are `cell-missing`). A `done` record wins over a `failed` one of another folder. */
export function mergeRecords(assets: readonly Asset[], folders: readonly string[]): { readonly cells: Cell[]; readonly done: string[]; readonly failed: { readonly asset: string; readonly code: string; readonly message: string }[]; readonly missing: string[]; readonly seconds: Readonly<Record<string, number>> } {
  const produced: Cell[] = [];
  const done: string[] = [];
  const failed: { asset: string; code: string; message: string }[] = [];
  const missing: string[] = [];
  const seconds: Record<string, number> = {};
  for (const asset of assets) {
    const records = folders.flatMap((folder) => readRecord(folder, asset) ?? []);
    const record = records.find((candidate) => candidate.outcome === "done") ?? records[0];
    if (record === undefined) {
      missing.push(asset.id);
      continue;
    }
    seconds[asset.id] = record.seconds;
    if (record.outcome === "done") {
      done.push(asset.id);
      produced.push(...record.cells);
    } else {
      failed.push({ asset: asset.id, ...record.failure });
      produced.push(...failedAsset(asset, record.failure.code, record.failure.message));
    }
  }
  return { cells: assemble(assets, produced), done, failed, missing, seconds };
}
