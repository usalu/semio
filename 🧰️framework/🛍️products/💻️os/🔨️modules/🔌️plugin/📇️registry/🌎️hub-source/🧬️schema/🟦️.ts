/** 🗄️ Persisted browser installation records over the shared plugin module wire contract. */
import storeSchemaModule from "./🔣️.json";
import { validateTrustedPluginModuleIndexV1, TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA, TrustedPluginModuleRefusalV1, type TrustedPluginModuleIndexEntryV1 } from "../../📦️deployment/🧬️schema/🟦️.ts";

const refuse = (reason: string): never => { throw new TrustedPluginModuleRefusalV1(`plugin store ${reason}`); };
function exactRecord(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return refuse("record requires an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) return refuse("record fields differ from its schema");
  return row;
}
const validDigest = (value: unknown): value is string => typeof value === "string" && /^[0-9a-f]{64}$/u.test(value);

/** 🗄️ The store's constants, read from the sibling schema module (`PluginModuleStoreV1`). */
export const PLUGIN_MODULE_STORE_V1: Readonly<{ name: string; storeRoot: string; serveRoute: string; lockPrefix: string; storeLock: string; programSeparator: string }> = Object.freeze({ ...storeSchemaModule["x-semio-constants"] });
/** 🔁️ The transfer retry policy of one plugin module file, read from the sibling schema module (`PluginModuleTransferRetryV1`). */
export const PLUGIN_MODULE_TRANSFER_RETRY_V1: Readonly<{ transientStatuses: readonly number[]; maxAttempts: number; backoffInitialMs: number; backoffMaxMs: number }> = Object.freeze({ ...storeSchemaModule["x-semio-transfer-retry"] });
/** 🪪️ `HubProgramIdV1`'s pattern and `PluginModuleSourceV1`'s values, read from the sibling schema module. */
export const HUB_PROGRAM_ID_PATTERN = new RegExp(storeSchemaModule.$defs.HubProgramIdV1.pattern, "u");
export const HUB_PROGRAM_ID_MAX_CHARS: number = storeSchemaModule.$defs.HubProgramIdV1.maxLength;
export const PLUGIN_MODULE_SOURCES_V1: readonly string[] = Object.freeze([...storeSchemaModule.$defs.PluginModuleSourceV1.enum]);
const STORE_RECORD_SCHEMA: string = storeSchemaModule.$defs.PluginModuleStoreRecordV1.properties.schema.const;
const STORE_RECORD_KEYS: readonly string[] = storeSchemaModule.$defs.PluginModuleStoreRecordV1.required;
const STORE_RECORD_TIME_MAX: number = storeSchemaModule.$defs.PluginModuleStoreRecordV1.properties.installedAtMs.maximum;

/** 🗄️ One installed plugin module of one catalog generation (`PluginModuleStoreRecordV1`). */
export type PluginModuleStoreRecordV1 = Readonly<{ schema: string; generationId: string; entry: TrustedPluginModuleIndexEntryV1; installedAtMs: number }>;

/** 🗄️ Checks one store record: its generation, its index entry (every index-entry rule) and its install time. */
export function validatePluginModuleStoreRecordV1(value: unknown): PluginModuleStoreRecordV1 {
  const record = exactRecord(value, STORE_RECORD_KEYS);
  if (record.schema !== STORE_RECORD_SCHEMA || !validDigest(record.generationId) || typeof record.installedAtMs !== "number" || !Number.isSafeInteger(record.installedAtMs) || record.installedAtMs < 0 || record.installedAtMs > STORE_RECORD_TIME_MAX)
    return refuse("store record is invalid");
  const index = validateTrustedPluginModuleIndexV1({ schema: TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA, generationId: record.generationId, modules: [record.entry] });
  return Object.freeze({ schema: STORE_RECORD_SCHEMA, generationId: record.generationId, entry: index.modules[0]!, installedAtMs: record.installedAtMs });
}

/** 🗄️ A new store record for one verified index entry of one generation. */
export function pluginModuleStoreRecordV1(generationId: string, entry: TrustedPluginModuleIndexEntryV1, installedAtMs: number): PluginModuleStoreRecordV1 {
  return validatePluginModuleStoreRecordV1({ schema: STORE_RECORD_SCHEMA, generationId, entry, installedAtMs });
}
