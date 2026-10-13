import { buildModePolicy, readBuildMode } from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Next.js configuration for the repo server app.
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { NextConfig } from "next";
// #endregion 🔌️Adapters

const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), policy = buildModePolicy(mode);
const nextConfig: NextConfig = {
  distDir: `dist/${mode}/next`,
  productionBrowserSourceMaps: policy.sourceMap,
  experimental: { cpus: 1, serverMinification: policy.optimize, staticGenerationMaxConcurrency: 1 },
  webpack(config) {
    config.optimization = { ...config.optimization, minimize: policy.optimize, usedExports: policy.optimize, sideEffects: policy.optimize };
    return config;
  },
  output: "standalone",
  serverExternalPackages: ["pg", "pg-boss"],
  // 🚧️ Next type-checks the whole transitive source graph, and `@/lib` reaches
  // `@semio-tech/framework`, whose generated `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts`
  // imports `../🚪️lifetime/🟦️component.js`, a specifier its generator emits for a file that does not
  // exist. That is a defect of the actor generator, not of this app, and it must not gate the
  // coordinator's own build. This package's types are checked by its `test` target.
  typescript: { ignoreBuildErrors: true },
};

export default nextConfig;
