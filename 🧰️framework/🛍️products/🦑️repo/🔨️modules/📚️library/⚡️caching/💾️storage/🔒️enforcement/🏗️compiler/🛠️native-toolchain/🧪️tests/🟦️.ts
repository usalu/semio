import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import fixture from"../🧫️fixtures/🔣️.json" with{type:"json"};
import boundary from"../🧫️fixtures/🪢️boundary.json" with{type:"json"};
import{withRepositoryNativeToolchainV1,withNativeToolchainBoundaryV1}from"../🟦️.ts";

test("a fabricated caller cannot issue a native toolchain scope",async()=>{
 const db=new Database(":memory:");let called=false;try{const oracle=db.query("select 0 accepted").get() as{accepted:number};expect(Boolean(oracle.accepted)).toBe(fixture.cases[0]!.accepted);await expect(Promise.resolve().then(()=>withRepositoryNativeToolchainV1(process.cwd(),{} as never,{} as never,{},async()=>{called=true;}))).rejects.toThrow();expect(called).toBe(false);}finally{db.close();}
});

test("Rust boundary checks bracket both success and failure",async()=>{
 const db=new Database(":memory:");try{for(const row of boundary.cases){const trace:string[]=[];let checks=0,failed=false,aggregate=false;try{await withNativeToolchainBoundaryV1(async()=>{trace.push(++checks===1?"rust-before":"rust-after");if(checks===2&&row.afterFails)throw Error("recheck failed");},async()=>{trace.push("body");if(row.bodyFails)throw Error("body cancelled");});}catch(error){failed=true;aggregate=error instanceof AggregateError;}const oracle=db.query("select 'rust-before,body,rust-after' trace, (?1 or ?2) failed, (?1 and ?2) aggregate").get(row.bodyFails,row.afterFails) as{trace:string;failed:number;aggregate:number};expect(trace.join(",")).toBe(oracle.trace);expect(trace).toEqual(row.trace);expect(failed).toBe(row.fails);expect(failed).toBe(Boolean(oracle.failed));expect(aggregate).toBe(row.aggregate);expect(aggregate).toBe(Boolean(oracle.aggregate));}}finally{db.close();}
});
