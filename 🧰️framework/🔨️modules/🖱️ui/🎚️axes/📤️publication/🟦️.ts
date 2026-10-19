import { existsSync, readFileSync } from "node:fs";
import type { UiAxesTarget } from "../📋️plan/🟦️.ts";

/** ✍️ Writes one generated text file; the composition root injects the repository's write-if-changed owner. */
export type UiAxesWriter = (path: string, content: string) => unknown;

/** 🔎️ Returns every missing or byte-stale UI axes projection. */
export function staleUiAxesTargets(targets: readonly UiAxesTarget[]): string[] {
  return targets.filter((target) => !existsSync(target.path) || readFileSync(target.path, "utf8") !== target.content).map(({ path }) => path);
}

/** 📤️ Publishes exactly the planned UI axis projections through the injected writer. */
export function publishUiAxes(targets: readonly UiAxesTarget[], write: UiAxesWriter): void {
  for (const target of targets) write(target.path, target.content);
}
