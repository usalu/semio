import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
export type ComponentStageFacetV1="model"|"browser";
/** 🪜️ Copies the closed component delivery facet into the exact finite stage dependency graph. */
export function componentStagePlanV1(value:unknown,deployable:boolean):readonly Readonly<{name:string;after:readonly string[]}>[]{
 if(validateJsonSchemaSubset(schema,value).length||typeof deployable!=="boolean")throw Error("Closed model/browser stage facet required");
 const browser=(value as{facet:ComponentStageFacetV1}).facet==="browser"&&deployable,rows=browser?[{name:"describe",after:[]},{name:"model",after:["describe"]},{name:"transpile",after:[]},{name:"optimize",after:["transpile"]},{name:"browser",after:["model","optimize"]},{name:"complete",after:["model","browser"]},{name:"discard",after:["complete"]}]:[{name:"describe",after:[]},{name:"model",after:["describe"]},{name:"complete",after:["model"]},{name:"discard",after:["complete"]}];
 return Object.freeze(rows.map(row=>Object.freeze({...row,after:Object.freeze(row.after)})));
}
