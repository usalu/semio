import rootConfig from"../../../../../../../../tsconfig.json"with{type:"json"};
import{requireRecord,requireExactKeys,requireStringArray,validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schemaData from"./🧬️schema/🔣️.json"with{type:"json"};
const schema=structuredClone(schemaData);
/** 📚️ Describes standard-library DATA separately from syntax lowering and runtime authority. */
export interface TypeScriptLibraryContractV1{readonly version:1;readonly libraries:readonly string[]}
/** 🧬️ Captures the closed authored standard-library set without accepting compiler option overrides. */
export function readTypeScriptLibraryContractV1(value:unknown):TypeScriptLibraryContractV1{const record=requireRecord(value,"TypeScript libraries");requireExactKeys(record,["version","libraries"],"TypeScript libraries");if(!Array.isArray(record.libraries)||record.libraries.length!==schema.properties.libraries.const.length)throw Error("Exact canonical TypeScript library roster required");const captured={version:record.version,libraries:requireStringArray(record.libraries,"TypeScript libraries")};if(validateJsonSchemaSubset(schema,captured).length)throw Error("Canonical TypeScript standard libraries required");return Object.freeze({version:1,libraries:Object.freeze([...captured.libraries])});}
const current=readTypeScriptLibraryContractV1({version:1,libraries:rootConfig.compilerOptions.lib});
/** ⌨️ Supplies the same canonical libraries to explicit-file checks that bypass tsconfig loading. */
export function typeScriptLibraryArgumentsV1():readonly string[]{return Object.freeze(["--lib",current.libraries.join(",")]);}
/** 📖️ Supplies TypeScript's actual declaration filenames without importing a compiler runtime. */
export function typeScriptLibraryFilesV1():readonly string[]{return Object.freeze(current.libraries.map(name=>"lib."+name.toLowerCase()+".d.ts"));}
