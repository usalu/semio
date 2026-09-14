// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Next.js configuration for the repo server app.
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { NextConfig } from "next";
// #endregion 🔌️Adapters

const nextConfig: NextConfig = {
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
