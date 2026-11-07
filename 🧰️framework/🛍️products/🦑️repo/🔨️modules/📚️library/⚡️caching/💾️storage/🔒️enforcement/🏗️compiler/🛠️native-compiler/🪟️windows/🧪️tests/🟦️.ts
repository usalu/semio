import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{withBuildTestInvocationV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withRepositoryWindowsCompilerV1}from"../🟦️.ts";
import{withWindowsCompilerLifetimeV1}from"../🧬️lifetime/🟦️.ts";
import lifetime from"./🧫️lifetime.json";

test("a fabricated tooling store cannot issue Windows compiler readiness",()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:windows-compiler-forged-store",10000),db=new Database(":memory:");let called=false;try{const oracle=db.query("select 0 accepted").get() as{accepted:number};expect(oracle.accepted).toBe(0);await expect(Promise.resolve().then(()=>withRepositoryWindowsCompilerV1(process.cwd(),{} as never,control,{},process.env,null,async()=>{called=true;}))).rejects.toThrow();expect(called).toBe(false);}finally{db.close();}
}));

test("provider verification loans drain and cannot escape their lifetime",()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:windows-compiler-lifetime",30000),db=new Database(":memory:");try{for(const row of lifetime.cases){let count=0,drained=false,escaped:(()=>Promise<void>)|undefined;await withWindowsCompilerLifetimeV1(control,async()=>{count++;if(row.pending&&count===2){await new Promise<void>(done=>setTimeout(done,5));drained=true;}},async recheck=>{escaped=recheck;if(row.pending)void recheck();});expect(count,row.name).toBe(row.verifications);expect((db.query("select case when ?1 then 3 else 2 end n").get(row.pending)as{n:number}).n).toBe(row.verifications);if(row.pending)expect(drained).toBe(true);await expect(escaped!()).rejects.toThrow();}}finally{db.close();}
}));
