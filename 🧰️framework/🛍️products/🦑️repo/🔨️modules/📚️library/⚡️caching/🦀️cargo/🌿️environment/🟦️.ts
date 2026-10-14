import { resolve } from "node:path";
import { repoCacheDirectory } from "../../🟦️.ts";

/** 🏛️ Carries caller-owned compiler values without importing artifact execution. */
export interface CanonicalExecutionEnvironment { readonly [key: string]: string | undefined }

/** 🔒️ Selects the shared absolute Cargo deliverable and compiler-unit store. */
export function canonicalArchitectureEnvironment(repoRoot: string, env: CanonicalExecutionEnvironment): CanonicalExecutionEnvironment {
  const cargoRoot = resolve(repoRoot, env.CARGO_TARGET_DIR ?? repoCacheDirectory(repoRoot, "cargo", "canonical-architecture"));
  return { ...env, CARGO_TARGET_DIR: cargoRoot, CARGO_BUILD_BUILD_DIR: cargoRoot };
}
