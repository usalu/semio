import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync,unlinkSync}from"node:fs";
import{join}from"node:path";
import{selectPackageManagerV1}from"../../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
import{sealedPolicyV1,sealedStampedFilesV1,sealedTreeOfCaptureV1,sealedTreeDriftV1}from"../../../../🏛️sealed/🟦️.ts";
import{readInputPrecisionV1,deriveInputsV1,type InputPrecisionHostV1}from"../../🟦️.ts";
import document from"../../🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
test("selected manager neutral roles preserve sealed locks and precise config inputs against SQLite",async()=>{
 const db=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);db.exec("create table files(manager text,role text,name text)");
 for(const row of fixture.cases)for(const role of["locks","scope"]as const)for(const name of row[role])db.query("insert into files values(?,?,?)").run(row.manager,role,name);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(artifacts,{recursive:true});
 try{for(const row of fixture.cases){
  const environment={SEMIO_PACKAGE_MANAGER_JAVASCRIPT:row.manager},manager=selectPackageManagerV1("javascript",environment),oracle=(role:string)=>(db.query("select name from files where manager=? and role=? order by name").all(row.manager,role)as{name:string}[]).map(x=>x.name),policy={...sealedPolicyV1,stampedFiles:[{ecosystem:"javascript",role:"locks"as const},"package.json"]},root=mkdtempSync(join(artifacts,"neutral-manager-"));
  try{for(const other of fixture.cases)for(const name of other.locks)writeFileSync(join(root,name),"lock:"+name);writeFileSync(join(root,"package.json"),"{}");expect([...sealedStampedFilesV1(policy,environment)].sort()).toEqual([...oracle("locks"),"package.json"].sort());const tree=sealedTreeOfCaptureV1(root,[],{directories:[],observations:[]},policy,environment)!;expect(sealedTreeDriftV1(root,tree,policy,environment).fresh).toBe(true);const changed=row.locks[0]!;writeFileSync(join(root,changed),"changed lock bytes:"+changed);expect(sealedTreeDriftV1(root,tree,policy,environment).changed.includes(changed)).toBe(true);
   const absent=row.locks[1]!;unlinkSync(join(root,absent));const missing=sealedTreeOfCaptureV1(root,[],{directories:[],observations:[]},policy,environment)!;expect(missing.absent.includes(absent)).toBe(true);writeFileSync(join(root,absent),"newly present");expect(sealedTreeDriftV1(root,missing,policy,environment).changed.includes(absent)).toBe(true);
   const other=fixture.cases.find(x=>x.manager!==row.manager)!;expect(sealedTreeDriftV1(root,tree,policy,{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:other.manager}).fresh).toBe(false);
  }finally{rmSync(root,{recursive:true,force:true});}
  const rule={id:"neutral-manager",description:"Selected manager configuration inputs",select:{project:"fixture",targets:["check"]},evidence:"static"as const,replaces:[],reads:[{manager:{ecosystem:"javascript",role:"scope"as const}}]},parsed=readInputPrecisionV1({...document,profiles:{},rules:[rule]}),host:InputPrecisionHostV1={exists:()=>false,readText:()=>undefined,listDirectory:()=>[],kinds:{},importClosure:async()=>[],cargoManifests:()=>[],cargoRoots:()=>[],cargoSources:()=>undefined,cargoToolchain:[],managerFiles:(_ecosystem,role)=>role==="locks"?manager.lockFileNames():manager.configurationNames("scope")};
  const result=await deriveInputsV1(parsed,rule,{project:"fixture",target:"check"},host);expect(result.patterns.filter(x=>!x.startsWith("!"))).toEqual(oracle("scope").map(x=>"{workspaceRoot}/"+x));await expect(deriveInputsV1(parsed,rule,{project:"fixture",target:"check"},{...host,managerFiles:undefined})).rejects.toThrow("manager");await expect(deriveInputsV1(parsed,rule,{project:"fixture",target:"check"},{...host,managerFiles:()=>["../foreign"]})).rejects.toThrow();
 }}finally{db.close();}
});
