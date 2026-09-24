import { existsSync } from "node:fs";
import { readUiAxes } from "../📥️source/🟦️.ts";
import { uiAxesTargets } from "../📋️plan/🟦️.ts";
import { publishUiAxes } from "../📤️publication/🟦️.ts";

/** 🌱️ Publishes the UI axis projections a fresh clone lacks. `🛂️manifest` value-imports the TypeScript projection, so
 * every repository script — this generator's own `ui-rs:generate` included — fails to load until it exists; present
 * projections stay owned by `ui-rs:generate` and `ui-rs:check`. Declared in `⚡️caching/🚀️bootstrap/🌱️sources/🔣️.json`. */
export function bootstrapUiAxes(repoRoot: string): void {
  publishUiAxes(uiAxesTargets(repoRoot, readUiAxes(repoRoot)).filter((target) => !existsSync(target.path)));
}
