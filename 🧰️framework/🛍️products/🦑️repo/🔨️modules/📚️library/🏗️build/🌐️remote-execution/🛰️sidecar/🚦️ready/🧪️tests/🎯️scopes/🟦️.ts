import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
import {READY_STAGES_V1,readyDependencyScopesV1,readyDerivedInodesV1,readyProjectRootsV1,readyScopeOwnersV1,readyScopesOfStageV1,readyStagesForRootV1,type ReadyStageV1} from "../../🟦️.ts";

const put=(root:string,path:string,text:string)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);};
const checkout=():string=>{
 const root=mkdtempSync(join(tmpdir(),"semio-scopes-"));
 put(root,"package.json","{}");put(root,"bun.lock","a\nb\nc\n");
 for(const [owner,lines] of [["alpha",10],["beta",20]] as const){put(root,`${owner}/package.json`,"{}");put(root,`${owner}/bun.lock`,"x\n".repeat(lines));}
 put(root,"gamma/package.json","{}");
 put(root,"framework/lib/📋️project.json",JSON.stringify({name:"lib"}));
 put(root,"alpha/app/📋️project.json",JSON.stringify({name:"app"}));
 put(root,"alpha/app/node_modules/dep/📋️project.json",JSON.stringify({name:"ghost"}));
 put(root,"beta/tool/📋️project.json",JSON.stringify({name:"tool"}));
 return root;
};
const stage=(projects:string):ReadyStageV1[]=>[
 {name:"dependencies",kind:"repo",arguments:["dependencies","sync","--all"],timeoutMilliseconds:1,inputs:[],bootstrapInput:false},
 {name:"workspace",kind:"repo",arguments:["run-many",`--projects=${projects}`,"--targets=build"],timeoutMilliseconds:1,inputs:[],bootstrapInput:false}];
const python=(root:string):string[]=>JSON.parse(spawnSync("python3",["-I","-c","import json,os,sys\nr=sys.argv[1]\nprint(json.dumps(sorted(['.']+[d for d in os.listdir(r) if not d.startswith('.') and d!='node_modules' and os.path.isfile(os.path.join(r,d,'bun.lock')) and os.path.isfile(os.path.join(r,d,'package.json'))])))",root],{encoding:"utf8"}).stdout);

test("owners are the root and every top folder with its own manifest and lockfile, like an independent python listing",()=>{
 const root=checkout();
 try{expect(readyScopeOwnersV1(root)).toEqual(python(root));expect(readyScopeOwnersV1(root)).toEqual([".","alpha","beta"]);}finally{rmSync(root,{recursive:true,force:true});}
});
test("project folders come from the project files and never from build output",()=>{
 const root=checkout();
 try{
  const found=readyProjectRootsV1(root,["lib","app","tool","ghost","absent"]);
  expect([...found.entries()].sort()).toEqual([["app","alpha/app"],["lib","framework/lib"],["tool","beta/tool"]]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("a plan needs the root and the owners of its projects, and falls back to every owner when it cannot tell",()=>{
 const root=checkout();
 try{
  expect(readyDependencyScopesV1(root,stage("lib"))).toEqual(["."]);
  expect(readyDependencyScopesV1(root,stage("lib,app"))).toEqual([".","alpha"]);
  expect(readyDependencyScopesV1(root,stage("app,tool"))).toBeNull();
  expect(readyDependencyScopesV1(root,stage("app,absent"))).toBeNull();
  expect(readyDependencyScopesV1(root,[stage("app")[0]!])).toBeNull();
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("the dependency stage is rewritten to one --scope per owner and its inode need counts only those lockfiles",()=>{
 const root=checkout();
 try{
  const plan=readyStagesForRootV1(stage("lib,app"),root),row=plan.find(item=>item.name==="dependencies")!;
  expect(row.arguments).toEqual(["dependencies","sync","--scope",".","--scope","alpha"]);
  expect(readyScopesOfStageV1(plan,"dependencies")).toEqual([".","alpha"]);
  expect(readyScopesOfStageV1(stage("lib"),"dependencies")).toBeNull();
  expect(readyStagesForRootV1(stage("app,tool"),root)).toEqual(stage("app,tool"));
  const all=readyDerivedInodesV1("dependencies",root)!,part=readyDerivedInodesV1("dependencies",root,[".","alpha"])!;
  expect(all).toBe(Math.ceil(33*3.6));
  expect(part).toBe(Math.ceil(13*3.6));
  expect(readyDerivedInodesV1("tools",root,["."])).toBeNull();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the declared workspace stage of this checkout needs the root and the owner of the norm plugin only",()=>{
 let root=import.meta.dir;while(root!==dirname(root)&&!(existsSync(join(root,"bun.lock"))&&existsSync(join(root,"nx.json"))))root=dirname(root);
 if(!existsSync(join(root,"nx.json")))return;
 const owners=readyDependencyScopesV1(root,READY_STAGES_V1);
 expect(owners).toEqual([".","✏️s"]);
 const plan=readyStagesForRootV1(READY_STAGES_V1,root);
 expect(plan.find(row=>row.name==="dependencies")!.arguments).toEqual(["dependencies","sync","--scope",".","--scope","✏️s"]);
 expect(readyDerivedInodesV1("dependencies",root,owners)!).toBeLessThan(readyDerivedInodesV1("dependencies",root)!/2);
},120000);
