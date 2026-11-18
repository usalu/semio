/** 🪧️ Law: a read whose owner count has not advanced says, in both languages, which owners it waits for and why, and a look that found nothing is never progress. The sign of the progress watch is made of what owners did (done, taken, rows, children's processor time, records from other machines), never of how often the cache was asked; the tail names every missing owner by its reason (too large for this machine, running here, parked, waiting for the fleet); and a read that waits only for owners this machine can never infer fails with that typed reason, not with a generic stall.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import fs from"node:fs";const readFileSync=(path:string,encoding:"utf8")=>fs.readFileSync(path,encoding);
import{join}from"node:path";
import{createProgressWatchV1,createTailReporterV1,takeMissingRowsV1,takeProgressSignV1,takeStallKindV1,takeTailTextV1,TAKE_POLICY_V1}from"../../🟨️.mjs";

const source=readFileSync(join(import.meta.dir,"..","..","..","🟨️.mjs"),"utf8");
const base={finished:4371,taken:4371,cursor:4399,projectIndex:4399,deferred:0,rows:0,done:4371,producers:3,cpu:0};

test("asking the cache again changes nothing about the sign: lookups that found nothing are not progress",()=>{
 const before=takeProgressSignV1(base);
 expect(takeProgressSignV1({...base,looks:9000,lookMilliseconds:123})).toBe(before);
 expect(takeProgressSignV1({...base,taken:4372})).not.toBe(before);
 expect(takeProgressSignV1({...base,producers:4})).not.toBe(before);
 expect(takeProgressSignV1({...base,rows:7})).not.toBe(before);
});

test("a read that only asks the cache again stalls the watch after the declared window",()=>{
 let at=0,looks=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000});
 let hit;
 for(at=0;at<=300000&&hit===undefined;at+=2000){looks+=84;hit=watch.check(takeProgressSignV1({...base,looks}));}
 expect(hit?.sinceMilliseconds).toBeGreaterThanOrEqual(300000);
});

test("the watch of the real read builds its sign from the declared keys and never from the lookup count",()=>{
 const body=source.slice(source.indexOf("const watch=createProgressWatchV1()"),source.indexOf("},TAKE_POLICY_V1.watchMilliseconds)"));
 expect(body).toContain("takeProgressSignV1(");
 expect(body).not.toContain("looks.count");
});

test("the tail reporter speaks once per window of no change and again only after the next window, and a change starts it over",()=>{
 let at=0;
 const reporter=createTailReporterV1({now:()=>at,windowMilliseconds:60000});
 expect(reporter.check("a")).toBeUndefined();
 at=59999;expect(reporter.check("a")).toBeUndefined();
 at=60000;expect(reporter.check("a")).toEqual({sinceMilliseconds:60000});
 at=90000;expect(reporter.check("a")).toBeUndefined();
 at=120000;expect(reporter.check("a")).toEqual({sinceMilliseconds:120000});
 at=130000;expect(reporter.check("b")).toBeUndefined();
 at=190000;expect(reporter.check("b")).toEqual({sinceMilliseconds:60000});
});

test("every missing owner has exactly one reason: too large, running, parked or waiting for the fleet",()=>{
 const rows=takeMissingRowsV1(["wfc","flow","run","park","other"],{never:new Set(["wfc","flow"]),running:new Map([["run",340]]),deferred:new Set(["park"]),megabytes:name=>({wfc:4400,flow:9100}[name as string]),limitMegabytes:5200});
 expect(rows.map(row=>[row.name,row.reason])).toEqual([["wfc","heavy"],["flow","heavy"],["run","running"],["park","parked"],["other","fleet"]]);
 expect(rows[0]).toMatchObject({megabytes:4400,limitMegabytes:5200});
 expect(rows[2]).toMatchObject({seconds:340});
});

test("the tail text names the owners by reason in English and German, with the numbers",()=>{
 const rows=takeMissingRowsV1(["wfc","flow","run","other"],{never:new Set(["wfc","flow"]),running:new Map([["run",340]]),deferred:new Set(),megabytes:name=>({wfc:4400,flow:9100}[name as string]),limitMegabytes:5200});
 const fields={minutes:4,done:4371,total:4455,rows};
 const en=takeTailTextV1("en",fields),de=takeTailTextV1("de",fields);
 for(const text of[en,de]){for(const piece of["wfc","flow","run","other","9100","5200","340","4371","4455"])expect(text).toContain(piece);expect(text).not.toContain("{");expect(text).not.toContain("undefined");}
 expect(en).toContain("too large");expect(en).toContain("waiting for the fleet");
 expect(de).toContain("zu groß");expect(de).toContain("wartet auf die Flotte");
});

test("a stall in which every missing owner is too large for this machine is the typed heavy refusal, anything else the generic stall",()=>{
 const heavy=takeMissingRowsV1(["a","b"],{never:new Set(["a","b"]),running:new Map(),deferred:new Set(),megabytes:()=>9000,limitMegabytes:5000});
 const mixed=takeMissingRowsV1(["a","b"],{never:new Set(["a"]),running:new Map(),deferred:new Set(),megabytes:()=>9000,limitMegabytes:5000});
 expect(takeStallKindV1(heavy)).toBe("heavy");
 expect(takeStallKindV1(mixed)).toBe("stalled");
 expect(takeStallKindV1([])).toBe("stalled");
});

test("the policy declares the tail report window and it is shorter than the stall window",()=>{
 expect(TAKE_POLICY_V1.tailReportMilliseconds).toBe(60000);
 expect(TAKE_POLICY_V1.tailReportMilliseconds).toBeLessThan(TAKE_POLICY_V1.noProgressMilliseconds);
});

test("the real read reports the tail from the watcher and fails the all-heavy stall with the heavy refusal",()=>{
 const body=source.slice(source.indexOf("const watch=createProgressWatchV1()"),source.indexOf("},TAKE_POLICY_V1.watchMilliseconds)"));
 expect(body).toContain("createTailReporterV1(");
 expect(body).toContain("takeMissingRowsV1(");
 expect(body).toContain("takeTailTextV1(");
 expect(body).toContain('takeStallKindV1(left)==="heavy"');
 expect(body).toContain("refuseHeavy(");
});

test("the tail text carries the words of this machine's own pool when it has one",()=>{
 const rows=takeMissingRowsV1(["other"],{never:new Set(),running:new Map(),deferred:new Set(),megabytes:()=>undefined,limitMegabytes:undefined});
 const pool={state:"pulling",children:0,running:0,ownersPerMinute:0,text:{en:"waiting: the machine swaps (162 MB/s out), no child runs",de:"wartet: die Maschine lagert aus (162 MB/s), kein Kindprozess läuft"}};
 const fields={minutes:6,done:4371,total:4455,rows,pool};
 expect(takeTailTextV1("en",fields)).toContain("the machine swaps (162 MB/s out)");
 expect(takeTailTextV1("de",fields)).toContain("die Maschine lagert aus (162 MB/s)");
 expect(takeTailTextV1("en",{...fields,pool:undefined})).not.toContain("pool");
 expect(takeTailTextV1("en",fields)).toContain("pool");
 expect(takeTailTextV1("de",fields)).toContain("Pool");
});

test("the real read asks the pool status and hands it to the tail text",()=>{
 const body=source.slice(source.indexOf("const watch=createProgressWatchV1()"),source.indexOf("},TAKE_POLICY_V1.watchMilliseconds)"));
 expect(source).toContain("readOwnerPoolStatusV1(");
 expect(body).toContain("await ownPoolStatus()");
 expect(body).toContain("rows:left,pool}");
});
