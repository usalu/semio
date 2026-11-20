import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import corpus from"./📥️.json" with{type:"json"};
import schema from"./🔣️.json" with{type:"json"};
import{buildCapacityAskV1,withBuildCapacityV1}from"../../🟦️.ts";

test("selective tools setup requests ordinary host capacity while tool queries stay unreserved",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");try{for(const row of corpus.cases){const oracle=db.query("select case when not exists(select 1 from json_each(?1) where value in ('--help','-h')) and (json_extract(?1,'$[0]')='preflight' or (json_extract(?1,'$[0]')='tools' and json_extract(?1,'$[1]') in ('exec','setup'))) then 'command' else null end operation").get(JSON.stringify(row.args)) as{operation:string|null};expect(oracle.operation).toBe(row.operation);expect(buildCapacityAskV1(row.args,{})?.operation??null).toBe(oracle.operation);}}finally{db.close();}
});

test("preflight can report the disk shortage without collecting the build cache",async()=>{
 const db=new Database(":memory:"),directory=mkdtempSync(join(tmpdir(),"preflight-capacity-"));
 try{
  for(const row of corpus.cases){
   const oracle=db.query("select json_extract(?1,'$[0]')='tools' and json_extract(?1,'$[1]') in ('exec','setup') and not exists(select 1 from json_each(?1) where value in ('--help','-h')) as writes").get(JSON.stringify(row.args)) as {writes:number};
   expect(Boolean(oracle.writes)).toBe(row.diskAdmission);
   const diskRead=Error("disk admission requested");
   const run=withBuildCapacityV1(row.args,{LANG:"en",SEMIO_CAS_ROOT:"cas",SEMIO_HOST_CAPACITY:"off"},async()=>true,{
    directory,write:()=>{},strip:args=>args,disk:{partition:()=>{throw diskRead;}}
   });
   if(row.diskAdmission)await expect(run).rejects.toThrow(diskRead.message);
   else expect(await run).toBe(true);
  }
 }finally{db.close();rmSync(directory,{recursive:true,force:true});}
});
