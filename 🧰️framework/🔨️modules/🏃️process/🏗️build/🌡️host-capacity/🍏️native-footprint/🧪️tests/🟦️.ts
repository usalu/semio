/** 🧪️ Contract: the macOS footprint is read natively through `proc_pid_rusage` (flavor 4, `ri_phys_footprint`), decoded exactly, refused processes answer null, and on a Mac the number equals the one `footprint(1)` prints for the same process in a fraction of the time. Oracle: Ajv for the fixtures, the `footprint` tool for the live read.
 * @see ./🧫️fixtures/🔣️.json
 * @see ../🧬️schema/🔣️.json */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{DARWIN_FOOTPRINT_LAYOUT_V1 as L,readDarwinFootprintV1,systemDarwinFootprintPortV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";

const ajv=new Ajv({strict:true,allowUnionTypes:true});ajv.addSchema({...schema,$id:"footprint"});
const children:{kill:()=>void}[]=[];afterAll(()=>{for(const child of children)child.kill();});

test("every reading of the fixture matches the schema and decodes to the stated footprint through a fake libproc",()=>{
 expect(fixture.readings.length).toBeGreaterThanOrEqual(5);
 for(const reading of fixture.readings){
  expect(ajv.validate({$ref:"footprint#/definitions/Reading"},reading),reading.name).toBe(true);
  const calls:number[][]=[],kernel={proc_pid_rusage:(pid:number,flavor:number,buffer:Buffer)=>{calls.push([pid,flavor,buffer.length]);const view=new DataView(buffer.buffer,buffer.byteOffset,buffer.byteLength);for(const[offset,value]of Object.entries(reading.counters))view.setBigUint64(Number(offset),BigInt(value),true);return reading.result;}};
  expect(readDarwinFootprintV1(kernel,4242),reading.name).toBe(reading.footprintBytes);
  expect(calls,reading.name).toEqual([[4242,L.flavor,L.size]]);
 }
});

test("a pid that is no process id is never asked of the kernel",()=>{
 const kernel={proc_pid_rusage:()=>{throw Error("no call expected");}};
 for(const pid of[0,-1,1.5,Number.NaN])expect(readDarwinFootprintV1(kernel,pid)).toBeNull();
});

test("the layout is the one of struct rusage_info_v4: two 64-bit counters inside the buffer, the footprint after the resident size",()=>{
 expect(L).toEqual({library:"/usr/lib/libproc.dylib",flavor:4,size:512,residentSize:64,physicalFootprint:72});
 expect(L.physicalFootprint+8).toBeLessThanOrEqual(296);expect(L.size).toBeGreaterThanOrEqual(296);expect(L.physicalFootprint).toBe(L.residentSize+8);
});

test("a host that is not macOS has no native port and the caller reads the footprint another way",()=>{
 for(const platform of["linux","win32","freebsd"] as const)expect(systemDarwinFootprintPortV1(platform)).toBeNull();
});

test("on macOS the native read equals footprint(1) for the same process, refuses a process that is gone and is cheap",async()=>{
 if(process.platform!=="darwin")return;
 const port=systemDarwinFootprintPortV1()!;expect(port).not.toBeNull();
 const child=Bun.spawn(["sleep","30"]);children.push(child);await Bun.sleep(100);
 const tool=Bun.spawnSync(["footprint","-f","bytes","--noCategories","-p",String(child.pid)]),match=/phys_footprint:\s+(\d+)\s+B/u.exec(new TextDecoder().decode(tool.stdout));
 expect(match).not.toBeNull();expect(port.footprint(child.pid)).toBe(Number(match![1]));
 expect(port.footprint(process.pid)).toBeGreaterThan(1_000_000);
 child.kill();await child.exited;expect(port.footprint(child.pid)).toBeNull();
 const started=performance.now();for(let round=0;round<200;round++)port.footprint(process.pid);
 expect((performance.now()-started)/200,"milliseconds per process").toBeLessThan(0.5);
});
