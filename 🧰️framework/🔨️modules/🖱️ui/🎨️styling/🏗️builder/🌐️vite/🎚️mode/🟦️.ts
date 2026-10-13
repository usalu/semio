import { buildModePolicy, readBuildMode, type BuildMode } from "../../../../../🏃️process/🏗️build/🎚️mode/🟦️.ts";
import type { OwnedBuildOptions } from "../../../../🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";

/** 🌐️ Applies compiler intent to browser optimization without optimizing prototype bundles. */
export function semioViteBuild(overrides?: OwnedBuildOptions, mode: BuildMode = readBuildMode(process.env.SEMIO_BUILD_MODE)): OwnedBuildOptions & { readonly sourcemap: boolean; readonly minify: false | "esbuild"; readonly cssMinify: boolean } {
  const policy = buildModePolicy(mode);
  return { target: "es2022", ...overrides, sourcemap: policy.sourceMap, minify: policy.optimize ? "esbuild" : false, cssMinify: policy.optimize, reportCompressedSize: false };
}
