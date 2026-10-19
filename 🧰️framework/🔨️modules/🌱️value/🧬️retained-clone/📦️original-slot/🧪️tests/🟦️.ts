import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import Ajv from 'ajv/dist/2020.js';
import {applyPatch,type Operation} from 'fast-json-patch';
/** 🧪️ Independent RFC6902 validates every portable original-slot payload exchange. */
export function verifyOriginalSlot():void{
 const corpus=JSON.parse(readFileSync(new URL('🧫️fixtures/🔣️.json',import.meta.url),'utf8')) as {policy:{maximumItems:number,combinedBodyCapacity:number,separateRelease:boolean,originalIssuer:boolean,foreignKeyRefused:boolean,returnBeforeMutation:boolean},cases:{name:string,original:Record<string,unknown>,replacement:Record<string,unknown>,patch:Operation[],expected:Record<string,unknown>}[]};
 const schema=JSON.parse(readFileSync(new URL('../🧬️schema/🔣️.json',import.meta.url),'utf8'));const valid=new Ajv({strict:true}).compile(schema);assert(valid(corpus),JSON.stringify(valid.errors));
 for(const row of corpus.cases){assert.deepEqual(applyPatch(structuredClone(row.original),row.patch,true,false).newDocument,row.expected,row.name);assert.deepEqual(row.replacement,row.expected,row.name);}
 console.log('[DEBUG] Original-slot independent RFC6902/Ajv outputs '+corpus.cases.length+' passed');
}
