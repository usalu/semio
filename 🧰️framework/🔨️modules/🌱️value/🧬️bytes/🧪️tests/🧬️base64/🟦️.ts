import assert from "node:assert/strict";
import { Buffer } from "node:buffer";
import ts from "typescript";
import { join } from "node:path";
import ownedSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import Ajv from "ajv/dist/2020.js";
import { BundleScript } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { encodeBase64, decodeBase64, type IntrinsicBytesControl } from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with { type:"json" };
import schema from "./🧬️schema/🔣️.json" with { type:"json" };

/** 🧬️ The same neutral octets are independently encoded by Node's base64 implementation. */
export class TestScript extends BundleScript {
  async run(): Promise<void> {
    const ajv=new Ajv({strict:true});const check=ajv.compile(schema);const owned=ajv.addSchema(ownedSchema);
    const source=join(import.meta.dir,"../../🟦️.ts");const program=ts.createProgram([source],{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,allowImportingTsExtensions:true,noUncheckedIndexedAccess:true,types:[],skipLibCheck:true,noEmit:true});
    assert.deepEqual(ts.getPreEmitDiagnostics(program).map(item=>ts.flattenDiagnosticMessageText(item.messageText,"\n")),[]);
    assert(check(fixture),JSON.stringify(check.errors));
    const control:IntrinsicBytesControl={maximumOutputBytes:256*1024,progress:()=>true};
    for(const item of fixture.cases){
      const bytes=Uint8Array.from(item.octets);
      assert(owned.getSchema(ownedSchema.$id+"#/$defs/Octets")!(item.octets));assert.equal(encodeBase64(bytes,control),item.base64);
      assert.deepEqual(decodeBase64(item.base64,control),bytes);
      assert.equal(Buffer.from(bytes).toString("base64"),item.base64);
      assert.deepEqual([...Buffer.from(item.base64,"base64")],item.octets);
    }
    for(const item of fixture.invalid)assert.throws(()=>decodeBase64(item,control));
    const bytes=Uint8Array.from({length:131072},(_,index)=>index%256),text=Buffer.from(bytes).toString("base64");
    assert.equal(encodeBase64(bytes,control),text);
    assert.deepEqual(decodeBase64(text,control),bytes);
    assert.throws(()=>encodeBase64(bytes,{...control,maximumOutputBytes:text.length-1}),/limit/);
    assert.throws(()=>decodeBase64(text,{...control,maximumOutputBytes:bytes.length-1}),/limit/);
    assert.throws(()=>encodeBase64(bytes,{...control,progress:()=>false}),/cancelled/);
    assert.throws(()=>decodeBase64(text,{...control,progress:()=>false}),/cancelled/);
    for(const operation of ["encode","decode"]as const){
      const events:{phase:string,completed:number,total:number}[]=[];
      const bounded={...control,progress(event:{phase:string,completed:number,total:number}){assert(owned.getSchema(ownedSchema.$id+"#/$defs/Progress")!(event));events.push(event);return event.completed<8192;}};
      assert.throws(()=>operation==="encode"?encodeBase64(bytes,bounded):decodeBase64(text,bounded),/cancelled/);
      assert.equal(events[0]!.completed,0);assert(events.some(event=>event.completed>=8192));
      for(let index=1;index<events.length;index++)assert(events[index]!.completed-events[index-1]!.completed<=4096);
    }
    assert.equal(encodeBase64(new Uint8Array(),{...control,maximumOutputBytes:0}),"");
    assert.deepEqual(decodeBase64("",{...control,maximumOutputBytes:0}),new Uint8Array());
    assert.throws(()=>decodeBase64("",{...control,maximumOutputBytes:NaN}),/limit/);
    console.log("intrinsic-bytes: neutral="+fixture.cases.length+" independent-base64=12 invalid="+fixture.invalid.length+" allocation-limits=2 cancellation=4 strict-source=1");
  }
}
