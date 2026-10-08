import {expect,test} from "bun:test";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧮️facts/🔣️.json";
import {checkJpgBaselineFacts} from "../🟦️.ts";
import type {JpgBaselineFacts} from "../🧮️facts/🟦️.ts";

test("native JPEG baseline facts agree with independent JSON Schema admission",async()=>{
 const ajv=new Ajv({strict:false});const declared={...schema,$schema:"http://json-schema.org/draft-07/schema#"};
 for(const item of corpus.cases){expect(ajv.validate(declared,item.facts)).toBe(true);expect(await checkJpgBaselineFacts(item.facts as JpgBaselineFacts)).toEqual(item.codes);}
 console.log(`[DEBUG] JPEG baseline independent Ajv facts=${corpus.cases.length}`);
});
test("native JPEG baseline fact traversal reports progress and cancels inside components",async()=>{
 const facts={...corpus.cases[0]!.facts,components:Array.from({length:corpus.cancelComponents},()=>({id:1,horizontal:1,vertical:1}))} as JpgBaselineFacts;
 const canceled=new AbortController();let reached=false;
 await expect(checkJpgBaselineFacts(facts,{signal:canceled.signal,onProgress:position=>{if(position===corpus.cancelAfter){reached=true;canceled.abort();}}})).rejects.toThrow("canceled");expect(reached).toBe(true);
});
