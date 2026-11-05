import{expect,test}from"bun:test";
import Ajv from"ajv";
import lodash from"lodash";
import fixture from"./🔣️.json";
import schema from"./🧬️.json";
import{resolve}from"node:path";
test("closed backing initialization laws agree with independent chunking",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
 const chunks=lodash.chunk(Array.from({length:fixture.pageBytes},(_,index)=>index),fixture.maximumInitializationBytes);
 expect(chunks.map(row=>row.length)).toEqual([2048,2048]);
 const oracle=fixture.copyBudgets.map(budget=>lodash.chunk(chunks[0],Math.max(1,budget-fixture.metadataBytes))[0].length);
 const accepted=fixture.copyBudgets.map((budget,index)=>budget<=fixture.metadataBytes?0:oracle[index]);expect(accepted).toEqual(fixture.expectedInitialized);
 for(const budget of fixture.copyBudgets){const initialized=Math.max(0,Math.min(fixture.maximumInitializationBytes,budget-fixture.metadataBytes));expect(initialized).toBe(fixture.expectedInitialized[fixture.copyBudgets.indexOf(budget)]);if(initialized)expect(initialized+fixture.metadataBytes).toBeLessThanOrEqual(budget);}
});
test("original backing source preserves explicit funded custody",async()=>{
 const file=Bun.file(resolve(import.meta.dir,"../../🖼️wire-turn/📬️typed-operation/📦️backing/🦀️.rs"));expect(await file.exists()).toBe(true);if(!await file.exists())return;const source=await file.text();for(const port of["RetainedWorkOutcome","ManuallyDrop","ControlledRetirement","birth_demands","initialization_demands","take_page","close_step","TypedOperationResultPage::from_original"])expect(source).toContain(port);for(const bypass of["NativeEncodeControl::new","try_serialize_cold","bytes.to_vec()"])expect(source).not.toContain(bypass);
});
