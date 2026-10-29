import{test,expect}from"bun:test";
import Ajv from"ajv";
import{mkdtempSync,writeFileSync,rmSync,symlinkSync,linkSync,mkdirSync}from"node:fs";
import{join}from"node:path";
import{observeCurrentSourceRevisionV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
test("current Source observations track edits and restoration with independent Bun hashing",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned test artifacts required");const directory=mkdtempSync(join(artifacts,"current-source-")),file=join(directory,"source.mjs");
 try{for(const row of fixture.cases){expect(row.edits.length).toBe(row.changed.length);writeFileSync(file,row.initial);const admitted=new Bun.CryptoHasher("sha256").update(row.initial).digest("hex");for(const[index,text]of row.edits.entries()){writeFileSync(file,text);const current=observeCurrentSourceRevisionV1(file,1024);expect(current.sha256).toBe(new Bun.CryptoHasher("sha256").update(text).digest("hex"));expect(current.byteLength).toBe(Buffer.byteLength(text));expect(current.text).toBeNull();expect(current.sha256!==admitted).toBe(row.changed[index]);}}}finally{rmSync(directory,{recursive:true,force:true});}
});
test("current Source observation retains physical one-link and no-follow limits",()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned test artifacts required");const directory=mkdtempSync(join(artifacts,"current-guard-")),file=join(directory,"s"),alias=join(directory,"a");
 try{writeFileSync(file,"value");linkSync(file,alias);expect(()=>observeCurrentSourceRevisionV1(file,1024)).toThrow();rmSync(alias);expect(()=>observeCurrentSourceRevisionV1(file,1)).toThrow();const target=join(directory,"d");mkdirSync(target);writeFileSync(join(target,"s"),"value");symlinkSync(target,alias,process.platform==="win32"?"junction":"dir");expect(()=>observeCurrentSourceRevisionV1(join(alias,"s"),1024)).toThrow();}finally{rmSync(directory,{recursive:true,force:true});}
});
