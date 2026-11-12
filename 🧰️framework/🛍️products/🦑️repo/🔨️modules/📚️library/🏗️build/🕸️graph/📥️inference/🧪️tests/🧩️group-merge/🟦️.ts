/** 🧩️ Contract: the command source groups of one project that hold the same files are merged and renumbered in order of first use exactly as an independent Python model merges them, however long the lists are, and merging thousands of lists of thousands of files costs memory in the order of the lists themselves, never of their serialised text. */
import{expect,test}from"bun:test";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{mergeCommandSourceGroupsV1}from"../../🧱️project-assembly/🟨️.mjs";

const python=testPythonV1()!,oracle=join(import.meta.dir,"🐍️.py"),assembly=fileURLToPath(new URL("../../🧱️project-assembly/🟨️.mjs",import.meta.url));
const model=(project:unknown)=>{const run=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify(project))});expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString());};
const random=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};

const world=(seed:number,groups:number,bases:number)=>{
 const next=random(seed),pool=Array.from({length:bases},(_,at)=>Array.from({length:3+at},(_,item)=>"{workspaceRoot}/📦️pkg"+at+"/🔣️"+item+".ts"));
 const namedInputs:Record<string,unknown>={default:["{projectRoot}/**/*"],ownerPrivate:[{closureRef:"x"}]},used:number[]=[];
 let number=0;
 for(let at=0;at<groups;at++){
  number+=1+Math.floor(next()*3);
  const base=pool[Math.floor(next()*bases)]!,variant=next();
  namedInputs["commandSources"+number]=variant<0.15?[...base].reverse():variant<0.3?[...base,{externalDependencies:["a"]}]:variant<0.4?[]:[...base];
  used.push(number);
 }
 const targets=Object.fromEntries(used.map((value,at)=>["t"+at,{inputs:["default","commandSources"+value,{input:"commandSources"+used[(at*7)%used.length],fallback:"commandSources"+value}]}]));
 return{name:"p"+seed,root:"x",namedInputs,targets};
};

for(const[seed,groups,bases]of[[1,1,1],[2,12,2],[3,40,5],[4,200,3],[5,300,40]] as const)test("the merge equals the Python model: seed "+seed+", "+groups+" groups",()=>{
 const input=world(seed,groups,bases);
 expect(JSON.parse(JSON.stringify(mergeCommandSourceGroupsV1(JSON.parse(JSON.stringify(input)))))).toEqual(model(input));
});

test("a project without equal groups is returned as it is",()=>{
 const input={name:"p",root:"x",namedInputs:{commandSources0:["a"],commandSources1:["b"],other:["a"]},targets:{t:{inputs:["commandSources1"]}}};
 expect(mergeCommandSourceGroupsV1(input)).toBe(input);
});

test("thousands of long equal lists cost memory in the order of the lists, never of their text",async()=>{
 const child=`
  const {mergeCommandSourceGroupsV1}=await import(${JSON.stringify(assembly)});
  const files=Array.from({length:8000},(_,at)=>"{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🧪️tests/"+at+"/🟦️.ts");
  const namedInputs={},targets={};
  for(let at=0;at<1500;at++){namedInputs["commandSources"+at]=at%500===0?[...files,"extra"+at]:[...files];targets["t"+at]={inputs:["commandSources"+at]};}
  const text=JSON.stringify(namedInputs.commandSources1).length*1500;
  Bun.gc(true);const before=process.resourceUsage().maxRSS;
  const merged=mergeCommandSourceGroupsV1({name:"p",root:"x",namedInputs,targets});
  const after=process.resourceUsage().maxRSS;
  console.log(JSON.stringify({text,grown:(after-before)*1024,groups:Object.keys(merged.namedInputs).length,first:merged.targets.t7.inputs[0],last:merged.targets.t1499.inputs[0]}));
 `;
 const run=Bun.spawnSync([process.execPath,"-e",child]);expect(run.exitCode,run.stderr.toString()).toBe(0);
 const answer=JSON.parse(run.stdout.toString().trim().split("\n").pop()!) as {text:number;grown:number;groups:number;first:string;last:string};
 expect(answer.groups).toBe(4);expect(answer.first).toBe("commandSources1");expect(answer.last).toBe("commandSources1");
 expect(answer.grown).toBeLessThan(answer.text/4);
},120000);
