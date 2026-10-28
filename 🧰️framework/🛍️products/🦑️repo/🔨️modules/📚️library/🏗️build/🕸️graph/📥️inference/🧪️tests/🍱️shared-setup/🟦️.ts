/** 🍱️ Contract: the whole-workspace setup of an inference is computed once per share. A later call of the same command reuses it only while every observation it was made from still holds, replays exactly those observations into its own capture (so a project record made from it names the same facts), recomputes it when one of them moved or when the key differs, and leaves an unshared call exactly as it was. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const nodes=await import("../../📦️project-nodes/🟨️.mjs") as any,observation=await import("../../../../👁️observation/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
const scratch=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"shared-setup-")));roots.push(root);return root;};
const rowsOf=(rows:readonly {kind:string;path:string;fact:unknown}[])=>rows.map(row=>JSON.stringify([row.kind,row.path,row.fact])).sort();
const newShare=()=>({cache:new Map<string,unknown>(),computed:0,reused:0});
const capture=(run:()=>Promise<unknown>)=>observation.observeInferenceSnapshotV1(()=>observation.captureObservedSourceInputsV1(run,undefined,false)) as Promise<{value:any;observations:{kind:string;path:string;fact:unknown}[]}>;
const setupOf=(file:string,read=(path:string)=>observation.observedFs.readFileSync(path,"utf8") as string)=>async()=>({text:read(file),exists:observation.observedFs.existsSync(file)});

test("a setup is computed once per share, then served with the very observations it was made from",async()=>{
 const root=scratch(),file=join(root,"Cargo.toml");writeFileSync(file,"[package]\nname = \"a\"\n");
 const share=newShare(),checkpoints:boolean[]=[],checkpoint=async(force=false)=>{checkpoints.push(force);};
 const first=await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file),checkpoint));
 expect({computed:share.computed,reused:share.reused}).toEqual({computed:1,reused:0});
 const second=await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file),checkpoint));
 expect({computed:share.computed,reused:share.reused}).toEqual({computed:1,reused:1});
 expect(second.value).toBe(first.value);
 expect(rowsOf(second.observations)).toEqual(rowsOf(first.observations));
 const independent=await capture(setupOf(file));
 expect(rowsOf(second.observations)).toEqual(rowsOf(independent.observations));
 expect(first.observations.length).toBeGreaterThan(0);
});

test("a setup whose observation moved is computed again, and the new value is the one served afterwards",async()=>{
 const root=scratch(),file=join(root,"Cargo.toml");writeFileSync(file,"one\n");
 const share=newShare(),checkpoint=async()=>{};
 await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file),checkpoint));
 await sleep(30);writeFileSync(file,"two\n");
 const moved=await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file),checkpoint));
 expect(moved.value.text).toBe("two\n");expect({computed:share.computed,reused:share.reused}).toEqual({computed:2,reused:0});
 const again=await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file),checkpoint));
 expect(again.value).toBe(moved.value);expect({computed:share.computed,reused:share.reused}).toEqual({computed:2,reused:1});
});

test("a removed file moves the setup too, and another key never shares",async()=>{
 const root=scratch(),file=join(root,"Cargo.toml");writeFileSync(file,"x\n");
 const share=newShare(),checkpoint=async()=>{},read=(path:string)=>{try{return observation.observedFs.readFileSync(path,"utf8") as string;}catch{return "missing";}};
 await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file,read),checkpoint));
 await capture(()=>nodes.sharedSetupV1(share,"other",setupOf(file,read),checkpoint));
 expect({computed:share.computed,reused:share.reused,keys:share.cache.size}).toEqual({computed:2,reused:0,keys:2});
 await sleep(30);rmSync(file);
 const gone=await capture(()=>nodes.sharedSetupV1(share,"key",setupOf(file,read),checkpoint));
 expect(gone.value).toEqual({text:"missing",exists:false});expect(share.computed).toBe(3);
});

test("a failing setup is not remembered",async()=>{
 const share=newShare(),checkpoint=async()=>{};
 await expect(capture(()=>nodes.sharedSetupV1(share,"key",async()=>{throw new Error("unreadable");},checkpoint))).rejects.toThrow("unreadable");
 const ok=await capture(()=>nodes.sharedSetupV1(share,"key",async()=>({fine:true}),checkpoint));
 expect(ok.value).toEqual({fine:true});expect(share.computed).toBe(2);expect(share.reused).toBe(0);
});
