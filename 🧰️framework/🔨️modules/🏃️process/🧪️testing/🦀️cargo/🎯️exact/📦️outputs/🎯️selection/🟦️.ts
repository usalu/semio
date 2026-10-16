import{types}from"node:util";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schema from"./🧬️schema/🔣️.json";
const contract=structuredClone(schema);
/** 🎯️ Selects one completed receipt without accepting a directory or producer claim from DATA. */
export interface ExactCargoLawOutputSelectionV1{readonly version:1;readonly receiptIndex:number}
/** 🔐️ Captures closed primitive selection fields without invoking accessor or proxy code. */
export function readExactCargoLawOutputSelectionV1(value:unknown):ExactCargoLawOutputSelectionV1{if(!value||typeof value!=="object"||types.isProxy(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))throw Error("Plain exact Cargo output selection required");const row:Record<string,unknown>=Object.create(null),keys=Reflect.ownKeys(value);if(keys.length!==2)throw Error("Closed exact Cargo output selection required");for(const key of keys){if(typeof key!=="string")throw Error("String output selection fields required");const field=Object.getOwnPropertyDescriptor(value,key);if(!field||!("value"in field)||!field.enumerable)throw Error("Primitive output selection fields required");row[key]=field.value;}if(validateJsonSchemaSubset(contract,row).length)throw Error("Finite exact Cargo output selection required");return Object.freeze(row)as unknown as ExactCargoLawOutputSelectionV1;}
