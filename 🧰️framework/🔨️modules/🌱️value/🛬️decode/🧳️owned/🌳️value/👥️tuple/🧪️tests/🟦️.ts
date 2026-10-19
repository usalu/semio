import assert from 'node:assert/strict';
import Ajv from 'ajv';
import schema from '../🧬️schema/🔣️.json';
import fixture from '../🧫️fixtures/🔣️.json';
/** 👥️ Ajv tuple admission and Buffer JSON roundtrip independently verify heterogeneous original wire values. */
export function verifyOriginalTupleOracle():void{
 assert(new Ajv({strict:true}).compile(schema)(fixture));
 const pair=new Ajv({strict:true}).compile({type:'array',items:[{type:'integer',minimum:0,maximum:4294967295},{type:'string'}],additionalItems:false,minItems:2,maxItems:2});
 const triple=new Ajv({strict:true}).compile({type:'array',items:[{type:'integer',minimum:0,maximum:65535},{type:'integer',minimum:0,maximum:255},{type:'integer',minimum:0,maximum:255}],additionalItems:false,minItems:3,maxItems:3});
 for(const law of fixture.cases){assert((law.kind==='pair'?pair:triple)(law.input));assert.deepEqual(law.input,law.output);assert.deepEqual(JSON.parse(Buffer.from(JSON.stringify(law.input),'utf8').toString('utf8')),law.output);}
 for(const refusal of fixture.refusals)assert(!pair(refusal));
 console.log('[DEBUG] Original tuple receiving 2 neutral values and 2 length refusals match Ajv and Buffer; physical custody requires native law');
}