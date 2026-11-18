import { existsSync } from "node:fs";
import { join, resolve } from "node:path";

/** 🦀️ Native executable name of the jco-identical component transpile for a platform. */
export function nativeTranspileBinaryNameV1(platform: string = process.platform): string {
  return platform === "win32" ? "semio-native-transpile.exe" : "semio-native-transpile";
}

/** 📍️ The staged native transpile of a platform: `SEMIO_NATIVE_TRANSPILE_BIN` when set, else the release tool directory the `📜️stage.ts` script fills.
 * @see {@link ./📜️stage.ts} */
export function nativeTranspileBinaryV1(repoRoot: string, environment: Readonly<Record<string, string | undefined>> = process.env, platform: string = process.platform): string {
  const selected = environment.SEMIO_NATIVE_TRANSPILE_BIN;
  const path = selected ? resolve(repoRoot, selected) : join(repoRoot, ".cache", "b", "p", "m", "browser", "release", "tools", "native-transpile", platform, nativeTranspileBinaryNameV1(platform));
  if (!existsSync(path)) throw new Error(`native transpile is not staged at ${path}; run \`repo ./📜️stage.ts\` in ${join("🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "🌐️browser-bundle", "🏗️materialization", "🦀️native-transpile")}`);
  return path;
}

/** 🧾️ The command line of one component transpile: jco's `transpile <component> -o <dir> --name <name> --map ...` surface, executed natively. */
export function nativeTranspileArgsV1(component: string, outDir: string, componentBase: string, maps: readonly string[]): readonly string[] {
  return ["transpile", component, "-o", outDir, "--name", componentBase, ...maps.flatMap((map) => ["--map", map])];
}

/** 🗺️ The import maps of every plugin component: the pure and host-async framework worlds share one shim module. */
export const PLUGIN_COMPONENT_IMPORT_MAPS_V1: readonly string[] = ["semio:framework/pure=./🟨️.js", "semio:framework/host-async=./🟨️.js"];
