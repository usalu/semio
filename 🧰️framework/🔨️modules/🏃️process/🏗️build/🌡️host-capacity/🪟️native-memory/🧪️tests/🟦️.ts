/** 🧪️ Contract: native Windows memory is read through a typed port; any other platform answers the typed unknown without loading FFI, every answer is shaped by the schema, and the structure layouts match the kernel32 headers. Oracle: Ajv, an independent validator, and a hand-built MEMORYSTATUSEX decoded through a fake kernel. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{WINDOWS_MEMORY_LAYOUT_V1 as L,readWindowsMemoryV1,readWindowsProcessesV1,systemWindowsMemoryPortV1,windowsMemoryUnknownV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";

const ajv=new Ajv({strict:true,allowUnionTypes:true});ajv.addSchema({...schema,$id:"native"});
const check=(name:string,value:unknown):boolean=>ajv.validate({$ref:"native#/definitions/"+name},value) as boolean;

test("a host that is not Windows answers the typed unknown in both languages and the schema agrees",()=>{
 for(const platform of["darwin","linux","freebsd"] as const){
  const port=systemWindowsMemoryPortV1(platform),memory=port.memory(),table=port.processes();
  expect(memory).toEqual(table);expect(memory.known).toBe(false);
  if(!memory.known){expect(memory.reason).toBe("platform");expect(memory.text.en.length).toBeGreaterThan(0);expect(memory.text.de).toContain("Windows");}
  expect(check("Unknown",memory)).toBe(true);expect(check("Answer",table)).toBe(true);
 }
});

test("the schema accepts good facts, tables and unknowns and rejects every broken field",()=>{
 const facts={known:true,totalBytes:1,availableBytes:1,commitLimitBytes:2,commitAvailableBytes:2,loadPercent:50};
 const table={known:true,unreadable:1,rows:[{pid:4,parent:0,name:"System",workingSetBytes:null,privateBytes:null},{pid:8,parent:4,name:"a.exe",workingSetBytes:10,privateBytes:5}]};
 const unknown={known:false,reason:"host-call",call:"CreateToolhelp32Snapshot",error:5,detail:"",text:{en:"x",de:"y"}};
 expect(check("Facts",facts)).toBe(true);expect(check("Table",table)).toBe(true);expect(check("Unknown",unknown)).toBe(true);
 for(const broken of[{...facts,loadPercent:101},{...facts,totalBytes:-1},{...facts,extra:1},{...facts,known:false}])expect(check("Facts",broken)).toBe(false);
 for(const broken of[{...table,unreadable:-1},{...table,rows:[{pid:1}]}])expect(check("Table",broken)).toBe(false);
 for(const broken of[{...unknown,reason:"other"},{...unknown,text:{en:"x"}},{...unknown,known:true}])expect(check("Unknown",broken)).toBe(false);
 expect(check("Unknown",windowsMemoryUnknownV1("host-call","OpenProcess",5,"denied"))).toBe(true);
 expect(windowsMemoryUnknownV1("host-call","OpenProcess",5).text.de).toBe("der Windows-Aufruf OpenProcess schlug fehl (Fehler 5)");
});

test("the structure layout constants match the kernel32 headers",()=>{
 expect(L.memoryStatus).toEqual({size:64,load:4,totalPhysical:8,availablePhysical:16,totalPageFile:24,availablePageFile:32});
 expect(L.processEntry.size).toBe(568);expect(L.processEntry.size%8).toBe(0);expect(L.processEntry.executableName+L.processEntry.executableNameCharacters*2).toBeLessThanOrEqual(L.processEntry.size);
 expect(L.memoryCounters).toEqual({size:80,workingSet:16,privateUsage:72});expect(L.snapshotProcesses).toBe(2);expect(L.queryLimitedInformation).toBe(0x1000);expect(BigInt(L.invalidHandle)).toBe(2n**64n-1n);
});

test("a fake kernel decodes the memory status and the process walk, counting unreadable processes and closing every handle",()=>{
 const closed:bigint[]=[];
 const memory=readWindowsMemoryV1({GlobalMemoryStatusEx:(buffer:Buffer)=>{const view=new DataView(buffer.buffer,buffer.byteOffset);expect(view.getUint32(0,true)).toBe(64);view.setUint32(4,48,true);view.setBigUint64(8,64n,true);view.setBigUint64(16,32n,true);view.setBigUint64(24,80n,true);view.setBigUint64(32,50n,true);return 1;}});
 expect(memory).toEqual({known:true,totalBytes:64,availableBytes:32,commitLimitBytes:80,commitAvailableBytes:50,loadPercent:48});
 expect(check("Facts",memory)).toBe(true);
 const procs=[{pid:4,parent:0,name:"System"},{pid:10,parent:4,name:"app.exe"}];let at=0;
 const put=(buffer:Buffer,index:number)=>{const view=new DataView(buffer.buffer,buffer.byteOffset),row=procs[index]!;view.setUint32(8,row.pid,true);view.setUint32(32,row.parent,true);[...row.name].forEach((char,offset)=>view.setUint16(44+offset*2,char.charCodeAt(0),true));view.setUint16(44+row.name.length*2,0,true);};
 const table=readWindowsProcessesV1({CreateToolhelp32Snapshot:()=>99n,Process32FirstW:(_h:bigint,buffer:Buffer)=>{at=0;put(buffer,0);return 1;},Process32NextW:(_h:bigint,buffer:Buffer)=>{at++;if(at>=procs.length)return 0;put(buffer,at);return 1;},OpenProcess:(_a:number,_i:number,pid:number)=>pid===4?0n:7n,K32GetProcessMemoryInfo:(_h:bigint,buffer:Buffer)=>{const view=new DataView(buffer.buffer,buffer.byteOffset);view.setBigUint64(16,4096n,true);view.setBigUint64(72,1024n,true);return 1;},CloseHandle:(handle:bigint)=>{closed.push(handle);return 1;},GetLastError:()=>18});
 expect(table).toEqual({known:true,unreadable:1,rows:[{pid:4,parent:0,name:"System",workingSetBytes:null,privateBytes:null},{pid:10,parent:4,name:"app.exe",workingSetBytes:4096,privateBytes:1024}]});
 expect(check("Table",table)).toBe(true);expect(closed).toEqual([7n,99n]);
 const refused=readWindowsProcessesV1({CreateToolhelp32Snapshot:()=>BigInt(L.invalidHandle),GetLastError:()=>5});
 expect(refused).toMatchObject({known:false,reason:"host-call",call:"CreateToolhelp32Snapshot",error:5});
});
