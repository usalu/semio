import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {observeJavascriptDependencyGenerationProducerV1} from "../../../📦️dependencies/🧾️generation/💾️storage/🟦️.ts";
import {fleetFetchContextV1} from "../../🟦️.ts";
import {fleetMirrorBaseV1,fleetMirrorTransportEnvironmentV1,refreshFleetMirrorStateV1} from "../🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-mirror-client-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const serve=(mirror:boolean)=>startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"},...(mirror?{mirror:{directory:directory("mirror")}}:{})});

test("a healthy probe turns on the transport settings; a server without the mirror, a stopped server or SEMIO_FLEET_MIRROR=off turn them off",async()=>{
 const root=directory("root");mkdirSync(join(root,".cache","f"),{recursive:true});
 const ambient={SEMIO_WORKSPACE_ROOT:root};
 const withMirror=await serve(true),context=(endpoint:string)=>fleetFetchContextV1({},root,{storeRoot:undefined,remote:{endpoint,instance:"main",token:"w"}});
 try{
  expect(fleetMirrorBaseV1(ambient)).toBeUndefined();
  expect((await refreshFleetMirrorStateV1(context(withMirror.url)))?.healthy).toBe(true);
  expect(fleetMirrorBaseV1(ambient)).toBe(withMirror.url);
  expect(fleetMirrorTransportEnvironmentV1("bun",{},ambient)).toEqual({BUN_CONFIG_REGISTRY:`${withMirror.url}/mirror/npm/`});
  expect(fleetMirrorTransportEnvironmentV1("go",{GOPROXY:"https://proxy.golang.org,direct"},ambient)).toEqual({GOPROXY:`${withMirror.url}/mirror/go|https://proxy.golang.org,direct`});
  expect(fleetMirrorTransportEnvironmentV1("playwright",{},ambient),"playwright downloads carry no pinned hash: never through the mirror").toEqual({});
  expect(fleetMirrorTransportEnvironmentV1("bun",{BUN_CONFIG_REGISTRY:"https://private.example/"},ambient)).toEqual({});
  expect(fleetMirrorTransportEnvironmentV1("uv",{},ambient)).toEqual({});
  expect(fleetMirrorBaseV1({...ambient,SEMIO_FLEET_MIRROR:"off"})).toBeUndefined();
 }finally{await withMirror.stop();}
 const plain=await serve(false);
 try{expect((await refreshFleetMirrorStateV1(context(plain.url)))?.healthy).toBe(false);expect(fleetMirrorBaseV1(ambient)).toBeUndefined();}finally{await plain.stop();}
 const dead=plain.url;expect((await refreshFleetMirrorStateV1(context(dead)))?.healthy).toBe(false);
 writeFileSync(join(root,".cache","f","mirror-state.json"),JSON.stringify({version:1,checkedAtMs:1,base:"http://x:1",healthy:true}));expect(fleetMirrorBaseV1(ambient)).toBeUndefined();
});

test("the mirror never enters a dependency generation key: the producer observation is identical with the mirror on and off, and the control proves the observation would see a registry in the ambient environment",()=>{
 const root=directory("keys");mkdirSync(join(root,".cache","f"),{recursive:true});
 const ambientEnv={PATH:"/usr/bin",SEMIO_WORKSPACE_ROOT:root},observe=(env:Record<string,string>)=>JSON.stringify(observeJavascriptDependencyGenerationProducerV1(root,join(root,"store"),[""],{home:join(root,"home"),config:join(root,"config"),env} as never));
 const off=observe(ambientEnv);
 writeFileSync(join(root,".cache","f","mirror-state.json"),JSON.stringify({version:1,checkedAtMs:Date.now(),base:"http://192.0.2.1:6090",healthy:true}));
 const transport={...fleetMirrorTransportEnvironmentV1("bun",{},ambientEnv),...fleetMirrorTransportEnvironmentV1("go",{},ambientEnv)};
 expect(Object.keys(transport).sort()).toEqual(["BUN_CONFIG_REGISTRY","GOPROXY"]);
 expect(observe(ambientEnv)).toBe(off);
 expect(observe({...ambientEnv,BUN_CONFIG_REGISTRY:transport.BUN_CONFIG_REGISTRY!})).not.toBe(off);
});
