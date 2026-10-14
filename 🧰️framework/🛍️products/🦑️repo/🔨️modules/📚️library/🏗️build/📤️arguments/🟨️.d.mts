import type {NativeOwnerCapabilities} from "../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 🧬️ Validates the closed repository JSON contract. */
export function conformsOwnerContractV1(contract:Readonly<Record<string,unknown>>,value:unknown):boolean;
/** 📜️ Parses one discovered literal native command into its schema-owned authority. */
export function declaredNativeOwnerCommandV1(manifest:string,workingDirectory:string,command:string):NativeOwnerCapabilities["command"];
/** 📤️ Encodes the closed literal argument envelope. */
export function encodeOwnerArgumentsV1(arguments_:readonly string[]):string;
/** 📥️ Consumes only the private original argument transport. */
export function consumeOwnerArgumentsV1(environment:Readonly<Record<string,string|undefined>>):{arguments:string[];environment:Record<string,string|undefined>};
