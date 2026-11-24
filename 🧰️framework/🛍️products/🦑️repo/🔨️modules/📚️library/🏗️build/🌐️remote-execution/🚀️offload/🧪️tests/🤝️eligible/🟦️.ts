import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {remoteExecutionHostPropertiesV1} from "../../../🟦️.ts";
import {RemoteExecutionContentErrorV1,type RemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import {eligibleFleetWorkersV1,fleetLinkedSlotsV1,writeOffloadMemoV1,type FleetWorkerV1} from "../../🚦️admission/🟦️.ts";
import {detectWorkerRustcV1,offloadRustcUnitV1} from "../../🟦️.ts";
import {createTestCheckoutV1,testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";

const worker=(origin:string,extra:Readonly<Record<string,string>>,running=0,concurrency=4,draining=false):FleetWorkerV1=>({origin,properties:[{name:"OSFamily",value:"macos"},...Object.entries(extra).map(([name,value])=>({name,value}))],concurrency,running,draining});
const linker=(id:string)=>({"semio-linker":id,"semio-target":"aarch64-apple-darwin"});

test("capability is the scheduler's own rule over the advertised properties: a worker without the required linker, a draining one and this machine itself are no capacity, and the free slots are counted over the capable ones only",()=>{
 const fleet={own:"g1-server",workers:[worker("g1-m01",linker("aaaa")),worker("g1-m02",linker("bbbb"),1,4),worker("g1-m03",{"semio-target":"aarch64-apple-darwin"}),worker("g1-m04",linker("aaaa"),0,4,true),worker("g1-server",linker("aaaa"))]};
 expect(eligibleFleetWorkersV1(fleet,[{name:"semio-linker",value:"aaaa"}])).toEqual({eligible:1,slots:4,total:4,unmet:[]});
 expect(eligibleFleetWorkersV1(fleet,[{name:"semio-linker",value:"bbbb"}])).toEqual({eligible:1,slots:3,total:4,unmet:[]});
 expect(eligibleFleetWorkersV1(fleet,[{name:"semio-linker",value:"cccc"},{name:"semio-target",value:"aarch64-apple-darwin"}])).toEqual({eligible:0,slots:0,total:4,unmet:["semio-linker=cccc"]});
 expect(eligibleFleetWorkersV1({own:"x",workers:[]},[{name:"semio-linker",value:"aaaa"}])).toEqual({eligible:0,slots:0,total:0,unmet:["semio-linker=aaaa"]});
});

test("a unit that no worker can take is declined at once with its own reason and touches the content store never; with a capable worker the attempt goes on to the upload",async()=>{
 const checkout=createTestCheckoutV1("eligible");checkout.compile("a","pub fn a()->u32{1}\n");checkout.crate("c","pub fn c()->u32{a::a()+1}\n");
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-eligible-"))),environment={...checkout.environment("c"),...testRustEnvironmentV1()};
 const touched:string[]=[],content=new Proxy({},{get:(_target,name)=>async()=>{touched.push(String(name));throw Error("content store touched");}}) as RemoteExecutionContentV1;
 const ports=(workers:readonly FleetWorkerV1[])=>({content,endpoint:"grpc://127.0.0.1:1",token:"t",instance:"main",workers:{own:"me",workers}});
 const run=(workers:readonly FleetWorkerV1[])=>offloadRustcUnitV1({args:checkout.arguments("c",["a"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"],ports:ports(workers)});
 try{
  const foreign=await run([worker("g1-m01",{"semio-toolchain":"0".repeat(16),"semio-target":"aarch64-apple-darwin"})]),started=Date.now();
  expect(foreign).toMatchObject({kind:"local",reason:"no-compatible-worker"});expect((foreign as {detail:string}).detail).toContain("semio-toolchain=");
  expect(touched).toEqual([]);expect(Date.now()-started).toBeLessThan(2000);
  expect(await run([])).toMatchObject({kind:"local",reason:"no-compatible-worker"});expect(touched).toEqual([]);
  const offer=detectWorkerRustcV1(checkout.root,environment,["rustc"])!,rows=[...remoteExecutionHostPropertiesV1(),...offer.properties,{name:"semio-toolchain",value:offer.toolchain.digest},{name:"semio-target",value:offer.toolchain.host}];
  const capable=await run([{origin:"g1-m02",properties:rows,concurrency:4,running:0,draining:false}]);
  expect(capable).toMatchObject({kind:"local",reason:"infrastructure"});expect(touched.length).toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the linked slots a build sizes its final jobs to never exceed the free slots of the workers that advertise this compiler's linker, and stay the scheduler's count when the linker or the worker list is unknown",async()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-linked-")));
 const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch(request){const path=new URL(request.url).pathname;if(path==="/v1/fleet/idle")return Response.json({candidates:5,slots:7});if(path==="/status")return Response.json({workers:[{origin:"g1-m01",properties:[{name:"semio-linker",value:"abcdabcdabcdabcd"}],concurrency:2,running:1},{origin:"g1-m02",properties:[{name:"semio-linker",value:"1111111111111111"}],concurrency:8,running:0}]});return new Response("no",{status:404});}});
 try{
  const environment={SEMIO_UNIT_CACHE_ROOT:root},ports={fleet:{fetch:((input:string|URL|Request)=>fetch(String(input).replace(/^https?:\/\/[^/]+/u,`http://127.0.0.1:${server.port}`)))as typeof fetch}};
  expect(await fleetLinkedSlotsV1(environment,ports)).toBe(7);
  await Bun.write(join(root,"offload-linker.json"),JSON.stringify({sysroot:"/x",signature:"",identity:"abcdabcdabcdabcd"}));writeOffloadMemoV1(root,"workers",{atMs:0,own:"",workers:[]});
  expect(await fleetLinkedSlotsV1(environment,ports)).toBe(1);
  await Bun.write(join(root,"offload-linker.json"),JSON.stringify({sysroot:"/x",signature:"",identity:"ffffffffffffffff"}));rmSync(join(root,"offload-workers.json"),{force:true});
  expect(await fleetLinkedSlotsV1(environment,ports)).toBe(0);
 }finally{server.stop(true);rmSync(root,{recursive:true,force:true});}
});

test("a content store that answered 507 is not asked again by every unit: the first unit pays one attempt, the next ones within the hold-off decline at once without touching the store, and another failure (413, 503) sets no hold-off",async()=>{
 const checkout=createTestCheckoutV1("storefull");checkout.compile("a","pub fn a()->u32{1}\n");checkout.crate("c","pub fn c()->u32{a::a()+1}\n");
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-full-"))),environment={...checkout.environment("c"),...testRustEnvironmentV1()};
 let touched=0,failure:Error=new RemoteExecutionContentErrorV1(8,"Content store answered 507 for cas/x: Insufficient storage");
 const content=new Proxy({},{get:()=>async()=>{touched++;throw failure;}}) as RemoteExecutionContentV1;
 const offer=detectWorkerRustcV1(checkout.root,environment,["rustc"])!,rows=[...remoteExecutionHostPropertiesV1(),...offer.properties,{name:"semio-toolchain",value:offer.toolchain.digest},{name:"semio-target",value:offer.toolchain.host}];
 const ports={content,endpoint:"grpc://127.0.0.1:1",token:"t",instance:"main",workers:{own:"me",workers:[{origin:"g1-m02",properties:rows,concurrency:4,running:0,draining:false}]}};
 const run=()=>offloadRustcUnitV1({args:checkout.arguments("c",["a"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"],ports});
 try{
  failure=new RemoteExecutionContentErrorV1(8,"Content store answered 413 for cas/x: too large");
  expect(await run()).toMatchObject({kind:"local",reason:"infrastructure"});const afterLarge=touched;expect(afterLarge).toBeGreaterThan(0);
  expect(await run()).toMatchObject({kind:"local",reason:"infrastructure"});expect(touched).toBeGreaterThan(afterLarge);
  failure=new RemoteExecutionContentErrorV1(8,"Content store answered 507 for cas/x: Insufficient storage");
  expect(await run()).toMatchObject({kind:"local",reason:"infrastructure"});const first=touched;
  const second=await run();expect(second).toMatchObject({kind:"local",reason:"infrastructure"});expect((second as {detail:string}).detail).toContain("full");expect(touched).toBe(first);
 }finally{rmSync(root,{recursive:true,force:true});}
});
