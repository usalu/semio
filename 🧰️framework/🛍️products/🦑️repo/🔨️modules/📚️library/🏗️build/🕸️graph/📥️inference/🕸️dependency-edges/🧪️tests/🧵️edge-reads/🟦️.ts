/** 🧵️ Contract: the lookups the edges of an owner record are enough to say which owners keep their edges when the environment moves. Over seeded workspaces of JavaScript and Cargo owners with relative imports, package imports, path dependencies and a workspace manifest, one environment edit is applied (an owner added at an unused place, at a place an import looked up in vain, below the root an import resolved to; an owner removed; a package renamed; a workspace dependency repointed); every owner that is not edited itself and whose edges differ between the old and the new environment, as the real inferrer computes them, must be among the consumers the recorded lookups name, and the consumers must be fewer than all owners. The streamed edges record the same lookups as the full discovery. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,posix}from"node:path";
import{createEdgeReadsRecorderV1,diffEnvironmentV1,edgeConsumersV1}from"../../../../🕰️generations/🧵️edge-reads/🟨️.mjs";
import{environmentPartsV1}from"../../../../🕰️generations/🌍️environment/🟨️.mjs";

const inference=await import("../../../🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const implementation=await (inference as any).loadCurrentImplementationV1();
type Project={name:string;root:string;files:Record<string,string>;package?:string};
type World={projects:Project[]};
const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
const relative=(from:string,to:string)=>posix.relative(from,to);

const makeWorld=(seed:number,count:number):World=>{
 const random=seeded(seed),pick=(list:readonly string[])=>list[Math.floor(random()*list.length)]!;
 const js:Project[]=[],crates:Project[]=[];
 for(let at=0;at<count;at++){const nested=at>2&&random()<0.25,root=nested?pick(js.map(project=>project.root))+"/n"+at:"area"+(at%4)+"/p"+at;js.push({name:"p"+at,root,files:{},package:"@w/p"+at});}
 for(let at=0;at<Math.ceil(count/3);at++)crates.push({name:"c"+at,root:"crates/c"+at,files:{}});
 const roots=[...js,...crates].map(project=>project.root),ghosts=Array.from({length:4},(_,at)=>"ghost"+at+"/g");
 for(const project of js){
  const dependencies:Record<string,string>={};
  for(let line=0;line<Math.floor(random()*3);line++)dependencies[pick(js.map(other=>other.package!))]="1.0.0";
  project.files[project.root+"/package.json"]=JSON.stringify({name:project.package,dependencies});
  for(let file=0;file<1+Math.floor(random()*3);file++){
   const lines:string[]=[];
   for(let line=0;line<1+Math.floor(random()*4);line++){
    const kind=random(),target=kind<0.3?pick(roots)+"/file0.ts":kind<0.4?pick(roots)+"/inner/file0.ts":kind<0.5?pick(ghosts)+"/file0.ts":"";
    lines.push(kind<0.5?`import x${line} from "${relative(project.root,target).startsWith(".")?relative(project.root,target):"./"+relative(project.root,target)}";`:kind<0.75?`import "${pick(js.map(other=>other.package!))}/sub";`:kind<0.85?`import z${line} from "node:path";`:`import w${line} from "left-pad";`);
   }
   project.files[project.root+"/file"+file+".ts"]=lines.join("\n")+"\nexport const v="+file+";\n";
  }
 }
 for(const project of crates){
  const lines=['[package]','name = "'+project.name+'"','version = "0.1.0"','','[dependencies]'];
  for(let line=0;line<Math.floor(random()*4);line++){const target=random()<0.3?pick(ghosts):pick(crates.map(other=>other.root));lines.push("d"+line+' = { path = "'+relative(project.root,target)+'" }');}
  if(random()<0.5)lines.push("w = { workspace = true }");
  project.files[project.root+"/Cargo.toml"]=lines.join("\n")+"\n";
 }
 const workspace:Project={name:"workspace",root:".",files:{"Cargo.toml":'[workspace]\nmembers = ["crates/*"]\n\n[workspace.dependencies]\nw = { path = "crates/c0" }\n'}};
 return{projects:[...js,...crates,workspace]};
};

const clone=(world:World):World=>structuredClone(world);
const ownerOf=(world:World)=>{const roots=world.projects.filter(project=>project.root!==".").sort((a,b)=>b.root.length-a.root.length);return(file:string)=>roots.find(project=>file.startsWith(project.root+"/"))?.name??"workspace";};
const materialize=(world:World)=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"edge-reads-")));
 for(const project of world.projects)for(const[path,text]of Object.entries(project.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 return root;
};
const contextOf=(world:World,root:string,extra:Record<string,unknown>={})=>{
 const projects=Object.fromEntries(world.projects.map(project=>[project.name,{name:project.name,root:project.root,targets:{}}])),of=ownerOf(world),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(world.projects.map(project=>[project.name,[]]));
 for(const project of world.projects)for(const file of Object.keys(project.files))projectFileMap[of(file)]!.push({file});
 return{workspaceRoot:root,projects,fileMap:{projectFileMap},selection:{kind:"all"},completeSourceImports:false,moduleGraphProjects:new Set(Object.keys(projects)),...extra};
};
const partsOf=(world:World,root:string)=>{
 const text=(project:Project)=>project.files[project.root+"/package.json"];
 return environmentPartsV1({owners:world.projects.map(project=>[project.name,project.root,null] as [string,string,string|null]),packages:world.projects.filter(text).map(project=>[JSON.parse(text(project)!).name,project.name] as [string,string]),modules:[],workspaces:[["Cargo.toml",world.projects.find(project=>project.name==="workspace")!.files["Cargo.toml"]!]],lock:null,taxonomy:null});
};
const pairs=(edges:{source:string;target:string;sourceFile:string}[],name:string)=>edges.filter(edge=>edge.source===name).map(edge=>[edge.target,edge.sourceFile]).sort((a,b)=>JSON.stringify(a)<JSON.stringify(b)?-1:1);

type Edit={label:string;apply:(world:World,random:()=>number)=>string[]};
const edits:Edit[]=[
 {label:"owner added at an unused place",apply(world){world.projects.push({name:"fresh",root:"fresh/f",files:{"fresh/f/package.json":JSON.stringify({name:"@w/fresh"}),"fresh/f/file0.ts":"export const v=0;\n"}});return["fresh"];}},
 {label:"owner added where an import found nothing",apply(world){world.projects.push({name:"ghosted",root:"ghost0/g",files:{"ghost0/g/package.json":JSON.stringify({name:"@w/ghosted"}),"ghost0/g/file0.ts":"export const v=0;\n"}});return["ghosted"];}},
 {label:"owner added below the root an import resolved to",apply(world,random){const host=world.projects.filter(project=>project.package)[Math.floor(random()*world.projects.filter(project=>project.package).length)]!;world.projects.push({name:"inner",root:host.root+"/inner",files:{[host.root+"/inner/package.json"]:JSON.stringify({name:"@w/inner"}),[host.root+"/inner/file0.ts"]:"export const v=0;\n"}});return["inner"];}},
 {label:"owner added at a crate path nothing resolved",apply(world){world.projects.push({name:"ghostcrate",root:"ghost1/g",files:{"ghost1/g/Cargo.toml":'[package]\nname = "ghostcrate"\n'}});return["ghostcrate"];}},
 {label:"owner removed",apply(world,random){const victims=world.projects.filter(project=>project.name!=="workspace");const victim=victims[Math.floor(random()*victims.length)]!;world.projects=world.projects.filter(project=>project!==victim);return[victim.name];}},
 {label:"package renamed",apply(world,random){const victims=world.projects.filter(project=>project.package);const victim=victims[Math.floor(random()*victims.length)]!;const json=JSON.parse(victim.files[victim.root+"/package.json"]!);json.name="@w/renamed";victim.files[victim.root+"/package.json"]=JSON.stringify(json);victim.package="@w/renamed";return[victim.name];}},
 {label:"workspace dependency repointed",apply(world){const workspace=world.projects.find(project=>project.name==="workspace")!;workspace.files["Cargo.toml"]=workspace.files["Cargo.toml"]!.replace("crates/c0","crates/c1");return["workspace"];}},
];

test("seeded workspaces: every owner whose edges differ under an edited environment is a consumer of the edited rows, and the consumers are few",async()=>{
 let changed=0,kept=0,flagged=0,runs=0;const perEdit:Record<string,{changed:number;flagged:number;kept:number}>={};
 for(let seed=0;seed<4;seed++)for(const edit of edits){
  const random=seeded(7000+seed),a=makeWorld(26110301+seed,10+seed*5),b=clone(a),editedNames=new Set(edit.apply(b,random));
  const rootA=materialize(a),rootB=materialize(b);
  try{
   const recorder=createEdgeReadsRecorderV1(),contextA=contextOf(a,rootA,{edgeReads:recorder});
   const full=await cacheInternals.createDependenciesImplementation({},contextA) as {source:string;target:string;sourceFile:string}[];
   const metadata=cacheInternals.createMetadataImplementation({nodes:Object.fromEntries(Object.entries(contextA.projects).map(([name,data])=>[name,{name,type:"lib",data}]))},{},contextA);
   const streamRecorder=createEdgeReadsRecorderV1(),streamingA=await implementation.createStreamingEdgesV1({},{...contextA,edgeReads:streamRecorder});
   const facts=(name:string)=>({metadata:{semio:{moduleGraph:metadata[name].metadata.semio.moduleGraph}}});
   for(const project of a.projects){
    const streamed=streamingA.edgesOf(project.name,facts(project.name)) as {source:string;target:string;sourceFile:string}[];
    expect(pairs(streamed,project.name),`${edit.label} seed ${seed} ${project.name}: streamed = full`).toEqual(pairs(full,project.name));
    expect(streamRecorder.finish(project.name),`${edit.label} seed ${seed} ${project.name}: same lookups`).toEqual(recorder.finish(project.name));
   }
   const contextB=contextOf(b,rootB),streamingB=await implementation.createStreamingEdgesV1({},contextB);
   const partsA=partsOf(a,rootA),partsB=partsOf(b,rootB),rows=new Map<string,{root:string;nativeRoot?:string;reads?:any}>();
   for(const project of a.projects)if(b.projects.some(other=>other.name===project.name)&&!editedNames.has(project.name))rows.set(project.name,{root:project.root,reads:recorder.finish(project.name)});
   const table=(world:World)=>new Map(world.projects.map(project=>[project.name,[project.root,null] as (string|null)[]]));
   const consumers=edgeConsumersV1({diff:diffEnvironmentV1(partsA,partsB),rows,tables:[table(a),table(b)]});
   const here=perEdit[edit.label]??={changed:0,flagged:0,kept:0};
   for(const name of rows.keys()){
    const before=streamingA.edgesOf(name,facts(name)) as {source:string;target:string;sourceFile:string}[],after=streamingB.edgesOf(name,facts(name)) as {source:string;target:string;sourceFile:string}[];
    kept++;here.kept++;
    if(JSON.stringify(pairs(before,name))!==JSON.stringify(pairs(after,name))){changed++;here.changed++;expect(consumers.has(name),`${edit.label} seed ${seed}: ${name} changed its edges but is no consumer`).toBe(true);}
    if(consumers.has(name)){flagged++;here.flagged++;}
   }
   runs++;
  }finally{rmSync(rootA,{recursive:true,force:true});rmSync(rootB,{recursive:true,force:true});}
 }
 expect(changed).toBeGreaterThan(10);
 expect(flagged).toBeLessThan(kept*0.5);
},300000);
