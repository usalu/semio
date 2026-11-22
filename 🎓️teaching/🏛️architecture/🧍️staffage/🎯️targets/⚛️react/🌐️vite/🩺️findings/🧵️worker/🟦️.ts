/** 🧵️ The thread that checks authored asset texts: it parses one text per request, runs `validateAsset` and posts the progress of the check and then its findings, so the thread that serves modules never runs a check.
 * @see ../🟦️.ts — the service that owns these threads
 * @see ../../../../../🔨️modules/🗂️catalog/🟦️.ts — `validateAsset` */
// #region 🔌️Adapters
import { parentPort } from "node:worker_threads";
// #endregion 🔌️Adapters
import { validateAsset, type Issue } from "../../../../../🔨️modules/🗂️catalog/🟦️.ts";
import type { Asset, Text } from "../../../../../🧬️schema/🟦️.ts";
import { CHECK_THREW } from "../🟦️.ts";

/** 📨️ One text to check, named by the id its answers carry. */
export type FindingsQuestion = { readonly id: number; readonly text: string };

/** 📬️ What the thread posts: the progress of a check, then its findings. */
export type FindingsAnswer = { readonly id: number; readonly progress: readonly [done: number, total: number, label: Text] } | { readonly id: number; readonly issues: readonly Issue[] };

/** 🩺️ The findings of one authored asset text; a check that throws is itself the finding `CHECK_THREW`. */
export function findingsOfText(text: string, progress?: (done: number, total: number, label: Text) => void): readonly Issue[] {
  try {
    return validateAsset((JSON.parse(text) as { asset: Asset }).asset, progress === undefined ? {} : { progress });
  } catch {
    return [CHECK_THREW];
  }
}

parentPort?.on("message", ({ id, text }: FindingsQuestion) => {
  const issues = findingsOfText(text, (done, total, label) => parentPort!.postMessage({ id, progress: [done, total, label] } satisfies FindingsAnswer));
  parentPort!.postMessage({ id, issues } satisfies FindingsAnswer);
});
