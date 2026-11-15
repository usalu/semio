import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {cpSync,existsSync,utimesSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {FLEET_FETCH_POLICY_V1,fleetFetchContextV1} from "../../🟦️.ts";
import {syncFleetIdentityV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {fleetInstallIndexV1,fleetInstallSeedV1,fleetInstallFingerprintV1,fleetInstallLinkTextV1,rewriteFleetInstallLinksV1,pruneFleetInstallsV1,fleetInstallAcquireV1,fleetInstallKeyV1,fleetInstallManifestV1,fleetInstallPublishV1,fleetInstallRestoreV1,fleetInstallScopeV1,fleetInstallHostV1,fleetInstallRuntimeV1,type FleetInstallRuntimeV1,type FleetInstallRequestV1,type FleetInstallShapeV1} from "../🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-install-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const roots=["node_modules","w/0/node_modules","w/1/node_modules"];
const runtime={bun:"1.4.2",platform:"darwin" as const,arch:"arm64",recipeSha256:"a".repeat(64)};
const projection="b".repeat(64);

const populate=(generation:string)=>{
 for(const root of roots)mkdirSync(join(generation,...root.split("/")),{recursive:true});
 for(let index=0;index<60;index++){const folder=join(generation,"node_modules",`pkg-${index}`);mkdirSync(folder,{recursive:true});writeFileSync(join(folder,"index.js"),`module.exports=${index};\n`.repeat(40+index));writeFileSync(join(folder,"package.json"),JSON.stringify({name:`pkg-${index}`,version:"1.0.0"}));}
 mkdirSync(join(generation,"node_modules",".bin"),{recursive:true});symlinkSync("../pkg-3/index.js",join(generation,"node_modules",".bin","pkg-3"));
 writeFileSync(join(generation,"w","0","node_modules","w0.txt"),"w0");writeFileSync(join(generation,"w","1","node_modules","w1.txt"),"w1");
 settle(generation);
};
const settle=(generation:string)=>{
 const walk=(path:string)=>{for(const name of readdirSync(path)){const child=join(path,name),facts=lstatSync(child);if(facts.isDirectory()){walk(child);}else if(!facts.isSymbolicLink())utimesSync(child,1_700_000_000,1_700_000_000);}};
 for(const root of roots)walk(join(generation,...root.split("/")));
};
const blobsOf=(store:string):string[]=>{const base=join(store,"v1","fleet","install","blobs"),names:string[]=[];if(existsSync(base))for(const folder of readdirSync(base))if(!folder.startsWith("."))names.push(...readdirSync(join(base,folder)));return names.sort();};

const scanOf=(generation:string)=>async():Promise<FleetInstallShapeV1>=>{
 const rows:string[]=[];let directories=0,files=0,links=0;
 const walk=(path:string,relative:string)=>{for(const name of readdirSync(path).sort()){const child=join(path,name),facts=lstatSync(child),here=`${relative}/${name}`;if(facts.isSymbolicLink()){links++;rows.push(`l ${here} ${readlinkSync(child)}`);}else if(facts.isDirectory()){directories++;rows.push(`d ${here}`);walk(child,here);}else{files++;rows.push(`f ${here} ${createHash("sha256").update(readFileSync(child)).digest("hex")}`);}}};
 for(const root of roots){const path=join(generation,...root.split("/"));if(existsSync(path)){rows.push(`r ${root}`);directories++;walk(path,root);}}
 return {entries:rows.length,directories,files,links,sha256:createHash("sha256").update(rows.join("\n")).digest("hex"),version:1,maximumPathUnits:7} as FleetInstallShapeV1;
};

const template=directory("identity");mkdirSync(join(template,".cache","f"),{recursive:true});writeFileSync(join(template,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:template,key:"k"}));
await syncFleetIdentityV1({root:template});
const LOCK='{"lockfileVersion":1,"packages":{"left":["left@1.0.0","",{},"sha512-AAAA"],"right":["right@2.0.0","",{},"sha512-BBBB"],"app":["app@workspace:."]}}';
const join_=(generation:string,store:string)=>{
 mkdirSync(join(generation,".cache","f"),{recursive:true});cpSync(join(template,".cache","f","identity"),join(generation,".cache","f","identity"),{recursive:true});
 writeFileSync(join(generation,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:store,key:"k"}));
 try{mkdirSync(join(store,"v1","origins","g1-m01"),{recursive:true});}catch{return;}cpSync(join(template,".cache","f","identity","registration.json"),join(store,"v1","origins","g1-m01","fleet-auth.json"));
 if(!existsSync(join(generation,"bun.lock")))writeFileSync(join(generation,"bun.lock"),LOCK);
};
const request=(generation:string,store:string,remote?:{endpoint:string;instance:string;token:string|undefined;authenticated?:boolean}):FleetInstallRequestV1=>(join_(generation,store),{
 context:fleetFetchContextV1({},generation,{storeRoot:store,remote}),projectionSha256:projection,runtime,directory:generation,lockFile:"bun.lock",storeRoot:"/stores/default",roots,check:()=>{},scan:scanOf(generation),
 remove:async path=>{rmSync(path,{recursive:true,force:true});},installed:()=>"c".repeat(64),
});

const python=async(program:string,input:string)=>{const child=Bun.spawn(["python3","-I","-c",program,input],{stdout:"pipe",stderr:"pipe"}),[out,code]=await Promise.all([new Response(child.stdout).text(),child.exited]);expect(code).toBe(0);return out.trim().split("\n");};

test("a published install tree is restored on another machine with the same shape; python's tarfile lists the same members",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two");populate(one);
 expect(await fleetInstallPublishV1(request(one,store))).toBe(true);
 const key=fleetInstallKeyV1(projection,runtime),context=request(one,store).context,manifest=await fleetInstallManifestV1(context,key);
 expect(manifest?.index.packedBytes).toBeGreaterThan(1000);expect(manifest?.shape.links).toBe(1);
 const index=await fleetInstallIndexV1(join(store,"v1","fleet","install"),manifest!);expect(index?.units.length).toBe(62);
 const members:string[]=[];
 for(const unit of index!.units)members.push(...await python("import sys,tarfile\nprint('\\n'.join(m.name.rstrip('/') for m in tarfile.open(sys.argv[1],'r:gz').getmembers() if m.isfile() or m.issym()))",join(store,"v1","fleet","install","blobs",unit.sha256.slice(0,2),unit.sha256)));
 members.push(...index!.links.map(link=>link.path));
 const expected:string[]=[];const collect=(path:string,relative:string)=>{for(const name of readdirSync(path)){const child=join(path,name),facts=lstatSync(child);if(facts.isDirectory())collect(child,`${relative}/${name}`);else expected.push(`${relative}/${name}`);}};
 for(const root of roots)collect(join(one,...root.split("/")),root);
 expect(members.filter(Boolean).sort()).toEqual(expected.sort());
 expect(await fleetInstallRestoreV1(request(two,store))).toBe(true);
 expect(await scanOf(two)()).toEqual(await scanOf(one)());
 expect(readlinkSync(join(two,"node_modules",".bin","pkg-3"))).toBe("../pkg-3/index.js");
 expect(readFileSync(join(two,"node_modules","pkg-7","index.js"),"utf8")).toBe(readFileSync(join(one,"node_modules","pkg-7","index.js"),"utf8"));
});

test("generations that share packages share blobs: the second pack stores only what differs, and the same tree packs to the same blobs from another path",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two"),three=directory("three");populate(one);
 expect(await fleetInstallPublishV1(request(one,store))).toBe(true);const before=blobsOf(store);
 cpSync(one,two,{recursive:true,verbatimSymlinks:true});rmSync(join(two,".cache"),{recursive:true,force:true});settle(two);
 writeFileSync(join(two,"node_modules","pkg-9","index.js"),"changed\n");settle(two);
 const events:string[]=[];
 expect(await fleetInstallPublishV1({...request(two,store),projectionSha256:"d".repeat(64),event:event=>events.push(event.stage)})).toBe(true);
 const after=blobsOf(store);expect(after.length-before.length,"one changed package and one new index").toBe(2);expect(before.every(name=>after.includes(name))).toBe(true);
 cpSync(one,three,{recursive:true,verbatimSymlinks:true});rmSync(join(three,".cache"),{recursive:true,force:true});settle(three);
 const other=directory("store-b");expect(await fleetInstallPublishV1(request(three,other))).toBe(true);
 expect(blobsOf(other),"an independent store receives byte-identical blobs").toEqual(before);
});

test("an installed and sealed generation that was never published is seeded into the fleet; seeding twice publishes nothing more",async()=>{
 const store=directory("store"),generations=[directory("s0"),directory("s1"),directory("s2")];
 generations.forEach((generation,index)=>{populate(generation);writeFileSync(join(generation,"node_modules","own.txt"),String(index));settle(generation);});
 const requests=generations.map((generation,index)=>({...request(generation,store),projectionSha256:index.toString(16).padStart(64,"1")}));
 expect(await fleetInstallSeedV1(requests)).toEqual({published:3,present:0,failed:0});
 const blobs=blobsOf(store).length;
 expect(await fleetInstallSeedV1(requests)).toEqual({published:0,present:3,failed:0});expect(blobsOf(store).length).toBe(blobs);
 for(const [index,generation] of generations.entries()){const target=directory("fresh");expect(await fleetInstallRestoreV1({...request(target,store),projectionSha256:index.toString(16).padStart(64,"1")})).toBe(true);expect(readFileSync(join(target,"node_modules","own.txt"),"utf8")).toBe(String(index));expect(await scanOf(target)()).toEqual(await scanOf(generation)());}
});

test("a second publish of the same state is skipped",async()=>{
 const store=directory("store"),one=directory("one");populate(one);const events:string[]=[];
 expect(await fleetInstallPublishV1(request(one,store))).toBe(true);
 expect(await fleetInstallPublishV1({...request(one,store),event:event=>events.push(event.stage)})).toBe(true);expect(events).toEqual(["skip"]);
 expect(readdirSync(join(store,"v1","fleet","install")).filter(name=>name.endsWith(".json")).length).toBe(1);
});

test("a tree with an absolute symbolic link is never published",async()=>{
 const store=directory("store"),one=directory("one");populate(one);symlinkSync("/Users/someone/checkout/.cache/b/d",join(one,"node_modules","absolute"));
 expect(await fleetInstallPublishV1(request(one,store))).toBe(false);expect(existsSync(join(store,"v1","fleet","install"))).toBe(false);
});

test("a tampered archive is refused and leaves the target without any placed root",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two");populate(one);await fleetInstallPublishV1(request(one,store));
 const folder=join(store,"v1","fleet","install","blobs"),manifest=JSON.parse(readFileSync(join(store,"v1","fleet","install",fleetInstallKeyV1(projection,runtime)+".json"),"utf8")),victim=blobsOf(store).find(name=>name!==manifest.index.sha256)!,file=join(folder,victim.slice(0,2),victim),bytes=readFileSync(file);bytes[bytes.length>>1]^=0xff;writeFileSync(file,bytes);
 expect(await fleetInstallRestoreV1(request(two,store))).toBe(false);
 for(const root of roots)expect(existsSync(join(two,...root.split("/")))).toBe(false);
});

test("a restored tree whose shape differs from the publisher's scan is removed",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two");populate(one);await fleetInstallPublishV1(request(one,store));
 const liar={...request(two,store),scan:async()=>({...(await scanOf(two)()),entries:1})};
 expect(await fleetInstallRestoreV1(liar)).toBe(false);for(const root of roots)expect(existsSync(join(two,...root.split("/")))).toBe(false);
});

test("two machines that want the same state: one installs under the fleet claim, the other waits and restores; the install runs once",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-secret"}});
 try{
  const store=directory("store"),remote={endpoint:server.url,instance:"main",token:"w-secret",authenticated:true},machines=[directory("a"),directory("b")];let installs=0;
  const work=async(generation:string)=>{
   const request_=request(generation,store,remote),acquired=await fleetInstallAcquireV1(request_);
   if(acquired.restored)return "restored";
   installs++;await Bun.sleep(900);populate(generation);const published=await acquired.publish();await acquired.release();expect(published).toBe(true);return "installed";
  };
  const outcomes=await Promise.all(machines.map(work));
  expect(installs).toBe(1);expect(outcomes.sort()).toEqual(["installed","restored"]);
  expect(await scanOf(machines[0]!)()).toEqual(await scanOf(machines[1]!)());
  const third=directory("c");expect((await fleetInstallAcquireV1(request(third,store,remote))).restored).toBe(true);
 }finally{await server.stop();}
},60000);

test("with the cache server unreachable the install still proceeds locally and publishes to the store",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"}});
 const remote={endpoint:server.url,instance:"main",token:"w",authenticated:true};await server.stop();
 const store=directory("store"),one=directory("one"),started=performance.now(),acquired=await fleetInstallAcquireV1(request(one,store,remote));
 expect(acquired.restored).toBe(false);populate(one);expect(await acquired.publish()).toBe(true);await acquired.release();expect(performance.now()-started).toBeLessThan(30000);
});

test("a machine without a group store gets a no-op acquisition",async()=>{
 const one=directory("one"),acquired=await fleetInstallAcquireV1({...request(one,"/unused"),context:fleetFetchContextV1({SEMIO_FLEET_FETCH:"off"},one)});
 expect(acquired.restored).toBe(false);expect(await acquired.publish()).toBe(false);
});

test("the store keeps only the newest published states and removes the blobs nobody names",async()=>{
 const store=directory("store"),keep=FLEET_FETCH_POLICY_V1.installKeepStates;let clock=Date.now();
 for(let index=0;index<keep+3;index++){
  const generation=directory(`g${index}`);populate(generation);writeFileSync(join(generation,"node_modules","state.txt"),String(index));
  const base=request(generation,store);
  expect(await fleetInstallPublishV1({...base,projectionSha256:index.toString(16).padStart(64,"0"),context:fleetFetchContextV1({},generation,{storeRoot:store,remote:undefined,now:()=>clock+=1000})})).toBe(true);
 }
 const folder=join(store,"v1","fleet","install"),names=readdirSync(folder);
 expect(names.filter(name=>name.endsWith(".json")).length).toBe(keep);
 const before=blobsOf(store).length;expect(await pruneFleetInstallsV1(folder,clock)).toBe(0);expect(blobsOf(store).length).toBe(before);
 const removed=await pruneFleetInstallsV1(folder,clock+2*3600000);expect(removed,"the state files and indexes of the three dropped states").toBeGreaterThanOrEqual(6);
 const kept=new Set<string>();for(const name of readdirSync(folder).filter(name=>name.endsWith(".json"))){const manifest=JSON.parse(readFileSync(join(folder,name),"utf8")),index=await fleetInstallIndexV1(folder,manifest);kept.add(manifest.index.sha256);for(const unit of index!.units)kept.add(unit.sha256);}
 expect(blobsOf(store).sort()).toEqual([...kept].sort());
});

test("absolute links into the dependency store are published and rewritten to the restoring checkout's store; other absolute links stay refused",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two");populate(one);
 symlinkSync("/stores/default/j/owner20/w/0/node_modules/react",join(one,"node_modules","react"));
 expect(await fleetInstallPublishV1(request(one,store))).toBe(true);
 const moved={...request(two,store),storeRoot:"/elsewhere/.cache/b/d/m"};
 expect(await fleetInstallRestoreV1(moved)).toBe(true);
 expect(readlinkSync(join(two,"node_modules","react"))).toBe("/elsewhere/.cache/b/d/m/j/owner20/w/0/node_modules/react");
 expect(readlinkSync(join(two,"node_modules",".bin","pkg-3"))).toBe("../pkg-3/index.js");
 const outside=directory("outside");populate(outside);symlinkSync("/Users/someone/checkout/x",join(outside,"node_modules","x"));
 expect(await fleetInstallPublishV1({...request(outside,directory("store2"))})).toBe(false);
});

test("links to workspace packages and to the store below one worktree root are published and follow the restoring worktree",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two");populate(one);
 symlinkSync("/Users/Shared/semio/🧰️framework/pkg/rust",join(one,"node_modules","semio-pkg"));
 symlinkSync("/Users/Shared/semio/.cache/b/d/m/j/owner20/node_modules/react",join(one,"node_modules","react"));
 const published={...request(one,store),storeRoot:"/Users/Shared/semio"};
 expect(await fleetInstallPublishV1(published)).toBe(true);
 expect(await fleetInstallRestoreV1({...request(two,store),storeRoot:"/Volumes/other/checkout"})).toBe(true);
 expect(readlinkSync(join(two,"node_modules","semio-pkg"))).toBe("/Volumes/other/checkout/🧰️framework/pkg/rust");
 expect(readlinkSync(join(two,"node_modules","react"))).toBe("/Volumes/other/checkout/.cache/b/d/m/j/owner20/node_modules/react");
 const foreign=directory("foreign");populate(foreign);symlinkSync("/Users/ueli/Documents/semio/x",join(foreign,"node_modules","x"));
 expect(await fleetInstallPublishV1({...request(foreign,directory("store3")),storeRoot:"/Users/Shared/semio"})).toBe(false);
});

test("Windows junction targets (backslashes, extended prefix) compare and rewrite like the POSIX store prefix, and the rewritten link keeps the Windows spelling",async()=>{
 expect(fleetInstallLinkTextV1("\\\\?\\C:\\s\\j\\x","win32")).toBe("C:/s/j/x");expect(fleetInstallLinkTextV1("C:\\s\\j","linux")).toBe("C:\\s\\j");
 const make=(store:string)=>{const tree=directory("junction");mkdirSync(join(tree,"node_modules"),{recursive:true});symlinkSync(store+"\\j\\owner\\pkg",join(tree,"node_modules","pkg"));symlinkSync("..\\rel",join(tree,"node_modules","rel"));return tree;};
 const left=make("C:\\stores\\a"),right=make("D:\\other"),none=()=>{};
 const a=await fleetInstallFingerprintV1(left,["node_modules"],"C:\\stores\\a",none,"win32"),b=await fleetInstallFingerprintV1(right,["node_modules"],"D:\\other",none,"win32");
 expect(a).toBe(b);
 expect(await rewriteFleetInstallLinksV1(left,["node_modules"],"C:\\stores\\a","D:\\other",none,"win32")).toBe(1);
 expect(readlinkSync(join(left,"node_modules","pkg"))).toBe("D:\\other\\j\\owner\\pkg");expect(readlinkSync(join(left,"node_modules","rel"))).toBe("..\\rel");
 expect(await fleetInstallFingerprintV1(left,["node_modules"],"D:\\other",none,"win32")).toBe(b);
});

test("attack: a pack from another lockfile, an unsigned or re-signed pack and a pack of an unregistered origin are never restored; the right one is",async()=>{
 const store=directory("store"),one=directory("one"),two=directory("two");populate(one);
 expect(await fleetInstallPublishV1(request(one,store))).toBe(true);
 const key=fleetInstallKeyV1(projection,runtime),manifestFile=join(store,"v1","fleet","install",key+".json");
 const manifest=JSON.parse(readFileSync(manifestFile,"utf8"));
 expect(manifest.lock.packages,"the two packages with a tarball integrity").toBe(2);expect(manifest.publisher.origin).toBe("g1-m01");
 const other=request(directory("c"),store);writeFileSync(join(other.directory,"bun.lock"),LOCK.replace("sha512-BBBB","sha512-EVIL"));
 expect(await fleetInstallRestoreV1(other),"a lockfile that pins other integrities").toBe(false);
 const rewrite=(change:(value:any)=>void)=>{const value=JSON.parse(readFileSync(manifestFile,"utf8"));change(value);writeFileSync(manifestFile,JSON.stringify(value)+"\n");};
 rewrite(value=>{value.fingerprint="f".repeat(64);});
 expect(await fleetInstallRestoreV1(request(two,store)),"a manifest edited after signing").toBe(false);
 rewrite(value=>{value.fingerprint=manifest.fingerprint;value.lock.sha256="e".repeat(64);});
 expect(await fleetInstallRestoreV1(request(directory("d"),store)),"a lock digest edited to match another lockfile").toBe(false);
 rewrite(value=>{value.lock=manifest.lock;value.publisher.origin="g1-m77";});
 expect(await fleetInstallRestoreV1(request(directory("e"),store)),"signed by an origin the group store does not register").toBe(false);
 rewrite(value=>{value.publisher=manifest.publisher;});
 const good=request(directory("f"),store);expect(await fleetInstallRestoreV1(good),"the original manifest restores").toBe(true);
 const lone=directory("g");populate(lone);
 const stranger=request(lone,store);rmSync(join(lone,".cache","f","identity"),{recursive:true,force:true});
 expect(await fleetInstallPublishV1({...stranger,replace:true} as never,true),"a machine without a fleet identity cannot publish a pack").toBe(false);
});

const manifestCount=(store:string):number=>{const folder=join(store,"v1","fleet","install");return existsSync(folder)?readdirSync(folder).filter(name=>name.endsWith(".json")).length:0;};
const scopeRequest=(generation:string,store:string,scope:number,klass:FleetInstallRuntimeV1,change:number):FleetInstallRequestV1=>{
 const base=request(generation,store);if(change>0)writeFileSync(join(generation,"bun.lock"),LOCK.replace("sha512-BBBB",`sha512-C${change}`));
 return {...base,projectionSha256:createHash("sha256").update(`scope-${scope}-${change}`).digest("hex"),runtime:klass};
};
const fundMachine=async(store:string,klass:FleetInstallRuntimeV1,changes:ReadonlyMap<number,number>=new Map(),breakQualify:ReadonlySet<number>=new Set())=>{
 const outcomes:string[]=[],trees:string[]=[],published:boolean[]=[];let installs=0;
 for(const scope of [0,1,2]){
  const generation=directory(`scope-${scope}`);trees.push(generation);let qualified=0;
  const result=await fleetInstallScopeV1(scopeRequest(generation,store,scope,klass,changes.get(scope)??0),{
   install:async()=>{installs++;populate(generation);writeFileSync(join(generation,"node_modules","scope.txt"),`scope ${scope}`);settle(generation);},
   qualify:async()=>{if(breakQualify.has(scope)&&qualified++===0)throw Error("injected: tree does not qualify");return scanOf(generation)();},
  });
  outcomes.push(result.origin);published.push(result.published);
 }
 return {installs,outcomes,trees,published};
};
const linux=(arch:string):FleetInstallRuntimeV1=>({...runtime,platform:"linux",arch});

test("scope law: an equal second machine of the class performs 0 installs, one changed lock installs 1 scope, another class installs and publishes under its own key",async()=>{
 const store=directory("store"),first=await fundMachine(store,runtime);
 expect(first.outcomes).toEqual(["installed","installed","installed"]);expect(first.installs).toBe(3);expect(first.published).toEqual([true,true,true]);expect(manifestCount(store)).toBe(3);
 const second=await fundMachine(store,runtime);
 expect(second.outcomes).toEqual(["restored","restored","restored"]);expect(second.installs).toBe(0);expect(second.published).toEqual([false,false,false]);expect(manifestCount(store)).toBe(3);
 for(let index=0;index<3;index++){expect(await scanOf(second.trees[index]!)()).toEqual(await scanOf(first.trees[index]!)());expect(readFileSync(join(second.trees[index]!,"node_modules","scope.txt"),"utf8")).toBe(`scope ${index}`);}
 const third=await fundMachine(store,runtime,new Map([[1,1]]));
 expect(third.outcomes).toEqual(["restored","installed","restored"]);expect(third.installs).toBe(1);expect(manifestCount(store)).toBe(4);
 const musl=await fundMachine(store,linux("x64-musl"));
 expect(musl.outcomes).toEqual(["installed","installed","installed"]);expect(manifestCount(store)).toBe(7);
 const gnu=await fundMachine(store,linux("x64-gnu"));
 expect(gnu.outcomes,"a musl pack is never restored on glibc").toEqual(["installed","installed","installed"]);expect(manifestCount(store)).toBe(10);
 const again=await fundMachine(store,linux("x64-musl"));expect(again.installs).toBe(0);expect(again.outcomes).toEqual(["restored","restored","restored"]);
 const keys=readdirSync(join(store,"v1","fleet","install")).filter(name=>name.endsWith(".json"));
 expect(new Set(keys).size).toBe(10);
 const arches=keys.map(name=>JSON.parse(readFileSync(join(store,"v1","fleet","install",name),"utf8")).runtime.arch).sort();
 expect(arches).toEqual(["arm64","arm64","arm64","arm64","x64-gnu","x64-gnu","x64-gnu","x64-musl","x64-musl","x64-musl"]);
});

test("a damaged pack is a miss and never a partial tree: the scope is installed here, the tree equals a clean install and the fleet pack is repaired",async()=>{
 const store=directory("store"),first=await fundMachine(store,runtime),base=join(store,"v1","fleet","install","blobs");
 const blobs=readdirSync(base).filter(folder=>!folder.startsWith(".")).flatMap(folder=>readdirSync(join(base,folder)).map(name=>join(base,folder,name)));
 const victim=blobs.find(path=>readFileSync(path).length>200)!;writeFileSync(victim,readFileSync(victim).subarray(0,40));
 const second=await fundMachine(store,runtime);
 expect(second.installs,"exactly the scope that owned the damaged blob is installed").toBeGreaterThanOrEqual(1);expect(second.outcomes).toContain("installed");
 for(let index=0;index<3;index++){expect(await scanOf(second.trees[index]!)()).toEqual(await scanOf(first.trees[index]!)());expect(existsSync(join(second.trees[index]!,"node_modules","pkg-59"))).toBe(true);}
 const third=await fundMachine(store,runtime);expect(third.installs,"the repaired pack restores").toBe(0);
});

test("a restored tree that does not qualify is removed, the scope is installed here and published again",async()=>{
 const store=directory("store");await fundMachine(store,runtime);
 const second=await fundMachine(store,runtime,new Map(),new Set([1]));
 expect(second.outcomes).toEqual(["restored","installed","restored"]);expect(second.installs).toBe(1);expect(second.published).toEqual([false,true,false]);
 const clean=directory("clean");populate(clean);writeFileSync(join(clean,"node_modules","scope.txt"),"scope 1");settle(clean);
 expect(await scanOf(second.trees[1]!)()).toEqual(await scanOf(clean)());
});

test("a cancellation during a restore propagates, runs no installation and leaves no placed root behind",async()=>{
 const store=directory("store");await fundMachine(store,runtime);
 const generation=directory("cancelled"),base=scopeRequest(generation,store,0,runtime,0);let calls=0,installs=0;
 const cancelled={...base,check:()=>{if(++calls>4)throw Error("cancelled by the caller");}};
 await expect(fleetInstallScopeV1(cancelled,{install:async()=>{installs++;},qualify:async()=>scanOf(generation)()})).rejects.toThrow("cancelled by the caller");
 expect(installs).toBe(0);for(const root of roots)expect(existsSync(join(generation,...root.split("/"))),root).toBe(false);
});

test("the key follows the libc of Linux and stays as it was for macOS and Windows; python computes the same keys",async()=>{
 const base:FleetInstallRuntimeV1={bun:"1.4.2",platform:"linux",arch:"x64",recipeSha256:"d".repeat(64)};
 const glibc=fleetInstallRuntimeV1(base,fleetInstallHostV1("linux","x64",()=>["ld-linux-x86-64.so.2"])),musl=fleetInstallRuntimeV1(base,fleetInstallHostV1("linux","x64",()=>["ld-musl-x86_64.so.1"]));
 expect(glibc.arch).toBe("x64-gnu");expect(musl.arch).toBe("x64-musl");expect(fleetInstallKeyV1(projection,glibc)).not.toBe(fleetInstallKeyV1(projection,musl));
 const mac:FleetInstallRuntimeV1={...base,platform:"darwin",arch:"arm64"},win:FleetInstallRuntimeV1={...base,platform:"win32",arch:"x64"};
 expect(fleetInstallRuntimeV1(mac,fleetInstallHostV1("darwin","arm64"))).toEqual(mac);expect(fleetInstallRuntimeV1(win,fleetInstallHostV1("win32","x64"))).toEqual(win);
 const program="import sys,json,hashlib\nfor row in json.loads(sys.argv[1]):\n print('inst-'+hashlib.sha256(json.dumps(row,separators=(',',':'),ensure_ascii=False).encode()).hexdigest())";
 const rows=[glibc,musl,mac,win].map(item=>[projection,item.bun,item.platform,item.arch,item.recipeSha256]);
 expect(await python(program,JSON.stringify(rows))).toEqual([glibc,musl,mac,win].map(item=>fleetInstallKeyV1(projection,item)));
});

test("wiring: the funding producer and the synchronisation build their request with the one builder and the producer restores through the scope law",()=>{
 const library=fileURLToPath(new URL("../../../../../",import.meta.url)),read=(path:string)=>readFileSync(join(library,path),"utf8");
 const storage=read("⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"),producer=read("⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts");
 expect(storage).toContain("export async function javascriptDependencyFleetRequestV1");expect(storage.match(/javascriptDependencyFleetRequestV1\(/gu)?.length).toBeGreaterThanOrEqual(2);
 expect(storage).toContain("fleetInstallRuntimeV1(");
 expect(producer).toContain("javascriptDependencyFleetRequestV1(");expect(producer).toContain("fleetInstallScopeV1(");
 expect(producer.indexOf("fleetInstallScopeV1(")).toBeLessThan(producer.indexOf("publishRepositoryDependencyProducedExportsV1(workspace,root,row"));
});
