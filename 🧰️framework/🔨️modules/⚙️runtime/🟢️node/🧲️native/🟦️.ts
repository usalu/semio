import { createRequire } from "node:module";
import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
import type { RuntimeNativeSystemV1 } from "../../📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

let provided: RuntimeNativeSystemV1 | undefined;

/** 🧲️ Gives the native-binding surface of Node: the module `SEMIO_RUNTIME_NATIVE_PROVIDER` designates when set, otherwise the built-in core (`🦀️.rs`, built once per source into the user cache and loaded through `process.dlopen`); no setting is needed. */
export function nativeSystemFromProviderV1(): RuntimeNativeSystemV1 {
  if (provided !== undefined) return provided;
  const designation = process.env[schema.definitions.Protocol.const.addon.environment.provider];
  if (designation === undefined || designation === "") {
    const require = createRequire(import.meta.url);
    const addon = require("./🏗️addon/🟦️.ts") as typeof import("./🏗️addon/🟦️.ts"), system = require("./🧮️system/🟦️.ts") as typeof import("./🧮️system/🟦️.ts");
    provided = system.createNodeNativeSystemV1(addon.loadNodeNativeCoreV1());
    return provided;
  }
  const module = createRequire(import.meta.url)(designation) as { nativeSystemV1?: RuntimeNativeSystemV1 };
  if (module.nativeSystemV1 === undefined) throw new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1("reasonNativeNoExport", { designation }));
  provided = module.nativeSystemV1;
  return provided;
}
