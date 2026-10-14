import { buildModePolicy, readBuildMode, type BuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 🧭️ Owns coordinator artifact paths, prototype optimization and admitted worker counts. */
export function coordinatorBuildPolicy(mode: BuildMode, workers: number) {
  const policy = buildModePolicy(mode);
  if (!Number.isInteger(workers) || workers < 1) throw Error("Coordinator compiler workers must be a positive integer");
  return { output: `dist/${readBuildMode(mode)}/repo-coordinator/next`, sourceMap: policy.sourceMap, workers, optimize: policy.optimize, optimization: { minimize: policy.optimize, usedExports: policy.optimize, sideEffects: policy.optimize } };
}
