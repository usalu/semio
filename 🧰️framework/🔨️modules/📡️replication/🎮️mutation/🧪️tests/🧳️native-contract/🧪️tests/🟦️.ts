import assert from 'node:assert/strict';
import Ajv from 'ajv';
import {applyPatch,type Operation} from 'fast-json-patch';
import corpus from '../🧫️fixtures/🔣️.json';
import schema from '../🧬️schema/🔣️.json';
/** 🧳️ Independent RFC 6902 keeps unchanged and authored zero distinct for native delta metadata. */
export function verifyNativeDiffContractOracle():void{
 const validate=new Ajv({strict:true}).compile(schema);assert(validate(corpus),JSON.stringify(validate.errors));
 for(const item of corpus.cases){const patch:Operation[]=item.value===null?[]:[{op:'add',path:'/value',value:item.value}];const wire=applyPatch({},patch,true,false).newDocument;assert.deepEqual({empty:patch.length===0,wire},item.expected);}
 console.log('[DEBUG] Native diff vocabulary: '+corpus.cases.length+' complete RFC 6902/Ajv projections');
}
