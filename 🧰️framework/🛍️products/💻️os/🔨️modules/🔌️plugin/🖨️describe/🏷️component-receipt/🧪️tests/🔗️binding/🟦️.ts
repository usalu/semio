import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🔗️ Actual receipt binding compares explicitly selected mode and every content field without promoting prototype into a release catalog. */
test("neutral component receipt binding requires the explicit selected mode and full content tuple",async()=>{
 const api=await import("../../🧬️schema/🟦️.ts"),receiptSchema=(await import("../../🧬️schema/🔣️.json")).default,fixtures=(await import("../🧫️fixtures/🔣️.json")).default,base=fixtures.receipts.find(row=>row.id==="shipped-release")?.value;if(!base)throw Error("Actual canonical receipt fixture required");
 const validate=new Ajv({strict:false}).compile({...receiptSchema,$ref:"#/$defs/ComponentReceiptBindingV1"}),database=new Database(":memory:");expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 try{for(const row of corpus.cases){
  const receipt=api.componentReceiptValueV1({...base,profile:row.receipt}),expected={profile:row.expected,pluginId:receipt.pluginId,packageId:receipt.packageId,wasmSha256:receipt.wasmSha256,semanticDescriptorSha256:receipt.semanticDescriptorSha256};if(row.changed!=="none")Object.assign(expected,{[row.changed]:row.changed.endsWith("Sha256")?"ff".repeat(32):expected[row.changed as keyof typeof expected]+"-different"});expect(validate(expected)).toBe(true);
  const actual=Reflect.apply(api.componentReceiptBindsV1,undefined,[receipt,expected]),oracle=Boolean((database.query("select json_extract(?1,'$.profile')=json_extract(?2,'$.profile') and json_extract(?1,'$.pluginId')=json_extract(?2,'$.pluginId') and json_extract(?1,'$.packageId')=json_extract(?2,'$.packageId') and json_extract(?1,'$.wasmSha256')=json_extract(?2,'$.wasmSha256') and json_extract(?1,'$.semanticDescriptorSha256')=json_extract(?2,'$.semanticDescriptorSha256') admitted").get(JSON.stringify(receipt),JSON.stringify(expected))as{admitted:number}).admitted);expect(oracle).toBe(row.accept);console.log("[DEBUG] Actual selected component receipt binding "+JSON.stringify({row,actual,oracle}));expect(actual).toBe(oracle);
 }for(const mode of["wasm-prototype","wasm-release","dev","host-tool"]){const value={profile:mode,pluginId:base.pluginId,packageId:base.packageId,wasmSha256:base.wasmSha256,semanticDescriptorSha256:base.semanticDescriptorSha256};expect(validate(value)).toBe(false);}}finally{database.close();}
});
