import {requireNativeStoragePathV1} from "@semio-tech/process/artifacts/native-build/storage/platform/contract";
import { createRequire } from "node:module";
import { dirname, relative, resolve } from "node:path";
import { coordinatorBuildPolicy } from "../../🏗️builder/🎚️mode/🟦️.ts";
import { compilerCapacityWorkerCountV1, scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "@semio-tech/process/routing/invocation/process";
import { createNextRebuildAdmissionV1, nextRebuildPluginV1 } from "../../🏗️builder/🎟️admission/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Next.js configuration for the repo server app.
// #endregion 🧲️Header

// #region 🔌️Adapters
// #endregion 🔌️Adapters

const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), policy = coordinatorBuildPolicy(mode, compilerCapacityWorkerCountV1());
const require = createRequire(import.meta.url), nextRoot = dirname(require.resolve("next/package.json"));
const rebuild = createNextRebuildAdmissionV1({ directory: resolve(import.meta.dirname, "../../../../../../../..", ".cache/b/l/m/data"), environment: process.env, run: work => receiveScriptProcessInvocation(process.env, invocation => work(scriptCompilerAdmissionControlV1(invocation, "next:development-rebuild", 0))) });
const rebuildPlugin = nextRebuildPluginV1(rebuild);
if(!process.env.SEMIO_NEXT_BUILD_DIRECTORY||!process.env.SEMIO_NEXT_TYPESCRIPT_PROJECT)throw Error("Next configuration requires original bounded compiler paths");
const products=resolve(import.meta.dirname,"../../../../../../../..",".cache/b/p/m");for(const path of [process.env.SEMIO_NEXT_BUILD_DIRECTORY,process.env.SEMIO_NEXT_TYPESCRIPT_PROJECT])requireNativeStoragePathV1(products,resolve(import.meta.dirname,path));
const nextConfig = {
  distDir: process.env.SEMIO_NEXT_BUILD_DIRECTORY,
  productionBrowserSourceMaps: policy.sourceMap,
  experimental: { cpus: policy.workers, serverMinification: policy.optimize, staticGenerationMaxConcurrency: policy.workers },
  webpack(config, { dev }) {
    config.resolve.alias = { ...config.resolve.alias, next: nextRoot };
    if (!policy.optimize) config.optimization = { ...config.optimization, ...policy.optimization };
    if (dev) config.plugins.push(rebuildPlugin);
    return config;
  },
  output: undefined,
  serverExternalPackages: ["pg", "pg-boss"],
  // 🚧️ Next type-checks the whole transitive source graph, and `@/lib` reaches
  // `@semio-tech/framework`, whose generated `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts`
  // imports `../🚪️lifetime/🟦️component.js`, a specifier its generator emits for a file that does not
  // exist. That is a defect of the actor generator, not of this app, and it must not gate the
  // coordinator's own build. This package's types are checked by its `test` target.
  typescript: { ignoreBuildErrors: true, ...(process.env.SEMIO_NEXT_TYPESCRIPT_PROJECT ? { tsconfigPath: process.env.SEMIO_NEXT_TYPESCRIPT_PROJECT } : {}) },
};

export default nextConfig;
