import { join, relative } from "node:path";
import type { UiAxes } from "../📥️source/🟦️.ts";
import { emitUiAxesRust, emitUiAxesTypeScript } from "../📽️projection/🟦️.ts";

export interface UiAxesTarget {
  readonly path: string;
  readonly content: string;
}

/** 📋️ Plans the two implementation projections from one neutral source. */
export function uiAxesTargets(repoRoot: string, axes: UiAxes): readonly UiAxesTarget[] {
  return [
    { path: join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs"), content: emitUiAxesRust(axes) },
    { path: join(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts"), content: emitUiAxesTypeScript(axes) },
  ];
}

/** 🧾️ Produces the canonical read-only generator protocol from the shared byte plan. */
export function uiAxesPreview(repoRoot: string, targets: readonly UiAxesTarget[]) {
  const nodes = targets.map((target) => ({ bytesBase64: Buffer.from(target.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, target.path).replaceAll("\\", "/").normalize("NFC") })).sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
  return { contractId: "ui-axes", nodes, schemaVersion: 1, staleRemovals: [] as string[] };
}
