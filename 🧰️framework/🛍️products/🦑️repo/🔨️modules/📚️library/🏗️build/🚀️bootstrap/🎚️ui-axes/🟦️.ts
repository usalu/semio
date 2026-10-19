import { existsSync } from "node:fs";
import { writeGeneratedFileIfChanged } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { uiAxesTargets } from "../../../../../../../🔨️modules/🖱️ui/🎚️axes/📋️plan/🟦️.ts";
import { publishUiAxes } from "../../../../../../../🔨️modules/🖱️ui/🎚️axes/📤️publication/🟦️.ts";
import { readUiAxes } from "../../../../../../../🔨️modules/🖱️ui/🎚️axes/📥️source/🟦️.ts";

/** 🌱️ Publishes the UI axis projections a fresh clone lacks. `🛂️manifest` value-imports the TypeScript projection, so
 * every repository script — this generator's own `ui-rs:generate` included — fails to load until it exists; present
 * projections stay owned by `ui-rs:generate` and `ui-rs:check`. Declared in `🏗️build/🚀️bootstrap/🌱️sources/🔣️.json`. */
export function bootstrapUiAxes(repoRoot: string): void {
  publishUiAxes(uiAxesTargets(repoRoot, readUiAxes(repoRoot)).filter((target) => !existsSync(target.path)), writeGeneratedFileIfChanged);
}
