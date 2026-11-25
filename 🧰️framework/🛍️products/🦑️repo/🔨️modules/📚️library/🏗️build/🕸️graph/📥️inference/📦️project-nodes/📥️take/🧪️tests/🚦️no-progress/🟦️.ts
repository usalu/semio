/** 🚦️ Contract: a read that makes no owner progress for the declared time stops with a plain text in English and German that names what it waits on, and a large machine is recognised by the memory and cores it declares.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,readdirSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../../../../🐢️speed/🟨️.mjs";
import{TAKE_POLICY_V1,createCpuProgressV1,takeChildrenCpuV1,takeOwnCpuV1,takeWorkCpuV1,takeCpuMillisecondsV1,takeTreeCpuMillisecondsV1,createProgressWatchV1,takeNoProgressTextV1,takeStrongMachineV1,takeLoadStretchV1,takeStopVerdictV1,takeQuarantineReadV1,takeQuarantineWriteV1,takeNoProgressWindowV1}from"../../🟨️.mjs";

test("the policy declares five minutes without owner progress",()=>{
 expect(TAKE_POLICY_V1.noProgressMilliseconds).toBe(300000);
 expect(TAKE_POLICY_V1.watchMilliseconds).toBeGreaterThan(0);
 expect(TAKE_POLICY_V1.watchMilliseconds).toBeLessThanOrEqual(TAKE_POLICY_V1.noProgressMilliseconds/10);
});

test("a sign that never changes stalls the watch after the declared time and not before",()=>{
 let at=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000});
 expect(watch.check(7)).toBeUndefined();
 at=299999;expect(watch.check(7)).toBeUndefined();
 at=300000;expect(watch.check(7)).toEqual({sinceMilliseconds:300000});
});

test("every change of the sign restarts the time",()=>{
 let at=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000});
 watch.check(1);
 for(let step=1;step<=20;step++){at=step*200000;expect(watch.check(step+1)).toBeUndefined();}
 at+=300000;expect(watch.check(21)).toEqual({sinceMilliseconds:300000});
});

test("the text names the owners that run, the machine that holds them and what the read waits on, in both languages",()=>{
 const fields={minutes:5,done:22,total:4775,running:"procedural-generation2d-rs (2325 s, 0 rows), mit-bestand-demonstrator (828 s, 0 rows)",waiting:"memory headroom (21842 of 14336 MB reserved)"};
 const en=takeNoProgressTextV1("en",fields),de=takeNoProgressTextV1("de",fields);
 for(const text of[en,de]){expect(text).toContain("procedural-generation2d-rs");expect(text).toContain("memory headroom");expect(text).toContain("22");expect(text).toContain("4775");expect(text).not.toContain("{");}
 expect(en).toContain("no owner");
 expect(de).toContain("kein Projekt");
});

test("a machine is large when it has the declared memory and cores, and only then",()=>{
 expect(takeStrongMachineV1({memoryMegabytes:32768,cores:10})).toBe(true);
 expect(takeStrongMachineV1({memoryMegabytes:16384,cores:10})).toBe(false);
 expect(takeStrongMachineV1({memoryMegabytes:32768,cores:4})).toBe(false);
});

test("owners that run and use CPU are progress, owners that use none are not, and an unreadable sample is no progress",async()=>{
 let at=0,used:number|undefined=0;
 const cpu=createCpuProgressV1({sample:()=>used,now:()=>at,policy:{cpuProgressPercent:10}});
 expect(await cpu.tick()).toBe(0);
 at=10000;used=5000;expect(await cpu.tick()).toBe(1);
 at=20000;used=5500;expect(await cpu.tick()).toBe(1);
 at=30000;used=undefined;expect(await cpu.tick()).toBe(1);
 at=40000;used=9000;expect(await cpu.tick()).toBe(2);
});

test("a slow owner that burns CPU never stalls the watch while a hung one does",async()=>{
 let at=0,used=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000}),cpu=createCpuProgressV1({sample:()=>used,now:()=>at});
 for(let step=1;step<=60;step++){at=step*10000;used+=8000;expect(watch.check(["rows0",await cpu.tick()].join("|"))).toBeUndefined();}
 let hit;
 for(let step=1;step<=31&&hit===undefined;step++){at+=10000;hit=watch.check(["rows0",await cpu.tick()].join("|"));}
 expect(hit).toEqual({sinceMilliseconds:300000});
});

test("the CPU of the children is the sum below the root, parsed from the time columns of the process table",()=>{
 expect(takeCpuMillisecondsV1("0:12.34")).toBe(12340);
 expect(takeCpuMillisecondsV1("01:02:03")).toBe(3723000);
 expect(takeCpuMillisecondsV1("2-00:00:01")).toBe(172801000);
 const rows=[{pid:1,parent:0,milliseconds:9},{pid:2,parent:1,milliseconds:100},{pid:3,parent:2,milliseconds:50},{pid:4,parent:9,milliseconds:7}];
 expect(takeTreeCpuMillisecondsV1(rows,1)).toBe(150);
});

test("a real child that burns CPU raises the sample of the children and one that sleeps does not",async()=>{
 const{spawn}=await import("node:child_process"),busy=spawn(process.execPath,["-e","while(process.cpuUsage().user+process.cpuUsage().system<800000);setTimeout(()=>{},30000);"],{stdio:"ignore"}),idle=spawn(process.execPath,["-e","setTimeout(()=>{},30000);"],{stdio:"ignore"});
 try{
  const first=await takeChildrenCpuV1(),deadline=Date.now()+25000;
  let second=first;
  while(Date.now()<deadline&&second!-first!<500){await new Promise(resolve=>setTimeout(resolve,250));second=await takeChildrenCpuV1();}
  expect(first).toBeDefined();expect(second!-first!).toBeGreaterThanOrEqual(500);
 }finally{busy.kill();idle.kill();}
},40000);

test("the default window is the authored window times the speed factor of this machine and an explicit window is used as given",()=>{
 const factor=localMachineSpeedFactorV1(),expected=scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds,factor);
 let clock=0;const watch=createProgressWatchV1({now:()=>clock});watch.check(1);
 clock=expected-1;expect(watch.check(1)).toBeUndefined();clock=expected;expect(watch.check(1)).toEqual({sinceMilliseconds:expected});
 let other=0;const given=createProgressWatchV1({now:()=>other,windowMilliseconds:1000});given.check(1);other=999;expect(given.check(1)).toBeUndefined();other=1000;expect(given.check(1)).toBeDefined();
 expect(scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds,2.83)).toBe(849000);
});

test("the CPU of this process is user plus system time in milliseconds and the work of a read adds it to its children",async()=>{
 expect(takeOwnCpuV1({user:1500000,system:500000})).toBe(2000);
 const table=" 1 0 0:01.00\n 2 1 0:02.50\n";
 const run=(_command:string,_args:string[],_options:unknown,done:(error:Error|null,out:string)=>void)=>done(null,table);
 const work=await takeWorkCpuV1(1,run,{user:3000000,system:1000000});
 expect(work).toBe(2500+4000);
});

test("the CPU of a thread of this process that burns is counted, one that sleeps adds none, and the operating system process table agrees",async()=>{
 const start=takeOwnCpuV1(),deadline=Date.now()+20000;
 while(takeOwnCpuV1()-start<300&&Date.now()<deadline);
 const burned=takeOwnCpuV1()-start;
 expect(burned).toBeGreaterThanOrEqual(300);
 const idle=takeOwnCpuV1();
 await new Promise(resolve=>setTimeout(resolve,400));
 expect(takeOwnCpuV1()-idle).toBeLessThan(200);
 if(process.platform!=="win32"){
  const{execFileSync}=await import("node:child_process"),table=takeCpuMillisecondsV1(execFileSync("ps",["-o","time=","-p",String(process.pid)],{encoding:"utf8"}).trim());
  expect(Math.abs(table-takeOwnCpuV1())).toBeLessThan(250);
 }
},30000);

test("an owner inferred in a busy parent never stalls the watch while a parent that burns nothing does",async()=>{
 let at=0,used=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000}),cpu=createCpuProgressV1({sample:()=>used,now:()=>at});
 for(let step=1;step<=90;step++){at=step*10000;used+=7000;expect(watch.check(["rows0",await cpu.tick()].join("|"))).toBeUndefined();}
 let hit;
 for(let step=1;step<=31&&hit===undefined;step++){at+=10000;used+=100;hit=watch.check(["rows0",await cpu.tick()].join("|"));}
 expect(hit).toEqual({sinceMilliseconds:300000});
});

test("the window stretches with the load per core up to the declared maximum and never shrinks below one",()=>{
 expect(takeLoadStretchV1(0.5,10)).toBe(1);
 expect(takeLoadStretchV1(10,10)).toBe(1);
 expect(takeLoadStretchV1(85,10)).toBe(8.5);
 expect(takeLoadStretchV1(108,10)).toBe(10.8);
 expect(takeLoadStretchV1(5000,10)).toBe(TAKE_POLICY_V1.loadStretchMaximum);
 expect(takeLoadStretchV1(Number.NaN,10)).toBe(1);
 expect(TAKE_POLICY_V1.noProgressMilliseconds*TAKE_POLICY_V1.loadStretchMaximum).toBeLessThanOrEqual(TAKE_POLICY_V1.noProgressCeilingMilliseconds);
});

test("a silent read stops only when it is stuck: no CPU use on an idle host, or at the hard ceiling",()=>{
 const window=300000,at=(silent:number,load:number,cpu?:number)=>takeStopVerdictV1({silentMilliseconds:silent,windowMilliseconds:window,load,cores:10,cpuDeltaMilliseconds:cpu});
 expect(at(299999,2,0)).toEqual({stop:false,reason:"stretched"});
 expect(at(300000,2,0)).toEqual({stop:true,reason:"stuck"});
 expect(at(300000,2,5000)).toEqual({stop:false,reason:"alive"});
 expect(at(300000,2,undefined)).toEqual({stop:true,reason:"stuck"});
 expect(at(1200000,100,0)).toEqual({stop:false,reason:"stretched"});
 expect(at(3000000,100,0)).toEqual({stop:false,reason:"starved"});
 expect(at(3000000,100,2000)).toEqual({stop:false,reason:"alive"});
 expect(at(TAKE_POLICY_V1.noProgressCeilingMilliseconds,100,9000)).toEqual({stop:true,reason:"ceiling"});
});

test("the CPU threshold of a starved host stretches so a process that gets a tenth of its share still counts as alive",async()=>{
 let at=0,used=0;
 const plain=createCpuProgressV1({sample:()=>used,now:()=>at}),stretched=createCpuProgressV1({sample:()=>used,now:()=>at,stretch:()=>10});
 at=10000;used=200;
 expect(await plain.tick()).toBe(0);expect(await stretched.tick()).toBe(0);
 at=20000;used=400;
 expect(await plain.tick()).toBe(0);expect(plain.delta()).toBe(200);
 expect(await stretched.tick()).toBe(1);
});

test("an unreadable process table leaves the own CPU of the read in the sample",async()=>{
 const failing=(_command:string,_args:string[],_options:unknown,done:(error:Error|null,out:string)=>void)=>done(new Error("ps"),"");
 expect(await takeWorkCpuV1(1,failing,{user:3000000,system:1000000})).toBe(4000);
});

test("the window without progress is the authored window times the speed factor",()=>{
 expect(takeNoProgressWindowV1()).toBe(scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds,localMachineSpeedFactorV1()));
});

test("a quarantined owner is written atomically, expires after the declared time and is replaced by name",()=>{
 const folder=mkdtempSync(join(tmpdir(),"quarantine-")),path=join(folder,"deep","owner-quarantine.json");
 try{
  expect(takeQuarantineReadV1(path)).toEqual([]);
  expect(takeQuarantineWriteV1(path,["a","b"],"stuck",1000)).toBe(true);
  expect(takeQuarantineReadV1(path,2000)).toEqual(["a","b"]);
  expect(takeQuarantineWriteV1(path,["b","c"],"ceiling",3000)).toBe(true);
  expect(takeQuarantineReadV1(path,4000)).toEqual(["a","b","c"]);
  expect(JSON.parse(readFileSync(path,"utf8")).find((row:{name:string})=>row.name==="b").reason).toBe("ceiling");
  expect(takeQuarantineReadV1(path,1000+TAKE_POLICY_V1.quarantineMilliseconds)).toEqual(["b","c"]);
  expect(readdirSync(join(folder,"deep"))).toEqual(["owner-quarantine.json"]);
  expect(takeQuarantineReadV1(join(folder,"missing.json"))).toEqual([]);
 }finally{rmSync(folder,{recursive:true,force:true});}
});

test("the watch of the real read asks the verdict before it stops and quarantines what runs",()=>{
 const text=readFileSync(new URL("../../../🟨️.mjs",import.meta.url),"utf8");
 expect(text).toContain("takeStopVerdictV1(");
 expect(text).toContain("takeQuarantineWriteV1(quarantinePath");
 expect(text).toContain("takeQuarantineReadV1(");
});
