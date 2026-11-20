/** 📊️ Law: the counters of the adopt and poll path count every outcome once, equal an independent Python model over 200 seeded event sequences, print one line in English and German that names every number, and the hot names are read from the store once per interval and counted as listed.
 * @see ../../🟨️.mjs `createAdoptCountersV1` `createHotNamesV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{join}from"node:path";
import{TAKE_POLICY_V1,createAdoptCountersV1,createHotNamesV1,recordTakeLookFailureV1}from"../../🟨️.mjs";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

const seeded=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const outcomes=["fetched","present","stale","incomplete","miss","fault","bogus"],lanes=["adopt","infer","pass","poll"],kinds=["file","closure","analysis","readdirNames","exists",undefined,""];

test("200 seeded event sequences: the counters equal the independent Python model",()=>{
 const python=Bun.which("python3")??"python3",oracle=join(import.meta.dir,"🐍️.py");
 for(let seed=1;seed<=200;seed++){
  const next=seeded(seed),pick=<T,>(list:T[])=>list[Math.floor(next()*list.length)]!,counters=createAdoptCountersV1(),events:any[]=[];
  for(let step=0;step<60;step++){
   const kind=pick(["list","look","outcome","staleRow","skip","gate","inferredListed"]);
   if(kind==="list"){const names=Array.from({length:Math.floor(next()*5)},()=>"o"+Math.floor(next()*20));counters.list(names);events.push({k:kind,names});}
   else if(kind==="look"){const lane=pick(lanes);counters.look(lane);events.push({k:kind,lane});}
   else if(kind==="outcome"){const o=pick(outcomes);counters.outcome(o);events.push({k:kind,o});}
   else if(kind==="staleRow"){const row=pick(kinds);counters.staleRow(row);events.push({k:kind,kind:row??null});}
   else if(kind==="skip"){const name="o"+Math.floor(next()*20);counters.skipInferring(name);events.push({k:kind,name});}
   else if(kind==="gate"){const ms=Math.floor(next()*3000);counters.gateWait(ms);events.push({k:kind,ms});}
   else{counters.inferredListed();events.push({k:kind});}
  }
  const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({events}),encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);
  expect(JSON.parse(JSON.stringify(counters.counts())),"seed "+seed).toEqual(JSON.parse(run.stdout));
 }
});

test("the line names every number in English and in German and the two texts take the same fields",()=>{
 const counters=createAdoptCountersV1();
 counters.list(["a","b","c"]);counters.look("adopt");counters.look("infer");counters.look("pass");counters.outcome("fetched");counters.outcome("present");counters.outcome("stale");counters.staleRow("file");counters.staleRow("file");counters.staleRow("closure");counters.outcome("incomplete");counters.outcome("miss");counters.outcome("fault");counters.skipInferring("a");counters.gateWait(2600);counters.inferredListed();
 const en=counters.line("en"),de=counters.line("de");
 expect(en).toBe("adoption: 3 owners listed as stored in the fleet, 3 looks (1 by the adopt loop, 1 before a local inference, 1 by the pass), 2 adopted (1 fetched, 1 present), 1 stale (closure 1, file 2), 1 incomplete, 1 missed, 1 faulted (unknown 1), 0 cancelled; set aside: 1 listed owners already inferring here, 1 waits of looks at the memory gate (3 s), 1 owners inferred here although the fleet lists them");
 expect(de).toContain("3 Projekte von der Flotte als gespeichert gemeldet");expect(de).toContain("(closure 1, file 2)");expect(de).toContain("(3 s)");
 const fields=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]!).sort();
 const texts=schema.definitions.Texts.const;expect(fields(texts.de.adopt)).toEqual(fields(texts.en.adopt));
 expect(en).not.toContain("{");expect(de).not.toContain("{");
 expect(createAdoptCountersV1().line("en")).toContain("0 owners listed");
});

test("unexpected and unrecognized failures use the bounded unknown counter, and cancellation stays separate",()=>{
 const counters=createAdoptCountersV1(),failure={code:"BUILD_TAKE_MISS",outcome:"fault",lookupFault:"https://example.invalid/private-owner"};
 expect(recordTakeLookFailureV1(failure,counters,false)).toBe("fault");
 expect(recordTakeLookFailureV1(Error("broken probe"),counters,false)).toBe("miss");
 expect(recordTakeLookFailureV1(failure,counters,true)).toBe("miss");
 expect(recordTakeLookFailureV1({code:"BUILD_TAKE_MISS",outcome:"miss"},counters,false)).toBe("miss");
 expect(recordTakeLookFailureV1({code:"BUILD_TAKE_MISS",outcome:"fetched"},counters,false)).toBe("stale");
 expect(counters.counts()).toMatchObject({fault:2,miss:1,stale:1});
 expect(counters.lookups()).toEqual({misses:1,cancelled:1,faults:{...Object.fromEntries(schema.definitions.LookupFaultCounts.required.map(key=>[key,0])),unknown:2}});
 expect(counters.line("en")).not.toContain("private-owner");
});

test("the hot names are read once per interval, the first read at once, and every name counts as listed once",()=>{
 let time=1000,reads=0;const counters=createAdoptCountersV1(),names=[["a","b"],["b","c"],["d"]];
 const hot=createHotNamesV1({read:()=>new Set(names[Math.min(reads++,2)]),counters,now:()=>time,policy:{...TAKE_POLICY_V1,hotNamesMilliseconds:10000}});
 expect([...hot()].sort()).toEqual(["a","b"]);expect(reads).toBe(1);
 time+=9999;expect([...hot()].sort()).toEqual(["a","b"]);expect(reads).toBe(1);
 time+=1;expect([...hot()].sort()).toEqual(["b","c"]);expect(reads).toBe(2);
 time+=10000;hot();expect(counters.counts().listed).toBe(4);
 for(let call=0;call<1000;call++)hot();expect(reads).toBe(3);
});
