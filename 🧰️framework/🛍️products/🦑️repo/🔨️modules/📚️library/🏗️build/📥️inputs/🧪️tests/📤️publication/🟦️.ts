import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv/dist/2020";
import{loadProductFunctionsSyncV1}from"../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import{closeSync,existsSync,fstatSync,linkSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{randomUUID}from"node:crypto";
import{concurrentEditV1,retryOnConcurrentEditSyncV1,publishFileDurablySyncV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

test("closure publication retries a replaced entry and refuses stable hard links and heals corrupt bytes by replacing them",()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned test artifacts directory required");mkdirSync(artifacts,{recursive:true});
 const root=mkdtempSync(join(artifacts,"closure-publication-")),db=new Database(":memory:"),file=new URL("../../🟨️.mjs",import.meta.url);
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 try{for(const row of fixture.cases){
  const path=join(root,randomUUID()+".json"),contract="good",replacement=row.kind.startsWith("replacement");writeFileSync(path,row.kind==="corrupt"?"evil":contract);if(row.kind==="linked")linkSync(path,path+".alias");let old:number|undefined,changed=false;
  const stat=(name:string)=>{if(name===path&&replacement&&!changed){changed=true;old=openSync(path,"r");writeFileSync(path+".next",row.kind==="replacement-corrupt"?"evil":contract);renameSync(path+".next",path);return fstatSync(old);}return lstatSync(name);};
  const ports={join,dirname,lstatSync:stat,readFileSync,publishFileDurablySyncV1,persistedExists:existsSync,rmSync,randomUUID,REVISION_POLICY_V1:{maximumPathUnits:256},VERIFIED_CLOSURES_V1:new Map(),noteBuildGraphStagedLeafV1:()=>{},concurrentEditV1,retryOnConcurrentEditSyncV1};
  const publish=loadProductFunctionsSyncV1(file,["publishInputClosureV1"],ports).publishInputClosureV1;let actual="accepted";
  try{publish(path,contract);}catch{actual="refused";}finally{if(old!==undefined)closeSync(old);}
  const oracle=db.query("SELECT CASE WHEN ? IN ('standing','replacement','corrupt','replacement-corrupt') THEN 'accepted' ELSE 'refused' END verdict").get(row.kind)as{verdict:string};expect(actual,row.name).toBe(row.expected);expect(actual,row.name).toBe(oracle.verdict);if(row.kind==="linked")expect(lstatSync(path).nlink).toBe(2);if(actual==="accepted")expect(readFileSync(path,"utf8")).toBe(contract);if(replacement)expect(changed).toBe(true);
 }}finally{db.close();rmSync(root,{recursive:true,force:true});}
});
