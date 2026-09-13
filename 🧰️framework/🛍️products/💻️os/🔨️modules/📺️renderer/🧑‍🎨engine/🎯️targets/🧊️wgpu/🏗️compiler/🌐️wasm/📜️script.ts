import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { delimiter, dirname, join, relative, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { stageArtifacts } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { runTool } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts";
import { preparedBinaryen } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts";
import { buildBudgetMs } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

export type TrunkRendererBuild = { readonly rustPackageRoot: string; readonly workspace: string; readonly profile: string; readonly toolWorkspace?: string; readonly stateRoot?: string; readonly signal?: AbortSignal };

/** 🎯️ Gives finite compiler results one owner per optimization profile. */
export function wasmRendererDirectory(root: string, profile: string): string {
  if (profile !== "dev" && profile !== "release") throw new Error("Select the wasm or wasm-release Nx target");
  return join(root, "dist", `wasm-${profile}`);
}

/** 🏗️ Compiles only Rust renderer bytes; browser composition consumes its completed artifacts separately. */
export async function buildTrunkRenderer(options: TrunkRendererBuild): Promise<void> {
  const { rustPackageRoot: root, workspace, profile } = options, output = wasmRendererDirectory(root, profile), toolWorkspace = options.toolWorkspace ?? workspace;
  const budget = buildBudgetMs(), signal = AbortSignal.any([options.signal ?? new AbortController().signal, ...(budget > 0 ? [AbortSignal.timeout(budget)] : [])]);
  const manifest = Bun.TOML.parse(readFileSync(join(root, "Cargo.toml"), "utf8")) as { package: { name: string } };
  const lock = Bun.TOML.parse(readFileSync(join(toolWorkspace, "Cargo.lock"), "utf8")) as { package: { name: string; version: string }[] };
  const bindgen = [...new Set(lock.package.filter(row => row.name === "wasm-bindgen").map(row => row.version))];
  if (bindgen.length !== 1) throw new Error("Renderer tooling requires one locked wasm-bindgen version");
  const optimizer = preparedBinaryen(toolWorkspace), environment = { ...process.env };
  for (const key of Object.keys(environment)) if (key.toUpperCase().startsWith("TRUNK_") || ["NO_COLOR", "FORCE_COLOR"].includes(key.toUpperCase())) delete environment[key];
  const pathKey = Object.keys(environment).find(key => key.toUpperCase() === "PATH") ?? "PATH";
  environment[pathKey] = `${dirname(optimizer)}${delimiter}${environment[pathKey] ?? ""}`;
  const version = await runTool("wasm-bindgen", ["--version"], root, signal, true, environment);
  if (version.trim() !== `wasm-bindgen ${bindgen[0]}`) throw new Error(`Prepared wasm-bindgen must match Cargo.lock: ${bindgen[0]}`);
  const staging = join(resolve(workspace, options.stateRoot ?? repoCacheDirectory(workspace, "trunk")), "staging");
  mkdirSync(staging, { recursive: true });
  const temporary = mkdtempSync(join(staging, `${profile}-`)), dist = join(temporary, "dist");
  try {
    const href = relative(temporary, join(root, "Cargo.toml")).replaceAll("\\", "/").replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;");
    writeFileSync(join(temporary, "index.html"), `<!doctype html><html><head><link data-trunk rel="rust" href="${href}" data-wasm-opt="z" data-type="worker" data-bindgen-target="web" /></head><body></body></html>\n`);
    writeFileSync(join(temporary, "Trunk.toml"), `required_version = "=0.21.14"\noffline = true\n[build]\nlocked = true\nfrozen = true\ntarget = "index.html"\ndist = "dist"\nfilehash = false\nrelease = ${profile === "release"}\n[tools]\nwasm_bindgen = ${JSON.stringify(bindgen[0])}\nwasm_opt = "version_130"\n`);
    environment.CARGO_TARGET_DIR = join(temporary, "target");
    await runTool("cargo", ["metadata", "--locked", "--offline", "--format-version=1", "--manifest-path", join(root, "Cargo.toml")], root, signal, "ignore", environment);
    await runTool("trunk", ["build", "--config", join(temporary, "Trunk.toml"), "--skip-version-check", "--offline", "true", "--color", "never"], root, signal, false, environment);
    const files = new Map<string, string>();
    for (const entry of readdirSync(dist, { recursive: true, withFileTypes: true })) if (entry.isFile() && entry.name !== "index.html") {
      const path = join(entry.parentPath, entry.name);
      files.set(relative(dist, path).replaceAll("\\", "/"), path);
    }
    for (const name of [`${manifest.package.name}.js`, `${manifest.package.name}_bg.wasm`]) if (!files.has(name)) throw new Error(`Trunk omitted its renderer artifact: ${name}`);
    await stageArtifacts(output, `wgpu-renderer:${profile}`, files, { signal, leaseDirectory: repoCacheDirectory(workspace, "agents", "resource-leases") });
    console.log(`[nx-trunk] Published ${files.size} renderer files: ${output}`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}

class BuildScript extends BundleScript {
  async run([profile, ...args]: string[]): Promise<void> {
    if (args.length) throw new Error("Renderer compilation accepts only its Nx profile");
    const controller = new AbortController(), stop = (): void => controller.abort();
    process.once("SIGINT", stop); process.once("SIGTERM", stop);
    try { await buildTrunkRenderer({ rustPackageRoot: this.root, workspace: this.repoRoot, profile, signal: controller.signal }); }
    finally { process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop); }
  }
}

if (import.meta.main) await new ScriptRouter(resolve(import.meta.dir, "../../📦️packages/🦀️rust")).register("build", BuildScript).run(process.argv.slice(2));
