/** 🪣️ Contract: the import edges of a workspace are collected a slice of owners at a time and are the same edges for every slice size and for the independent Python model; the complete module facts are retained for the owners that ask for them and for nobody else, so a re-edge of owners that only get their edges again holds no facts, however many owners there are. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{testPythonV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const inference=await import("../../../🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const{EDGE_CHUNK_OWNERS}=await import("../../🟨️.mjs");
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type World={projects:Record<string,{root:string;files:Record<string,string>}>;byPackage:Record<string,string>;retain:string[]};
const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};

const makeWorld=(seed:number,count:number,retain:number):World=>{
 const random=seeded(seed),names=Array.from({length:count},(_,at)=>"p"+at),projects:World["projects"]={},byPackage:Record<string,string>={};
 names.forEach((name,at)=>{const nested=at>2&&random()<0.25,root=nested?names[Math.floor(random()*at)]!+"/"+name:"area"+(at%4)+"/"+name;projects[name]={root:projects[names[Math.floor(random()*Math.max(1,at))]!]?.root===root?root+"x":root,files:{}};byPackage["@w/"+name]=name;});
 for(const name of names){
  const root=projects[name]!.root;
  for(let file=0;file<2+Math.floor(random()*3);file++){
   const lines:string[]=[];
   for(let line=0;line<1+Math.floor(random()*4);line++){
    const other=names[Math.floor(random()*count)]!,kind=random();
    lines.push(kind<0.4?`import x${line} from "../../${projects[other]!.root}/file0.ts";`:kind<0.6?`import "@w/${other}/sub";`:kind<0.75?`export { y${line} } from "./file${Math.floor(random()*3)}.ts";`:kind<0.9?`import z${line} from "node:path";`:`import w${line} from "left-pad";`);
   }
   projects[name]!.files[root+"/file"+file+".ts"]=lines.join("\n")+"\nexport const v="+file+";\n";
  }
 }
 return{projects,byPackage,retain:names.slice(0,retain)};
};

const materialize=(world:World)=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"re-edge-")));
 for(const project of Object.values(world.projects))for(const[path,text]of Object.entries(project.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 return root;
};

const collect=async(world:World,root:string,chunk:number|undefined)=>{
 const edges:[string,string,string][]=[],order:string[]=[],publication:Record<string,{files:{file:string}[]}>={};
 const projectFiles=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,Object.keys(project.files).map(file=>({file}))]));
 const projects=Object.fromEntries(Object.entries(world.projects).map(([name,project])=>[name,{root:project.root}]));
 await cacheInternals.collectImportEdges(root,projectFiles,projects,new Map(Object.entries(world.byPackage)),undefined,(source:string,target:string,file:string)=>{if(source===target)return;edges.push([source,target,file]);order.push(source);},publication,undefined,{moduleGraphProjects:new Set(world.retain),...(chunk===undefined?{}:{edgeChunkOwners:chunk})});
 return{edges:[...new Map(edges.map(edge=>[edge.join("\0"),edge])).values()].sort((a,b)=>a.join("\0")<b.join("\0")?-1:1),order,published:Object.fromEntries(Object.entries(publication).map(([name,value])=>[name,value.files.map(row=>row.file).sort()]))};
};

test("the chunk of the graph is the chunk of the claim registry",async()=>{
 const{CLAIMS_POLICY_V1}=await import("../../../../🕰️generations/🧩️claims/🟨️.mjs");
 expect(EDGE_CHUNK_OWNERS).toBe(CLAIMS_POLICY_V1.chunkOwners);
});

test("12 seeded worlds: every slice size makes the edges of the independent model and the facts stay with the retained owners",async()=>{
 const worlds=Array.from({length:12},(_,at)=>makeWorld(26102026+at,10+at*7,1+at%3)),run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({worlds}))});
 expect(run.exitCode).toBe(0);const answers=JSON.parse(run.stdout.toString()).answers as {edges:[string,string,string][];published:Record<string,string[]>}[];
 let total=0;
 for(const[at,world]of worlds.entries()){
  const root=materialize(world);
  try{
   const names=Object.keys(world.projects),owners=new Map(names.map((name,index)=>[name,index]));
   for(const chunk of[1,3,7,undefined]){
    const found=await collect(world,root,chunk);
    expect(found.edges,`world ${at} chunk ${chunk}`).toEqual(answers[at]!.edges);
    expect(found.published,`world ${at} chunk ${chunk}`).toEqual(answers[at]!.published);
    const size=chunk??EDGE_CHUNK_OWNERS,slices=found.order.map(name=>Math.floor(owners.get(name)!/size));
    expect(slices,`world ${at} chunk ${chunk}: the edges of slice k come before those of slice k+1`).toEqual([...slices].sort((a,b)=>a-b));
   }
   total+=answers[at]!.edges.length;
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 expect(total).toBeGreaterThan(150);
});

test("nothing is retained when the retained set is empty, however many owners are collected",async()=>{
 const world={...makeWorld(7,60,0)},root=materialize(world);
 try{const found=await collect(world,root,4);expect(found.published).toEqual({});expect(found.edges.length).toBeGreaterThan(20);}finally{rmSync(root,{recursive:true,force:true});}
});
