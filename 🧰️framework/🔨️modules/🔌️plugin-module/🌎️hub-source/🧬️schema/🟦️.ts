/** 🗄️ The plugin module store's schema constants, read from the sibling schema document (`🔣️.json`): the store layout, the transfer retry policy, the hub program id pattern and the module source values. The record validation over these constants stays with the plugin registry hub source. */
import storeSchemaModule from "./🔣️.json" with { type: "json" };

/** 🗄️ The store's constants, read from the sibling schema module (`PluginModuleStoreV1`). */
export const PLUGIN_MODULE_STORE_V1: Readonly<{ name: string; storeRoot: string; serveRoute: string; lockPrefix: string; storeLock: string; programSeparator: string }> = Object.freeze({ ...storeSchemaModule["x-semio-constants"] });
/** 🔁️ The transfer retry policy of one plugin module file, read from the sibling schema module (`PluginModuleTransferRetryV1`). */
export const PLUGIN_MODULE_TRANSFER_RETRY_V1: Readonly<{ transientStatuses: readonly number[]; maxAttempts: number; backoffInitialMs: number; backoffMaxMs: number }> = Object.freeze({ ...storeSchemaModule["x-semio-transfer-retry"] });
/** 🪪️ `HubProgramIdV1`'s pattern and `PluginModuleSourceV1`'s values, read from the sibling schema module. */
export const HUB_PROGRAM_ID_PATTERN = new RegExp(storeSchemaModule.$defs.HubProgramIdV1.pattern, "u");
export const HUB_PROGRAM_ID_MAX_CHARS: number = storeSchemaModule.$defs.HubProgramIdV1.maxLength;
export const PLUGIN_MODULE_SOURCES_V1: readonly string[] = Object.freeze([...storeSchemaModule.$defs.PluginModuleSourceV1.enum]);
