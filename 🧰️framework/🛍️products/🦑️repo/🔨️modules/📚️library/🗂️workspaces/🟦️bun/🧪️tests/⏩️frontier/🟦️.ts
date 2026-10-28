import{test,expect}from"bun:test";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import schema from"../../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

const source=readFileSync(new URL("../../🟨️.cjs",import.meta.url),"utf8"),tree=ts.createSourceFile("ownership.js",source,ts.ScriptTarget.Latest,true);
function implementation(name:string,bindings:Record<string,unknown>):any{const node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);if(!node)throw Error("Actual ownership implementation required");return new Function(...Object.keys(bindings),node.getText(tree)+";return "+name+";")(...Object.values(bindings));}

test("frontier continuations are bounded while final replay catches changes across every batch",async()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema.$defs.FrontierBatchCase),batch=schema.$defs.FrontierReplayProtocol.const.progressBatchPaths;
 try{for(const row of corpus.cases){
  expect(validate(row)).toBe(true);expect(database.query("SELECT (17+?1)/?2 + ((17+?1)%?2 != 0) AS count").get(row.paths,batch)).toEqual({count:row.callbacks});
  const states=new Map(Array.from({length:row.paths},(_,index)=>["/"+index,"original"])),owner={root:"/",physical:{},observations:{paths:new Map([...states].map(([path,state])=>[path,{path,state}])),frontier:new Map()},ownership:{ownershipPath:"/policy",ownershipSource:"policy"},texts:new Map()},progress={completedOperations:17},stop=new AbortController(),events:{path:string;completedOperations:number}[]=[],replay=implementation("replayCurrentMembershipV1",{frontierReplayProtocol:schema.$defs.FrontierReplayProtocol.const,fs:{lstatSync:(path:string)=>states.get(path)},nativeOperations:()=>({}),ownershipStateV1:(value:unknown)=>value,readManifestText:()=>"policy",limits:{maximumPolicyBytes:65536},bounded:(_root:string,path:string)=>path});
  const accepted=await replay(owner,{signal:stop.signal,frontierBatchPaths:batch,advance:async(event:{path:string;completedOperations:number})=>{events.push(event);if(row.mutation==="first"&&events.length===1)states.set("/0","changed");if(row.mutation==="last"&&events.length===row.callbacks)states.set("/"+(row.paths-1),"changed");}},progress);
  expect(accepted).toBe(row.accepted);expect(events.length).toBe(row.callbacks);expect(events.at(-1)).toEqual({operation:"frontier",path:"/"+(row.paths-1),completedOperations:17+row.paths});expect(progress.completedOperations).toBe(17+row.paths);
 }}finally{database.close();}
});

test("published ownership authenticates the global frontier after every awaited caller before returning",async()=>{
 const events:string[]=[],signal=new AbortController().signal,owner={root:"/root",texts:new Map(),documents:new Map(),observations:{},physical:{}},current={owner,report:{scopes:[]},completedOperations:0},control={signal,advance:async()=>{events.push("continuation");}},publish=implementation("publishCurrentOwnership",{controlledOwnership:async()=>current,nativeOperations:()=>({}),readManifestText:()=>"policy",limits:{maximumPolicyBytes:65536},requireCurrentMembershipV1:async()=>{events.push("frontier");},currentFence:async()=>{events.push("frontier");},bounded:(_root:string,path:string)=>path});
 const custody=await publish({ownershipPath:"/policy",ownershipSource:"policy"},"/root",{kind:"all"},control);await custody.recheck();
 expect(events).toEqual(["continuation","frontier","continuation","frontier"]);
});
