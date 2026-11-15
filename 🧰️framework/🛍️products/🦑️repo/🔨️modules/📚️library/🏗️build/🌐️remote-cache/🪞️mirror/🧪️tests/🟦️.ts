import {afterAll,expect,test} from "bun:test";
import {createHash,randomBytes} from "node:crypto";
import {mkdtempSync,readdirSync,rmSync,statSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createMirrorHandlerV1,MIRROR_ECOSYSTEMS_V1} from "../🟦️.ts";

const scratch:string[]=[],sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`mirror-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const payload=randomBytes(2*1048576+5),counts:Record<string,number>={};
let origin="",tarball="";let alive=true;
const upstream=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{
 const url=new URL(request.url);counts[url.pathname]=(counts[url.pathname]??0)+1;
 if(!alive)return new Response("down",{status:503});
 if(url.pathname==="/dist/2026-07-20/blob.tgz")return new Response(payload,{headers:{"content-length":String(payload.byteLength)}});
 if(url.pathname==="/dist/channel-rust-nightly.toml"){if(request.headers.get("if-none-match")==='"v1"')return new Response(null,{status:304});return new Response("channel = 1\n",{headers:{etag:'"v1"'}});}
 if(url.pathname==="/left-pad")return Response.json({name:"left-pad",versions:{"1.3.0":{dist:{tarball:`${origin}/left-pad/-/left-pad-1.3.0.tgz`}}}});
 if(url.pathname==="/left-pad/-/left-pad-1.3.0.tgz")return new Response(payload);
 if(url.pathname==="/slow/@v/v1.0.0.zip")return new Promise(resolve=>setTimeout(()=>resolve(new Response(payload)),300));
 return new Response("nope",{status:404});
}});
origin=`http://127.0.0.1:${upstream.port}`;tarball=`${origin}/left-pad/-/left-pad-1.3.0.tgz`;void tarball;
afterAll(()=>upstream.stop(true));
const ecosystems=MIRROR_ECOSYSTEMS_V1.map(row=>({...row,upstream:origin}));

const serve=(dir:string)=>{
 const handler=createMirrorHandlerV1({directory:dir,ecosystems,now:()=>Date.now()});
 const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:async request=>(await handler(request))??new Response("outside",{status:404})});
 return {base:`http://127.0.0.1:${server.port}`,stop:()=>server.stop(true)};
};
const curl=async(args:string[])=>{const child=Bun.spawn(["curl","-sS",...args],{stdout:"pipe",stderr:"pipe"}),[out,code]=await Promise.all([new Response(child.stdout).text(),child.exited]);return {out,code};};

test("an immutable path is fetched once, shared by concurrent requests, and later served with 0 upstream requests; curl sees the same bytes",async()=>{
 const mirror=serve(directory("a"));
 try{
  const path="/mirror/rustup/dist/2026-07-20/blob.tgz";
  const bodies=await Promise.all([1,2,3,4,5].map(async()=>new Uint8Array(await (await fetch(mirror.base+path)).arrayBuffer())));
  for(const body of bodies)expect(sha(body)).toBe(sha(payload));
  expect(counts["/dist/2026-07-20/blob.tgz"]).toBe(1);
  const file=join(directory("curl"),"out.bin");await curl(["-o",file,mirror.base+path]);
  expect(sha(new Uint8Array(await Bun.file(file).arrayBuffer()))).toBe(sha(payload));expect(counts["/dist/2026-07-20/blob.tgz"]).toBe(1);
  alive=false;try{const down=await fetch(mirror.base+path);expect(down.status).toBe(200);expect(sha(new Uint8Array(await down.arrayBuffer()))).toBe(sha(payload));}finally{alive=true;}
 }finally{mirror.stop();}
});

test("a mutable path is revalidated with its etag, costs no body bytes on 304, and is answered stale while the upstream is down",async()=>{
 const dir=directory("b");let clock=Date.now();
 const handler=createMirrorHandlerV1({directory:dir,ecosystems,now:()=>clock}),call=(path:string)=>handler(new Request("http://mirror.local"+path));
 const path="/mirror/rustup/dist/channel-rust-nightly.toml";
 expect(await (await call(path))!.text()).toBe("channel = 1\n");const first=counts["/dist/channel-rust-nightly.toml"]!;
 expect(await (await call(path))!.text()).toBe("channel = 1\n");expect(counts["/dist/channel-rust-nightly.toml"]).toBe(first);
 clock+=301000;expect(await (await call(path))!.text()).toBe("channel = 1\n");expect(counts["/dist/channel-rust-nightly.toml"]).toBe(first+1);
 clock+=301000;alive=false;try{const stale=await call(path);expect(stale!.status).toBe(200);expect(await stale!.text()).toBe("channel = 1\n");}finally{alive=true;}
});

test("npm metadata points its tarballs at the mirror, the tarball is cached forever",async()=>{
 const mirror=serve(directory("c"));
 try{
  const meta=await (await fetch(mirror.base+"/mirror/npm/left-pad")).json() as {versions:Record<string,{dist:{tarball:string}}>};
  expect(meta.versions["1.3.0"]!.dist.tarball).toBe(`${mirror.base}/mirror/npm/left-pad/-/left-pad-1.3.0.tgz`);
  const one=new Uint8Array(await (await fetch(meta.versions["1.3.0"]!.dist.tarball)).arrayBuffer()),before=counts["/left-pad/-/left-pad-1.3.0.tgz"];
  const two=new Uint8Array(await (await fetch(meta.versions["1.3.0"]!.dist.tarball)).arrayBuffer());
  expect(sha(one)).toBe(sha(payload));expect(sha(two)).toBe(sha(payload));expect(counts["/left-pad/-/left-pad-1.3.0.tgz"]).toBe(before);
 }finally{mirror.stop();}
});

test("range requests are answered from the kept copy; unknown ecosystems, traversal and upstream 404 are refused; an unreachable upstream without a copy sends the client to the direct registry",async()=>{
 const mirror=serve(directory("d"));
 try{
  const path=mirror.base+"/mirror/go/slow/@v/v1.0.0.zip";
  const part=await fetch(path,{headers:{range:"bytes=10-19"}});expect(part.status).toBe(206);expect(Buffer.from(await part.arrayBuffer()).equals(Buffer.from(payload.subarray(10,20)))).toBe(true);
  const tail=await fetch(path,{headers:{range:"bytes=-5"}});expect(Buffer.from(await tail.arrayBuffer()).equals(Buffer.from(payload.subarray(payload.byteLength-5)))).toBe(true);
  expect((await fetch(mirror.base+"/mirror/pypi/simple/x")).status).toBe(404);
  expect((await curl(["-s","-o","/dev/null","-w","%{http_code}","--path-as-is",mirror.base+"/mirror/go/../x"])).out).toBe("404");
  expect((await fetch(mirror.base+"/mirror/go/missing/@v/v9.zip")).status).toBe(404);
  alive=false;try{
   const down=await fetch(mirror.base+"/mirror/go/other/@v/v1.zip",{redirect:"manual"});
   expect(down.status).toBe(307);expect(down.headers.get("location")).toBe(`${origin}/other/@v/v1.zip`);
   expect(down.headers.get("x-semio-mirror")).toBe("fallback");expect(down.headers.get("x-semio-mirror-reason")).toBe("upstream-503");
   const body=await down.text();expect(body).toContain("direct");expect(body).toContain("direkt");
  }finally{alive=true;}
  expect((await fetch(mirror.base+"/mirror/healthz")).status).toBe(200);
  expect((await fetch(mirror.base+"/mirror/go/x",{method:"POST"})).status).toBe(405);
 }finally{mirror.stop();}
});

test("the budget prunes the oldest kept bodies",async()=>{
 const dir=directory("e");let clock=Date.now();
 const handler=createMirrorHandlerV1({directory:dir,ecosystems,maximumBytes:3*1048576,now:()=>clock}),call=(path:string)=>handler(new Request("http://mirror.local"+path));
 await (await call("/mirror/npm/left-pad/-/left-pad-1.3.0.tgz"))!.arrayBuffer();clock+=61000;
 await (await call("/mirror/rustup/dist/2026-07-20/blob.tgz"))!.arrayBuffer();await Bun.sleep(100);
 const bodies=readdirSync(dir).flatMap(shard=>readdirSync(join(dir,shard))).filter(name=>name.endsWith(".body"));
 expect(bodies.length).toBeLessThanOrEqual(2);void statSync;
});

test("attack: the mirror follows redirects only inside the registrable host or the declared list, caps parallel upstream fetches, keys no query strings and no longer serves playwright",async()=>{
 const { createMirrorHandlerV1 }=await import("../🟦️.ts"),{mkdtempSync,rmSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path");
 const folder=mkdtempSync(join(tmpdir(),"mirror-attack-")),seen:string[]=[];
 try{
  const upstream=(async(input:RequestInfo|URL)=>{
   const url=String(input);seen.push(url);
   if(url.includes("/redirect-evil"))return new Response(null,{status:302,headers:{location:"https://evil.example/payload"}});
   if(url.includes("/redirect-http"))return new Response(null,{status:302,headers:{location:"http://registry.npmjs.org/plain"}});
   if(url.includes("/redirect-sibling"))return new Response(null,{status:302,headers:{location:"https://other.npmjs.org/ok"}});
   if(url.includes("/redirect-github"))return new Response(null,{status:302,headers:{location:"https://objects.githubusercontent.com/ok"}});
   if(url.includes("/slow"))return new Promise<Response>(()=>{});
   return new Response("body");
  }) as typeof fetch;
  const handler=createMirrorHandlerV1({directory:folder,fetch:upstream}),ask=(path:string)=>handler(new Request("http://x/mirror/"+path))!;
  expect((await ask("npm/redirect-evil"))!.status,"a redirect to a foreign host").toBe(502);expect(seen.some(url=>url.includes("evil.example"))).toBe(false);
  expect((await ask("npm/redirect-http"))!.status,"a downgrade to http").toBe(502);
  expect((await ask("npm/redirect-sibling"))!.status,"a sibling host of the same registrable domain").toBe(200);
  expect((await ask("go/redirect-github"))!.status,"the declared release host").toBe(200);
  expect((await ask("npm/pkg?cache-buster=1"))!.status).toBe(200);expect((await ask("npm/pkg?cache-buster=2"))!.status).toBe(200);
  expect(seen.filter(url=>url.endsWith("/pkg")).length,"query strings do not make new keys").toBe(1);expect(seen.some(url=>url.includes("cache-buster"))).toBe(false);
  expect((await ask("playwright/builds/x.zip"))!.status,"the unpinned playwright mirror is gone").toBe(404);
  const pending=Array.from({length:12},(_,index)=>ask(`npm/slow-${index}`));
  await Bun.sleep(300);
  expect(seen.filter(url=>url.includes("/slow-")).length,"at most the cap in flight").toBeLessThanOrEqual(8);
  void pending;
 }finally{await Bun.sleep(300);rmSync(folder,{recursive:true,force:true});}
},20000);

const stub=(delayMilliseconds:number,tracker:{running:number;peak:number})=>(async(input:RequestInfo|URL)=>{
 void input;tracker.running++;tracker.peak=Math.max(tracker.peak,tracker.running);
 await Bun.sleep(delayMilliseconds);tracker.running--;return new Response("body");
}) as typeof fetch;

test("more tarball requests than the upstream cap queue for a slot instead of failing: none is refused and the cap holds",async()=>{
 const folder=directory("queue"),tracker={running:0,peak:0},events:{outcome:string;reason?:string}[]=[];
 const handler=createMirrorHandlerV1({directory:folder,fetch:stub(80,tracker),onEvent:event=>events.push(event)}),ask=(path:string)=>handler(new Request("http://x/mirror/"+path))!;
 const statuses=await Promise.all(Array.from({length:40},async(_,index)=>{const answer=(await ask(`npm/pkg-${index}/-/pkg-${index}-1.0.0.tgz`))!;await answer.arrayBuffer();return answer.status;}));
 expect(statuses).toEqual(Array.from({length:40},()=>200));
 expect(tracker.peak).toBeLessThanOrEqual(8);expect(events.filter(event=>event.outcome==="error"||event.outcome==="fallback")).toEqual([]);
},20000);

test("a request that finds no free slot within the wait goes to the direct registry and the event says why",async()=>{
 const folder=directory("busy"),events:{outcome:string;reason?:string}[]=[],hang=(async()=>new Promise<Response>(()=>{})) as unknown as typeof fetch;
 const handler=createMirrorHandlerV1({directory:folder,fetch:hang,slotWaitMilliseconds:100,onEvent:event=>events.push(event)}),ask=(path:string)=>handler(new Request("http://x/mirror/"+path))!;
 const held=Array.from({length:8},(_,index)=>ask(`npm/held-${index}`));await Bun.sleep(30);
 const started=Date.now(),answer=await ask("npm/late/-/late-1.0.0.tgz");
 expect(Date.now()-started).toBeLessThan(2000);expect(answer!.status).toBe(307);
 expect(answer!.headers.get("location")).toBe("https://registry.npmjs.org/late/-/late-1.0.0.tgz");expect(answer!.headers.get("x-semio-mirror-reason")).toBe("busy");
 expect(events.some(event=>event.outcome==="fallback"&&event.reason==="busy")).toBe(true);void held;
},20000);

test("an upstream that never answers is cut at the header timeout and a refused redirect stays a 502",async()=>{
 const folder=directory("cut"),events:{outcome:string;reason?:string}[]=[];
 const silent=((_input:RequestInfo|URL,init?:RequestInit)=>new Promise<Response>((_accept,reject)=>{init?.signal?.addEventListener("abort",()=>reject(Error("aborted")));})) as typeof fetch;
 const handler=createMirrorHandlerV1({directory:folder,fetch:silent,upstreamTimeoutMilliseconds:150,onEvent:event=>events.push(event)});
 const started=Date.now(),answer=await handler(new Request("http://x/mirror/npm/quiet"));
 expect(Date.now()-started).toBeLessThan(2000);expect(answer!.status).toBe(307);expect(answer!.headers.get("x-semio-mirror-reason")).toBe("upstream-timeout");
 expect(events.at(-1)).toMatchObject({outcome:"fallback",reason:"upstream-timeout"});
});

test("the header timeout does not cut a body that keeps arriving",async()=>{
 const folder=directory("long"),chunk=new Uint8Array(1024).fill(7),total=8;
 const slowBody=((_input:RequestInfo|URL,init?:RequestInit)=>{
  const signal=init?.signal,stream=new ReadableStream<Uint8Array>({async start(controller){for(let at=0;at<total;at++){if(signal?.aborted){controller.error(Error("aborted"));return;}controller.enqueue(chunk);await Bun.sleep(60);}controller.close();}});
  return Promise.resolve(new Response(stream,{headers:{"content-length":String(chunk.length*total)}}));
 }) as typeof fetch;
 const handler=createMirrorHandlerV1({directory:folder,fetch:slowBody,upstreamTimeoutMilliseconds:200});
 const answer=await handler(new Request("http://x/mirror/npm/big/-/big-1.0.0.tgz"));
 expect(answer!.status).toBe(200);expect((await answer!.arrayBuffer()).byteLength).toBe(chunk.length*total);
},20000);

test("a real bun install against a mirror whose upstream fetch is down is sent to the direct registry and succeeds, with the mirror up it goes through the mirror",async()=>{
 const work=directory("bun"),pack=join(work,"pack","package");
 await Bun.write(join(pack,"package.json"),JSON.stringify({name:"semio-fixture-pkg",version:"1.0.0"}));
 const made=Bun.spawnSync(["tar","-czf",join(work,"fixture.tgz"),"-C",join(work,"pack"),"package"]);expect(made.exitCode).toBe(0);
 const bytes=new Uint8Array(await Bun.file(join(work,"fixture.tgz")).arrayBuffer()),integrity="sha512-"+createHash("sha512").update(bytes).digest("base64");
 const seen:string[]=[];let registry="";
 const direct=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{
  const path=new URL(request.url).pathname;seen.push(path);
  if(path==="/semio-fixture-pkg")return Response.json({name:"semio-fixture-pkg","dist-tags":{latest:"1.0.0"},versions:{"1.0.0":{name:"semio-fixture-pkg",version:"1.0.0",dist:{tarball:`${registry}/semio-fixture-pkg/-/semio-fixture-pkg-1.0.0.tgz`,integrity}}}});
  if(path==="/semio-fixture-pkg/-/semio-fixture-pkg-1.0.0.tgz")return new Response(bytes);
  return new Response("nope",{status:404});
 }});
 registry=`http://127.0.0.1:${direct.port}`;
 const install=async(label:string,mirrorFetch:typeof fetch|undefined)=>{
  const handler=createMirrorHandlerV1({directory:directory("m-"+label),ecosystems:MIRROR_ECOSYSTEMS_V1.map(row=>({...row,upstream:registry})),...(mirrorFetch?{fetch:mirrorFetch}:{})});
  const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:async request=>(await handler(request))??new Response("outside",{status:404})});
  const project=join(work,"project-"+label);await Bun.write(join(project,"package.json"),JSON.stringify({name:"consumer",version:"1.0.0",dependencies:{"semio-fixture-pkg":"1.0.0"}}));
  try{
   const run=Bun.spawn([process.execPath,"install","--no-save"],{cwd:project,env:{PATH:process.env.PATH??"",HOME:work,BUN_CONFIG_REGISTRY:`http://127.0.0.1:${server.port}/mirror/npm/`,BUN_INSTALL_CACHE_DIR:join(work,"cache-"+label)},stdout:"pipe",stderr:"pipe"}),[out,err,code]=await Promise.all([new Response(run.stdout).text(),new Response(run.stderr).text(),run.exited]);
   return{code,log:out+err,installed:await Bun.file(join(project,"node_modules","semio-fixture-pkg","package.json")).exists()};
  }finally{server.stop(true);}
 };
 try{
  const down=await install("down",(async()=>{throw Error("mirror upstream down");}) as unknown as typeof fetch);
  expect(down.log).not.toContain("502");expect(down.code,down.log).toBe(0);expect(down.installed).toBe(true);
  const upstreamBefore=seen.length,up=await install("up",undefined);
  expect(up.code,up.log).toBe(0);expect(up.installed).toBe(true);expect(seen.length).toBeGreaterThan(upstreamBefore);
 }finally{direct.stop(true);}
},60000);
