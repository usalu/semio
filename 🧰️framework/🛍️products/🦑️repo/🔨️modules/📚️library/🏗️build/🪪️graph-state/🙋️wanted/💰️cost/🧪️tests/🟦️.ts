import Ajv from"ajv";
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{wantedPolicyV1}from"../../🟦️.ts";
import{classifyOwnersV1,clampOwnerCostTableV1,createOwnerCostRecorderV1,ownerCostCeilingV1,describeHeavyV1,emptyOwnerCostTableV1,estimateOwnerCostV1,estimateRemainingMillisecondsV1,mergeOwnerCostTablesV1,orderByCostV1,ownerCostPathV1,ownerCostPolicyV1,ownerCostTextV1,ownerGrantV1,planSoloV1,publishOwnerCostsV1,readOwnerCostFileV1,readOwnerCostsV1,recordOwnerCostFileV1,recordOwnerCostV1,soloGrantV1,waveEvidenceV1,writeOwnerCostFileV1,type OwnerCostInputV1}from"../🟦️.ts";

/** 💰️ Contract: projects are weighed by memory, one owner at a time. The recorded cost of an owner is its own, the heavy ones are found by the grant of the machine, the order of a serving machine follows the cost, and the table survives files, merges and the fleet. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-cost-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Table",definitions:schema.definitions});
const sample=(name:string,megabytes:number,atMs:number,stopped=false,kind="project"):OwnerCostInputV1=>({name,kind,megabytes,seconds:12,rows:34,stopped,atMs});
const BASE=wantedPolicyV1,SIXTEEN=ownerGrantV1(8192,BASE);

test("the grant of a 16 GB machine is the admitted half less the base, divided by the footprint factor and the margin; a serving machine has none",()=>{
 expect(SIXTEEN).toEqual({footprintMegabytes:4392,managedMegabytes:Math.floor(4392/2/1.25)});expect(SIXTEEN.managedMegabytes).toBe(1756);
 expect(ownerGrantV1(9400,BASE).managedMegabytes).toBe(Number.POSITIVE_INFINITY);
 expect(ownerGrantV1(16384,BASE).footprintMegabytes).toBe(Number.POSITIVE_INFINITY);
 expect(ownerGrantV1(1000,BASE)).toEqual({footprintMegabytes:0,managedMegabytes:0});
});

test("a recorded owner is estimated by its own newest samples, never by its kind; seeds, kinds and the unknown follow in that order",()=>{
 let table=emptyOwnerCostTableV1();
 for(const [at,megabytes] of [100,200,9000,300,250,260,270,280].entries())table=recordOwnerCostV1(table,sample("@semio-tech/a",megabytes,at+1));
 table=recordOwnerCostV1(table,sample("@semio-tech/light",90,50));
 expect(estimateOwnerCostV1(table,"@semio-tech/a")).toEqual({megabytes:300,source:"recorded"});
 expect(table.owners["@semio-tech/a"]!.samples).toHaveLength(ownerCostPolicyV1.samples);
 expect(estimateOwnerCostV1(table,"@semio-tech/light")).toEqual({megabytes:90,source:"recorded"});
 expect(estimateOwnerCostV1(table,"@semio-tech/s-dev-composition","project")).toEqual({megabytes:13976,source:"seed"});
 expect(estimateOwnerCostV1(table,"workspace","root:language:cargo")).toEqual({megabytes:3100,source:"seed"});
 expect(estimateOwnerCostV1(table,"another-root","root:language:cargo")).toEqual({megabytes:3100,source:"kind"});
 expect(estimateOwnerCostV1(table,"@semio-tech/never")).toEqual({megabytes:ownerCostPolicyV1.unknownMegabytes,source:"unknown"});
 expect(estimateOwnerCostV1(recordOwnerCostV1(table,sample("@semio-tech/s-dev-composition",900,99)),"@semio-tech/s-dev-composition")).toEqual({megabytes:900,source:"recorded"});
});

test("on a 16 GB machine the heavy owners are never local and the light ones are: a mix of the measured owners",()=>{
 const table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("@semio-tech/os-kernel",1500,5));
 const names=["@semio-tech/s-dev-composition","@semio-tech/framework-rs","@semio-tech/repo-lib","workspace","@semio-tech/os-kernel","@semio-tech/value-rs","@semio-tech/never"];
 const kinds:Record<string,string>={workspace:"root:language:cargo","@semio-tech/value-rs":"language:cargo"};
 const classes=classifyOwnersV1(names,name=>estimateOwnerCostV1(table,name,kinds[name]),SIXTEEN);
 expect(classes.heavy.map(row=>row.name)).toEqual(["@semio-tech/s-dev-composition","@semio-tech/repo-lib","@semio-tech/framework-rs","workspace"]);
 expect(classes.light).toEqual(["@semio-tech/os-kernel","@semio-tech/value-rs","@semio-tech/never"]);
 expect(classes.heavy[0]).toEqual({name:"@semio-tech/s-dev-composition",megabytes:13976,source:"seed"});
});

test("the same owners on a 32 GB machine are all light: the grant is the only difference",()=>{
 const classes=classifyOwnersV1(["@semio-tech/s-dev-composition","workspace"],name=>estimateOwnerCostV1(undefined,name),ownerGrantV1(16384,BASE));
 expect(classes.heavy).toEqual([]);
});

test("light owners fill the grant by count (the wave rule) while the heavy ones stay out of it, whatever number of light ones there are",()=>{
 const light=Array.from({length:300},(_,at)=>`@semio-tech/l${at}`),names=[...light,"workspace"];
 const classes=classifyOwnersV1(names,name=>estimateOwnerCostV1(undefined,name,name==="workspace"?"root:language:cargo":"project"),SIXTEEN);
 expect(classes.heavy.map(row=>row.name)).toEqual(["workspace"]);expect(classes.light).toHaveLength(300);
});

test("the serving order is the heavy ones of the requesters first, then by cost, heaviest first; equal cost in the order given; every name once",()=>{
 let table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("b",500,1));
 table=recordOwnerCostV1(table,sample("a",500,1));table=recordOwnerCostV1(table,sample("c",4000,1));table=recordOwnerCostV1(table,sample("d",10,1));
 const estimate=(name:string)=>estimateOwnerCostV1(table,name);
 expect(orderByCostV1(["d","b","a","c","c","e"],estimate)).toEqual(["c","e","b","a","d"]);
 expect(orderByCostV1(["d","b","a","c"],estimate,new Set(["d"]))).toEqual(["d","c","b","a"]);
 expect(orderByCostV1(["x","y"],estimate)).toEqual(["x","y"]);
});

test("a wave proves the cost of its only project at the peak, nothing of several and nothing below the floor",()=>{
 expect(waveEvidenceV1({peakMegabytes:18814,managedMegabytes:4838,peakProjects:["@semio-tech/s-dev-composition"]})).toEqual({name:"@semio-tech/s-dev-composition",megabytes:13976});
 expect(waveEvidenceV1({peakMegabytes:8365,managedMegabytes:4332,peakProjects:["workspace","a","b"]})).toBeUndefined();
 expect(waveEvidenceV1({peakMegabytes:4300,managedMegabytes:4200,peakProjects:["a"]})).toBeUndefined();
 expect(waveEvidenceV1({peakMegabytes:4300,managedMegabytes:4200,peakProjects:[]})).toBeUndefined();
});

test("the table is a file that validates against its schema with Ajv, survives damage and foreign content as empty, and merges equal samples once",()=>{
 const directory=temp("file"),path=ownerCostPathV1(directory);
 expect(path).toBe(join(directory,".cache/f/graph-costs/owner-costs.json"));
 expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 const first=recordOwnerCostFileV1(path,sample("a",700,10)),second=recordOwnerCostFileV1(path,sample("a",9000,20,true));
 expect(validate(second)).toBe(true);expect(validate(JSON.parse(readFileSync(path,"utf8")))).toBe(true);
 expect(readOwnerCostFileV1(path).owners.a!.samples.map(row=>row.megabytes)).toEqual([700,9000]);expect(first.owners.a!.samples).toHaveLength(1);
 writeFileSync(path,"{damaged");expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 writeFileSync(path,JSON.stringify({version:2,owners:{}}));expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 const poisoned={version:1,owners:{a:{kind:"p",rows:0,seconds:0,samples:[{megabytes:14000,atMs:1,stopped:true,hostMegabytes:16384}]}}};
 writeFileSync(path,JSON.stringify(poisoned));expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 expect(mergeOwnerCostTablesV1([poisoned as never,second]).owners.a!.samples.map(row=>row.megabytes)).toEqual([700,9000]);
 const merged=mergeOwnerCostTablesV1([second,second,undefined,recordOwnerCostV1(emptyOwnerCostTableV1(),sample("a",800,15))]);
 expect(merged.owners.a!.samples.map(row=>row.megabytes)).toEqual([700,800,9000]);
 expect(validate(JSON.parse(JSON.stringify({version:2,owners:{a:{kind:"p",rows:0,seconds:0,samples:[]}}})))).toBe(false);
 writeOwnerCostFileV1(join(directory,"blocked","x","table.json"),second);
});

test("the heavy owners are worded in English and German with their physical cost, the heaviest first, and the rest as a count",()=>{
 const heavy=Array.from({length:8},(_,at)=>({name:`p${at}`,megabytes:1000*(8-at),source:"recorded" as const}));
 expect(describeHeavyV1(heavy.slice(0,2),"en")).toBe("p0 (about 16000 MB), p1 (about 14000 MB)");
 expect(describeHeavyV1(heavy.slice(0,1),"de")).toBe("p0 (etwa 16000 MB)");
 expect(describeHeavyV1(heavy,"en").endsWith("and 2 more")).toBe(true);expect(describeHeavyV1(heavy,"de").endsWith("und 2 weitere")).toBe(true);
 for(const key of Object.keys(schema.definitions.Texts.const.en) as (keyof typeof schema.definitions.Texts.const.en)[])expect(ownerCostTextV1(key,"de").length).toBeGreaterThan(3);
 expect(Object.keys(schema.definitions.Texts.const.de)).toEqual(Object.keys(schema.definitions.Texts.const.en));
});

test("the fleet carries the table of the serving machine: published, read, damaged or absent",async()=>{
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},root=temp("client"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  expect(await readOwnerCostsV1({client})).toBeUndefined();
  const table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("@semio-tech/s-dev-composition",13976,5));
  expect(await publishOwnerCostsV1({client,server:"g1-server",table})).toBe(true);
  expect(await readOwnerCostsV1({client})).toEqual(table);
 }finally{await server.stop(true);}
},60000);

test("the recorder keeps only notable owners, writes merged and rarely, and flushes at once when an owner was stopped",()=>{
 const directory=temp("recorder"),path=ownerCostPathV1(directory),MB=1048576;let time=1_000_000;
 const recorder=createOwnerCostRecorderV1({path,now:()=>time,rowsOf:key=>key==="heavy"?18748:0});
 recorder.record({key:"tiny",kind:"project",takenBytes:5*MB,seconds:1,stopped:false});
 recorder.record({key:"big",kind:"project",takenBytes:900*MB,seconds:30,stopped:false});
 expect(Object.keys(recorder.table().owners)).toEqual(["big"]);expect(readOwnerCostFileV1(path).owners.big?.samples[0]?.megabytes).toBe(900);
 time+=100;recorder.record({key:"big",kind:"project",takenBytes:100*MB,seconds:3,stopped:false});
 expect(readOwnerCostFileV1(path).owners.big?.samples).toHaveLength(1);
 time+=ownerCostPolicyV1.flushMilliseconds;recorder.record({key:"big",kind:"project",takenBytes:110*MB,seconds:3,stopped:false});
 expect(readOwnerCostFileV1(path).owners.big?.samples.map(sample=>sample.megabytes)).toEqual([900,100,110]);
 recordOwnerCostFileV1(path,sample("other-writer",777,5));
 recorder.record({key:"heavy",kind:"root:language:cargo",takenBytes:4000*MB,seconds:60,stopped:true});
 const file=readOwnerCostFileV1(path);expect(file.owners.heavy).toMatchObject({rows:18748,samples:[{megabytes:4000,stopped:true}]});expect(file.owners["other-writer"]?.samples[0]?.megabytes).toBe(777);
 expect(validate(file)).toBe(true);
});

test("the order follows the demand first: a project asked for by two requests comes before a heavier one asked for by one, then the heavy ones, then the cost, and a project is listed once",()=>{
 const cost=(name:string)=>({megabytes:name==="big"?9000:name==="mid"?2000:100,source:"recorded" as const});
 expect(orderByCostV1(["big","mid","small","mid","small"],cost,new Set(),new Map([["mid",2],["small",2]]))).toEqual(["mid","small","big"]);
 expect(orderByCostV1(["a","big","b"],cost,new Set(["a"]),new Map())).toEqual(["a","big","b"]);
 expect(orderByCostV1(["x","y"],cost)).toEqual(["x","y"]);
});

test("the time left comes from what the read finished, else from the measured table (the seconds of the owners, the median for an unmeasured one, run in parallel), null when nothing was measured, zero when nothing is left",()=>{
 const table=["a","b","c"].reduce((sum,name,at)=>recordOwnerCostV1(sum,{name,kind:"project",megabytes:500,seconds:[10,20,30][at]!,rows:1,stopped:false,atMs:1}),emptyOwnerCostTableV1());
 expect(estimateRemainingMillisecondsV1({table,remaining:[],parallel:1})).toBe(0);
 expect(estimateRemainingMillisecondsV1({table:undefined,remaining:["a"],parallel:1})).toBeNull();
 expect(estimateRemainingMillisecondsV1({table:emptyOwnerCostTableV1(),remaining:["a"],parallel:1})).toBeNull();
 expect(estimateRemainingMillisecondsV1({table,remaining:["a","c"],parallel:1})).toBe(40_000);
 expect(estimateRemainingMillisecondsV1({table,remaining:["a","unknown"],parallel:2})).toBe(15_000);
 expect(estimateRemainingMillisecondsV1({table,remaining:["a","b"],parallel:1,observed:{done:30,milliseconds:60_000}})).toBe(4000);
 expect(estimateRemainingMillisecondsV1({table,remaining:["a"],parallel:1,observed:{done:0,milliseconds:60_000}})).toBe(10_000);
});

test("the solo grant is what ONE owner can be given while nothing else runs: the host less the parent floor and the system reserve, lowered to what is available now less the floor; a 16 GB Mac gives a heavy owner 4214 managed MB at rest and less when busy, never a serving machine's unlimited grant",()=>{
 const host=16384,reserve=ownerCostPolicyV1.soloReserveMegabytes,floor=ownerCostPolicyV1.floorMegabytes;
 expect(soloGrantV1({hostMegabytes:host},BASE)).toEqual({footprintMegabytes:host-reserve-floor,managedMegabytes:Math.floor((host-reserve-floor)/2/1.25)});
 expect(soloGrantV1({hostMegabytes:host},BASE).managedMegabytes).toBe(4214);
 expect(soloGrantV1({hostMegabytes:host,availableMegabytes:12000},BASE).managedMegabytes).toBe(Math.floor((12000-BASE.availableFloorMegabytes)/2/1.25));
 expect(soloGrantV1({hostMegabytes:host,availableMegabytes:20000},BASE).managedMegabytes).toBe(4214);
 expect(soloGrantV1({hostMegabytes:host,availableMegabytes:7599},BASE).managedMegabytes).toBe(2220);
 expect(soloGrantV1({hostMegabytes:host,availableMegabytes:1000},BASE)).toEqual({footprintMegabytes:0,managedMegabytes:0});
 expect(soloGrantV1({hostMegabytes:4000},BASE)).toEqual({footprintMegabytes:0,managedMegabytes:0});
 expect(Number.isFinite(soloGrantV1({hostMegabytes:65536},BASE).managedMegabytes)).toBe(true);
 expect(soloGrantV1({hostMegabytes:host},BASE).managedMegabytes).toBeGreaterThan(SIXTEEN.managedMegabytes);
});

test("a learned cost never refuses an owner: every heavy owner is attempted alone, and only a machine that can give one owner nothing refuses them",()=>{
 const heavy=classifyOwnersV1(["a","b","c"],name=>({megabytes:{a:3492,b:6000,c:3000}[name]!,source:"recorded" as const}),SIXTEEN).heavy,grant=soloGrantV1({hostMegabytes:16384},BASE);
 expect(heavy.map(row=>row.name)).toEqual(["b","a","c"]);
 expect(planSoloV1(heavy,grant)).toEqual({fit:heavy,over:[]});
 expect(planSoloV1(heavy,{footprintMegabytes:0,managedMegabytes:0})).toEqual({fit:[],over:heavy});
});

test("a poisoned cost heals at the read: every sample is lowered to the host less the floor, whatever table it came from, and an estimate at the ceiling is saturated and light",()=>{
 const ceiling=ownerCostCeilingV1(16384);
 expect(ceiling).toBe(16384-ownerCostPolicyV1.floorMegabytes);
 const poisoned=["@semio-tech/framework-os-dev","repo"].reduce((table,name,at)=>recordOwnerCostV1(table,sample(name,[19510,7132][at]!,10+at)),emptyOwnerCostTableV1()),healed=clampOwnerCostTableV1(poisoned,ceiling);
 expect(clampOwnerCostTableV1(healed,ceiling)).toBe(healed);
 expect(healed.owners["@semio-tech/framework-os-dev"]!.samples[0]!.megabytes).toBe(ceiling);expect(healed.owners.repo!.samples[0]!.megabytes).toBe(7132);
 const context={ceilingMegabytes:ceiling},os=estimateOwnerCostV1(poisoned,"@semio-tech/framework-os-dev","project",undefined,context);
 expect(os).toEqual({megabytes:ceiling,source:"recorded",saturated:true});
 expect(estimateOwnerCostV1(poisoned,"@semio-tech/s-dev-composition","project",undefined,context)).toEqual({megabytes:ceiling,source:"seed",saturated:true});
 const names=["@semio-tech/framework-os-dev","repo"],classes=classifyOwnersV1(names,name=>estimateOwnerCostV1(poisoned,name,"project",undefined,context),SIXTEEN);
 expect(classes.heavy.map(row=>row.name)).toEqual(["repo"]);expect(classes.light).toEqual(["@semio-tech/framework-os-dev"]);
 expect(recordOwnerCostV1(emptyOwnerCostTableV1(),sample("x",27488,1),ownerCostPolicyV1,ceiling).owners.x!.samples[0]!.megabytes).toBe(ceiling);
});

test("an old sample decays by the half-life, so a lighter run ages a heavy one out",()=>{
 const table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("a",4000,1_000)),half=ownerCostPolicyV1.halfLifeMilliseconds,at=(nowMs:number)=>estimateOwnerCostV1(table,"a",undefined,undefined,{ceilingMegabytes:Infinity,nowMs}).megabytes;
 expect(at(1_000)).toBe(4000);expect(at(1_000+half)).toBe(2000);expect(at(1_000+3*half)).toBe(500);
});

test("the file and the recorder heal a poisoned file: the read is lowered, and the next flush writes the lowered values back",()=>{
 const directory=temp("heal"),path=ownerCostPathV1(directory),MB=1048576,ceiling=ownerCostCeilingV1(16384);
 writeOwnerCostFileV1(path,recordOwnerCostV1(emptyOwnerCostTableV1(),sample("poisoned",39020,5)));
 expect(readOwnerCostFileV1(path,ceiling).owners.poisoned!.samples[0]!.megabytes).toBe(ceiling);expect(readOwnerCostFileV1(path).owners.poisoned!.samples[0]!.megabytes).toBe(39020);
 const recorder=createOwnerCostRecorderV1({path,ceilingMegabytes:ceiling,now:()=>9});recorder.record({key:"poisoned",kind:"project",takenBytes:30000*MB,seconds:1,stopped:true});
 expect(readOwnerCostFileV1(path).owners.poisoned!.samples.map(row=>row.megabytes)).toEqual([ceiling,ceiling]);
});

test("the solo texts exist in English and German and name the owners and the numbers",()=>{
 const fields={count:1,named:"@semio-tech/framework-os-kernel (about 6984 MB)",grant:8428};
 for(const key of["solo","exceeds"] as const)for(const language of["en","de"] as const){const text=ownerCostTextV1(key,language,fields);expect(text).toContain("framework-os-kernel");expect(text).toContain("8428");}
 expect(ownerCostTextV1("exceeds","de",fields)).not.toBe(ownerCostTextV1("exceeds","en",fields));
});

test("an owner this machine measured is judged by its own samples: the inflated samples of a bigger fleet host never refuse it, and they only inform an owner this machine never measured",()=>{
 const mine=16384,big=65536,context={ceilingMegabytes:12000,hostMegabytes:mine,nowMs:100},local=(name:string,megabytes:number)=>({...sample(name,megabytes,100),hostMegabytes:mine}),fleet=(name:string,megabytes:number)=>({...sample(name,megabytes,100),hostMegabytes:big});
 const own=recordOwnerCostV1(emptyOwnerCostTableV1(),local("@semio-tech/repo-lib",1670)),remote=recordOwnerCostV1(recordOwnerCostV1(emptyOwnerCostTableV1(),fleet("@semio-tech/repo-lib",8277)),fleet("@semio-tech/only-fleet",8000)),merged=mergeOwnerCostTablesV1([own,remote]);
 expect(merged.owners["@semio-tech/repo-lib"]!.samples.map(row=>row.hostMegabytes).sort()).toEqual([mine,big]);
 expect(estimateOwnerCostV1(merged,"@semio-tech/repo-lib",undefined,undefined,context)).toEqual({megabytes:1670,source:"recorded"});
 expect(estimateOwnerCostV1(merged,"@semio-tech/only-fleet",undefined,undefined,context)).toEqual({megabytes:8000,source:"recorded"});
 expect(estimateOwnerCostV1(merged,"@semio-tech/repo-lib",undefined,undefined,{ceilingMegabytes:12000,nowMs:100}).megabytes).toBe(8277);
 expect(classifyOwnersV1(["@semio-tech/repo-lib"],name=>estimateOwnerCostV1(merged,name,undefined,undefined,context),SIXTEEN).heavy).toEqual([]);
 expect(recordOwnerCostV1(emptyOwnerCostTableV1(),sample("x",1,1)).owners.x!.samples[0]!.hostMegabytes).toBeGreaterThan(0);
 expect(validate(merged)).toBe(true);
});
