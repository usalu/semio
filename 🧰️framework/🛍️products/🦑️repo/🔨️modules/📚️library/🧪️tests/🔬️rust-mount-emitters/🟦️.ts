import { describe, expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

/** 🔎️ Every TypeScript source of the framework and the plugin roots that writes a Rust `#[path = …]` mount into text (template or `JSON.stringify` interpolation). */
export function rustMountEmitters(root: string): string[] {
  const files = fastGlob.sync(["🧰️framework/**/*.ts", "✏️s/**/*.ts"], { cwd: root, ignore: ["**/node_modules/**", "**/dist/**", "**/🧪️tests/**", "**/🧫️fixtures/**", "**/📜️script.ts", "**/🗑️generated/**"], onlyFiles: true, followSymbolicLinks: false });
  const emitters: string[] = [];
  for (const file of files) {
    const text = readFileSync(join(root, file), "utf8");
    if (!text.includes("#[path")) continue;
    const emits = text.split("\n").some((line) => {
      const trimmed = line.trim();
      if (trimmed.startsWith("//") || trimmed.startsWith("*") || trimmed.startsWith("/**")) return false;
      return /#\[path = (?:\$\{JSON\.stringify|"\$\{|\$\{)/u.test(line) || /`[^`]*#\[path = "\$\{[^`]*`/u.test(line) || /`[^`]*#\[path = "\.\.\/[^`]*`/u.test(line);
    });
    if (emits) emitters.push(file);
  }
  return emitters.sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
}

/** 🧭️ The exact set of generators allowed to write a `#[path]` mount; a new emitter must be reviewed against the crate-boundary rules first (docs: the S7 audit of the wave-0 result). */
export const RUST_MOUNT_EMITTERS = [
  "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌲️inline-mounts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🔸️part2/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🧰️support-hoist/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧱️scaffold/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗂️collection-catalog/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🧬️mutation-tree/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧭️reference-resolution/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts",
];

describe("rust mount emitters", () => {
  test("only the audited generators write #[path] mounts", () => {
    expect(rustMountEmitters(getWorkspaceRoot())).toEqual(RUST_MOUNT_EMITTERS);
  }, 120_000);
});
