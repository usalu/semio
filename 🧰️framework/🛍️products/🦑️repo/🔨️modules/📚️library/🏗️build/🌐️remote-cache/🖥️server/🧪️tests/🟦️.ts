import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {collectRemoteCacheDirectoryV1,scanRemoteCacheDirectoryV1} from "../../🧹️collector/🟦️.ts";
import {serveRemoteCacheV1,type RemoteCacheCommandContextV1} from "../../⌨️command/🟦️.ts";
import {RemoteCacheDirectoryBusyV1,startRemoteCacheServerV1,type RemoteCacheServerV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dir,"../../🔬️action-result/🧪️tests/🐍️oracle/🐍️.py"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-srv-${label}-`));scratch.push(path);return path;};
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const hex=(value:string)=>Uint8Array.from(value.match(/../gu)??[],byte=>Number.parseInt(byte,16));
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

async function serve(options:Partial<Parameters<typeof startRemoteCacheServerV1>[0]>={}):Promise<RemoteCacheServerV1>{
 return startRemoteCacheServerV1({directory:directory("store"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{},...options});
}

async function run(command:string,args:string[]):Promise<{code:number;out:string;err:string}>{
 const child=Bun.spawn([command,...args],{stdout:"pipe",stderr:"pipe"}),[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 return {code,out,err};
}

async function runPython(program:string,input:unknown):Promise<unknown>{
 const child=Bun.spawn([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"}),[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 expect(code,err).toBe(0);return JSON.parse(out);
}

const certificate=(folder:string):{certificate:string;key:string}|undefined=>{
 const openssl=Bun.which("openssl");if(!openssl)return undefined;
 const certificate=join(folder,"cert.pem"),key=join(folder,"key.pem"),made=Bun.spawnSync([openssl,"req","-x509","-newkey","rsa:2048","-nodes","-keyout",key,"-out",certificate,"-days","2","-subj","/CN=localhost","-addext","subjectAltName=DNS:localhost,IP:127.0.0.1"],{stdout:"pipe",stderr:"pipe"});
 return made.exitCode===0&&existsSync(certificate)&&existsSync(key)?{certificate,key}:undefined;
};

test.skipIf(!python||!Bun.which("openssl"))("over TLS the server is reachable by a client that verifies the certificate chain (Python, curl, fetch) and by nobody who does not trust it",async()=>{
 const folder=directory("tls"),pair=certificate(folder);if(!pair)return;
 const server=await serve({tls:pair,tokens:{write:"tls-token"},validation:"off"}),blob="hello over tls",key=sha(blob);
 try{
  expect(server.url).toMatch(/^https:\/\/127\.0\.0\.1:[0-9]+$/u);
  const pythonProgram="import http.client,json,ssl,sys\nrequest=json.loads(sys.stdin.read())\ncontext=ssl.create_default_context(cafile=request['ca'])\nc=http.client.HTTPSConnection('127.0.0.1',request['port'],context=context,timeout=30)\nc.request('PUT','/cas/'+request['key'],body=request['blob'].encode(),headers={'Authorization':'Bearer tls-token'})\nput=c.getresponse();put.read()\nc.request('GET','/cas/'+request['key'],headers={'Authorization':'Bearer tls-token'})\nget=c.getresponse();body=get.read().decode()\nc2=http.client.HTTPSConnection('127.0.0.1',request['port'],context=ssl.create_default_context(),timeout=30)\ntry:\n  c2.request('GET','/healthz');trusted=c2.getresponse().status\nexcept ssl.SSLError as error:\n  trusted='refused'\nprint(json.dumps({'put':put.status,'get':get.status,'body':body,'untrusted':trusted}))";
  expect(await runPython(pythonProgram,{ca:pair.certificate,port:server.port,key,blob})).toEqual({put:200,get:200,body:blob,untrusted:"refused"});
  const curl=Bun.which("curl");
  if(curl){const asked=await run(curl,["-sS","--cacert",pair.certificate,"-H","Authorization: Bearer tls-token",`${server.url}/cas/${key}`]);expect(asked.code,asked.err).toBe(0);expect(asked.out).toBe(blob);const refused=await run(curl,["-sS",`${server.url}/healthz`]);expect(refused.code,"curl without the CA refuses").not.toBe(0);}
  const trusted=await fetch(`${server.url}/cas/${key}`,{headers:{authorization:"Bearer tls-token"},tls:{ca:await Bun.file(pair.certificate).text()}});expect(await trusted.text()).toBe(blob);
  await expect(fetch(`${server.url}/healthz`)).rejects.toThrow();
  const plain=await fetch(`http://127.0.0.1:${server.port}/healthz`).then(response=>response.status,()=>"refused");expect(plain).not.toBe(200);
 }finally{await server.stop();}
 await expect(serve({tls:{certificate:join(folder,"missing.pem"),key:pair.key}})).rejects.toThrow("TLS file required");
},90000);

type Vector={name:string;kind:"result"|"tree";hex:string;valid:boolean;blobs?:string[]};

test.skipIf(!python)("real action results travel the protocol in Bazel's order: a result is refused until its blobs are stored, read back unchanged, and answered as a miss once a blob is gone",async()=>{
 const run=Bun.spawnSync([python!,"-I",oracle,"vectors"],{stdout:"pipe",stderr:"pipe"});expect(run.exitCode,run.stderr.toString()).toBe(0);
 const vectors=(JSON.parse(run.stdout.toString()) as Vector[]).filter(row=>row.valid&&row.kind==="result"),server=await serve(),counters=async()=>(await (await fetch(server.url+"/status")).json() as {counters:{ac:{stale:number;rejected:number}}}).counters.ac;
 try{
  let withBlobs=0,stale=0;
  for(const [index,row] of vectors.entries()){
   const result=hex(row.hex),key=sha(row.name),blobs=[...new Set(row.blobs??[])],base=`${server.url}/n${index}`;
   if(blobs.length){
    withBlobs++;
    const early=await fetch(`${base}/ac/${key}`,{method:"PUT",body:result});expect(early.status,row.name+" before its blobs").toBe(400);expect(await early.text()).toContain("is not in the content store");
    for(const blob of blobs){const bytes=hex(blob);expect((await fetch(`${base}/cas/${sha(bytes)}`,{method:"PUT",body:bytes})).status).toBe(200);}
   }
   expect((await fetch(`${base}/ac/${key}`,{method:"PUT",body:result})).status,row.name).toBe(200);
   const back=await fetch(`${base}/ac/${key}`);expect(back.status).toBe(200);expect(Buffer.from(await back.arrayBuffer()).toString("hex"),row.name+" reads back byte for byte").toBe(row.hex);
   if(blobs.length){
    const victim=hex(blobs[0]!);await server.store.remove(`n${index}`,"cas",sha(victim));
    const gone=await fetch(`${base}/ac/${key}`);expect(gone.status,row.name+" after a blob left the store").toBe(404);stale++;
    expect((await fetch(`${base}/ac/${key}`)).status,"the stale result was dropped").toBe(404);
    expect((await fetch(`${base}/cas/${sha(victim)}`,{method:"PUT",body:victim})).status).toBe(200);
    expect((await fetch(`${base}/ac/${key}`,{method:"PUT",body:result})).status,"stored again once the blob is back").toBe(200);
   }
  }
  expect(withBlobs).toBeGreaterThanOrEqual(3);expect((await counters()).stale).toBe(stale);expect((await counters()).rejected).toBe(withBlobs);
  const malformed=await fetch(`${server.url}/ac/${sha("junk")}`,{method:"PUT",body:new Uint8Array([0x12,0xff,0xff,0xff])});expect(malformed.status).toBe(400);expect(await malformed.text()).toContain("Invalid action result");
  const oversize=await serve({maximumActionResultBytes:1024});
  try{const big=await fetch(`${oversize.url}/ac/${sha("big")}`,{method:"PUT",body:new Uint8Array(2048)});expect(big.status).toBe(413);}finally{await oversize.stop();}
 }finally{await server.stop();}
},60000);

const hammer=`import concurrent.futures,hashlib,http.client,json,sys,urllib.parse
request=json.loads(sys.stdin.read())
parts=urllib.parse.urlsplit(request['url'])
who=request['who']
def call(method,path,body=None):
  c=http.client.HTTPConnection(parts.hostname,parts.port,timeout=60)
  try:
    c.request(method,path,body=body)
    r=c.getresponse()
    return r.status,r.read()
  finally:
    c.close()
def blob(i):
  data=(f'blob-{(i//4)%60}-'*40).encode()
  s,_=call('PUT','/cas/'+hashlib.sha256(data).hexdigest(),data)
  return ('cas',s)
def nx(i):
  data=(f'{who}-{i}'.ljust(256,'x')).encode()
  s,_=call('PUT','/v1/cache/key'+str((i//4)%12),data)
  return ('nx',s,(i//4)%12,data.decode() if s==200 else None)
def ac(i):
  s,_=call('PUT','/ac/'+hashlib.sha256(str((i//4)%5).encode()).hexdigest(),bytes([0x20,i%100]))
  return ('ac',s)
def read(i):
  s,b=call('GET','/cas/'+hashlib.sha256((f'blob-{(i//4)%60}-'*40).encode()).hexdigest())
  return ('get',s,hashlib.sha256(b).hexdigest() if s==200 else None,(i//4)%60)
jobs=[]
with concurrent.futures.ThreadPoolExecutor(max_workers=24) as pool:
  for i in range(960):
    jobs.append(pool.submit([blob,nx,ac,read][i%4],i))
  rows=[j.result() for j in jobs]
print(json.dumps(rows))`;

test.skipIf(!python)("three client processes with 24 threads each write overlapping keys: blobs verify, a write-once key has exactly one winner whose bytes are what is stored, results are never torn, nothing partial stays",async()=>{
 const folder=directory("hammer"),server=await serve({directory:folder});
 try{
  const rows=(await Promise.all(["a","b","c"].map(who=>runPython(hammer,{url:server.url,who})))).flat() as unknown[][];
  const statuses=new Map<string,number>();for(const row of rows){const label=`${row[0]}:${row[1]}`;statuses.set(label,(statuses.get(label)??0)+1);}
  for(const label of statuses.keys())expect(["cas:200","nx:200","nx:409","ac:200","get:200","get:404"],label).toContain(label);
  const winners=new Map<number,string[]>();for(const row of rows)if(row[0]==="nx"&&row[1]===200)winners.set(row[2] as number,[...(winners.get(row[2] as number)??[]),row[3] as string]);
  expect([...winners.keys()].sort((a,b)=>a-b)).toEqual(Array.from({length:12},(_,index)=>index));
  for(const [index,values] of winners){expect(values.length,`key${index} has exactly one winner`).toBe(1);expect(await (await fetch(`${server.url}/v1/cache/key${index}`)).text()).toBe(values[0]!);}
  for(const row of rows)if(row[0]==="get"&&row[1]===200)expect(row[2]).toBe(sha((`blob-${row[3]}-`).repeat(40)));
  for(let index=0;index<5;index++){const body=new Uint8Array(await (await fetch(`${server.url}/ac/${sha(String(index))}`)).arrayBuffer());expect(body.length).toBe(2);expect(body[0]).toBe(0x20);}
  const usage=server.store.usage(),disk=await scanRemoteCacheDirectoryV1(join(folder,"v1"));let bytes=0;for(const row of disk.values())bytes+=row.size;
  expect(usage.entries).toBe(disk.size);expect(usage.bytes).toBe(bytes);expect(usage.reservedBytes).toBe(0);
  expect(readdirSync(join(folder,"v1",".tmp")).flatMap(name=>readdirSync(join(folder,"v1",".tmp",name))),"no partial write is left").toEqual([]);
  for(const [key] of disk)if(key.startsWith("main/cas/")){const content=Buffer.from(await (await fetch(`${server.url}/cas/${key.split("/")[3]}`)).arrayBuffer());expect(sha(content)).toBe(key.split("/")[3]!);}
 }finally{await server.stop();}
},120000);

test.skipIf(!python)("an outside collector may run while clients write and read: no request fails, and the server's view heals to the disk",async()=>{
 const folder=directory("outside"),server=await serve({directory:folder,maximumBytes:2**22,maximumEntryBytes:2**20,maintenanceSeconds:1});
 try{
  const traffic=Promise.all(["a","b"].map(who=>runPython(hammer,{url:server.url,who}))),collections:unknown[]=[];
  for(let round=0;round<4;round++){await new Promise(resolve=>setTimeout(resolve,150));collections.push(await collectRemoteCacheDirectoryV1({root:join(folder,"v1"),maximumBytes:20_000,maximumAgeMilliseconds:0,dryRun:false,verifyResults:true}));}
  const rows=(await traffic).flat() as unknown[][];
  for(const row of rows)expect([200,404,409,507],`${row[0]} ${row[1]}`).toContain(row[1] as number);
  expect(collections.some(row=>(row as {locked:boolean}).locked===false)).toBe(true);
  await new Promise(resolve=>setTimeout(resolve,2600));
  const disk=await scanRemoteCacheDirectoryV1(join(folder,"v1"));let bytes=0;for(const row of disk.values())bytes+=row.size;
  const status=await (await fetch(server.url+"/status")).json() as {entries:number;bytes:number};
  expect(status.entries,"maintenance re-read the directory").toBe(disk.size);expect(status.bytes).toBe(bytes);
  const gone=[...disk.keys()].find(key=>key.startsWith("main/nx/"));
  if(gone){rmSync(join(folder,"v1",...gone.split("/")));expect((await fetch(`${server.url}/v1/cache/${gone.split("/")[3]}`)).status,"a value removed by another process is a miss, not an error").toBe(404);}
 }finally{await server.stop();}
},120000);

test("one server owns a directory: a living owner blocks a second server, a dead owner's lock is taken over",async()=>{
 const folder=directory("owner"),first=await serve({directory:folder});
 try{await expect(serve({directory:folder})).rejects.toBeInstanceOf(RemoteCacheDirectoryBusyV1);}finally{await first.stop();}
 expect(existsSync(join(folder,"v1",".server.lock")),"the lock is released on stop").toBe(false);
 writeFileSync(join(folder,"v1",".server.lock"),`2147483646\n${Date.now()}\n`);
 const second=await serve({directory:folder});await second.stop();
 mkdirSync(join(folder,"v1"),{recursive:true});writeFileSync(join(folder,"v1",".server.lock"),`${process.pid}\n${Date.now()}\n`);
 await expect(serve({directory:folder})).rejects.toThrow("Another remote cache server uses");
});

test("the environment switches a server to read-only and rejects a value that is neither true nor false",async()=>{
 const folder=directory("env"),lines:string[]=[],controller=new AbortController();
 const context=(environment:Record<string,string>,write:(line:string)=>void):RemoteCacheCommandContextV1=>({root:folder,environment,signal:controller.signal,progress:async()=>{},write,tools:async()=>[]});
 const running=serveRemoteCacheV1(["--port","0","--directory","store","--lang","en"],context({SEMIO_REMOTE_CACHE_SERVER_READ_ONLY:"true"},line=>lines.push(line)));
 const address=await new Promise<string>((resolve,reject)=>{const started=Date.now(),timer=setInterval(()=>{const line=lines.find(candidate=>candidate.startsWith("[remote-cache] listening "));if(line){clearInterval(timer);resolve(line.slice(25));}else if(Date.now()-started>20000){clearInterval(timer);reject(Error("no listening line"));}},20);});
 expect((await fetch(`${address}/cas/${sha("x")}`,{method:"PUT",body:"x"})).status).toBe(403);
 expect(((await (await fetch(`${address}/status`)).json()) as {readOnly:boolean}).readOnly).toBe(true);expect(lines.some(line=>line.includes("(read-only, plain HTTP"))).toBe(true);
 controller.abort();await running;
 await expect(serveRemoteCacheV1(["--port","0"],context({SEMIO_REMOTE_CACHE_SERVER_READ_ONLY:"maybe"},()=>{}))).rejects.toThrow("Unknown option for remote-cache serve");
});
