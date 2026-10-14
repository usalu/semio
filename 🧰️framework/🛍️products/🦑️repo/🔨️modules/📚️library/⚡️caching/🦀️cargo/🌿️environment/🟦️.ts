import { resolve } from "node:path";
import {cargoTargetDirectory,cargoBuildDirectory} from "../🗂️directories/🟦️.ts";

/** 🏛️ Carries caller-owned compiler values without importing artifact execution. */
export interface CanonicalExecutionEnvironment { readonly [key: string]: string | undefined }

/** 🔒️ Selects the shared absolute Cargo deliverable and compiler-unit store. */
export function canonicalArchitectureEnvironment(repoRoot: string, env: CanonicalExecutionEnvironment): CanonicalExecutionEnvironment {
  return { ...env, CARGO_TARGET_DIR: resolve(cargoTargetDirectory(repoRoot,env)), CARGO_BUILD_BUILD_DIR:resolve(cargoBuildDirectory(repoRoot,env)) };
}
