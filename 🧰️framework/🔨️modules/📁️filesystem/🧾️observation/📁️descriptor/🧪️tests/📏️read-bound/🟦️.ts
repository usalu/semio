/** 📏️ Law: a relative file read is bounded by 64 KiB unless the caller declares a larger bound up to the schema maximum; a refusal names the file and the size in English and German. */
import {expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {withArtifactRelativeDirectoryV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json"with{type:"json"};
import cases from "./🔣️.json"with{type:"json"};

test("the default bound stays and a declared bound admits larger reads",async()=>{
 const policy=schema.definitions.RelativeProtocol.const,root=mkdtempSync(join(tmpdir(),"semio-read-bound-"));
 try{
  expect(policy.maximumReadBytes).toBe(65536);
  for(const row of cases){
   writeFileSync(join(root,"f"),Buffer.alloc(row.fileBytes,7));
   const buffer=Buffer.alloc(row.read),attempt=()=>withArtifactRelativeDirectoryV1(root,()=>{},directory=>directory.withFile("f",async file=>file.read(buffer,0,row.read,0),row.options??undefined));
   if(row.accepted){
    expect(await attempt(),row.id).toBe(row.read);
    expect(buffer.equals(readFileSync(join(root,"f")).subarray(0,row.read)),row.id).toBe(true);
    continue;
   }
   const refusal=await attempt().then(()=>null,error=>error as Error);
   expect(refusal,row.id).not.toBeNull();
   expect(refusal!.message,row.id).toContain(join(root,"f"));
   expect(refusal!.message,row.id).toContain(" / ");
   expect(refusal!.message,row.id).toMatch(/Begrenztes Lesen|Deklarierte Lesegrenze/u);
   if((row as {at?:string}).at!=="declare")expect(refusal!.message,row.id).toContain(`${row.read} bytes`);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
