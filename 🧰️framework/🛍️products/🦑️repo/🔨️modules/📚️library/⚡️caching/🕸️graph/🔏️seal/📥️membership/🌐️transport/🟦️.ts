import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ Carries exact original source membership outside mutable resolver reflection. */
export type NxGraphSourceProofV1=Readonly<{version:1;sourceMembership:string|null}>;

/** 🔏️ Refuses foreign or substituted source-proof transport before graph work. */
export function readNxGraphSourceProofV1(value:unknown):NxGraphSourceProofV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Original graph source proof transport refused");return value as NxGraphSourceProofV1;}
