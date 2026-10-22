import {expect,test} from "bun:test";
import {createGenerationFilesV1,generationRecheckWindowV1} from "../🟦️.ts";

const port=()=>({read:async(path:string)=>path,write:async()=>undefined,names:async()=>[],list:async()=>[],remove:async()=>undefined});
const store=(milliseconds:number)=>{const state={calls:0,active:0,peak:0};return{state,root:"/store",recheck:async()=>{state.calls++;state.active++;state.peak=Math.max(state.peak,state.active);await Bun.sleep(milliseconds);state.active--;}};};

/** 🧪️ Rechecks the store once per window of operations, one at a time, even when the recheck is slow and 32 readers run at once. */
test("a slow recheck is called once per window and never concurrently",async()=>{
 const sequential=store(1),files=createGenerationFilesV1({store:sequential,base:port(),requirePath:(_root,path)=>path});
 for(let index=0;index<generationRecheckWindowV1*3+5;index++)await files.read("/store/"+index);
 expect(sequential.state.calls).toBe(3);
 const slow=store(40),shared=createGenerationFilesV1({store:slow,base:port(),requirePath:(_root,path)=>path}),workers=32,each=200;
 await Promise.all(Array.from({length:workers},async()=>{for(let index=0;index<each;index++)await shared.read("/store/"+index);}));
 expect(slow.state.peak).toBe(1);expect(slow.state.calls).toBeGreaterThan(0);expect(slow.state.calls).toBeLessThanOrEqual(Math.ceil(workers*each/generationRecheckWindowV1));
});

test("a refused recheck refuses the operation that fell on the window and the next window checks again",async()=>{
 let calls=0;const files=createGenerationFilesV1({store:{root:"/store",recheck:async()=>{calls++;if(calls===1)throw Error("Original cache store identity changed");}},base:port(),requirePath:(_root,path)=>path,window:2});
 await files.read("a");await expect(files.read("b")).rejects.toThrow("identity changed");await files.read("c");await files.read("d");
 expect(calls).toBe(2);
});

test("every path is admitted against the store root before the port sees it",async()=>{
 const seen:string[]=[],files=createGenerationFilesV1({store:store(0),base:{...port(),read:async(path:string)=>{seen.push(path);return path;}},requirePath:(root,path)=>{if(!path.startsWith(root))throw Error("escapes");return path;}});
 await files.read("/store/x");await expect(files.read("/elsewhere/x")).rejects.toThrow("escapes");expect(seen).toEqual(["/store/x"]);
});
