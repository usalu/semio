import {test,expect} from "bun:test";
import {resolve} from "node:path";
import {tmpdir} from "node:os";
import {descriptorEmitterPublicationFilesV1,descriptorEmitterRequestV1,requireDescriptorEmitterCapabilitiesV1} from "../../📋️contract/🟦️.ts";
import emitterSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import capabilityCases from "./🔣️capabilities.json" with {type:"json"};
import publicationCases from "./🔣️publication.json" with {type:"json"};
import readerSchema from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import Ajv from "ajv";

test("descriptor capability admission agrees with the independent schema validator",()=>{
 const validate=new Ajv().compile(emitterSchema.definitions.Capabilities);
 for(const {output,accepted} of capabilityCases){
  let independent=false;
  try{independent=validate(JSON.parse(output));}catch{}
  expect(independent,output).toBe(accepted);
  const admit=()=>requireDescriptorEmitterCapabilitiesV1(output);
  if(accepted)expect(admit,output).not.toThrow();
  else expect(admit,output).toThrow();
 }
});

/** 📏️ The managed descriptor emitter request must satisfy the closed artifact reader schema, or every component describe stage refuses with "Actual closed artifact reader custody required". Target `🛠️emitter/🧪️tests/📏️request/🟦️.ts`. */
test("descriptor emitter request is admitted by the closed artifact reader schema for every mode and platform",()=>{
 const ajv=new Ajv({strict:false}).compile(readerSchema),root=resolve(tmpdir(),"workspace",".cache","b","p","m");
 for(const mode of ["prototype","release"])for(const platform of ["darwin","linux","win32"]){const request=descriptorEmitterRequestV1(root,{version:1,mode,platform});expect(validateJsonSchemaSubset(readerSchema as never,request as never),`${mode}/${platform}`).toEqual([]);expect(ajv(request),`${mode}/${platform}`).toBe(true);}
});

test("the emitter reads its compilation index within the bound the retention writer enforces",async()=>{
 const {readFileSync}=await import("node:fs"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),sources=(await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/📥️sources/🧬️schema/🔣️json/🟦️.ts")).default;
 expect(source).toContain('"index.json"),limits.maximumRetainedIndexBytes.const');
 expect(source).not.toContain('"index.json"),limits.maximumManifestBytes.const');
 expect(sources.properties.maximumRetainedIndexBytes.const).toBeGreaterThan(sources.properties.maximumManifestBytes.const);
});

test("sealed descriptor work uses the admitted compiler home after the live Cargo profile changes",async()=>{
 const [{readFileSync},{default:ts},{Database},{classifyLeaseResourceV1,leaseOrderPolicyV1}]=await Promise.all([import("node:fs"),import("typescript"),import("bun:sqlite"),import("../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts")]);
 const path=new URL("../../🟦️.ts",import.meta.url),tree=ts.createSourceFile(path.pathname,readFileSync(path,"utf8"),ts.ScriptTarget.ES2022,true);
 const functionSource=(name:string)=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!.getText(tree).replace(/^export /u,"");
 const program=ts.transpileModule(functionSource("withCompilerStoreDescriptorEmitterV1")+"\n"+functionSource("withDescriptorEmitterTaskV1"),{compilerOptions:{target:ts.ScriptTarget.ES2022},reportDiagnostics:true});
 expect(program.diagnostics).toEqual([]);
 const workspace="/repo",invocation=Object.freeze({}),compiler=Object.freeze({storage:{},environment:{CARGO_HOME:"/repo/cargo"}}),start=Object.freeze({}),held:string[]=[],events:string[]=[],emitters=new WeakMap<object,object>();
 const under=async<T>(name:string,body:()=>Promise<T>)=>{held.push(name);events.push("open:"+name);try{return await body();}finally{expect(held.pop()).toBe(name);events.push("close:"+name);}};
 const ports={
  join:(...parts:string[])=>parts.join("/"),
  DESCRIPTOR_EMITTER_MANIFEST_V1:"descriptor/Cargo.toml",
  requireRepositoryCompilerStoreV1:(actual:unknown,root:string,source:unknown,control:unknown)=>{expect([actual,root,source,control]).toEqual([compiler,workspace,invocation,start]);events.push("borrow:compiler");return "release";},
  readDescriptorEmitterIntentV1:(value:unknown)=>value,
  descriptorEmitterCargoArgumentsV1:()=>["--release"],
  repositoryCargoNetworkV1:()=>({offline:true}),
  descriptorEmitterRequestV1:()=>({path:"/products/descriptor"}),
  withRepositoryCargoInputReaderV1:()=>{throw Error("Cargo profile policy Source differs from running canonical policy");},
  withRepositoryCacheStoreV1:(_workspace:unknown,_partition:unknown,_invocation:unknown,control:unknown,body:(store:unknown,phase:unknown)=>Promise<unknown>)=>{expect(control).toBe(start);return body({root:"/products",control:{},storage:{}},{});},
  withRepositoryArtifactV1:(_options:unknown,body:(reader:unknown)=>Promise<unknown>)=>under("artifact",()=>body({})),
  withRepositoryArtifactTaskV1:(_reader:unknown,_invocation:unknown,body:(view:unknown)=>Promise<unknown>)=>body({control:{}}),
  emitters,
 };
 const api=new Function(...Object.keys(ports),program.outputText+"\nreturn {withCompilerStoreDescriptorEmitterV1,withDescriptorEmitterTaskV1};")(...Object.values(ports));
 let escaped:unknown;
 await api.withCompilerStoreDescriptorEmitterV1(workspace,invocation,compiler,start,async(emitter:unknown)=>{
  escaped=emitter;
  expect(emitters.get(emitter as object)).toMatchObject({cargoHome:compiler.environment.CARGO_HOME});
  expect(held).toEqual(["artifact"]);
  await under("writer",async()=>{expect(held).toEqual(["artifact","writer"]);});
 });
 expect(()=>api.withDescriptorEmitterTaskV1(escaped,invocation,()=>Promise.resolve())).toThrow("Live actual descriptor emitter task required");
 expect(held).toEqual([]);
 const db=new Database(":memory:");
 try{
  expect(events).toEqual(["borrow:compiler","open:artifact","open:writer","close:writer","close:artifact"]);
  const resources:Readonly<Record<string,string>>={compiler:"rustup:bootstrap",artifact:"artifact:test",writer:"repo:compiler-writer:test"};
  const policy=leaseOrderPolicyV1({});
  expect(Object.values(resources).map(resource=>classifyLeaseResourceV1(policy,resource).rank)).toEqual([1,6,7]);
  db.exec("create table event(position integer, direction text, rank integer)");
  const insert=db.query("insert into event values(?,?,?)");
  events.forEach((event,position)=>{const [direction,name]=event.split(":");insert.run(position,direction==="close"?"close":"open",classifyLeaseResourceV1(policy,resources[name!]!).rank);});
  const violations=db.query("select position from (select *,lag(rank) over(partition by direction order by position) as prior from event) where (direction='open' and rank<prior) or (direction='close' and rank>prior)").all();
  expect(violations).toEqual([]);
 }finally{db.close();}
});

test("the component driver starts the descriptor reader in its compiler-store task before taking the stage writer",async()=>{
 const [{readFileSync},{default:ts},{captureComponentProductLeaseTaskV1},{withLeaseTaskV1,currentLeaseLineageV1}]=await Promise.all([import("node:fs"),import("typescript"),import("../../../../📦️model/🔒️lease-task/🟦️.ts"),import("../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts")]);
 const path=new URL("../../../../🏗️build/🧩️driver-stages/🟦️.ts",import.meta.url),tree=ts.createSourceFile(path.pathname,readFileSync(path,"utf8"),ts.ScriptTarget.ES2022,true);
 const declarations=new Map<string,string>();
 const visit=(node:import("typescript").Node)=>{if(ts.isVariableDeclaration(node)&&node.initializer){const name=node.name.getText(tree);if(name==="withProductLeaseTask"||name==="withDescribe")declarations.set(name,node.initializer.getText(tree));}node.forEachChild(visit);};
 visit(tree);
 expect(declarations.size).toBe(2);
 const stage=Object.freeze({}),reader=Object.freeze({}),writer=Object.freeze({}),emitter=Object.freeze({}),events:string[]=[];
 await withLeaseTaskV1(async()=>{
  const compilerTask=currentLeaseLineageV1()![0];
  const ports={
   captureComponentProductLeaseTaskV1,repoRoot:"/repo",options:{invocation:{}},store:{storage:{}},leaseDirectory:"/leases",
   withCompilerStoreDescriptorEmitterV1:async(_root:unknown,_invocation:unknown,_store:unknown,control:unknown,body:(emitter:unknown,control:unknown)=>Promise<void>)=>{expect(control).toBe(stage);events.push("reader");await body(emitter,reader);},
   withCompilerWorkspaceStageWriteV1:async(_storage:unknown,_directory:unknown,control:unknown,body:(control:unknown)=>Promise<void>)=>{expect(control).toBe(reader);events.push("writer");await body(writer);},
  };
  const source=[...declarations].map(([name,initializer])=>"const "+name+"="+initializer+";").join("\n")+"\nreturn withDescribe;";
  const describe=new Function(...Object.keys(ports),source)(...Object.values(ports));
  await withLeaseTaskV1(async()=>{
   const compilationTask=currentLeaseLineageV1()![0];
   await describe({control:stage},async(actualEmitter:unknown,actualWriter:unknown)=>{
    const lineage=currentLeaseLineageV1()!;
    expect(lineage).toContain(compilerTask);
    expect(lineage).not.toContain(compilationTask);
    expect([actualEmitter,actualWriter]).toEqual([emitter,writer]);
   });
  });
 });
 expect(events).toEqual(["reader","writer"]);
 const source=readFileSync(path,"utf8");
 expect(source).toMatch(/stage\.name==="describe"\|\|stage\.capacity==="cleanup"\?stage/u);
 const preflight=source.indexOf("if(!buildsEmitter)await checkDescriptorEmitterCapabilitiesV1");
 expect(preflight).toBeGreaterThan(0);
 expect(preflight).toBeLessThan(source.indexOf("return withRepositoryComponentLaneV1"));
});


test("the prerequisite rebuilds only an absent selected publication before the reader and the driver",async()=>{
 const [{readFileSync},{resolve,join},{default:ts},{Database}]=await Promise.all([import("node:fs"),import("node:path"),import("typescript"),import("bun:sqlite")]);
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("emitter.ts",source,ts.ScriptTarget.ES2022,true);
 const absent=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="descriptorEmitterAbsentV1")!;
 const javascript=ts.transpileModule(absent.getText(tree).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022},reportDiagnostics:true});
 expect(javascript.diagnostics).toEqual([]);
 const root=resolve("fixture-workspace"),products=join(root,"products"),control=Object.freeze({});
 const db=new Database(":memory:");
 try{
  const selected=db.query("SELECT json_extract(?1,'$.code')='ENOENT' AS needed");
  for(const row of publicationCases){
   const status=selected.get(JSON.stringify(row)) as {needed:number|null};
   expect(Boolean(status.needed)).toBe(row.build);
   const intent={version:1,mode:row.mode,platform:row.platform},path=descriptorEmitterRequestV1(products,intent).path;
   const files=descriptorEmitterPublicationFilesV1(intent);
   if(row.file!==null)expect(files).toContain(row.file);
   const ports={join,descriptorEmitterPublicationFilesV1,checkResourceLeaseControl:()=>undefined,
    lstat:async(actual:string)=>{if(row.file!==null&&actual===join(path,row.file))throw Object.assign(Error("fixture refusal"),{code:row.code});},
   };
   const check=new Function(...Object.keys(ports),javascript.outputText+"\nreturn descriptorEmitterAbsentV1;")(...Object.values(ports));
   if(row.code==="EACCES")await expect(check(path,intent,control)).rejects.toThrow("fixture refusal");
   else expect(await check(path,intent,control)).toBe(row.build);
  }
 }finally{db.close();}
 const driver=readFileSync(new URL("../../../../🏗️build/🧩️driver-stages/🟦️.ts",import.meta.url),"utf8"),run=driver.split("async function runRepositoryComponentDriverV1")[1]!;
 const prerequisite=run.indexOf("await ensureFreshDescriptorEmitterV1(repoRoot,options.invocation,options.control,options.log)");
 expect(prerequisite).toBeGreaterThan(0);
 expect(prerequisite).toBeLessThan(run.indexOf("withRepositoryCacheStoreV1(repoRoot,\"dependencies\""));
 expect(run.slice(prerequisite-80,prerequisite)).toContain("!options.emitterHeld&&!planBuildsEmitterV1(repoRoot,plan)");
 expect(driver).not.toContain("ensureDescriptorEmitterV1");
 expect(source).not.toContain("requireFreshDescriptorEmitterV1");
});
