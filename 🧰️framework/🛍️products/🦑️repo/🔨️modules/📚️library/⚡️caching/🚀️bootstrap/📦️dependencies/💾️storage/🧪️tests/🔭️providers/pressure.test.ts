import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{createRequire}from"node:module";
import{mkdirSync,mkdtempSync,rmSync,symlinkSync,unlinkSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import*as issued from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import*as leases from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{readJavascriptDependencySourceOwnerExecutionV1}from"../../🟦️.ts";
import{javascriptInstallSchemaV1}from"../../🧱️schemas/🟦️.ts";
import corpus from"./pressure.json";
import schema from"./pressure.schema.json";

if(corpus.birth.capabilities.artifacts!==true)throw Error("Authored provider pressure artifacts required");
declareBuildTestEnvelopeV1({...corpus.birth,capabilities:{artifacts:corpus.birth.capabilities.artifacts}});

interface VersionOracleV1{compatible(version:string,range:string):boolean}
const versionOracleV1:VersionOracleV1={compatible:(version,range)=>Boolean((createRequire(import.meta.url)("semver")as{satisfies:(version:string,range:string)=>boolean}).satisfies(version,range))};
const link=(target:string,path:string)=>symlinkSync(target,path,process.platform==="win32"?"junction":"dir");
const put=(path:string,value:unknown)=>{mkdirSync(path,{recursive:true});writeFileSync(join(path,"package.json"),JSON.stringify(value));};
const artifacts=()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Original pressure test artifact root required");return root;};
const summary=(result:Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerExecutionV1>>)=>result.relations.flatMap(relation=>relation.edges.map(edge=>({source:relation.source,memberScope:relation.memberScope,ownerScope:relation.ownerScope,name:edge.name,range:edge.range,version:edge.provider?.version??null}))).sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));

test("7352 declared provider edges fit unchanged original work bounds with repeated shared manifests",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);expect(javascriptInstallSchemaV1.$defs.SourceOwnerExecutionProtocol.const.maximumWork).toBe(4194304);
 await withBuildTestInvocationV1(async original=>{
  const control=issued.createIssuedResourceControlV1(original,"dependencies:provider-pressure",0),root=mkdtempSync(join(artifacts(),"pp-")),workspace=join(root,"s"),cache=join(root,"n"),generation="a".repeat(64),stage=join(cache,"j",generation.slice(0,20)),row=corpus.pressure,db=new Database(":memory:");
  try{
   db.exec("create table members(path text primary key, edge_count integer);create table providers(name text primary key, version text, requested text)");
   put(workspace,{name:"pressure-scope",version:"1.0.0"});put(stage,{name:"pressure-scope",version:"1.0.0"});
   const filler=Object.fromEntries(Array.from({length:row.providerManifestEdges},(_,index)=>["metadata-dependency-"+index,"*"]));
   for(let index=0;index<row.dependencies;index++){const name="dep"+index;put(join(stage,"node_modules",name),{name,version:row.version,dependencies:filler});expect(versionOracleV1.compatible(row.version,row.range)).toBe(true);db.query("insert into providers values(?,?,?)").run(name,row.version,row.range);}
   const members:string[]=[];
   for(let index=0;index<row.members;index++){const path="p"+String(index).padStart(2,"0"),count=index===row.members-1?row.lastMemberDependencies:row.dependencies,dependencies=Object.fromEntries(Array.from({length:count},(_,edge)=>["dep"+edge,row.range]));members.push(path);put(join(workspace,path),{name:"member"+index,version:"1.0.0",dependencies});put(join(stage,"w",String(index)),{name:"member"+index,version:"1.0.0",dependencies});mkdirSync(join(stage,"w",String(index),"node_modules"));for(let edge=0;edge<count;edge++)link(join(stage,"node_modules","dep"+edge),join(stage,"w",String(index),"node_modules","dep"+edge));link(join(stage,"w",String(index),"node_modules"),join(workspace,path,"node_modules"));db.query("insert into members values(?,?)").run(path,count);}
   link(join(stage,"node_modules"),join(workspace,"node_modules"));
   const context=Object.freeze({root:cache,invocation:original,control,check:()=>leases.checkResourceLeaseControl(control),recheck:async()=>{leases.checkResourceLeaseControl(control);}}),input={version:1,requested:["."],owners:{version:1,scopes:[{manifest:"package.json",packages:members}]},generations:[{scope:".",generation}]};
   const result=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context as never,control,input),actual=summary(result),expected=(db.query("select m.path source,'.' memberScope,'.' ownerScope,p.name,p.requested range,p.version from members m join providers p on cast(substr(p.name,4) as integer)<m.edge_count").all()as typeof actual).sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)));
   expect(actual.length).toBe(row.expectedEdges);expect(actual).toEqual(expected);expect(result.authority).toBe(false);
  }finally{db.close();rmSync(root,{recursive:true,force:true});}
 });
},120000);

test("provider pressure memoization preserves namespaces ranges missing paths and both replay identities",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 await withBuildTestInvocationV1(async original=>{
  const control=issued.createIssuedResourceControlV1(original,"dependencies:provider-isolation",0);
  for(const row of corpus.isolation){const root=mkdtempSync(join(artifacts(),"pi-")),workspace=join(root,"s"),cache=join(root,"n"),ga=join(cache,"j","a".repeat(20)),gb=join(cache,"j","b".repeat(20)),first=join(ga,"node_modules/dep"),second=join(gb,"node_modules/dep");let rechecks=0,fired=false;
   try{
    for(const[folder,name,version,range]of[[workspace,"scope-a","1.0.0",null],[join(workspace,"b"),"scope-b","1.0.0",null],[join(workspace,"p"),"member-a","1.0.0","^1.0.0"],[join(workspace,"b/q"),"member-b","1.0.0",row.mutation==="range"?"^1.0.0":"^2.0.0"]]as const)put(folder,{name,version,...(range?{dependencies:{dep:range},optionalDependencies:{ghost:"*"}}:{})});
    put(first,{name:"dep",version:"1.2.3"});put(second,{name:"dep",version:"2.4.0"});put(join(ga,"w/0"),{name:"member-a",version:"1.0.0",dependencies:{dep:"^1.0.0"},optionalDependencies:{ghost:"*"}});put(join(gb,"w/0"),{name:"member-b",version:"1.0.0",dependencies:{dep:row.mutation==="range"?"^1.0.0":"^2.0.0"},optionalDependencies:{ghost:"*"}});
    for(const stage of[ga,gb])mkdirSync(join(stage,"w/0/node_modules"));
    for(const[folder,target]of[[workspace,join(ga,"node_modules")],[join(workspace,"p"),join(ga,"w/0/node_modules")],[join(workspace,"b"),join(gb,"node_modules")],[join(workspace,"b/q"),join(gb,"w/0/node_modules")]]as const)link(target,join(folder,"node_modules"));
    if(row.mutation==="cycle")link(join(ga,"w/0/node_modules/dep"),join(ga,"w/0/node_modules/dep"));
    if(row.mutation==="link-replaced"){put(join(ga,"alternate"),{name:"dep",version:"1.2.3"});link(first,join(ga,"w/0/node_modules/dep"));}
    const context=Object.freeze({root:cache,invocation:original,control,check:()=>leases.checkResourceLeaseControl(control),recheck:async()=>{leases.checkResourceLeaseControl(control);if(++rechecks===2){if(row.mutation==="negative-created"){put(join(ga,"node_modules/ghost"),{name:"ghost",version:"1.0.0"});fired=true;}else if(row.mutation==="manifest"){writeFileSync(join(first,"package.json"),JSON.stringify({name:"dep",version:"1.2.4"}));fired=true;}else if(row.mutation==="link-replaced"){unlinkSync(join(ga,"w/0/node_modules/dep"));link(join(ga,"alternate"),join(ga,"w/0/node_modules/dep"));fired=true;}else if(row.mutation==="relink"){mkdirSync(join(ga,"alias"));link(first,join(ga,"alias/dep"));link(join(ga,"alias/dep"),join(ga,"w/0/node_modules/dep"));fired=true;}}}}),input={version:1,requested:[".","b"],owners:{version:1,scopes:[{manifest:"package.json",packages:["p"]},{manifest:"b/package.json",packages:["b/q"]}]},generations:[{scope:".",generation:"a".repeat(64)},{scope:"b",generation:"b".repeat(64)}]};
    let accepted=true,result:Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerExecutionV1>>|undefined;try{result=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context as never,control,input);}catch{accepted=false;}
    expect(accepted,row.name).toBe(row.accepted);if(["negative-created","manifest","relink","link-replaced"].includes(row.mutation))expect(fired,row.name+" actual mutation").toBe(true);
    if(result){const dependencies=summary(result).filter(edge=>edge.name==="dep");expect(dependencies.map(edge=>edge.version).sort()).toEqual(["1.2.3","2.4.0"]);for(const edge of dependencies)expect(versionOracleV1.compatible(edge.version!,edge.range)).toBe(true);expect(result.relations.find(relation=>relation.source==="p")!.edges.find(edge=>edge.name==="dep")!.provider!.path).not.toBe(result.relations.find(relation=>relation.source==="b/q")!.edges.find(edge=>edge.name==="dep")!.provider!.path);}
   }finally{rmSync(root,{recursive:true,force:true});}
  }
 });
},120000);
