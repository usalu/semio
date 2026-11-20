import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {closeBuildOwnerAdmissionV1,readBuildGraphSelectionV1,type BuildOwnerRegistryV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const registry=(names:readonly string[],catalogProviders:BuildOwnerRegistryV1["catalogProviders"]=[]):BuildOwnerRegistryV1=>({
 version:1,owners:names.map(name=>({name,root:name,manifest:name+"/📋️project.json",packages:[]})),catalogProviders,
});

interface Facts {
 targets:Record<string,string[]>;
 taskTargets?:Record<string,string[]>;
 dependencies?:string[];
 inputs?:Record<string,string[]>;
 generated?:Record<string,string[]>;
 files?:string[];
 observedFiles?:string[];
}

test("owner admission matches a sweep oracle through dependency, task and provider changes",async()=>{
 const python=process.platform==="win32"?"python":"python3",result=spawnSync(python,["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(fixture.cases),encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 const expected=JSON.parse(result.stdout);
 for(const row of fixture.cases){
  const rows=row.rows as Record<string,Facts>,calls:unknown[]=[],controller=new AbortController();
  let admission,error;
  try{
   admission=await closeBuildOwnerAdmissionV1(readBuildGraphSelectionV1(row.selection),registry(Object.keys(rows),row.providers),async(owner,targets,inputs)=>{
    controller.signal.throwIfAborted();
    calls.push([owner.name,targets,inputs]);
    const known=rows[owner.name]!,available=targets.length&&known.taskTargets!==undefined?known.taskTargets:known.targets;
    if(calls.length===row.abortAfter)controller.abort(Error("admission fixture cancelled"));
    return {project:{name:owner.name,targets:Object.fromEntries(Object.entries(available).map(([target,dependsOn])=>[target,{dependsOn}]))},files:known.files??[],
     dependencies:known.dependencies??[],...(known.generated===undefined?{}:{generatedDependencies:known.generated}),...(known.inputs===undefined?{}:{dependencyInputs:known.inputs}),
     ...(known.observedFiles===undefined?{}:{observedFiles:known.observedFiles})};
   });
  }catch(cause){error=(cause as Error).message;}
  expect({calls,admission,error},row.name).toEqual(expected[row.name]);
 }
});

test("the deep admission chain serializes only its newly dirty owners",async()=>{
 const names=Array.from({length:fixture.depth},(_,index)=>"n"+String(index).padStart(5,"0")),calls:string[]=[];
 const stringify=JSON.stringify;
 let revisions=0;
 JSON.stringify=((value:unknown,...arguments_:unknown[])=>{
  if(Array.isArray(value)&&value.length===2&&value.every(member=>Array.isArray(member)&&member.length===0))revisions++;
  return Reflect.apply(stringify,JSON,[value,...arguments_]);
 }) as typeof stringify;
 try{
  const actual=await closeBuildOwnerAdmissionV1({kind:"owners",projects:[names[0]!],exclude:[]},registry(names),async owner=>{
   calls.push(owner.name);
   const next=names[Number(owner.name.slice(1))+1];
   return {project:{name:owner.name,targets:{}},files:[],dependencies:next===undefined?[]:[next]};
  });
  expect(actual.owners).toEqual(names);
  expect(calls).toEqual(names);
  expect(revisions).toBe(names.length);
 }finally{JSON.stringify=stringify;}
});
