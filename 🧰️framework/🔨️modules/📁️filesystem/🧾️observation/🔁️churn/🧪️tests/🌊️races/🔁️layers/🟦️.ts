/** 🔁️ Contract: a raw filesystem error is classified ONCE where it meets the churn policy: a transient race is retried by the layer that read, a persistent one only by the layers that take a new snapshot (never multiplied by the narrower ones), the error that finally escapes is the original object with its `code`, the process boundary turns it into a restart with a pause (EN/DE), a failed child's printed error is classified by the same table, and anything that is no race stands untouched. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {ConcurrentEditErrorV1,churnProtocolV1,concurrentEditOfV1,isConcurrentEditV1,retryOnConcurrentEditSyncV1,retryOnConcurrentEditV1,type ChurnRetryEventV1} from "../../../🟦️.ts";
import {failureChurnOfTailsV1,refusalClassOfTextV1} from "../../../🧾️refusals/🟦️.ts";
import {staleImplementationOfV1} from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import {runWithStaleReexecV1 as supervisor} from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🟨️.mjs";

const protocol=churnProtocolV1();
let sandbox="",saved:string|undefined;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"layers-")));mkdirSync(join(sandbox,"src"));writeFileSync(join(sandbox,"AGENTS.md"),"x");writeFileSync(join(sandbox,"package.json"),"{}");saved=process.env.SEMIO_WORKSPACE_ROOT;process.env.SEMIO_WORKSPACE_ROOT=sandbox;});
afterAll(()=>{if(saved===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=saved;rmSync(sandbox,{recursive:true,force:true});});
const vanished=(name:string):NodeJS.ErrnoException=>{try{readFileSync(join(sandbox,"src",name));}catch(error){return error as NodeJS.ErrnoException;}throw Error("A missing file was expected");};
const quiet={environment:{SEMIO_LANGUAGE:""},onRetry:()=>{},sleep:async()=>{},sleepSync:()=>{}};

test("a transient race (the file is back) is retried at once by the layer that read, announced in both languages",async()=>{
 const events:ChurnRetryEventV1[]=[],slept:number[]=[];let calls=0;
 const result=await retryOnConcurrentEditV1("read",async()=>{calls++;if(calls===1){const error=vanished("back.ts");writeFileSync(join(sandbox,"src","back.ts"),"here");throw error;}return readFileSync(join(sandbox,"src","back.ts"),"utf8");},{...quiet,onRetry:event=>events.push(event),sleep:async milliseconds=>{slept.push(milliseconds);}});
 expect(result).toBe("here");expect(calls).toBe(2);expect(slept).toEqual([protocol.profiles.read.backoffMs[0]]);
 expect(events.length).toBe(1);expect(events[0]!.reason).toBe("vanished");expect(events[0]!.path).toBe(join(sandbox,"src","back.ts"));
 expect(events[0]!.text).toContain("it vanished");expect(events[0]!.text).toContain("sie verschwand");
});

test("a persistent race is never retried by the read layer, by the snapshot layer in its declared attempts, and by the build layer around it: the layers do not multiply, the original error escapes",()=>{
 let calls=0,last:NodeJS.ErrnoException|undefined;const body=():never=>{calls++;last=vanished("gone.ts");throw last;};
 let caught:unknown;
 try{retryOnConcurrentEditSyncV1("read",body,quiet);}catch(error){caught=error;}
 expect(caught).toBe(last);expect(calls).toBe(1);
 calls=0;try{retryOnConcurrentEditSyncV1("snapshot",body,quiet);}catch(error){caught=error;}
 expect(caught).toBe(last);expect(calls).toBe(protocol.profiles.snapshot.maximumAttempts);
 calls=0;const slept:number[]=[];
 try{retryOnConcurrentEditSyncV1("build",()=>retryOnConcurrentEditSyncV1("snapshot",()=>retryOnConcurrentEditSyncV1("read",body,quiet),quiet),{...quiet,sleepSync:milliseconds=>{slept.push(milliseconds);}});}catch(error){caught=error;}
 expect(caught).toBe(last);expect((caught as NodeJS.ErrnoException).code).toBe("ENOENT");expect(caught).not.toBeInstanceOf(ConcurrentEditErrorV1);
 expect(calls).toBe(protocol.profiles.snapshot.maximumAttempts*protocol.profiles.build.maximumAttempts);
 expect(slept).toEqual([...protocol.profiles.build.backoffMs]);
 expect(concurrentEditOfV1(last)).toEqual({reason:"vanished",path:last!.path!,rank:protocol.profiles.build.rank});
 expect(concurrentEditOfV1(last,protocol.profiles.build.rank)).toBeUndefined();
 const fresh=vanished("fresh.ts");
 expect(concurrentEditOfV1(fresh,protocol.profiles.read.rank)).toBeUndefined();
 expect(concurrentEditOfV1(fresh,protocol.profiles.snapshot.rank)).toEqual({reason:"vanished",path:fresh.path!,rank:protocol.profiles.read.rank});
});

test("the verdict is made on the first sight of an error: the same error object keeps it when the file reappears later",()=>{
 const error=vanished("later.ts");
 expect(concurrentEditOfV1(error,protocol.profiles.snapshot.rank)?.reason).toBe("vanished");
 writeFileSync(join(sandbox,"src","later.ts"),"now here");
 expect(isConcurrentEditV1(error)).toBe(true);expect(concurrentEditOfV1(error,protocol.profiles.read.rank)).toBeUndefined();
 expect(concurrentEditOfV1(vanished("later2.ts"),protocol.profiles.snapshot.rank)).toBeDefined();
});

test("what is no race stands at once: outside the workspace, a write, another code, a plain error",()=>{
 const outside=(()=>{try{readFileSync("/this/path/does/not/exist.txt");}catch(error){return error as NodeJS.ErrnoException;}throw Error("x");})();
 const written=(()=>{try{renameSync(join(sandbox,"src","nope"),join(sandbox,"src","nope2"));}catch(error){return error as NodeJS.ErrnoException;}throw Error("x");})();
 for(const error of[outside,written,Object.assign(new Error("denied"),{code:"EACCES",path:join(sandbox,"src","a"),syscall:"open"}),new Error("plain"),Object.assign(new Error("no path"),{code:"ENOENT"})]){
  let calls=0;expect(()=>retryOnConcurrentEditSyncV1("snapshot",()=>{calls++;throw error;},quiet)).toThrow(error);expect(calls).toBe(1);expect(isConcurrentEditV1(error)).toBe(false);
 }
});

test("the process boundary reads a raw race as an edit: the stale-implementation finder, the restart with its pause and the notice in both languages, and the bound",async()=>{
 const error=Object.assign(new Error("ENOENT: no such file or directory, lstat"),{code:"ENOENT",syscall:"lstat",path:join(sandbox,"src","renamed.ts")});
 const found=staleImplementationOfV1(error);expect(found?.origin).toBe("edit");expect(found?.edit?.path).toBe(error.path);expect(found?.edit?.reason).toBe("vanished");
 expect(staleImplementationOfV1(new Error("wrapped",{cause:error}))?.origin).toBe("edit");
 expect(staleImplementationOfV1(Object.assign(new Error("denied"),{code:"EACCES",path:error.path}))).toBeUndefined();
 const written:string[]=[],slept:number[]=[],launched:string[][]=[];
 const outcome=await supervisor(async()=>{throw error;},{environment:{SEMIO_LANGUAGE:"",SEMIO_STALE_REEXEC:"0"},outermost:true,argv:["bun","script.ts","x"],execPath:"bun",execArgv:[],write:text=>{written.push(text);},sleep:async milliseconds=>{slept.push(milliseconds);},launch:async({args,environment})=>{launched.push([...args]);expect(environment.SEMIO_STALE_REEXEC).toBe("1");return 0;}});
 expect(outcome).toEqual({status:"reexecuted",code:0});expect(launched).toEqual([["script.ts","x"]]);expect(slept.length).toBe(1);
 expect(written.join("")).toContain("Another process kept editing");expect(written.join("")).toContain("Ein anderer Prozess");expect(written.join("")).toContain(error.path);
 const exhausted=await supervisor(async()=>{throw error;},{environment:{SEMIO_LANGUAGE:"en",SEMIO_STALE_REEXEC:String(6)},outermost:true,argv:["bun","script.ts"],execPath:"bun",execArgv:[],write:()=>{},sleep:async()=>{},launch:async()=>{throw Error("no more restarts");}});
 expect(exhausted.status).toBe("exhausted");
 const nested:string[]=[];
 expect(await supervisor(async()=>{throw error;},{environment:{SEMIO_LANGUAGE:"en"},outermost:false,write:text=>{nested.push(text);},sleep:async()=>{},launch:async()=>0})).toEqual({status:"yielded",code:75});
 expect(nested.join("")).toContain("ENOENT");
});

test("the printed error of a failed child is classified by the same table: a read of a vanished workspace path is a reobserve class, an outside path or a compiler error is not",()=>{
 const inside="ENOENT: no such file or directory, lstat '"+join(sandbox,"src","moved-away.ts")+"'";
 expect(refusalClassOfTextV1(inside)).toEqual({id:"filesystem-race",disposition:"reobserve",message:join(sandbox,"src","moved-away.ts")});
 expect(refusalClassOfTextV1("error TS2322: Type 'string' is not assignable\n"+inside)?.id).toBe("filesystem-race");
 expect(refusalClassOfTextV1("ENOENT: no such file or directory, lstat '/this/path/does/not/exist'")).toBeUndefined();
 expect(refusalClassOfTextV1("error TS2322: Type 'string' is not assignable to type 'number'")).toBeUndefined();
 const failure=failureChurnOfTailsV1([{task:"@a/x:build",text:"x".repeat(100)+"\n"+inside}]);
 expect(concurrentEditOfV1(failure,protocol.profiles.build.rank)?.reason).toBe("input");
 expect(concurrentEditOfV1(failure,protocol.profiles.snapshot.rank)).toBeUndefined();
});
