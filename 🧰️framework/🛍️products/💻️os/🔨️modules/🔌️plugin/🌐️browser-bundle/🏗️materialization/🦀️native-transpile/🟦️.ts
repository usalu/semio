import { accessSync, constants, statSync } from "node:fs";
import { join, resolve } from "node:path";
import type { BuildMode } from "@semio-tech/process/build-mode";

/** 🦀️ Native executable name of the jco-identical component transpile for a platform. */
export function nativeTranspileBinaryNameV1(platform: string = process.platform): string {
  return platform === "win32" ? "semio-native-transpile.exe" : "semio-native-transpile";
}

/** 📍️ Publication path owned by the selected native transpile compiler mode and platform. */
export function nativeTranspilePublicationPathV1(repoRoot: string, mode: BuildMode, platform: string): string {
  return join(repoRoot, ".cache", "b", "p", "m", "browser", mode, "tools", "native-transpile", platform, nativeTranspileBinaryNameV1(platform));
}

function nativeTranspilePathV1(repoRoot: string, mode: BuildMode, override: string | undefined, platform: string): string {
  return override ? resolve(repoRoot, override) : nativeTranspilePublicationPathV1(repoRoot, mode, platform);
}

function nativeTranspileExecutableExistsV1(path: string): boolean {
  try {
    const stat = statSync(path);
    if (!stat.isFile() || stat.size === 0) throw Error(`Native transpile executable is invalid at ${path}`);
    accessSync(path, constants.X_OK);
    return true;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return false;
    throw error;
  }
}

/** 🧭️ Selects the exact tool to prepare and refuses a missing configured override. */
export function nativeTranspileRequiresBuildV1(repoRoot: string, mode: BuildMode, environment: Readonly<Record<string, string | undefined>> = process.env, platform: string = process.platform, exists = nativeTranspileExecutableExistsV1): boolean {
  const override = environment.SEMIO_NATIVE_TRANSPILE_BIN;
  const path = nativeTranspilePathV1(repoRoot, mode, override, platform);
  if (exists(path)) return false;
  if (override) throw Error(`Configured native transpile does not exist at ${path}`);
  return true;
}

/** 🔍️ Resolves the configured executable or the selected native transpile publication. */
export function nativeTranspileBinaryV1(repoRoot: string, mode: BuildMode = "release", environment: Readonly<Record<string, string | undefined>> = process.env, platform: string = process.platform): string {
  const path = nativeTranspilePathV1(repoRoot, mode, environment.SEMIO_NATIVE_TRANSPILE_BIN, platform);
  if (!nativeTranspileExecutableExistsV1(path)) throw Error(`Native transpile is not staged at ${path}`);
  return path;
}

/** 🧾️ The command line of one component transpile: jco's `transpile <component> -o <dir> --name <name> --map ...` surface, executed natively. */
export function nativeTranspileArgsV1(component: string, outDir: string, componentBase: string, maps: readonly string[]): readonly string[] {
  return ["transpile", component, "-o", outDir, "--name", componentBase, ...maps.flatMap((map) => ["--map", map])];
}

/** 🗺️ The import maps of every plugin component: the pure and host-async framework worlds share one shim module. */
export const PLUGIN_COMPONENT_IMPORT_MAPS_V1: readonly string[] = ["semio:framework/pure=./🟨️.js", "semio:framework/host-async=./🟨️.js"];
