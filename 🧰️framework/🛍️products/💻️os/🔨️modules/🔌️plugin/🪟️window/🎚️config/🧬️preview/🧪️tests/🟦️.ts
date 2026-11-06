import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
import Ajv from 'ajv';
import {applyPatch} from 'fast-json-patch';
/** 🧪️ RFC6902 independently calculates ordered original Window zoom preview outputs. */
export function verifyWindowSemanticPreview():void{const corpus=JSON.parse(readFileSync(new URL('./🧫️fixtures/🔣️.json',import.meta.url),'utf8'))as{cases:{name:string,before:number,leaves:number[],expected:number}[]};const schema=JSON.parse(readFileSync(new URL('../🧬️schema/🔣️.json',import.meta.url),'utf8'));const valid=new Ajv({strict:true}).compile(schema);assert(valid(corpus),JSON.stringify(valid.errors));for(const law of corpus.cases){let current={zoom:law.before};for(const leaf of law.leaves)current=applyPatch(current,[{op:'replace',path:'/zoom',value:leaf}],true,false).newDocument;assert.deepEqual(current.zoom,law.expected,law.name);}}
