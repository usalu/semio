import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import Ajv from 'ajv/dist/2020.js';
import {applyPatch} from 'fast-json-patch';
/** 🧪️ RFC6902 independently calculates ordered original camera preview outputs. */
export function verifyWindowSemanticPreview():void{const corpus=JSON.parse(readFileSync(new URL('../🧫️fixtures/🔣️.json',import.meta.url),'utf8'))as{cases:{name:string,before:number[],leaves:number[][],expected:number[]}[]};const schema=JSON.parse(readFileSync(new URL('../🧬️schema/🔣️.json',import.meta.url),'utf8'));const valid=new Ajv({strict:true}).compile(schema);assert(valid(corpus),JSON.stringify(valid.errors));for(const law of corpus.cases){let current={eye:law.before};for(const leaf of law.leaves)current=applyPatch(current,[{op:'replace',path:'/eye',value:leaf}],true,false).newDocument;assert.deepEqual(current.eye,law.expected,law.name);}console.log('[DEBUG] Original Window semantic preview three independent RFC6902 outputs passed');}
