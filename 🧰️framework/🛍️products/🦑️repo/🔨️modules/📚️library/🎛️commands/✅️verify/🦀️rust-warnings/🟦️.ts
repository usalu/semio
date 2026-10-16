/** ⚠️ Plugin-crate and target scope for `verify rust-warnings`.
 * @see 📜️script.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

/** 🦀️ Shipping plugins, extensions, and their artifact libraries from the repository's package catalog. */
function pluginCrateNames(root: string, includeSupport = false): string[] {
  const workspace = (Bun.TOML.parse(readFileSync(join(root, "Cargo.toml"), "utf8")) as { workspace: { members: string[]; exclude?: string[] } }).workspace;
  const excluded = (workspace.exclude ?? []).map((pattern) => new Bun.Glob(pattern));
  const manifests = workspace.members.flatMap((member) => {
    const paths = [...new Bun.Glob(`${member}/Cargo.toml`).scanSync({ cwd: root, onlyFiles: true })];
    if (!paths.length) throw new Error(`[verify rust-warnings] member manifest missing for ${member}.`);
    return paths.filter((path) => !excluded.some((pattern) => pattern.match(path.slice(0, -"/Cargo.toml".length))));
  });
  const names = manifests.flatMap((path) => {
    const manifest = Bun.TOML.parse(readFileSync(join(root, path), "utf8"));
    const pkg = (manifest as { package?: { name?: string; metadata?: { semio?: { role?: string } } } }).package;
    const name = pkg?.name;
    if (!name) throw new Error(`[verify rust-warnings] package name missing in ${path}.`);
    return ["plugin", "extension"].includes(pkg?.metadata?.semio?.role ?? "") || name.startsWith("semio-s-artifact-") || name.startsWith("semio-framework-artifact-") || (includeSupport && name.startsWith("semio-s-plugin-")) ? [name] : [];
  });
  return [...new Set(names)].sort();
}

/** 🎯️ Native test surfaces, WASI component libraries, and browser actor/renderer libraries. */
export function rustWarningTargetScope(root: string, target: string | undefined): { packages: string[]; scopeArgs: string[]; targetArgs: string[]; packageArgs: Record<string, string[]> } {
  const renderer = "semio-framework-os-renderer-wgpu";
  if (target === "wasm32-wasip2") return { packages: pluginCrateNames(root), scopeArgs: ["--lib"], targetArgs: ["--target", target], packageArgs: {} };
  if (target === "wasm32-unknown-unknown") return { packages: ["semio-framework-actor", renderer], scopeArgs: ["--lib"], targetArgs: ["--target", target], packageArgs: {} };
  if (target && target !== "native") throw new Error(`[verify rust-warnings] unknown target ${target} (expected native | wasm32-wasip2 | wasm32-unknown-unknown).`);
  return { packages: ["semio-framework-actor", "semio-framework", "semio-framework-os-kernel", renderer, ...pluginCrateNames(root, true)], scopeArgs: ["--all-targets"], targetArgs: [], packageArgs: { [renderer]: ["--features", "native-bin"], "semio-framework-os-kernel": ["--features", "native-bin"] } };
}
