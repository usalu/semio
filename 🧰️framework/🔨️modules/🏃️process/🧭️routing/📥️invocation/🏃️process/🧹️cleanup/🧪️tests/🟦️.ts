import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{setTimeout as delay}from"node:timers/promises";
import corpus from"./🔣️.json";
import schema from"../../🧬️schema/🧹️cleanup.json";
import*as api from"../../🟦️.ts";
import{captureOwnedCommandV1}from"../../../../../🎛️owned-execution/🟦️.ts";

test("accepted cleanup funding distinguishes ordinary bodies from finite resource loans",async()=>{
 const database=new Database(":memory:");try{for(const row of corpus.cases){expect(new Ajv({strict:true,allowUnionTypes:true}).validate(schema.definitions.LifetimeCase,row)).toBe(true);const oracle=database.query("select ?1 is null or ?1>?2 accepted,case when ?1 is null then null when ?1>?2 then ?1-?2 end bodyMilliseconds,case when ?1 is null then ?2 when ?1>?2 then ?1 end cleanupMilliseconds").get(row.availableMilliseconds,row.reserveMilliseconds)as{accepted:number;bodyMilliseconds:number|null;cleanupMilliseconds:number|null};expect({...oracle,accepted:Boolean(oracle.accepted)}).toEqual({accepted:row.accepted,bodyMilliseconds:row.bodyMilliseconds,cleanupMilliseconds:row.cleanupMilliseconds});}
 await ordinary(async original=>{const control=api.createIssuedResourceControlV1(original,"cleanup:ordinary",0);await api.withAcceptedPhysicalCleanupV1(control,request(100),async(token,body)=>{expect(body.remainingMilliseconds()).toBe(null);const facts=api.readAcceptedPhysicalCleanupV1(token),{cleanupSignal,...funding}=facts;expect(new Ajv({strict:true,allowUnionTypes:true}).validate(schema.definitions.FundingFacts,funding)).toBe(true);expect(cleanupSignal.aborted).toBe(false);expect(facts.deadlineEpochMilliseconds).toBe(null);expect(facts.bodyDeadlineEpochMilliseconds).toBe(null);await delay(20);expect(body.remainingMilliseconds()).toBe(null);expect(()=>api.checkAcceptedPhysicalCleanupV1(token)).toThrow("body must stop");const begin=api.beginAcceptedPhysicalCleanupV1(token);expect(api.beginAcceptedPhysicalCleanupV1(token)).toBe(begin);await begin;const first=api.checkAcceptedPhysicalCleanupV1(token);expect(first).toBeGreaterThan(0);expect(first).toBeLessThanOrEqual(100);await delay(10);expect(api.checkAcceptedPhysicalCleanupV1(token)).toBeLessThan(first);expect(api.beginAcceptedPhysicalCleanupV1(token)).toBe(begin);expect(()=>api.requireIssuedResourceControlV1(body)).toThrow();});expect(original.control.remainingMilliseconds()).toBe(null);expect(original.control.signal.aborted).toBe(false);});
 }finally{database.close();}
});

test("ordinary cleanup drains accepted real children and retains finite descendant bounds",async()=>ordinary(async original=>{
 const control=api.createIssuedResourceControlV1(original,"cleanup:children",0);let child:Promise<string>|undefined,late:(()=>number)|undefined;
 await api.withAcceptedPhysicalCleanupV1(control,request(1000),async(token,body)=>{child=api.withIssuedResourceOperationV1(body,()=>captureOwnedCommandV1("node",["--eval","setTimeout(()=>console.log('accepted-child-complete'),10)"],process.cwd(),{control:body,env:api.scriptInvocationEnvironment(original,process.env),output:{maximumBytes:4096,maximumLines:8}}));void child.catch(()=>{});await api.beginAcceptedPhysicalCleanupV1(token);expect(await child).toContain("accepted-child-complete");expect(api.checkAcceptedPhysicalCleanupV1(token)).toBeGreaterThan(0);expect(()=>api.checkAcceptedPhysicalCleanupV1({...token})).toThrow();late=()=>api.checkAcceptedPhysicalCleanupV1(token);});expect(late).toBeDefined();expect(late!).toThrow();
 const narrowed=api.createIssuedResourceControlV1(original,"cleanup:finite-descendant",2000);await api.withAcceptedPhysicalCleanupV1(narrowed,request(100),async(token,body)=>{expect(body.remainingMilliseconds()).not.toBe(null);const facts=api.readAcceptedPhysicalCleanupV1(token);expect(facts.deadlineEpochMilliseconds).not.toBe(null);expect(facts.bodyDeadlineEpochMilliseconds).not.toBe(null);expect(facts.deadlineEpochMilliseconds!-facts.bodyDeadlineEpochMilliseconds!).toBe(100);await api.beginAcceptedPhysicalCleanupV1(token);expect(api.checkAcceptedPhysicalCleanupV1(token)).toBeLessThanOrEqual(2000);});
}));

test("ordinary cleanup expiry stops accepted body work without renewing its parent",async()=>{let settled=false,entered=false,parentActive=false;await expect(ordinary(async original=>{
 await expect(api.withAcceptedPhysicalCleanupV1(api.createIssuedResourceControlV1(original,"cleanup:expiry",0),request(20),async(token,body)=>{entered=true;const pending=api.withIssuedResourceOperationV1(body,async()=>{try{await delay(1000,undefined,{signal:body.signal});}finally{settled=true;}});void pending.catch(()=>{});await expect(api.beginAcceptedPhysicalCleanupV1(token)).rejects.toThrow();expect(settled).toBe(true);expect(()=>api.checkAcceptedPhysicalCleanupV1(token)).toThrow("cleanup deadline");})).rejects.toThrow();expect(entered).toBe(true);expect(settled).toBe(true);expect(original.control.signal.aborted).toBe(false);expect(original.control.remainingMilliseconds()).toBe(null);parentActive=true;
})).rejects.toThrow();expect(entered).toBe(true);expect(settled).toBe(true);expect(parentActive).toBe(true);});

test("ordinary cleanup preserves undefined and frozen body failures through bounded settlement",async()=>{
 for(const primary of[undefined,Object.freeze(Error("frozen physical primary"))]){let caught=false,value:unknown,parentActive=false;try{await ordinary(async original=>{try{return await api.withAcceptedPhysicalCleanupV1(api.createIssuedResourceControlV1(original,"cleanup:primary",0),request(100),async()=>{throw primary;});}finally{parentActive=!original.control.signal.aborted;}});}catch(error){caught=true;value=error;}expect(caught).toBe(true);expect(value).toBe(primary);expect(parentActive).toBe(true);}
});

test("ordinary parent cancellation retains accepted cleanup without reviving its Original",async()=>{
 const controller=new AbortController(),primary=Object.freeze(Error("ordinary body cancelled"));let cleanupCompleted=false,parentActive=false;
 await expect(ordinary(async original=>{try{await api.withScriptInvocationSignalV1(original,controller.signal,async alias=>api.withAcceptedPhysicalCleanupV1(api.createIssuedResourceControlV1(alias,"cleanup:cancel",0),request(100),async(token,body)=>{controller.abort(primary);expect(()=>api.requireIssuedResourceControlV1(body)).toThrow();await api.beginAcceptedPhysicalCleanupV1(token);expect(api.checkAcceptedPhysicalCleanupV1(token)).toBeGreaterThan(0);expect(api.readAcceptedPhysicalCleanupV1(token).cleanupSignal.aborted).toBe(false);cleanupCompleted=true;}));}finally{parentActive=!original.control.signal.aborted;}})).rejects.toBe(primary);
 expect(cleanupCompleted).toBe(true);expect(parentActive).toBe(true);
});

function request(reserveMilliseconds:number){return{version:1 as const,kind:"native-image"as const,resource:"native-fixed-image:portable-cleanup-law",reserveMilliseconds};}
function ordinary<T>(body:(original:api.IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{return api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"cleanup:ordinary-law",maximumElapsedMilliseconds:0},{fixture:"ordinary-cleanup"},Date.now()),body);}

test("canonical cleanup fixture includes ordinary preaccepted terminal reserves",async()=>{const{proveAcceptedPhysicalCleanupV1}=await import("../../🧪️tests/📥️canonical/🧹️cleanup/🟦️.ts");await proveAcceptedPhysicalCleanupV1();});
