import {expect,test} from "bun:test";
import Ajv from "ajv";
import {isIP} from "node:net";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,writeFileSync,readFileSync,readdirSync,rmSync,statSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import {advertiseFleetServiceV1,applyFleetHostnameV1,fleetHostnameCommandV1,readFleetOwnOriginV1,collectFleetAddressesV1,discoverFleetServiceV1,fleetAddressMirrorPathV1,fleetAdvertiseCommandV1,fleetAdvertiseGateV1,fleetHostLabelV1,fleetServiceRecordV1,fleetTcpProbeV1,parseAvahiBrowseV1,parseDnsSdAddressesV1,parseDnsSdLookupV1,connectFleetEndpointV1,createFleetAddressSetV1,fleetAddressPathV1,fleetAddressScopeV1,fleetAddressTextV1,fleetEndpointCandidatesV1,fleetLinkLocalV1,fleetNameV1,fleetSameNetworkV1,fleetUrlHostV1,orderFleetAddressesV1,publishFleetAddressSetV1,readFleetAddressSetV1,readFleetLastGoodV1,requireFleetAddressSetV1,resolveFleetEndpointV1,watchFleetAddressSetV1,type FleetAddressV1} from "../🟦️.ts";

type Rows=ReturnType<typeof import("node:os").networkInterfaces>;
const row=(address:string,interfaceName:string,netmask:string,family:"IPv4"|"IPv6"="IPv4",internal=false)=>({address,netmask,family,mac:"00:00:00:00:00:00",internal,cidr:null}) as never;
const home=(prefix:string,third:string):Rows=>({lo0:[row("127.0.0.1","lo0","255.0.0.0","IPv4",true)],en0:[row(`${prefix}.${third}.70`,"en0","255.255.255.0"),row("fe80::1234","en0","ffff:ffff:ffff:ffff::","IPv6")],en1:[row(`${prefix}.${third}.71`,"en1","255.255.255.0"),row("fd12:3456::71","en1","ffff:ffff:ffff:ffff::","IPv6")]}) as Rows;
const scratch=():string=>mkdtempSync(join(tmpdir(),"semio-addresses-"));

test("address set schema: a built set validates, a stray key or an IP in the wrong family does not",()=>{
 const validate=new Ajv({strict:false}).compile(schema);
 const set=createFleetAddressSetV1({origin:"g1-server",interfaces:home("10.1","2"),now:1000,generation:3});
 expect(validate(set)).toBe(true);
 expect(requireFleetAddressSetV1(JSON.parse(JSON.stringify(set)))).toEqual(set);
 expect(set.name).toBe("semio-g1-server");
 expect(set.addresses.map(item=>item.address)).toEqual(["10.1.2.70","10.1.2.71","fd12:3456::71"]);
 expect(set.addresses.some(item=>fleetLinkLocalV1(item.address)||item.address.startsWith("127."))).toBe(false);
 const stray={...set,extra:1};
 expect(validate(stray)).toBe(false);
 expect(()=>requireFleetAddressSetV1(stray)).toThrow("Closed fleet address set required");
 expect(()=>requireFleetAddressSetV1({...set,addresses:[{...set.addresses[0]!,family:"IPv6"}]})).toThrow("family");
 expect(()=>requireFleetAddressSetV1({...set,addresses:[{...set.addresses[0]!,address:"169.254.1.1"}]})).toThrow("address");
 expect(validate({...set,name:"Bad Name"})).toBe(false);
 expect(()=>requireFleetAddressSetV1({...set,name:"semio-other"})).toThrow("name");
});

test("classification agrees with node:net and an independent Python ipaddress oracle",()=>{
 const samples=["10.0.0.1","172.16.5.4","172.32.0.1","192.168.178.70","8.8.8.8","169.254.7.7","127.0.0.1","fe80::1","fd00::1","2001:db8::5","::1"];
 for(const sample of samples)expect(isIP(sample)).not.toBe(0);
 const python=spawnSync("python3",["-I","-c","import ipaddress,json,sys\nout={}\nfor a in sys.argv[1:]:\n i=ipaddress.ip_address(a)\n out[a]=[i.is_link_local,i.is_loopback,i.is_private and not i.is_loopback and not i.is_link_local]\nprint(json.dumps(out))",...samples],{encoding:"utf8"});
 if(python.status!==0)return;
 const oracle=JSON.parse(python.stdout) as Record<string,[boolean,boolean,boolean]>;
 for(const sample of samples){
  expect(fleetLinkLocalV1(sample)).toBe(oracle[sample]![0]);
  if(!oracle[sample]![0]&&!oracle[sample]![1])expect(fleetAddressScopeV1(sample)!=="global").toBe(oracle[sample]![2]&&!["172.32.0.1","2001:db8::5"].includes(sample));
 }
});

test("publication is atomic, idempotent, bumps the generation on change and refreshes the stamp at half the time to live",async()=>{
 const store=scratch();
 try{
  const base={storeRoot:store,origin:"g1-m07",interfaces:home("10.1","2")};
  const first=await publishFleetAddressSetV1({...base,now:1000,ttlMs:10_000});
  expect(first.changed).toBe(true);expect(first.set.generation).toBe(1);
  expect(readdirSync(join(store,"v1","origins","g1-m07")).filter(name=>name.endsWith(".tmp"))).toEqual([]);
  expect((await publishFleetAddressSetV1({...base,now:2000,ttlMs:10_000})).changed).toBe(false);
  const refreshed=await publishFleetAddressSetV1({...base,now:7000,ttlMs:10_000});
  expect(refreshed.refreshed).toBe(true);expect(refreshed.set.generation).toBe(1);expect(refreshed.set.observedAtMs).toBe(7000);
  const moved=await publishFleetAddressSetV1({...base,interfaces:home("172.20","9"),now:8000,ttlMs:10_000});
  expect(moved.changed).toBe(true);expect(moved.set.generation).toBe(2);
  const read=await readFleetAddressSetV1(store,"g1-m07",9000);
  expect(read?.fresh).toBe(true);expect(read?.set.addresses[0]?.address).toBe("172.20.9.70");
  expect((await readFleetAddressSetV1(store,"g1-m07",8000+10_001))?.fresh).toBe(false);
  expect(await readFleetAddressSetV1(store,"g1-m08",1)).toBeUndefined();
  expect(existsSync(fleetAddressPathV1(store,"g1-m07"))).toBe(true);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("the watcher republishes when the interfaces change and reports in the chosen language",async()=>{
 const store=scratch(),lines:string[]=[],control=new AbortController();
 try{
  let current=home("10.1","2"),turns=0,time=1000;
  await watchFleetAddressSetV1({storeRoot:store,origin:"g1-m07",interfaces:()=>current,signal:control.signal,language:"de",report:line=>lines.push(line),now:()=>time,wait:async()=>{turns++;time+=1000;if(turns===1)current=home("172.20","9");if(turns===3)control.abort();}});
  expect(lines).toHaveLength(2);
  expect(lines[0]).toContain("veröffentlicht");
  expect(lines[1]).toContain("geändert");
  expect((await readFleetAddressSetV1(store,"g1-m07",time))?.set.generation).toBe(2);
 }finally{rmSync(store,{recursive:true,force:true});}
});

const peerSet=(prefix:string,third:string,now=1000)=>createFleetAddressSetV1({origin:"g1-server",interfaces:home(prefix,third),now});
const mine=(prefix:string,third:string,last="5"):readonly FleetAddressV1[]=>collectFleetAddressesV1({en0:[row(`${prefix}.${third}.${last}`,"en0","255.255.255.0")]} as Rows);

test("resolver orders the same subnet first, then scope and family; IPv6 is bracketed; names follow the addresses",()=>{
 const set=peerSet("10.1","2");
 const ordered=orderFleetAddressesV1(set.addresses,mine("10.1","2"));
 expect(ordered.map(item=>item.address)).toEqual(["10.1.2.70","10.1.2.71","fd12:3456::71"]);
 const other=orderFleetAddressesV1([...set.addresses].reverse(),mine("10.1","3"));
 expect(other.map(item=>item.family)).toEqual(["IPv4","IPv4","IPv6"]);
 const two=createFleetAddressSetV1({origin:"g1-server",now:1,interfaces:{en0:[row("10.9.9.70","en0","255.255.255.0")],en1:[row("10.1.2.71","en1","255.255.255.0")]} as Rows});
 expect(orderFleetAddressesV1(two.addresses,mine("10.1","2")).map(item=>item.address)).toEqual(["10.1.2.71","10.9.9.70"]);
 expect(fleetSameNetworkV1("10.1.2.70","10.1.3.70",24)).toBe(false);
 expect(fleetSameNetworkV1("10.1.2.70","10.1.3.70",16)).toBe(true);
 expect(fleetUrlHostV1("fd12:3456::71")).toBe("[fd12:3456::71]");
 expect(fleetEndpointCandidatesV1({set,local:mine("10.1","2"),port:6094})).toEqual(["http://10.1.2.70:6094","http://10.1.2.71:6094","http://[fd12:3456::71]:6094"]);
 expect(fleetEndpointCandidatesV1({set:undefined,local:[],port:6094})).toEqual([]);
 expect(fleetEndpointCandidatesV1({set,local:mine("10.1","2"),port:6094,lastGood:"http://10.1.2.71:6094"})[0]).toBe("http://10.1.2.71:6094");
 expect(fleetNameV1("g1-m01")).toBe("semio-g1-m01");
 expect(()=>fleetNameV1("Ueli Mac")).toThrow("Declared fleet origin");
});

test("failover: a dead first address, a hanging second one and a working third; the answer is remembered and tried first next time",async()=>{
 const root=scratch(),store=scratch();
 try{
  await publishFleetAddressSetV1({storeRoot:store,origin:"g1-server",interfaces:home("10.1","2"),now:1000});
  const seen:string[]=[],alive=new Set(["http://[fd12:3456::71]:6094"]);
  const probe=async(endpoint:string,signal:AbortSignal):Promise<boolean>=>{seen.push(endpoint);if(endpoint.includes("10.1.2.71"))return await new Promise<boolean>(resolve=>signal.addEventListener("abort",()=>resolve(false)));if(endpoint.includes("10.1.2.70"))throw Error("refused");return alive.has(endpoint);};
  const lines:string[]=[];
  const found=await resolveFleetEndpointV1({root,storeRoot:store,origin:"g1-server",service:"cache",port:6094,local:mine("10.1","2"),probe,timeoutMs:40,now:2000,report:line=>lines.push(line),language:"en"});
  expect(found?.endpoint).toBe("http://[fd12:3456::71]:6094");
  expect(found?.attempts.map(item=>item.ok)).toEqual([false,false,true]);
  expect(lines).toHaveLength(3);
  expect(lines[2]).toContain("via published");
  expect(await readFleetLastGoodV1(root,"g1-server/cache")).toBe("http://[fd12:3456::71]:6094");
  seen.length=0;
  await resolveFleetEndpointV1({root,storeRoot:store,origin:"g1-server",service:"cache",port:6094,local:mine("10.1","2"),probe,timeoutMs:40,now:2000});
  expect(seen[0]).toBe("http://[fd12:3456::71]:6094");
  expect(await connectFleetEndpointV1({candidates:["http://a:1","http://b:1"],probe:async()=>false,timeoutMs:10})).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});rmSync(store,{recursive:true,force:true});}
});

test("subnet change: the server moves to another network, republishes, and the client reaches it without any configured address",async()=>{
 const root=scratch(),store=scratch();
 try{
  await publishFleetAddressSetV1({storeRoot:store,origin:"g1-server",interfaces:home("10.1","2"),now:1000});
  const old="http://10.1.2.70:6094";
  const first=await resolveFleetEndpointV1({root,storeRoot:store,origin:"g1-server",service:"cache",port:6094,local:mine("10.1","2"),probe:async endpoint=>endpoint===old,now:1500});
  expect(first?.endpoint).toBe(old);
  await publishFleetAddressSetV1({storeRoot:store,origin:"g1-server",interfaces:home("172.20","9"),now:2000});
  const probes:string[]=[],moved=await resolveFleetEndpointV1({root,storeRoot:store,origin:"g1-server",service:"cache",port:6094,local:mine("172.20","9","40"),probe:async endpoint=>{probes.push(endpoint);return endpoint==="http://172.20.9.70:6094";},now:2500});
  expect(probes[0]).toBe("http://172.20.9.70:6094");
  expect(moved?.endpoint).toBe("http://172.20.9.70:6094");
  const lines:string[]=[],stale=await resolveFleetEndpointV1({root,storeRoot:store,origin:"g1-server",service:"cache",port:6094,local:mine("172.20","1"),probe:async endpoint=>endpoint==="http://172.20.9.99:6094",discover:async()=>["172.20.9.99"],now:2000+400_000,report:line=>lines.push(line),language:"en"});
  expect(stale?.endpoint).toBe("http://172.20.9.99:6094");
  expect(stale?.via).toBe("dns-sd");
  expect(stale?.attempts.map(item=>item.ok)).toEqual([false,true]);
  expect(lines.some(line=>line.includes("via dns-sd"))).toBe(true);
  expect(fleetAddressScopeV1("100.64.0.9")).toBe("private");
 }finally{rmSync(root,{recursive:true,force:true});rmSync(store,{recursive:true,force:true});}
});

test("every message exists in English and German with the same variables",()=>{
 for(const [key,message] of Object.entries(texts.messages)){
  const variables=(text:string)=>[...text.matchAll(/\{([a-z0-9]+)\}/gu)].map(match=>match[1]).sort().join();
  expect(variables(message.en)).toBe(variables(message.de));
  expect(message.de).not.toBe(message.en);
  const filled=Object.fromEntries(variables(message.en).split(",").filter(Boolean).map(name=>[name,"x"]));
  expect(fleetAddressTextV1("en",key,filled)).not.toContain("{");
 }
 expect(()=>fleetAddressTextV1("en","addresses.none",{})).toThrow("variable missing");
});

const ROOT=resolve(import.meta.dir,"..","..","..","..");
const FLEET_ROOTS=[
 join(ROOT,"🤝️shared-worktree"),
 join(ROOT,"📚️library","🏗️build","🌐️remote-cache"),
 join(ROOT,"📚️library","🏗️build","🌐️remote-execution"),
 join(ROOT,"📚️library","🏗️build","🔣️policy.json")
];
const SKIP=new Set(["node_modules","🧪️tests","🧫️fixtures","🗑️generated","target","dist",".cache"]);
const TEXT=/\.(json|ts|mjs|sh|ps1|md|plist|service|xml|toml|yaml|yml)$/u;
const ALLOWED=new Set(["0.0.0.0","127.0.0.1","255.255.255.0","255.255.255.255","255.0.0.0","255.255.0.0"]);
const QUAD=/(?<![0-9.])((?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])(?:\.(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])){3})(?![0-9]|\.[0-9])/gu;
const walk=(path:string,found:string[]):void=>{
 if(!existsSync(path))return;
 if(statSync(path).isFile()){if(TEXT.test(path))found.push(path);return;}
 for(const name of readdirSync(path))if(!SKIP.has(name))walk(join(path,name),found);
};

test("law: no IPv4 literal in topology, config, service definitions or fleet code (test fixtures are exempt)",()=>{
 const files:string[]=[];
 for(const base of FLEET_ROOTS)walk(base,files);
 expect(files.length).toBeGreaterThan(50);
 const offences:string[]=[];
 for(const file of files){
  const lines=readFileSync(file,"utf8").split("\n");
  lines.forEach((line,index)=>{for(const match of line.matchAll(QUAD))if(!ALLOWED.has(match[1]!)&&!/version|semver|\bv[0-9]/iu.test(line.slice(Math.max(0,(match.index??0)-12),match.index)))offences.push(`${relative(ROOT,file)}:${index+1}: ${match[1]}`);});
 }
 expect(offences).toEqual([]);
});

test("law: the committed topology names machines and roles by origin id; no address, no network, no operating system host name",()=>{
 const topology=JSON.parse(readFileSync(join(ROOT,"🤝️shared-worktree","🗺️topology","🔣️.json"),"utf8")) as {groups:{id:string;roles:Record<string,string>;nfs:Record<string,unknown>;server:{network?:string;machines?:string[];advertise?:string[]};cache:Record<string,unknown>}[]};
 for(const group of topology.groups){
  expect(Object.keys(group.roles).sort()).toEqual(["cache","relay","scheduler","store"]);
  for(const origin of Object.values(group.roles)){expect(origin).toMatch(/^[a-z][a-z0-9-]{0,63}$/u);expect(origin.startsWith(`${group.id}-`)).toBe(true);expect(isIP(origin)).toBe(0);}
  expect(Object.keys(group.nfs)).toEqual(["export"]);
  expect(Object.keys(group.cache)).toEqual(["instance"]);
  expect(group.server.network).toBeUndefined();expect(group.server.machines).toBeUndefined();expect(group.server.advertise).toBeUndefined();
 }
});

const LABEL="(?:[A-Za-z0-9]*[-0-9A-Z][A-Za-z0-9-]*)",HOST_NAME=new RegExp(`(?<![A-Za-z0-9_.$~/-])${LABEL}\\.local(?![A-Za-z0-9_/-])|//[A-Za-z0-9.-]*\\.local\\b|\\buelis?-[A-Za-z]`,"u"),CODE_HOST_NAME=new RegExp(`["'\`][^"'\`\\s]*(?:${LABEL}|//[A-Za-z0-9.-]*)\\.local(?![A-Za-z0-9_/-])[^"'\`\\s]*["'\`]|\\buelis?-[A-Za-z]`,"u");
const DOCUMENT_HOST_NAME=/\buelis?-[A-Za-z]|\/\/[A-Za-z0-9.-]*\.local\b/iu;
const hostNameIn=(file:string,line:string):boolean=>(file.endsWith(".ts")||file.endsWith(".mjs")?CODE_HOST_NAME:file.endsWith(".md")?DOCUMENT_HOST_NAME:HOST_NAME).test(line);

test("law: no operating system host name (`*.local`, `ueli-*`) in the topology, the configuration, the service definitions or the fleet code (tests and fixtures are exempt)",()=>{
 const files:string[]=[];
 for(const base of FLEET_ROOTS)walk(base,files);
 expect(files.length).toBeGreaterThan(50);
 const offences:string[]=[];
 files.forEach(file=>readFileSync(file,"utf8").split("\n").forEach((line,index)=>{if(file!==import.meta.path&&hostNameIn(file,line))offences.push(`${relative(ROOT,file)}:${index+1}: ${line.trim().slice(0,100)}`);}));
 expect(offences).toEqual([]);
 for(const bad of["ueli-mac.local","Studio-7.local","uelis-mac-mini","http://studio.local:6094/x","x ueli-mac"])expect(hostNameIn("x.json",bad),bad).toBe(true);
 expect(hostNameIn("x.ts",'const host="studio-7.local";')).toBe(true);expect(hostNameIn("x.ts","const a=this.local&&b.local;")).toBe(false);
 for(const fine of["$HOME/.local/bin","~/.local/share","a/.local/b","local","the .local suffix","name.localhost"])expect(hostNameIn("x.sh",fine),fine).toBe(false);
});

test("DNS-SD: the record is named by the origin and carries group, roles and ports; the platform tools are the ones that need no sudo",()=>{
 const record=fleetServiceRecordV1({origin:"g1-server",group:"g1",roles:{store:2049,cache:6094,scheduler:8980,relay:0}});
 expect(record).toEqual({instance:"g1-server",type:"_semio-fleet._tcp",port:6094,txt:["origin=g1-server","name=semio-g1-server","group=g1","roles=cache,relay,scheduler,store","cache=6094","relay=0","scheduler=8980","store=2049"]});
 expect(fleetServiceRecordV1({origin:"g1-m01",group:"g1",roles:{member:0}}).port).toBe(0);
 expect(()=>fleetServiceRecordV1({origin:"g1-m01",group:"g1",roles:{}})).toThrow("roles");
 expect(()=>fleetServiceRecordV1({origin:"Ueli Mac",group:"g1",roles:{member:0}})).toThrow("origin");
 expect(fleetAdvertiseCommandV1(record,"darwin")).toEqual({command:"dns-sd",args:["-R","g1-server","_semio-fleet._tcp","local","6094",...record.txt]});
 expect(fleetAdvertiseCommandV1(record,"win32").command).toBe("dns-sd.exe");
 expect(fleetAdvertiseCommandV1(record,"linux")).toEqual({command:"avahi-publish",args:["-s","g1-server","_semio-fleet._tcp","6094",...record.txt]});
});

test("DNS-SD: the answers of dns-sd and avahi-browse are read; link-local and loopback addresses are left out",async()=>{
 const lookup="Lookup g1-server._semio-fleet._tcp.local\nDATE: ---Wed 25 Nov 2026---\n21:50:01.456  g1-server._semio-fleet._tcp.local. can be reached at Studio-7.local.:6094 (interface 4)\n name=semio-g1-server group=g1 roles=cache,store cache=6094\n";
 expect(parseDnsSdLookupV1(lookup)).toEqual({host:"Studio-7.local",port:6094,txt:{name:"semio-g1-server",group:"g1",roles:"cache,store",cache:"6094"}});
 expect(parseDnsSdLookupV1("nothing")).toBeUndefined();
 const addresses="Timestamp     A/R    Flags         IF  Hostname  Address  TTL\n21:50:02.345  Add  2  4  Studio-7.local.  10.1.2.70  120\n21:50:02.346  Add  2  4  Studio-7.local.  fe80::1%en0  120\n21:50:02.347  Add  2  4  Studio-7.local.  fd12:3456::71  120\n21:50:02.348  Rmv  0  4  Studio-7.local.  10.9.9.9  120\n";
 expect(parseDnsSdAddressesV1(addresses)).toEqual(["10.1.2.70","fd12:3456::71"]);
 const avahi="+;en0;IPv4;g1-server;_semio-fleet._tcp;local\n=;en0;IPv4;g1-server;_semio-fleet._tcp;local;studio-7.local;10.1.2.70;6094;\"name=semio-g1-server\" \"group=g1\"\n=;en0;IPv6;g1-server;_semio-fleet._tcp;local;studio-7.local;fe80::1;6094;\"group=g1\"\n=;en0;IPv4;g1-m02;_semio-fleet._tcp;local;m02.local;10.1.2.72;0;\"group=g1\"\n";
 expect(parseAvahiBrowseV1(avahi,"g1-server")).toEqual({addresses:["10.1.2.70"],port:6094,txt:{name:"semio-g1-server",group:"g1"}});
 expect(parseAvahiBrowseV1(avahi,"g1-m09")).toBeUndefined();
 const calls:string[]=[];
 const mac=await discoverFleetServiceV1({instance:"g1-server",platform:"darwin",run:async(command,args)=>{calls.push(`${command} ${args.join(" ")}`);return args[0]==="-L"?lookup:addresses;}});
 expect(mac).toEqual({addresses:["10.1.2.70","fd12:3456::71"],port:6094,txt:{name:"semio-g1-server",group:"g1",roles:"cache,store",cache:"6094"}});
 expect(calls).toEqual(["dns-sd -L g1-server _semio-fleet._tcp local","dns-sd -G v4v6 Studio-7.local"]);
 expect((await discoverFleetServiceV1({instance:"g1-server",platform:"linux",run:async()=>avahi}))?.addresses).toEqual(["10.1.2.70"]);
 expect(await discoverFleetServiceV1({instance:"g1-server",platform:"linux",run:async()=>""})).toBeUndefined();
});

test("DNS-SD: the advertiser follows the conflict gate, says so in the chosen language, and a missing tool is reported once",async()=>{
 const record=fleetServiceRecordV1({origin:"g1-m04",group:"g1",roles:{member:0}}),lines:string[]=[];
 let enabled=false,turns=0,running=0,starts=0;const control=new AbortController();
 await advertiseFleetServiceV1({record,signal:control.signal,language:"en",report:line=>lines.push(line),platform:"darwin",enabled:async()=>enabled,
  run:async(_command,_args,signal,onStart)=>{starts++;running++;onStart();await new Promise<void>(resolve=>signal.addEventListener("abort",()=>resolve(),{once:true}));running--;return 0;},
  wait:async()=>{turns++;if(turns===1)enabled=true;if(turns===3){expect(running).toBe(1);enabled=false;}if(turns===5){expect(running).toBe(0);control.abort();}}});
 expect(starts).toBe(1);
 expect(lines.filter(line=>line.startsWith("Advertising g1-m04"))).toHaveLength(1);
 expect(lines.some(line=>line.startsWith("Stopped advertising g1-m04"))).toBe(true);
 const missing:string[]=[],again=new AbortController();
 let round=0;
 await advertiseFleetServiceV1({record,signal:again.signal,language:"de",report:line=>missing.push(line),platform:"linux",enabled:async()=>true,run:async()=>-1,wait:async()=>{if(++round===4)again.abort();}});
 expect(missing).toHaveLength(1);
 expect(missing[0]).toContain("avahi-publish");expect(missing[0]).toContain("gemeinsamen Speicher");
});

test("the conflict gate: unique host names advertise; a duplicate or a macOS conflict rename, an unusable name and an alive origin without a published set keep a machine silent and name the duplicates",()=>{
 expect(fleetHostLabelV1("Uelis-Mac-mini.local")).toBe("uelis-mac-mini");
 expect(fleetHostLabelV1("Uelis Mac mini.fritz.box")).toBe("uelis-mac-mini");
 expect(fleetHostLabelV1("..")).toBeNull();
 const others=[{origin:"g1-m08",hostLabel:"uelis-mac-mini"},{origin:"g1-m10",hostLabel:"uelis-mac-mini"},{origin:"g1-m01",hostLabel:"semio-g1-m01"},{origin:"g1-m02",hostLabel:null}];
 expect(fleetAdvertiseGateV1({origin:"g1-m13",hostLabel:"uelis-mac-mini",others})).toEqual({enabled:false,duplicates:["g1-m08","g1-m10"],namedAs:null});
 expect(fleetAdvertiseGateV1({origin:"g1-m13",hostLabel:"uelis-mac-mini-2",others}),"the name macOS gave after a conflict").toEqual({enabled:false,duplicates:["g1-m08","g1-m10"],namedAs:null});
 expect(fleetAdvertiseGateV1({origin:"g1-m09",hostLabel:"semio-g1-m09",others})).toEqual({enabled:true,duplicates:[],namedAs:null});
 expect(fleetAdvertiseGateV1({origin:"g1-m09",hostLabel:null,others}).enabled).toBe(false);
 expect(fleetAdvertiseGateV1({origin:"g1-m09",hostLabel:"semio-g1-m09",others,unverified:["g1-m04"]}).enabled,"an alive origin whose name is not known yet").toBe(false);
 expect(fleetAdvertiseGateV1({origin:"g1-m08",hostLabel:"uelis-mac-mini",others:[{origin:"g1-m08",hostLabel:"uelis-mac-mini"}]}).enabled,"a machine is no duplicate of itself").toBe(true);
 expect(fleetAdvertiseGateV1({origin:"g1-server",hostLabel:"semio-g1-m08",others}),"a copy-pasted name of another origin").toEqual({enabled:false,duplicates:[],namedAs:"g1-m08"});
 expect(fleetAdvertiseGateV1({origin:"g1-m08",hostLabel:"semio-g1-m08",others:[]})).toEqual({enabled:true,duplicates:[],namedAs:null});
});

test("the watcher publishes the host label and the gate, warns in the chosen language about a duplicate",async()=>{
 const store=scratch(),lines:string[]=[],control=new AbortController();
 try{
  await publishFleetAddressSetV1({storeRoot:store,origin:"g1-m08",interfaces:home("10.1","2"),now:500,hostLabel:"uelis-mac-mini",advertise:{enabled:false,duplicates:["g1-m10"],namedAs:null}});
  let turns=0,time=1000;
  await watchFleetAddressSetV1({storeRoot:store,origin:"g1-m10",hostname:"Uelis-Mac-mini.local",interfaces:()=>home("10.1","3"),signal:control.signal,language:"de",report:line=>lines.push(line),now:()=>time,wait:async()=>{turns++;time+=1000;if(turns===2)control.abort();}});
  const own=await readFleetAddressSetV1(store,"g1-m10",time);
  expect(own?.set.hostLabel).toBe("uelis-mac-mini");
  expect(own?.set.advertise).toEqual({enabled:false,duplicates:["g1-m08"],namedAs:null});
  expect(lines.some(line=>line.includes("g1-m08")&&line.includes("semio-g1-m10"))).toBe(true);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("no mDNS: the published addresses answer, from the store and, while the store is unreadable, from the copy on this machine; the path is reported",async()=>{
 const root=scratch(),store=scratch(),lines:string[]=[];
 try{
  await publishFleetAddressSetV1({storeRoot:store,origin:"g1-server",interfaces:home("10.1","2"),now:1000});
  const up="http://10.1.2.70:6094",probe=async(endpoint:string):Promise<boolean>=>endpoint===up;
  const discover=async():Promise<readonly string[]>=>{throw Error("no mDNS on this machine");};
  const first=await resolveFleetEndpointV1({root,storeRoot:store,origin:"g1-server",service:"cache",port:6094,local:mine("10.1","2"),probe,discover,now:1500,report:line=>lines.push(line),language:"en"});
  expect(first?.via).toBe("published");
  expect(existsSync(fleetAddressMirrorPathV1(root,"g1-server"))).toBe(true);
  const gone=join(store,"unmounted"),second=await resolveFleetEndpointV1({root,storeRoot:gone,origin:"g1-server",service:"cache",port:6094,local:mine("10.1","2"),probe,discover,now:1600});
  expect(second?.via).toBe("last-good");
  rmSync(join(root,".cache","f","last-good.json"));
  const third=await resolveFleetEndpointV1({root,storeRoot:gone,origin:"g1-server",service:"cache",port:6094,local:mine("10.1","2"),probe,discover,now:1700,report:line=>lines.push(line),language:"de"});
  expect(third?.via).toBe("mirror");
  const none=await resolveFleetEndpointV1({root,origin:"g1-unknown",service:"cache",port:6094,local:[],probe,discover,report:line=>lines.push(line),language:"en"});
  expect(none).toBeUndefined();
  expect(lines.filter(line=>line.includes("via published")).length).toBe(1);
  expect(lines.some(line=>line.includes("über mirror"))).toBe(true);
  expect(lines.some(line=>line.includes("None of the 0 endpoints of g1-unknown"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(store,{recursive:true,force:true});}
});

test("a TCP probe tells a listening port from a closed one",async()=>{
 const {createServer}=await import("node:net"),server=createServer(),port=await new Promise<number>(resolve=>server.listen(0,"127.0.0.1",()=>resolve((server.address() as {port:number}).port)));
 try{
  expect(await fleetTcpProbeV1(`http://127.0.0.1:${port}`,new AbortController().signal)).toBe(true);
 }finally{await new Promise<void>(resolve=>server.close(()=>resolve()));}
 expect(await fleetTcpProbeV1(`http://127.0.0.1:${port}`,new AbortController().signal)).toBe(false);
 expect(await fleetTcpProbeV1("not a url",new AbortController().signal)).toBe(false);
});

const PRODUCT=resolve(ROOT,"..");
const SPAWN=/(spawn\w*|exec\w*|\$)\s*\(?\s*\[?\s*["'`](dns-sd(\.exe)?|avahi-[a-z-]+)["'`]/u;

test("law: no test file spawns dns-sd or avahi (discovery and advertising are ports that tests stub)",()=>{
 const files:string[]=[],walkTests=(path:string,inTests:boolean):void=>{
  if(statSync(path).isFile()){if(inTests&&/\.(ts|mjs|py|sh)$/u.test(path))files.push(path);return;}
  for(const name of readdirSync(path)){if(name==="node_modules"||name==="target"||name==="dist"||name===".cache"||name==="🗑️generated")continue;walkTests(join(path,name),inTests||name==="🧪️tests");}
 };
 walkTests(PRODUCT,false);
 expect(files.length).toBeGreaterThan(100);
 const offences=files.filter(file=>file!==import.meta.path&&SPAWN.test(readFileSync(file,"utf8"))).map(file=>relative(PRODUCT,file));
 expect(offences).toEqual([]);
 expect(SPAWN.test('Bun.spawn(["dns-sd","-R","x"])')).toBe(true);expect(SPAWN.test("spawnSync('avahi-publish',['-s'])")).toBe(true);expect(SPAWN.test('run("dns-sd",args)')).toBe(false);
});

test("hostname: the name is derived from the origin id in machine.json, never typed; it refuses without the file, is idempotent, asks before it changes anything and runs the platform tool only after a yes",async()=>{
 const root=scratch(),lines:string[]=[],calls:string[]=[];
 try{
  const base={root,language:"en" as const,report:(line:string)=>lines.push(line),run:async(command:string,args:readonly string[])=>{calls.push([command,...args].join(" "));return 0;}};
  await expect(applyFleetHostnameV1({...base,confirm:async()=>true,platform:"darwin",current:()=>undefined})).rejects.toThrow("machine.json is missing");
  await expect(applyFleetHostnameV1({...base,language:"de",confirm:async()=>true,platform:"darwin",current:()=>undefined})).rejects.toThrow("fehlt");
  mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-server",groupId:"g1"}));
  expect(readFleetOwnOriginV1(root,"en")).toBe("g1-server");
  expect(await applyFleetHostnameV1({...base,confirm:async()=>false,platform:"darwin",current:()=>"Uelis-Mac-mini"})).toBe("declined");
  expect(calls).toEqual([]);
  expect(await applyFleetHostnameV1({...base,confirm:async question=>{expect(question).toContain("semio-g1-server");return true;},platform:"darwin",current:()=>"Uelis-Mac-mini"})).toBe("applied");
  expect(calls).toEqual(["sudo scutil --set LocalHostName semio-g1-server"]);
  expect(await applyFleetHostnameV1({...base,confirm:async()=>{throw Error("must not ask");},platform:"darwin",current:()=>"Semio-G1-Server"})).toBe("already");
  expect(calls).toHaveLength(1);
  expect(await applyFleetHostnameV1({...base,confirm:async()=>true,platform:"linux",current:()=>"box"})).toBe("applied");
  expect(calls[1]).toBe("sudo hostnamectl set-hostname semio-g1-server");
  expect(await applyFleetHostnameV1({...base,confirm:async()=>true,platform:"win32",current:()=>undefined})).toBe("unsupported");
  await expect(applyFleetHostnameV1({...base,run:async()=>1,confirm:async()=>true,platform:"darwin",current:()=>"x"})).rejects.toThrow("ended with status 1");
  expect(lines.some(line=>line.includes("This machine is g1-server")&&line.includes("semio-g1-server"))).toBe(true);
  expect(fleetHostnameCommandV1("semio-g1-m02","darwin")).toEqual({command:"sudo",args:["scutil","--set","LocalHostName","semio-g1-m02"]});
 }finally{rmSync(root,{recursive:true,force:true});}
});
