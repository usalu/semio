import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./📥️.json" with{type:"json"};
import schema from"./🔣️.json" with{type:"json"};
import{createNativeDirectoryMeterV1,nativeDirectoryProtocolV1}from"../../🟦️.ts";
import{createNativeDirectoryUsageV1}from"../../📏️usage/🟦️.ts";

const flush=async()=>{for(let index=0;index<16;index++)await Promise.resolve();};
const sql=new Database(":memory:"),limits=corpus.limits;

test("slow complete observations reuse the explicit completion window at exact SQLite boundaries",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);expect(nativeDirectoryProtocolV1().meter.freshnessAnchor).toBe("completion");
 for(const row of corpus.slow){let clock=0,walks=0,oracleWalks=0,completed:number|undefined;const meter=createNativeDirectoryMeterV1({root:"/unused",...limits,now:()=>clock,measure:async()=>{walks++;clock+=row.duration;return 10;}});await meter.admit({mode:"recheck",requestBytes:0});completed=clock;oracleWalks++;const initial=completed;
 for(const[index,offset]of row.offsets.entries()){clock=initial+offset;const stale=sql.query("select ?1>=?2+?3 stale").get(clock,completed,nativeDirectoryProtocolV1().meter.intervalMilliseconds)as{stale:number};if(stale.stale){oracleWalks++;completed=clock+row.duration;}await meter.admit({mode:"recheck",requestBytes:0});expect(walks).toBe(row.walks[index]!);expect(walks).toBe(oracleWalks);}}
});

test("overlapping refreshes retire each baseline once and retain later reservations against SQLite debit identities",async()=>{
 const row=corpus.overlap;let walks=0;const pending:{done:(bytes:number)=>void}[]=[],cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,measure:async()=>{walks++;if(walks===1)return 0;return new Promise<number>(done=>pending.push({done}));}}),meter=createNativeDirectoryMeterV1({root:"/unused",...limits,now:()=>0,usage:cell});
 await meter.admit({mode:"reserve",requestBytes:row.oldAdmission});const first=meter.admit({mode:"commit",requestBytes:0}),second=meter.admit({mode:"commit",requestBytes:0});await flush();pending.shift()!.done(row.measured);await first;await flush();expect(walks).toBe(3);await meter.admit({mode:"reserve",requestBytes:row.newAdmission});pending.shift()!.done(row.measured);await second;
 sql.exec("drop table if exists debits;create table debits(id integer primary key,bytes integer,covered integer)");sql.query("insert into debits values(1,?1,1),(2,?2,0)").run(row.oldAdmission,row.newAdmission);const expected=sql.query("select ?1+(select sum(bytes) from debits where covered=0)+?2<=?3 accepted").get(row.measured,row.probe,limits.maximumPhysicalBytes)as{accepted:number};expect(Boolean(expected.accepted)).toBe(row.accepted);
 let accepted=true;const probe=meter.admit({mode:"reserve",requestBytes:row.probe}).catch(()=>{accepted=false;});await flush();expect(walks).toBe(4);pending.shift()!.done(row.finalMeasured);await probe;expect(accepted).toBe(Boolean(expected.accepted));
});

test("unsuccessful fresh observations preserve admission debits and never publish a successful completion",async()=>{
 for(const kind of["failure","cancel"]){const row=corpus.failure;let clock=0,walks=0;const pending:{done:(bytes:number)=>void;fail:(error:unknown)=>void}[]=[],cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,measure:(_,signal)=>{walks++;if(walks===1)return Promise.resolve(0);return new Promise<number>((done,fail)=>{pending.push({done,fail});signal.addEventListener("abort",()=>fail(signal.reason),{once:true});});}}),meter=createNativeDirectoryMeterV1({root:"/unused",...limits,now:()=>clock,usage:cell});await meter.admit({mode:"reserve",requestBytes:row.admission});const controller=new AbortController(),reason=Error(kind),failed=meter.admit({mode:"commit",requestBytes:0},controller.signal).catch(error=>error);await flush();clock=2001;if(kind==="cancel")controller.abort(reason);else pending[0]!.fail(reason);expect(await failed).toBe(reason);pending.shift();await flush();let accepted=true;const probe=meter.admit({mode:"reserve",requestBytes:row.probe}).catch(()=>{accepted=false;});await flush();expect(walks).toBe(3);pending.shift()!.done(row.finalMeasured);await flush();pending.shift()?.done(row.finalMeasured);await probe;const expected=sql.query("select ?1+?2<=?3 accepted").get(row.finalMeasured,row.probe,limits.maximumPhysicalBytes)as{accepted:number};expect(accepted).toBe(row.accepted);expect(accepted).toBe(Boolean(expected.accepted));}
});

test("fresh observations preserve the existing pre-scan unwritten admission boundary",async()=>{
 let physical=0;const meter=createNativeDirectoryMeterV1({root:"/unused",...limits,now:()=>0,measure:()=>physical});await meter.admit({mode:"reserve",requestBytes:80});await meter.admit({mode:"commit",requestBytes:0});const expected=sql.query("select ?1+?2<=?3 accepted").get(physical,80,limits.maximumPhysicalBytes)as{accepted:number};expect(Boolean(expected.accepted)).toBe(true);await expect(meter.admit({mode:"reserve",requestBytes:80})).resolves.toBeUndefined();
});

test("older refresh completion cannot replace newer measured usage",async()=>{
 const row=corpus.reordered;let walks=0;const pending:((bytes:number)=>void)[]=[],meter=createNativeDirectoryMeterV1({root:"/unused",...limits,now:()=>0,measure:()=>{walks++;if(walks===1)return 0;if(walks>=4)return row.newerMeasured;return new Promise<number>(done=>pending.push(done));}});await meter.admit({mode:"recheck",requestBytes:0});const first=meter.admit({mode:"commit",requestBytes:0}),second=meter.admit({mode:"commit",requestBytes:0});pending[1]!(row.newerMeasured);await second;pending[0]!(row.olderMeasured);await first;let accepted=true;await meter.admit({mode:"reserve",requestBytes:row.probe}).catch(()=>{accepted=false;});const expected=sql.query("select ?1+?2<=?3 accepted").get(row.newerMeasured,row.probe,limits.maximumPhysicalBytes)as{accepted:number};expect(accepted).toBe(row.accepted);expect(accepted).toBe(Boolean(expected.accepted));
});

test("exact admission counters survive a long lifetime and unsafe scalar requests are refused",async()=>{
 const row=corpus.lifetime,meter=createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:row.request,maximumPhysicalBytes:row.request,now:()=>0,measure:()=>0});for(let index=0;index<row.cycles;index++){await meter.admit({mode:"reserve",requestBytes:row.request});await meter.admit({mode:"commit",requestBytes:0});}const total=sql.query("select cast(?1 as integer)*?2 total").get(String(row.request),row.cycles)as{total:number};expect(total.total).toBeGreaterThan(Number.MAX_SAFE_INTEGER);await expect(meter.admit({mode:"reserve",requestBytes:row.request})).resolves.toBeUndefined();for(const bytes of corpus.unsafe){await expect(meter.admit({mode:"reserve",requestBytes:bytes})).rejects.toThrow("Closed directory meter request");expect(()=>createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:bytes,maximumPhysicalBytes:bytes,measure:()=>0})).toThrow("Closed directory meter limits");}
});

test("a measurement is kept between processes: a fresh cell starts from the record, a status answers from it without walking, and a damaged record is ignored",async()=>{
 const {mkdtempSync,rmSync,writeFileSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path"),{nativeDirectoryUsageFileV1}=await import("../../📏️usage/🟦️.ts");
 const directory=mkdtempSync(join(tmpdir(),"usage-record-")),file=join(directory,"usage.json");
 try{
  let walks=0;const input={root:"/unused",maximumEntries:10,measure:async()=>{walks++;return 4096;},record:nativeDirectoryUsageFileV1(file)};
  const first=createNativeDirectoryUsageV1({...input,now:()=>1000});
  expect(await first.recent(0)).toBe(4096);expect(walks).toBe(1);
  const second=createNativeDirectoryUsageV1({...input,now:()=>5000});
  expect(await second.recent(10_000),"a status process answers from the record").toBe(4096);expect(walks,"without a walk").toBe(1);
  expect(await second.recent(1000),"an older record than the bound is measured again").toBe(4096);expect(walks).toBe(2);
  writeFileSync(file,"{damaged");
  const third=createNativeDirectoryUsageV1({...input,now:()=>9000});
  expect(third.peek(),"a damaged record starts empty").toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
