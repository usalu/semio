import {countedCompilerCapacityRequestV1,readCompilerCapacityRequestV1,type CompilerCapacityRequestV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

export type ViteServiceCompilerPolicyV1 = "projected-no-optimizer";

type ViteServiceCompilerConfigurationV1 = Readonly<{
  inlineConfig: {configFile?: string | false};
  configFile?: string;
  configFileDependencies: readonly string[];
  server: {watch?: object | null};
  environments: Readonly<Record<string, {
    optimizeDeps: {noDiscovery?: boolean; include?: readonly string[]};
    dev: {preTransformRequests?: boolean; warmup?: readonly string[]};
  }>>;
}>;

/** 🎟️ Selects the authored request for cold startup and each admitted cold transform. */
export function readViteServiceCompilerCapacityV1(policy: ViteServiceCompilerPolicyV1 | undefined, captured: boolean, hostCores?: number): CompilerCapacityRequestV1 {
  if (policy === undefined) return countedCompilerCapacityRequestV1(hostCores);
  if (policy !== schema.const || !captured) throw Error("Vite compiler policy requires private projected inputs");
  return readCompilerCapacityRequestV1({kind: "exact", parallelism: schema.definitions.parallelism.const});
}

/** 🧊️ Checks resolved Vite settings before creating its service environments. */
export function requireViteServiceCompilerConfigurationV1(config: ViteServiceCompilerConfigurationV1): void {
  if (config.inlineConfig.configFile !== false || config.configFile !== undefined || config.configFileDependencies.length || config.server.watch !== null)
    throw Error("Projected Vite compiler policy requires no config loader or file watcher");
  for (const environment of Object.values(config.environments)) {
    if (environment.optimizeDeps.noDiscovery !== true || environment.optimizeDeps.include?.length || environment.dev.preTransformRequests !== false || environment.dev.warmup?.length)
      throw Error("Projected Vite compiler policy requires disabled optimization and warmup in every environment");
  }
}
