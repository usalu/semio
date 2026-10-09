import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";

test("original finished output contract agrees with strict Ajv independent cardinality and schema rules",()=>{
 const ajv=new Ajv({strict:true,allErrors:true});
 for(const row of fixture.cases){const value=(row.output as Record<string,unknown>)[row.name];let accepted=true;if(row.name!=="*"&&value!==null){if(["*","+","0","2","3"].includes(row.cardinality)){if(value===undefined){accepted=!['+','2','3'].includes(row.cardinality);}else{const list=value as Record<string,unknown>;accepted=ajv.compile({type:"object",required:["$schema"],properties:{$schema:{const:"list"}},additionalProperties:true})(list);if(accepted){const items=Object.entries(list).filter(([key])=>/^\+?[0-9]+$/.test(key)).map(([,item])=>item);const minimum=row.cardinality==="*"?0:row.cardinality==="+"?1:Number(row.cardinality),maximum=["*","+"].includes(row.cardinality)?undefined:Number(row.cardinality);accepted=ajv.compile({type:"array",minItems:minimum,...(maximum===undefined?{}:{maxItems:maximum}),items:{anyOf:[{type:"null"},{type:"object"}]}})(items);const first=items.find(item=>item!==null) as Record<string,unknown>|undefined;if(accepted&&first){const itemSchema=first.$schema??"";accepted=items.every(item=>item===null||ajv.compile({type:"object",...(itemSchema===""?{}:{required:["$schema"]}),properties:{$schema:{const:itemSchema}},additionalProperties:true})(item));}}}}else{accepted=ajv.compile({type:"array",minItems:row.cardinality==="!"?1:0,maxItems:1})(value===undefined?[]:[value]);}}expect(accepted).toBe(row.accepted);}
 console.log("[DEBUG] Original output contract perValueStrictAjv=true wholeCorpus=false independentCases=14 originalNative=unqualified");
});
