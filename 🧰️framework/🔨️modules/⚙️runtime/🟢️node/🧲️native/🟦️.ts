import { createRequire } from "node:module";
import { defaultNodeNativePortsV1, ensureNodeNativeAddonV1, loadNodeNativeCoreV1 } from "./🏗️addon/🟦️.ts";
import { createNodeNativeSystemV1 } from "./🧮️system/🟦️.ts";
import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
import type { RuntimeNativeSystemV1 } from "../../📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

let provided: RuntimeNativeSystemV1 | undefined;
let childEnvironment:Readonly<Record<string,string>>|undefined;

/** 🧲️ Gives the native-binding surface of Node: the module `SEMIO_RUNTIME_NATIVE_PROVIDER` designates when set, otherwise the built-in core (`🦀️.rs`, built once per source into the user cache and loaded through `process.dlopen`); no setting is needed. */
export function nativeSystemFromProviderV1(environment: Readonly<Record<string,string|undefined>> = process.env): RuntimeNativeSystemV1 {
  if (provided !== undefined) return provided;
  const designation = environment[schema.definitions.Protocol.const.addon.environment.provider];
  if (designation === undefined || designation === "") {
    const ports=defaultNodeNativePortsV1(environment),names=schema.definitions.Protocol.const.addon.environment,file=environment[names.addon]||ensureNodeNativeAddonV1(ports),absolute=createRequire(import.meta.url).resolve(file);
    provided = createNodeNativeSystemV1(loadNodeNativeCoreV1({...ports,host:{...ports.host,env:{...environment,[names.addon]:absolute}}}));
    childEnvironment=Object.freeze({[names.addon]:absolute});
    return provided;
  }
  const resolved=createRequire(import.meta.url).resolve(designation),module = createRequire(import.meta.url)(resolved) as { nativeSystemV1?: RuntimeNativeSystemV1 };
  if (module.nativeSystemV1 === undefined) throw new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1("reasonNativeNoExport", { designation }));
  provided = module.nativeSystemV1;
  childEnvironment=Object.freeze({[schema.definitions.Protocol.const.addon.environment.provider]:resolved});
  return provided;
}

/** 🪢️ Carries only the actual initialized Native implementation to its owned same-runtime child. */
export function nodeNativeChildEnvironmentV1():Readonly<Record<string,string>>{if(!provided||!childEnvironment)throw Error("Initialized Node Native implementation required");return childEnvironment;}
