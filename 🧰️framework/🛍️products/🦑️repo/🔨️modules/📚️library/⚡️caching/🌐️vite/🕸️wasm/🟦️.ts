import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { readBuildMode, type BuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { readWasmPackageOwnerV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import type { OwnedBuildPlugin } from "../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";

/** 🕸️ Resolves browser builds to canonical WASM snapshots instead of the active package projection. */
export function canonicalWasmBuildVitePlugin(workspace: string, mode: BuildMode): OwnedBuildPlugin {
  readBuildMode(mode);
  return { name: "semio-canonical-wasm-build", apply: "build", enforce: "pre", resolveId(id, importer) {
    const match = /^(@semio-tech\/[^/]+)(?:\/(.+))?$/u.exec(id);
    if (!match) return;
    const require = createRequire(importer && isAbsolute(importer) ? importer.split("?")[0]! : join(workspace, "package.json"));
    const manifestFile = (require.resolve.paths(match[1]!) ?? []).map(directory => join(directory, match[1]!, "package.json")).find(existsSync);
    if (!manifestFile) return;
    const manifest = JSON.parse(readFileSync(manifestFile, "utf8")), root = dirname(manifestFile), exported = manifest.exports?.[match[2] ? `./${match[2]}` : "."];
    const target = typeof exported === "string" ? exported : exported?.import ?? exported?.default ?? match[2] ?? manifest.module ?? manifest.main;
    if (typeof target !== "string") return;
    const active = resolve(root, target);
    let owner = manifest.semioWasmOwner, sourceDirectory = root;
    for (let directory = dirname(active); !owner && directory.startsWith(root) && directory !== root; directory = dirname(directory)) {
      const file = join(directory, "package.json");
      if (existsSync(file)) { const candidate = JSON.parse(readFileSync(file, "utf8")); if (candidate.semioWasmOwner) { owner = candidate.semioWasmOwner; sourceDirectory = directory; } }
    }
    if (!owner) {
      if (/(?:^|\/)(?:pkg|🕸️bindings)\//u.test(target) || manifest.files?.some((file: unknown) => typeof file === "string" && file.endsWith(".wasm"))) throw Error(`Missing canonical WASM owner for ${id}; build its WASM producer`);
      return;
    }
    try { owner = readWasmPackageOwnerV1(owner); } catch (error) { throw Error(`Invalid canonical WASM owner for ${id}`, { cause: error }); }
    const file = relative(sourceDirectory, active);
    if (file.startsWith("..") || isAbsolute(file)) throw Error(`WASM package export escapes its owner: ${id}`);
    const canonical = join(workspace, owner.sourceOwner, "dist/wasm", mode, file);
    if (!existsSync(canonical)) throw Error(`Missing ${mode} WASM snapshot for ${id}: ${canonical}`);
    return canonical;
  } };
}
