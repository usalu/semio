import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdir,mkdtemp,readdir,rm,truncate,writeFile} from "node:fs/promises";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import {createStagedPutsV1,stagedPutStampV1} from "../🟦️.ts";
import limits from "../🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

test("busy PUT staging keeps a finite admitted cohort and releases its private copies on expiry",async()=>{
 const db=new Database(":memory:"),base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());
 await mkdir(base,{recursive:true});
 const root=await mkdtemp(join(base,"s-"));
 try{
  for(const [name,row] of Object.entries(fixture).filter(([name])=>name!=="times") as [string,typeof fixture.count][]){
   const source=join(root,"source"),directory=join(root,name),stages=createStagedPutsV1();
   await mkdir(directory);
   await writeFile(source,"");await truncate(source,row.bytes);
   const stamp=(await stagedPutStampV1(source,row.bytes))!,put={hlc:"h",sha256:"d",bytes:row.bytes,executable:false};
   const expected=db.query("select case when ?1>?4 then 0 when ?1=0 then min(?2,?3) else min(?2,?3,?5/?1) end accepted").get(row.bytes,row.files,limits.maximumFiles,limits.maximumFileBytes,limits.maximumBytes) as {accepted:number};
   let retained=0;
   for(let i=0;i<row.files;i++){
    const path=String(i);
    if(!stages.admits(put))continue;
    const file=join(directory,path);
    await writeFile(file,"");await truncate(file,row.bytes);
    const image=(await stagedPutStampV1(file,row.bytes))!;
    if(await stages.retain(path,put,file,source,stamp,image,0,()=>true))retained++;
   }
   expect(retained).toBe(expected.accepted);
   expect((await readdir(directory)).length).toBe(expected.accepted);
   await stages.sweep(fixture.times[0]!,()=>true);
   expect((await readdir(directory)).length).toBe(expected.accepted);
   if(retained>0)expect(await stages.find("0",put,source,fixture.times[0]!)).toBeDefined();
   await stages.sweep(fixture.times[1]!,()=>true);
   expect(await readdir(directory)).toEqual([]);
  }
 }finally{db.close();await rm(root,{recursive:true,force:true});}
});
