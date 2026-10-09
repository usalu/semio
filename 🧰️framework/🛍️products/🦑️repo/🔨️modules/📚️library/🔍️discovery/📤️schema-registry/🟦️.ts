import { requireRecord, validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import type { SchemaRustEntryDump } from "../🟦️.ts";

/** 📤️ Supplies only freshly completed native registry output. */
export interface SchemaRegistryProducerV1 {
 readonly produce: () => Promise<string>;
}

/** 🧬️ Receives a fresh producer result through the canonical production Schema. */
export async function loadProductionSchemaEntriesV1(producer:SchemaRegistryProducerV1,schema:unknown):Promise<SchemaRustEntryDump>{
 const value:unknown=JSON.parse(await producer.produce());
 const registry={...requireRecord(schema,"framework.schema"),$ref:"#/$defs/SchemaExportEntries"};
 const errors=validateJsonSchemaSubset(registry,value);
 if(errors.length)throw Error("Native schema registry output violates its production Schema: "+errors.join("; "));
 return value as SchemaRustEntryDump;
}
