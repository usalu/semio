import { existsSync, readFileSync } from "node:fs";
import { writeGeneratedFileIfChanged } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts";
import type { UiAxesTarget } from "../📋️plan/🟦️.ts";

/** 🔎️ Returns every missing or byte-stale UI axes projection. */
export function staleUiAxesTargets(targets: readonly UiAxesTarget[]): string[] {
  return targets.filter((target) => !existsSync(target.path) || readFileSync(target.path, "utf8") !== target.content).map(({ path }) => path);
}

/** 📤️ Publishes exactly the planned UI axis projections. */
export function publishUiAxes(targets: readonly UiAxesTarget[]): void {
  for (const target of targets) writeGeneratedFileIfChanged(target.path, target.content);
}
