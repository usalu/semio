import {afterAll,expect,test} from "bun:test";
import {createHash,randomBytes} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {pinFromIntegrityV1,fleetBlobBytesV1,fleetBlobOpenV1,fleetBlobPublishBytesV1,fleetBlobStorePathV1,fleetFetchContextV1,pinnedFetchV1} from "../🟦️.ts";

const scratch:string[]=[],sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-fetch-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const run=async(argv:string[])=>{const child=Bun.spawn(argv,{stdout:"pipe",stderr:"pipe"}),[out,code]=await Promise.all([new Response(child.stdout).text(),child.exited]);return {out,code};};

const upstream=(payload:Uint8Array<ArrayBuffer>)=>{
 const served={requests:0,bytes:0};
 const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{if(new URL(request.url).pathname!=="/artifact.bin")return new Response("no",{status:404});served.requests++;served.bytes+=payload.byteLength;return new Response(payload);}});
 return {url:`http://127.0.0.1:${server.port}/artifact.bin`,served,stop:()=>server.stop(true)};
};

const machine=(root:string,store:string|undefined,ledger:string,remote?:{endpoint:string;instance:string;token:string|undefined;authenticated:boolean})=>fleetFetchContextV1({SEMIO_FLEET_FETCH_LEDGER:ledger},root,{storeRoot:store,remote});
const lines=(ledger:string)=>existsSync(ledger)?readFileSync(ledger,"utf8").trim().split("\n").filter(Boolean).map(line=>JSON.parse(line) as {source:string;bytes:number;direction?:string}):[];

test("a second machine takes a pinned artifact from the group store and fetches 0 bytes from the upstream; the stored bytes equal shasum and curl",async()=>{
 const payload=randomBytes(3*1048576+17),pin=sha(payload),origin=upstream(payload),store=directory("store"),first=directory("first"),second=directory("second");
 try{
  const ledgerOne=join(first,"ledger.jsonl"),ledgerTwo=join(second,"ledger.jsonl");
  const body=new Uint8Array(await (await pinnedFetchV1(machine(first,store,ledgerOne),{url:origin.url,sha256:pin,bytes:payload.byteLength})).arrayBuffer());
  expect(sha(body)).toBe(pin);expect(origin.served.requests).toBe(1);
  expect(lines(ledgerOne).filter(row=>row.source==="upstream").reduce((sum,row)=>sum+row.bytes,0)).toBe(payload.byteLength);
  const stored=fleetBlobStorePathV1(store,pin);expect(existsSync(stored)).toBe(true);
  const shasum=await run(["shasum","-a","256",stored]);expect(shasum.out.split(" ")[0]).toBe(pin);
  const downloaded=join(first,"curl.bin"),before=origin.served.requests;await run(["curl","-sS","-o",downloaded,origin.url]);
  expect((await run(["shasum","-a","256",downloaded])).out.split(" ")[0]).toBe(pin);expect(origin.served.requests).toBe(before+1);
  const again=new Uint8Array(await (await pinnedFetchV1(machine(second,store,ledgerTwo),{url:origin.url,sha256:pin,bytes:payload.byteLength})).arrayBuffer());
  expect(sha(again)).toBe(pin);expect(origin.served.requests).toBe(before+1);
  expect(lines(ledgerTwo).filter(row=>row.source==="upstream")).toEqual([]);
  expect(lines(ledgerTwo).filter(row=>row.source==="store"&&row.direction==="get").length).toBe(1);
  expect(readdirSync(join(store,"v1","fleet","fetch",pin.slice(0,2))).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 }finally{origin.stop();}
});

test("a body that does not match the pin is passed on but never published",async()=>{
 const payload=randomBytes(65536),origin=upstream(payload),store=directory("store"),root=directory("root");
 try{
  const wrong=sha("something else");
  const body=new Uint8Array(await (await pinnedFetchV1(machine(root,store,join(root,"l.jsonl")),{url:origin.url,sha256:wrong})).arrayBuffer());
  expect(body.byteLength).toBe(payload.byteLength);
  expect(existsSync(fleetBlobStorePathV1(store,wrong))).toBe(false);
  expect(existsSync(fleetBlobStorePathV1(store,sha(payload)))).toBe(false);
 }finally{origin.stop();}
});

test("a corrupt store file is refused, removed, and the upstream answers",async()=>{
 const payload=randomBytes(40000),pin=sha(payload),origin=upstream(payload),store=directory("store"),root=directory("root"),path=fleetBlobStorePathV1(store,pin);
 try{
  mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,randomBytes(40000));
  const context=machine(root,store,join(root,"l.jsonl"));
  expect(await fleetBlobOpenV1(context,pin)).toBeUndefined();expect(existsSync(path)).toBe(false);
  const body=new Uint8Array(await (await pinnedFetchV1(context,{url:origin.url,sha256:pin})).arrayBuffer());
  expect(sha(body)).toBe(pin);expect(origin.served.requests).toBe(1);expect(existsSync(path)).toBe(true);
 }finally{origin.stop();}
});

test("the content-addressed HTTP tier of the cache server serves a machine that has no store",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-secret",read:"r-secret"}});
 const payload=randomBytes(700000),pin=sha(payload),origin=upstream(payload);
 try{
  const remote={endpoint:server.url,instance:"main",token:"w-secret",authenticated:true},one=directory("one"),two=directory("two");
  await (await pinnedFetchV1(machine(one,undefined,join(one,"l.jsonl"),remote),{url:origin.url,sha256:pin})).arrayBuffer();
  expect(origin.served.requests).toBe(1);
  const ledger=join(two,"l.jsonl"),body=new Uint8Array(await (await pinnedFetchV1(machine(two,undefined,ledger,remote),{url:origin.url,sha256:pin})).arrayBuffer());
  expect(sha(body)).toBe(pin);expect(origin.served.requests).toBe(1);
  expect(lines(ledger).map(row=>row.source)).toEqual(["http"]);
  const wrongToken=machine(two,undefined,join(two,"x.jsonl"),{...remote,token:"nope"});
  expect(await fleetBlobBytesV1(wrongToken,pin)).toBeUndefined();
 }finally{origin.stop();await server.stop();}
});

test("with the cache server stopped and no store the same call still succeeds from the upstream",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"}});
 const remote={endpoint:server.url,instance:"main",token:"w",authenticated:true};await server.stop();
 const payload=randomBytes(50000),pin=sha(payload),origin=upstream(payload),root=directory("root");
 try{
  const started=performance.now(),body=new Uint8Array(await (await pinnedFetchV1(machine(root,undefined,join(root,"l.jsonl"),remote),{url:origin.url,sha256:pin})).arrayBuffer());
  expect(sha(body)).toBe(pin);expect(performance.now()-started).toBeLessThan(20000);expect(origin.served.requests).toBe(1);
 }finally{origin.stop();}
});

test("small blobs round trip through bytes and a Range request passes through untouched",async()=>{
 const store=directory("store"),root=directory("root"),context=machine(root,store,join(root,"l.jsonl")),bytes=randomBytes(1000),pin=await fleetBlobPublishBytesV1(context,bytes);
 expect(pin).toBe(sha(bytes));expect(Buffer.from((await fleetBlobBytesV1(context,pin))!).equals(Buffer.from(bytes))).toBe(true);
 const origin=upstream(randomBytes(100));
 try{const response=await pinnedFetchV1(context,{url:origin.url,sha256:pin},{headers:{range:"bytes=0-9"}});await response.arrayBuffer();expect(origin.served.requests).toBe(1);}finally{origin.stop();}
});

test("SEMIO_FLEET_FETCH=off disables every fleet tier",()=>{
 const context=fleetFetchContextV1({SEMIO_FLEET_FETCH:"off",SEMIO_FLEET_STORE_ROOT:"/nonexistent"},"/nonexistent");
 expect(context.storeRoot).toBeUndefined();expect(context.remote).toBeUndefined();
});

test("a sha512 pin (npm integrity) is served from the store after one download; the store file equals shasum -a 512",async()=>{
 const payload=randomBytes(300000),sri="sha512-"+createHash("sha512").update(payload).digest("base64"),origin=upstream(payload),store=directory("store"),first=directory("first"),second=directory("second");
 try{
  const pin=pinFromIntegrityV1(origin.url,sri,payload.byteLength)!;expect(pin.sha256.length).toBe(128);
  await (await pinnedFetchV1(machine(first,store,join(first,"l.jsonl")),pin)).arrayBuffer();expect(origin.served.requests).toBe(1);
  const stored=fleetBlobStorePathV1(store,pin.sha256);expect((await run(["shasum","-a","512",stored])).out.split(" ")[0]).toBe(pin.sha256);
  const again=new Uint8Array(await (await pinnedFetchV1(machine(second,store,join(second,"l.jsonl")),pin)).arrayBuffer());
  expect(createHash("sha512").update(again).digest("hex")).toBe(pin.sha256);expect(origin.served.requests).toBe(1);
  expect(pinFromIntegrityV1(origin.url,"sha1-AAAA")).toBeUndefined();
  const wrong=pinFromIntegrityV1(origin.url,"sha512-"+createHash("sha512").update("x").digest("base64"))!,fresh=directory("third");
  await (await pinnedFetchV1(machine(fresh,directory("store3"),join(fresh,"l.jsonl")),wrong)).arrayBuffer();
 }finally{origin.stop();}
});
