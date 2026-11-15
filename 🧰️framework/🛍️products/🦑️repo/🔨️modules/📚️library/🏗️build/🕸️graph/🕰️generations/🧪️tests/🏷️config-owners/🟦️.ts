/** 🏷️ Contract: the content of a project's config file dirties its owner whichever owner holds the fact about the file, so an owner that claims no facts by path (the root project: its config is read by the shared setup and belongs to the environment) is inferred again after a content edit that moves neither a mode nor a name. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {staleConfigOwnersV1} from "../../🏷️config-owners/🟨️.mjs";
import {applyOps,createWorld,seed} from "../🏃️world/🟦️.ts";

const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-config-"));
const project=(name:string,uses:string[]=[])=>({[`pkg/${name}/project.json`]:JSON.stringify({name,package:`@syn/${name}`,uses}),[`pkg/${name}/src/index.ts`]:"export const v=1;\n"});
const environmentHolds=(name:string)=>(compute:any)=>async(spec:any,signal?:AbortSignal)=>{
 const result=await compute(spec,signal);
 for(const owner of result.owners)if(owner.name===name){
  const moved=owner.rows.filter((row:any)=>row.kind==="file"&&row.path===owner.config);
  owner.rows=owner.rows.filter((row:any)=>!moved.includes(row));result.env.rows=[...result.env.rows,...moved];
 }
 return result;
};

test("a content edit of a config the environment holds dirties exactly its owner",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{...project("a"),...project("b"),"shared/x.json":"{}"});
  const world=createWorld(dir,{computeWrap:environmentHolds("a")});await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/a/project.json",content:JSON.stringify({name:"a",package:"@syn/a",uses:["shared/x.json"]})}]);
  const before=world.log.length,result=await world.refresh();
  expect(result.built).toBe(true);expect(world.log.slice(before).map(row=>row.full)).toEqual([["a"]]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("an edit of the config of an owner that holds its own facts still dirties only that owner",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{...project("a"),...project("b"),"shared/x.json":"{}"});
  const world=createWorld(dir,{computeWrap:environmentHolds("a")});await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/b/project.json",content:JSON.stringify({name:"b",package:"@syn/b",uses:["shared/x.json"]})}]);
  const before=world.log.length;await world.refresh();
  expect(world.log.slice(before).map(row=>row.full)).toEqual([["b"]]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

const python=`import json,posixpath,sys
case=json.load(sys.stdin);root=case["root"].replace("\\\\","/").rstrip("/")+"/" if case.get("root") else None
out=set()
for row in case["rows"]:
    text=row["path"].replace("\\\\","/")
    if root and text.startswith(root): text=text[len(root):]
    elif text.startswith("/") or (len(text)>1 and text[1]==":"): continue
    text=posixpath.normpath(text)
    if text in case["configs"]: out.add(case["configs"][text])
print(json.dumps(sorted(out)))`;
const cases=[
 {root:"/w",rows:[{path:"/w/📋️project.json"},{path:"/w/x/📋️project.json"},{path:"/other/📋️project.json"}],configs:{"📋️project.json":"workspace","x/📋️project.json":"x"}},
 {root:"C:\\w",rows:[{path:"C:\\w\\📋️project.json"},{path:"y/📋️project.json"},{path:"D:\\q\\📋️project.json"}],configs:{"📋️project.json":"workspace","y/📋️project.json":"y"}},
 {rows:[{path:"📋️project.json"},{path:"/abs/📋️project.json"}],configs:{"📋️project.json":"workspace"}},
];

test("the stale rows that name a config member map to their owners as an independent model maps them",()=>{
 for(const[at,example]of cases.entries()){
  const run=Bun.spawnSync(["python3","-I","-c",python],{stdin:new TextEncoder().encode(JSON.stringify(example))});
  expect(run.exitCode,run.stderr.toString()).toBe(0);
  const rows=example.rows.map(row=>({kind:"file",...row})),stale=new Set(rows.map((_,index)=>index));
  const named=[...staleConfigOwnersV1({rows,stale,configs:new Map(Object.entries(example.configs)),workspaceRoot:(example as any).root})].sort();
  expect(named,"case "+at).toEqual(JSON.parse(run.stdout.toString()));
 }
});
