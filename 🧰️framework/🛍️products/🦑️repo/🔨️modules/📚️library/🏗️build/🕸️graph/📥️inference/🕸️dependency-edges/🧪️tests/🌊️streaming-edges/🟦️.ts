/** 🌊️ Contract: an owner that only gets its edges again is resolved from the module facts of its published node, one owner at a time, without reading a source, and its edges are exactly the edges the full edge discovery gives it, which are exactly the edges of an independent Python model (imports to the deepest owner, bare imports and package.json dependencies to the workspace package). */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{testPythonV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const inference=await import("../../../🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const implementation=await (inference as any).loadCurrentImplementationV1();
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type World={projects:Record<string,{root:string;files:Record<string,string>}>;byPackage:Record<string,string>};
const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};

const makeWorld=(seed:number,count:number):World=>{
 const random=seeded(seed),names=Array.from({length:count},(_,at)=>"p"+at),projects:World["projects"]={},byPackage:Record<string,string>={};
 names.forEach((name,at)=>{const nested=at>2&&random()<0.25,root=nested?projects[names[Math.floor(random()*at)]!]!.root+"/"+name:"area"+(at%4)+"/"+name;projects[name]={root,files:{}};byPackage["@w/"+name]=name;});
 projects.workspace={root:".",files:{"workspace-file.ts":'import "@w/p0/sub";\n'}};
 for(const name of names){
  const root=projects[name]!.root,dependencies:Record<string,string>={};
  for(let line=0;line<Math.floor(random()*3);line++)dependencies["@w/"+names[Math.floor(random()*count)]!]="1.0.0";
  if(random()<0.3)dependencies["left-pad"]="1.0.0";
  projects[name]!.files[root+"/package.json"]=JSON.stringify({name:"@w/"+name,dependencies});
  for(let file=0;file<2+Math.floor(random()*3);file++){
   const lines:string[]=[];
   for(let line=0;line<1+Math.floor(random()*4);line++){
    const other=names[Math.floor(random()*count)]!,kind=random();
    lines.push(kind<0.4?`import x${line} from "../../${projects[other]!.root}/file0.ts";`:kind<0.6?`import "@w/${other}/sub";`:kind<0.75?`export { y${line} } from "./file${Math.floor(random()*3)}.ts";`:kind<0.9?`import z${line} from "node:path";`:`import w${line} from "left-pad";`);
   }
   projects[name]!.files[root+"/file"+file+".ts"]=lines.join("\n")+"\nexport const v="+file+";\n";
  }
 }
 return{projects,byPackage};
};

const materialize=(world:World)=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"streaming-edges-")));
 for(const project of Object.values(world.projects))for(const[path,text]of Object.entries(project.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 return root;
};

const pairs=(edges:{source:string;target:string;sourceFile:string}[],name:string)=>edges.filter(edge=>edge.source===name).map(edge=>[edge.target,edge.sourceFile]).sort((a,b)=>JSON.stringify(a)<JSON.stringify(b)?-1:1);

test("10 seeded worlds: the streamed edges of every owner equal the full discovery and the independent model",async()=>{
 const worlds=Array.from({length:10},(_,at)=>makeWorld(26102026+at,8+at*9)),run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({worlds}))});
 expect(run.exitCode).toBe(0);const answers=JSON.parse(run.stdout.toString()).answers as {edges:Record<string,string[][]>}[];
 let total=0,withManifest=0;
 for(const[at,world]of worlds.entries()){
  const root=materialize(world);
  try{
   const projects=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,{name,root:project.root,targets:{}}])),projectFileMap=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,Object.keys(project.files).map(file=>({file}))]));
   const context={workspaceRoot:root,projects,fileMap:{projectFileMap},selection:{kind:"all"},completeSourceImports:false,moduleGraphProjects:new Set(Object.keys(projects))};
   const full=await cacheInternals.createDependenciesImplementation({},context) as {source:string;target:string;sourceFile:string}[];
   const metadata=cacheInternals.createMetadataImplementation({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},{},context);
   const streaming=await implementation.createStreamingEdgesV1({},context);
   for(const name of Object.keys(world.projects)){
    const streamed=streaming.edgesOf(name,{metadata:{semio:{moduleGraph:metadata[name].metadata.semio.moduleGraph}}}) as {source:string;target:string;sourceFile:string}[];
    expect(streamed.every(edge=>edge.source===name),`world ${at} ${name}`).toBe(true);
    expect(pairs(streamed,name),`world ${at} ${name}: streamed = full`).toEqual(pairs(full,name));
    expect(pairs(streamed,name),`world ${at} ${name}: streamed = model`).toEqual(answers[at]!.edges[name]);
    total+=streamed.length;withManifest+=streamed.filter(edge=>edge.sourceFile.endsWith("package.json")).length;
   }
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 expect(total).toBeGreaterThan(300);expect(withManifest).toBeGreaterThan(20);
});

test("an owner without module facts and a withheld owner stream no edge",async()=>{
 const world=makeWorld(5,6),root=materialize(world);
 try{
  const projects=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,{name,root:project.root,targets:{}}]));
  const streaming=await implementation.createStreamingEdgesV1({},{workspaceRoot:root,projects,fileMap:{projectFileMap:{}},selection:{kind:"all"},completeSourceImports:false});
  expect(streaming.edgesOf("p1",undefined)).toEqual([]);expect(streaming.edgesOf("p1",{metadata:{semio:{moduleGraph:{version:1,files:[]}}}})).toEqual([]);expect(streaming.edgesOf("workspace",{metadata:{semio:{moduleGraph:{files:[{file:"workspace-file.ts",imports:["@w/p0/sub"],productionImports:["@w/p0/sub"]}]}}}})).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("memory budget: streaming the edges of 600 owners retains nothing, and a discovery that retains the facts of nobody publishes none",async()=>{
 const world=makeWorld(99,600),root=materialize(world),mb=(bytes:number)=>bytes/1048576;
 try{
  const projects=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,{name,root:project.root,targets:{}}])),projectFileMap=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,Object.keys(project.files).map(file=>({file}))]));
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap},selection:{kind:"all"},completeSourceImports:false,moduleGraphProjects:new Set<string>()};
  await cacheInternals.createDependenciesImplementation({},context);
  const metadata=cacheInternals.createMetadataImplementation({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},{},context);
  expect(Object.keys(metadata)).toEqual([]);
  const fullContext={...context,moduleGraphProjects:new Set(Object.keys(projects))};
  await cacheInternals.createDependenciesImplementation({},fullContext);
  const all=cacheInternals.createMetadataImplementation({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},{},fullContext),nodes=Object.entries(all).map(([name,value]:[string,any])=>[name,{metadata:{semio:{moduleGraph:value.metadata.semio.moduleGraph}}}] as const);
  expect(nodes.length).toBe(601);
  const streaming=await implementation.createStreamingEdgesV1({},context);
  Bun.gc(true);const before=process.memoryUsage().heapUsed;let edges=0;
  for(const[name,node]of nodes)edges+=streaming.edgesOf(name,node).length;
  Bun.gc(true);const grown=mb(process.memoryUsage().heapUsed-before);
  expect(edges).toBeGreaterThan(500);
  expect(grown,"heap grown by streaming 600 owners: "+grown.toFixed(1)+" MB").toBeLessThan(25);
 }finally{rmSync(root,{recursive:true,force:true});}
});
