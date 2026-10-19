import type { OwnedBuildOptions } from "@semio-tech/ui-react/build-tooling";

/** 🎚️ The compiler intent a browser build is optimized for; the repository's build-mode owner derives it and the composition root injects it. */
export type ViteBuildModePolicy = Readonly<{ optimize: boolean; sourceMap: boolean }>;

/** 🌐️ Applies compiler intent to browser optimization without optimizing prototype bundles. */
export function semioViteBuild(overrides: OwnedBuildOptions | undefined, policy: ViteBuildModePolicy): OwnedBuildOptions & { readonly sourcemap: boolean; readonly minify: false | "esbuild"; readonly cssMinify: boolean; readonly rollupOptions: { readonly treeshake: boolean; readonly [key: string]: unknown } } {
  return { target: "es2022", ...overrides, rollupOptions: { ...overrides?.rollupOptions, treeshake: policy.optimize }, sourcemap: policy.sourceMap, minify: policy.optimize ? "esbuild" : false, cssMinify: policy.optimize, reportCompressedSize: false };
}
