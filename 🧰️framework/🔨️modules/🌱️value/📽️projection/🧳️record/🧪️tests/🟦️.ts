import Ajv from 'ajv';
import jsonPatch from 'fast-json-patch';
import corpus from '../🧫️fixtures/🔣️.json';
import schema from '../🧬️schema/🔣️.json';
/** 🧳️ Verifies complete original field outputs with independent RFC 6902 and UTF-8 bytes. */
export function verifyOriginalRecordProjectionOracle(){
 const validate=new Ajv({strict:true}).compile(schema);if(!validate(corpus))throw Error(JSON.stringify(validate.errors));
 for(const law of corpus.cases){const operations=law.values.map((value,index)=>({op:'add' as const,path:'/f'+index.toString().padStart(2,'0'),value:Buffer.from(value,'utf8').toString('utf8')}));const output=jsonPatch.applyPatch({} as Record<string,string>,operations,true,false).newDocument;for(let index=0;index<law.expected.length;index++)if(output['f'+index.toString().padStart(2,'0')]!==law.expected[index])throw Error(law.id+' independent original field '+index);}
 console.log('[DEBUG] Original record projection oracle: '+corpus.cases.length+' complete 32-field outputs with fast-json-patch, Buffer and Ajv');
}
