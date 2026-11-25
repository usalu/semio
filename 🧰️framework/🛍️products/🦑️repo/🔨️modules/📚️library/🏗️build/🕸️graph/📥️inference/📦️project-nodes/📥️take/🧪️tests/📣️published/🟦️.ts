/** 📣️ Contract: the poll for the owners the fleet has not delivered yet looks first at the owners the pools of the fleet say they stored (`pool-published-<wave>.txt` below each origin of the shared store, appended by the pool child), instead of rotating through thousands of names 48 at a time; an owner whose look missed waits `hotRetryMilliseconds` before it is looked at again; files older than the age bound and unreadable stores count for nothing.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{TAKE_POLICY_V1,fleetGlobalsKeyV1,fleetPublishedNamesV1,takePollV1}from"../../🟨️.mjs";
import childSource from"../../../🌐️owner-pool/🧒️child/🟨️.mjs"with{type:"text"};

const store=()=>mkdtempSync(join(tmpdir(),"r194-"));
const publish=(root:string,origin:string,wave:string,names:string[],ageMs=0)=>{const dir=join(root,"v1","origins",origin);mkdirSync(dir,{recursive:true});const file=join(dir,`pool-published-${wave}.txt`);writeFileSync(file,names.join("\n")+"\n");const at=new Date(Date.now()-ageMs);utimesSync(file,at,at);};

test("the published names of every origin are read, old files and other files are not",async()=>{
 const root=store();publish(root,"g1-m01","abc123",["a","b"]);publish(root,"g1-m02","abc123",["b","c"]);publish(root,"g1-m03","def456",["old"],TAKE_POLICY_V1.publishedMaximumAgeMilliseconds+60000);
 writeFileSync(join(root,"v1","origins","g1-m01","pool.json"),"{\"x\":\"zz\"}");
 expect([...await fleetPublishedNamesV1(root)].sort()).toEqual(["a","b","c"]);
 expect((await fleetPublishedNamesV1(undefined)).size).toBe(0);expect((await fleetPublishedNamesV1(join(root,"missing"))).size).toBe(0);
});

test("the poll looks at the listed owners first, again after the retry time only, and rotates when nothing is listed",async()=>{
 const names=Array.from({length:200},(_,i)=>"n"+i),taken=new Set<string>(),looked:string[]=[];let clock=0,listed=new Set(["n150","n170","n199"]);
 const stopAfter=()=>looked.length>=8;
 const result=await takePollV1({queue:()=>names.filter(name=>!taken.has(name)),attempt:async name=>{looked.push(name);if(name==="n170")return"miss";taken.add(name);return"fetched";},arrived:()=>{},stop:()=>stopAfter(),hot:()=>listed,now:()=>clock,sleep:async()=>{clock+=1000;},policy:{...TAKE_POLICY_V1,hotRetryMilliseconds:5000}});
 expect(looked.slice(0,3).sort()).toEqual(["n150","n170","n199"]);
 expect(looked.filter(name=>name==="n170").length).toBeLessThanOrEqual(2);
 expect(result.found).toBeGreaterThanOrEqual(2);
});

test("a pool child that stored an owner appends its name to the shared published file of its wave",()=>{
 expect(childSource).toContain('"pool-published-"');expect(childSource).toContain("fleetGlobalsKeyV1(head.globals)");expect(childSource).toContain("appendFileSync(published, unit.name");
});

test("a reader lists only the owners of waves keyed with its own globals, and the key equals an independent digest",async()=>{
 const mine={registryDigest:"bb",commandRevision:"cc",late:true},other={...mine,commandRevision:"dd"};
 const python=spawnSync("python3",["-I","-c","import hashlib,json,sys;print(hashlib.sha256(json.dumps(json.loads(sys.argv[1]),separators=(',',':')).encode()).hexdigest()[:8])",JSON.stringify(mine)],{encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);expect(fleetGlobalsKeyV1(mine)).toBe(python.stdout.trim());expect(fleetGlobalsKeyV1(other)).not.toBe(fleetGlobalsKeyV1(mine));
 const root=store();
 publish(root,"g1-m01",`a1-${fleetGlobalsKeyV1(mine)}`,["a","b"]);publish(root,"g1-m02",`b2-${fleetGlobalsKeyV1(other)}`,["c"]);publish(root,"g1-m03","c3",["d"]);
 expect([...await fleetPublishedNamesV1(root,Date.now(),TAKE_POLICY_V1,mine)].sort()).toEqual(["a","b"]);
 expect([...await fleetPublishedNamesV1(root,Date.now(),TAKE_POLICY_V1,other)]).toEqual(["c"]);
 expect([...await fleetPublishedNamesV1(root)].sort()).toEqual(["a","b","c","d"]);
});

test("a scan of a store of 6,000 files of 12 origins never holds the event loop and equals an independent Python scan",async()=>{
 const mine={registryDigest:"r",commandRevision:"c",late:false},key=fleetGlobalsKeyV1(mine),root=store();
 try{
  for(let origin=0;origin<12;origin++)for(let wave=0;wave<500;wave++){const old=wave%3===0;publish(root,"g1-m"+origin,`${(origin*1000+wave).toString(16).padStart(12,"0")}${wave%5===0?"":"-"+key}`,["o"+origin+"-"+wave,"shared"+wave%7],old?TAKE_POLICY_V1.publishedMaximumAgeMilliseconds+60000:0);}
  let last=performance.now(),longest=0,ticks=0;const timer=setInterval(()=>{const at=performance.now();longest=Math.max(longest,at-last);last=at;ticks++;},1);
  const now=Date.now(),names=await fleetPublishedNamesV1(root,now,TAKE_POLICY_V1,mine);clearInterval(timer);
  const python=spawnSync("python3",["-I","-c","import os,re,sys,json\nroot,key,now,age=sys.argv[1],sys.argv[2],float(sys.argv[3]),float(sys.argv[4])\nnames=set()\nbase=os.path.join(root,'v1','origins')\nfor origin in os.listdir(base):\n for entry in os.scandir(os.path.join(base,origin)):\n  m=re.fullmatch(r'pool-published-[0-9a-f]+(?:-([0-9a-f]{8}))?\\.txt',entry.name)\n  if not m or m.group(1)!=key: continue\n  if now-entry.stat().st_mtime*1000>age: continue\n  names.update(line for line in open(entry.path,encoding='utf-8').read().split('\\n') if line)\nprint(json.dumps(sorted(names)))",root,key,String(now),String(TAKE_POLICY_V1.publishedMaximumAgeMilliseconds)],{encoding:"utf8",maxBuffer:1<<24});
  expect(python.status,python.stderr).toBe(0);expect([...names].sort()).toEqual(JSON.parse(python.stdout));
  expect(names.size).toBeGreaterThan(3000);expect(ticks).toBeGreaterThan(3);expect(longest).toBeLessThan(500);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
