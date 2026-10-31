import{physicalSourceOriginV1}from"../../../🔏️revision/🟨️.mjs";
/** 🪟️ Atomic optional groups finish inside one genuine owner callback; Native receiver lifetime is never resumed after release. */
import {expect,test} from "bun:test";
import {readFile} from "node:fs/promises";
import schema from "../../🧬️schema/🔣️.json";

const load=async()=>{
 const ts=require("typescript"),text=await readFile(new URL("../../🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8"),tree=ts.createSourceFile("storage.mjs",text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const pick=(name:string)=>tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text===name).getText(tree).replace(/^export /u,"");
 return pick("retireOptionalGraphHeadroomV1")+";"+pick("prepareBuildGraphOptionalHeadroomV1")+";return prepareBuildGraphOptionalHeadroomV1;";
};

const model=async(files:number,partials=0)=>{
 const code=await load(),policy=schema.definitions.OptionalQuota.definitions.DefaultPolicy.const,events:string[]=[];
 let held=false,remaining=files,windows=0;
 const entries=Array.from({length:files},(_,i)=>({path:"/graph/inference/np/"+String(i).padStart(8,"0"),bytes:1048576,modified:i,device:1,inode:i+1}));
 const control={publish:async()=>{events.push("progress");},yieldContinuation:async()=>{events.push(held?"yield-held":"yield-free");}};
 const image={root:"/graph",recheck:async()=>{},withExclusiveMaintenance:async(request:unknown,body:Function)=>{expect(request).toEqual({version:1,wait:false});windows++;held=true;try{return{acquired:true,value:await body(undefined,{kind:"mock"})};}finally{held=false;events.push("release");}}};
 const ports={requireNativePlatformImageWriterV1:(actual:unknown)=>{expect(actual).toBe(image);expect(held).toBe(true);},repositoryCachePartitionNativeImageV1:(actual:unknown)=>actual,retireBuildGraphOptionalGroupsHeadroomV1:async(actual:unknown)=>{expect(actual).toBe(image);expect(held).toBe(true);if(windows<=partials){remaining=Math.max(0,remaining-Math.ceil(files/(partials+1)));return{retiredBytes:1048576,retiredFiles:1,partial:true};}remaining=0;return{retiredBytes:files*1048576,retiredFiles:files};},validateJsonSchemaSubset:()=>[],quotaSchema:{definitions:{DefaultPolicy:{const:policy},Headroom:{}}},schema,checkResourceLeaseControl:()=>{},optionalInventoryV1:async()=>{expect(held).toBe(true);return entries.slice(files-remaining);},nativeStatfsSyncV1:()=>({bsize:1,bavail:(files-remaining)*1048576}),lstat:async(path:string)=>({isFile:()=>true,isSymbolicLink:()=>false,nlink:1,dev:1,ino:Number(path.slice(-8))+1,size:1048576,blocks:2048}),requireNativeStoragePathV1:(_root:string,path:string)=>path,unlink:async()=>{expect(held).toBe(true);remaining--;}};
 const run=new Function(...Object.keys(ports),code)(...Object.values(ports));
 const result=await run(image,control,{minimumAvailableBytes:policy.minimumAvailableBytes});
 return{result,windows,remaining,events,held};
};

test("atomic retirement completes its group within one owned exclusive callback",async()=>{
 const slow=await model(2000);
 expect(slow.windows).toBe(1);expect(slow.result).toBe(true);expect(slow.held).toBe(false);
 expect(slow.events.filter(event=>event==="release").length).toBe(slow.windows);expect(slow.events.filter(event=>event==="yield-free").length).toBe(slow.windows-1);
 const quick=await model(1024);
 expect(quick.windows).toBe(1);expect(quick.result).toBe(true);
},60000);

test("the declared window is a finite policy value",()=>{
 expect(Number.isSafeInteger(schema.definitions.OptionalQuota.definitions.DefaultPolicy.const.retirementWindowMilliseconds)).toBe(true);
 expect(schema.definitions.OptionalQuota.definitions.DefaultPolicy.const.retirementWindowMilliseconds).toBeGreaterThan(0);
 expect(schema.definitions.OptionalQuota.definitions.DefaultPolicy.const.retirementWindowMilliseconds).toBeLessThanOrEqual(60000);
});

test("a partial retirement defers after one consumed image window",async()=>{
 const windowed=await model(2000,1);
 expect(windowed.windows).toBe(1);expect(windowed.result).toBe(false);expect(windowed.held).toBe(false);
 expect(windowed.events.filter(event=>event==="release").length).toBe(1);
 expect(windowed.events.includes("yield-held")).toBe(false);
 expect(windowed.remaining).toBeGreaterThan(0);
});

test("the writer's maintenance reopens the graph image between windows instead of holding it",async()=>{
 const text=await readFile(new URL("../../🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8");
 expect(text).toContain('const maintain=async force=>{for(let again=true;again;force=false){again=false;const image=await openRepositoryCachePartitionV1(workspace,"graph",leasePhase);');
 expect(text).toContain("again=await maintainBuildGraphOptionalCachesV1(image,leasePhase)");
});
