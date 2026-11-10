/** 📈️ Slow-tier law: the heap held while the edges of every owner are streamed does not grow with the number of owners. The fast streaming test proves the bound at 60 owners; here the growth at 600 owners stays within the same bound and within a small margin of the growth at 60, so the memory of the stream is independent of the owner count. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";

const inference=await import("../../../🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const implementation=await (inference as any).loadCurrentImplementationV1();

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

const growth=async(count:number):Promise<number>=>{
 const world=makeWorld(99,count),root=materialize(world),mb=(bytes:number)=>bytes/1048576;
 try{
  const projects=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,{name,root:project.root,targets:{}}])),projectFileMap=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,Object.keys(project.files).map(file=>({file}))]));
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap},selection:{kind:"all"},completeSourceImports:false,moduleGraphProjects:new Set(Object.keys(projects))};
  await cacheInternals.createDependenciesImplementation({},context);
  const metadata=cacheInternals.createMetadataImplementation({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},{},context),nodes=Object.entries(metadata);
  const streaming=await implementation.createStreamingEdgesV1({},{...context,moduleGraphProjects:new Set<string>()});
  Bun.gc(true);const before=process.memoryUsage().heapUsed;let edges=0;
  for(const[name,node]of nodes)edges+=streaming.edgesOf(name,node as any).length;
  Bun.gc(true);
  expect(edges).toBeGreaterThan(count);
  return mb(process.memoryUsage().heapUsed-before);
 }finally{rmSync(root,{recursive:true,force:true});}
};

test("heap growth while streaming the edges of 600 owners stays within the bound of 60 owners: memory does not depend on the owner count",async()=>{
 const small=await growth(60),large=await growth(600);
 expect(small,"heap grown by streaming 60 owners: "+small.toFixed(1)+" MB").toBeLessThan(5);
 expect(large,"heap grown by streaming 600 owners: "+large.toFixed(1)+" MB").toBeLessThan(5);
 expect(large-small,"growth from 60 to 600 owners").toBeLessThan(3);
},600000);
