import {test,expect} from "bun:test";
import {mkdtempSync,mkdirSync,writeFileSync,rmSync,realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {withCurrentBunWorkspaceOwnershipV1,withPublishedBunWorkspaceOwnershipV1} from "../../🟦️.ts";

const control=()=>({signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}});
const noise=["node_modules/**","target/**"].map(pattern=>"!**/"+pattern);
const manifest=(name:string,workspace?:object)=>JSON.stringify({name,...(workspace?{semio:{workspace}}:{})});
const world=(extra:number)=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"semio-scope-capacity-")));
 for(let index=0;index<extra;index++){mkdirSync(join(root,"s"+index));writeFileSync(join(root,"s"+index,"package.json"),manifest("s"+index,{schemaVersion:1,members:["**",...noise],owners:[]}));}
 writeFileSync(join(root,"package.json"),manifest("root",{schemaVersion:1,members:["lib/**",...noise],owners:["*/package.json"]}));
 return root;
};
/** 🔢️ More declared installation scopes than the Source-owner protocol admits is named with its count and directories as a roster edit, never an opaque dense-rows failure. */
test("scope capacity: 69 declared scopes are refused naming count and directories, 63 are admitted",async()=>{
 const over=world(68),within=world(63);
 try{
  const error=await withCurrentBunWorkspaceOwnershipV1(over,{kind:"all"},control(),async()=>{}).then(()=>null,error=>error as Error&{reason?:string});
  expect(error?.message).toContain("Bun installation scope capacity exceeded (69 of 64): ");
  expect(error?.message).toContain("s0");
  expect(error?.reason).toBe("roster");
  const scopes=await withPublishedBunWorkspaceOwnershipV1(within,{kind:"all"},control(),async custody=>custody.scopes.length);
  expect(scopes).toBe(64);
 }finally{rmSync(over,{recursive:true,force:true});rmSync(within,{recursive:true,force:true});}
});
