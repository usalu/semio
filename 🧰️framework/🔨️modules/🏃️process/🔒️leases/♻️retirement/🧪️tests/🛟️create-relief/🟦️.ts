/** 🛟️ Contract: a lease store that is full is relieved before a create fails and once after it failed: idle lease databases and settled claims are retired, the create is tried again once, and a store that is still full is named with the directory in English and German. The disk is simulated through the open port of the lease module (a create fails with the open explanation of the SQLite module as soon as the directory holds four databases) and the free-space reader. */
import{test,expect}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync}from"node:fs";
import{isAbsolute,join}from"node:path";
import{acquireResourceLease,leaseStoreFullV1,leaseStoreNeedsReliefV1,relieveLeaseStoreV1,resourceLeasePathV1,withLeaseFreeSpaceReaderV1,type ResourceLeaseControl}from"../../../🟦️.ts";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{SqliteDatabaseV1,explainSqliteOpenFailureV1,type SqliteFilesystemV1}from"../../../../../⚙️runtime/🗃️sqlite/🟦️.ts";
import{resourceLeaseRetirementIdlePolicyV1,withResourceLeaseOpenPortV1}from"../../🟦️.ts";
declareBuildTestEnvelopeV1({owner:"test:leases",maximumElapsedMilliseconds:300000,capabilities:{artifacts:true}});
const policy=resourceLeaseRetirementIdlePolicyV1;
async function withScratch<T>(body:(scratch:string,original:Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0])=>Promise<T>):Promise<T>{return withBuildTestInvocationV1(async original=>{const capabilities=original.capabilities as{artifacts?:unknown};if(typeof capabilities.artifacts!=="string"||!isAbsolute(capabilities.artifacts))throw Error("Authored finite test output required");mkdirSync(capabilities.artifacts,{recursive:true});const scratch=mkdtempSync(join(capabilities.artifacts,"cr-"));try{return await body(scratch,original);}finally{rmSync(scratch,{recursive:true,force:true});}});}
const lease=(control:ResourceLeaseControl,directory:string,resource:string)=>acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control,timeoutMs:10000});
const diskOf=(errno:"EDQUOT"|"ENOSPC"):SqliteFilesystemV1=>({mkdirSync:()=>undefined,openSync:()=>{throw Object.assign(Error(errno),{code:errno});},writeSync:()=>0,closeSync:()=>undefined,unlinkSync:()=>undefined,lstatSync:()=>{throw Object.assign(Error("ENOENT"),{code:"ENOENT"});},statfsSync:()=>({bavail:0,bsize:4096,ffree:0})});
const databases=(directory:string)=>readdirSync(directory).filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name));
function fullDisk(directory:string,errno:"EDQUOT"|"ENOSPC",limit:number,seen:{failed:number;created:number}){
 return(path:string,options:Readonly<{readonly:boolean;readwrite:boolean;create:boolean}>)=>{
  if(options.create&&!existsSync(path)&&databases(directory).length>=limit){seen.failed++;throw explainSqliteOpenFailureV1(Object.assign(Error("unable to open database file"),{code:"SQLITE_CANTOPEN",errno:14,errcode:14}),path,diskOf(errno));}
  if(options.create&&!existsSync(path))seen.created++;
  return new SqliteDatabaseV1(path,{...options});
 };
}
async function idleStore(control:ResourceLeaseControl,directory:string,count:number){
 mkdirSync(directory,{recursive:true});
 const seed=await lease(control,directory,"relief-case:seed");seed.release();await relieveLeaseStoreV1(directory,control);
 for(let index=0;index<count;index++){const held=await lease(control,directory,"relief-case:"+index);held.release();const at=new Date(Date.now()-600000);utimesSync(resourceLeasePathV1(directory,"relief-case:"+index),at,at);}
}

test("a create that fails because the disk is full retires idle databases, tries once more and succeeds: the caller never sees the error",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:create-relief",300000),directory=join(scratch,"data"),seen={failed:0,created:0};
 await idleStore(control,directory,6);const permanent=databases(directory).length-6,limit=permanent+2;expect(permanent).toBeGreaterThanOrEqual(0);
 const held=await withResourceLeaseOpenPortV1(fullDisk(directory,"EDQUOT",limit,seen),()=>withLeaseFreeSpaceReaderV1(()=>({availableBytes:2**40,totalBytes:2**41}),()=>lease(control,directory,"relief-case:new")));
 try{expect(seen.failed,"the first create failed on the full disk").toBe(1);expect(seen.created,"and the retry created the database").toBe(1);expect(existsSync(resourceLeasePathV1(directory,"relief-case:new"))).toBe(true);expect(databases(directory).length,"idle databases were retired to make room").toBeLessThan(limit);}finally{held.release();}
}));

test("the relief runs before the create when the free space is low, so the create never fails; with enough free space nothing is retired",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:create-relief-early",300000),directory=join(scratch,"data"),seen={failed:0,created:0};
 await idleStore(control,directory,6);const permanent=databases(directory).length-6,limit=permanent+4;
 const roomy=await withLeaseFreeSpaceReaderV1(()=>({availableBytes:2**40,totalBytes:2**41}),()=>lease(control,directory,"relief-case:roomy"));roomy.release();
 expect(databases(directory).length,"no pressure: all six idle databases and the new one stay").toBe(permanent+7);
 const tight=await withResourceLeaseOpenPortV1(fullDisk(directory,"ENOSPC",limit,seen),()=>withLeaseFreeSpaceReaderV1(()=>({availableBytes:policy.createReserveBytes-1,totalBytes:2**41}),()=>lease(control,directory,"relief-case:tight")));
 try{expect(seen.failed,"the create found room because the relief ran first").toBe(0);expect(seen.created).toBe(1);expect(databases(directory).length).toBeLessThan(limit);}finally{tight.release();}
}));

test("a store that stays full after the relief fails once, after exactly one retry, naming the directory in English and German with the original code kept",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:create-relief-still",300000),directory=join(scratch,"data"),seen={failed:0,created:0};
 await idleStore(control,directory,3);const permanent=databases(directory).length-3;
 let caught:unknown;
 await withResourceLeaseOpenPortV1(fullDisk(directory,"EDQUOT",0,seen),()=>withLeaseFreeSpaceReaderV1(()=>({availableBytes:2**40,totalBytes:2**41}),async()=>{try{(await lease(control,directory,"relief-case:never")).release();}catch(error){caught=error;}}));
 const error=caught as Error&{space?:boolean;code?:string;probe?:string;relief?:{databases:number}};
 expect(error).toBeInstanceOf(Error);expect(seen.failed,"one failure and one retry").toBe(2);
 expect(error.message.startsWith(`No space left (EDQUOT) in ${directory}:`)).toBe(true);expect(error.message).toContain("Kein Platz mehr (EDQUOT)");
 expect(error.message).toContain("Retired 3 idle lease databases");expect(error.message).toContain("weiterhin voll");expect(error.message).toContain("tried the create again");
 expect(error.space).toBe(true);expect(error.code).toBe("SQLITE_CANTOPEN");expect(error.probe).toBe("EDQUOT");expect(leaseStoreFullV1(error)).toBe(true);expect(databases(directory).length).toBe(permanent);
}));

test("a failure that is not a full disk is not retried and not relieved",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:create-relief-other",300000),directory=join(scratch,"data");
 await idleStore(control,directory,3);const permanent=databases(directory).length-3;let attempts=0,caught:unknown;
 const port=(path:string,options:Readonly<{readonly:boolean;readwrite:boolean;create:boolean}>)=>{if(options.create&&!existsSync(path)){attempts++;throw Object.assign(Error("unable to open database file"),{code:"SQLITE_CANTOPEN",errno:14});}return new SqliteDatabaseV1(path,{...options});};
 await withResourceLeaseOpenPortV1(port,()=>withLeaseFreeSpaceReaderV1(()=>({availableBytes:2**40,totalBytes:2**41}),async()=>{try{(await lease(control,directory,"relief-case:other")).release();}catch(error){caught=error;}}));
 expect((caught as Error).message).toContain("unable to open database file");expect(attempts).toBe(1);expect(databases(directory).length,"nothing retired").toBe(permanent+3);
}));

test("the relief retires settled claims and idle databases directly, and the classifier and the pressure rule are exact",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:create-relief-direct",300000),directory=join(scratch,"data");
 await idleStore(control,directory,5);const permanent=databases(directory).length-5;
 const relief=await relieveLeaseStoreV1(directory,control);expect(relief.databases).toBe(5);expect(databases(directory).length).toBe(permanent);
 for(const error of[{space:true},{probe:"EDQUOT"},{probe:"ENOSPC"},{code:"ENOSPC"},{code:"EDQUOT"},{code:"SQLITE_FULL"},{errcode:13},{errcode:269}])expect(leaseStoreFullV1(error),JSON.stringify(error)).toBe(true);
 for(const error of[{probe:"ok"},{code:"SQLITE_BUSY"},{errcode:14},new Error("x"),null,undefined,"ENOSPC"])expect(leaseStoreFullV1(error),String(error)).toBe(false);
 const rule=(availableBytes:number|undefined,totalBytes:number|undefined)=>leaseStoreNeedsReliefV1(availableBytes===undefined?undefined:{availableBytes,totalBytes});
 expect(rule(undefined,undefined)).toBe(true);expect(rule(policy.createReserveBytes-1,undefined)).toBe(true);expect(rule(policy.createReserveBytes,undefined)).toBe(false);
 expect(rule(100<<20,1000<<20),"a low share of a large volume is the business of the sweep clock, not of the create").toBe(false);
}));
