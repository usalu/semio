/** 🚦️ Contract: a read that makes no owner progress for the declared time stops with a plain text in English and German that names what it waits on, and a large machine is recognised by the memory and cores it declares.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../../../../🐢️speed/🟨️.mjs";
import{TAKE_POLICY_V1,createCpuProgressV1,takeChildrenCpuV1,takeCpuMillisecondsV1,takeTreeCpuMillisecondsV1,createProgressWatchV1,takeNoProgressTextV1,takeStrongMachineV1}from"../../🟨️.mjs";

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
 const{spawn}=await import("node:child_process"),busy=spawn(process.execPath,["-e","const end=Date.now()+3000;while(Date.now()<end);"],{stdio:"ignore"});
 try{
  const first=await takeChildrenCpuV1();
  await new Promise(resolve=>setTimeout(resolve,1500));
  const second=await takeChildrenCpuV1();
  expect(first).toBeDefined();expect(second!-first!).toBeGreaterThan(500);
 }finally{busy.kill();}
},20000);

test("the default window is the authored window times the speed factor of this machine and an explicit window is used as given",()=>{
 const factor=localMachineSpeedFactorV1(),expected=scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds,factor);
 let clock=0;const watch=createProgressWatchV1({now:()=>clock});watch.check(1);
 clock=expected-1;expect(watch.check(1)).toBeUndefined();clock=expected;expect(watch.check(1)).toEqual({sinceMilliseconds:expected});
 let other=0;const given=createProgressWatchV1({now:()=>other,windowMilliseconds:1000});given.check(1);other=999;expect(given.check(1)).toBeUndefined();other=1000;expect(given.check(1)).toBeDefined();
 expect(scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds,2.83)).toBe(849000);
});
