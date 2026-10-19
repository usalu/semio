import{createCargoProfileFixtureV1,type CargoProfileFixtureV1}from"../📥️fixture/🟦️.ts";
import cases from"../🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
type Files=CargoProfileFixtureV1["files"];
/** 🧫️ Authors only the already characterized tiny ordinary target-rlib dependency fixture. */
export function createCargoContextFixtureV1():Files{if(validateJsonSchemaSubset(schema,corpus).length)throw Error("Closed context observation corpus required");const row=cases.cases.find(row=>row.name===corpus.case);if(!row)throw Error("Actual characterized context case required");return createCargoProfileFixtureV1(row,corpus.context).files;}
/** ✏️ Applies an exact declared edit once without supplying any expected compiler metadata. */
export function applyCargoContextPhaseV1(files:Files,name:string):Files{const phase=corpus.phases.find(row=>row.name===name),expected=createCargoContextFixtureV1(),paths=new Set(files.map(row=>row.path));if(!phase||paths.size!==files.length||files.length!==expected.length||expected.some(row=>!paths.has(row.path)))throw Error("Exact context observation phase/files required");const values=new Map(files.map(row=>[row.path,row.content]));for(const edit of phase.edits){const content=values.get(edit.path);if(content===undefined||content.split(edit.before).length!==2)throw Error("Context Source replacement must occur exactly once");values.set(edit.path,content.replace(edit.before,edit.after));}const result=files.map(row=>Object.freeze({path:row.path,content:values.get(row.path)!}));if(result.reduce((total,row)=>total+Buffer.byteLength(row.content),0)>corpus.limits.maximumSourceBytes)throw Error("Context Source byte bound exceeded");return Object.freeze(result);}

/** 🧹️ Settles every exact fixture cleanup while preserving the first failure, including undefined. */
export async function settleCargoContextObservationV1<T>(body:()=>Promise<T>,cleanups:readonly (()=>void|Promise<void>)[]):Promise<T>{let outcome:{kind:"value";value:T}|{kind:"failure";error:unknown};try{outcome={kind:"value",value:await body()};}catch(error){outcome={kind:"failure",error};}for(const cleanup of cleanups)try{await cleanup();}catch(error){if(outcome.kind==="value")outcome={kind:"failure",error};}if(outcome.kind==="failure")throw outcome.error;return outcome.value;}
