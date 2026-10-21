/** 🪟️ Contract: optional graph retirement holds the exclusive image custody only for a bounded window; the work continues in further windows and the image is shared in between, so a graph writer never waits behind one retirement for its whole length. */
import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import schema from "../../🧬️schema/🔣️.json";

const load=async()=>{
 const ts=require("typescript"),text=await readFile(new URL("../../🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("storage.mjs",text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const pick=(name:string)=>tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text===name).getText(tree).replace(/^export /u,"");
 return pick("retireOptionalGraphHeadroomV1")+";"+pick("prepareBuildGraphOptionalHeadroomV1")+";return prepareBuildGraphOptionalHeadroomV1;";
};

const model=async(files:number,windowMilliseconds:number,clockStep:number)=>{
 const code=await load(),policy={...schema.definitions.OptionalQuota.policy,retirementWindowMilliseconds:windowMilliseconds},events:string[]=[];
 let held=false,now=0,remaining=files,windows=0,exclusiveGap=0;
 const entries=Array.from({length:files},(_,i)=>({path:"/graph/inference/np/"+String(i).padStart(8,"0"),bytes:1048576,modified:i,device:1,inode:i+1}));
 const control={publish:async()=>{events.push("progress");},yieldContinuation:async()=>{events.push(held?"yield-held":"yield-free");}};
 const image={root:"/graph",recheck:async()=>{},withExclusiveMaintenance:async(request:unknown,body:Function)=>{expect(request).toEqual({version:1,wait:false});windows++;held=true;try{return{acquired:true,value:await body(undefined,{kind:"mock"})};}finally{held=false;events.push("release");}}};
 const ports={Date:{now:()=>now+=clockStep},validateJsonSchemaSubset:()=>[],quotaSchema:{policy,definitions:{Headroom:{}}},schema,checkResourceLeaseControl:()=>{},optionalInventoryV1:async()=>{expect(held).toBe(true);return entries.slice(files-remaining);},nativeStatfsSyncV1:()=>({bsize:1,bavail:(files-remaining)*1048576}),lstat:async(path:string)=>({isFile:()=>true,isSymbolicLink:()=>false,nlink:1,dev:1,ino:Number(path.slice(-8))+1,size:1048576,blocks:2048}),requireNativeStoragePathV1:(_root:string,path:string)=>path,unlink:async()=>{expect(held).toBe(true);remaining--;}};
 const run=new Function(...Object.keys(ports),code)(...Object.values(ports));
 const result=await run(image,control,{minimumAvailableBytes:policy.minimumAvailableBytes});
 return{result,windows,remaining,events,held,exclusiveGap};
};

test("retirement is split into bounded exclusive windows and finishes the work",async()=>{
 const slow=await model(2000,50,10);
 expect(slow.windows).toBeGreaterThan(1);expect(slow.result).toBe(true);expect(slow.held).toBe(false);
 expect(slow.events.filter(event=>event==="release").length).toBe(slow.windows);expect(slow.events.filter(event=>event==="yield-free").length).toBe(slow.windows-1);
 const quick=await model(2000,1000000,1);
 expect(quick.windows).toBe(1);expect(quick.result).toBe(true);
});

test("the declared window is a finite policy value",()=>{
 expect(Number.isSafeInteger(schema.definitions.OptionalQuota.policy.retirementWindowMilliseconds)).toBe(true);
 expect(schema.definitions.OptionalQuota.policy.retirementWindowMilliseconds).toBeGreaterThan(0);
 expect(schema.definitions.OptionalQuota.policy.retirementWindowMilliseconds).toBeLessThanOrEqual(60000);
});
