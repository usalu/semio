import{test,expect}from"bun:test";
import{createConnection,type Socket}from"node:net";
import{createInterface}from"node:readline";
import Ajv from"ajv";
import{startAdmissionBrokerV1,unitObservationsV1,compilerWrapperEnvironmentV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

/** ♻️ A unit served from the unit cache is reported to the broker with the measurements recorded when it was built: it joins the unit list and the critical path, is marked as cached, never enters the scheduling history, and cannot be recalled as a fresh compile. */
const ajv=new Ajv({strict:false});ajv.addSchema(schema,"admission");
const validate=(name:string)=>ajv.getSchema("admission#/definitions/"+name)!;
const key="a".repeat(64),unit=(crate:string,outDir:string)=>({v:1,pass:"p1",pid:4242,crate,package:crate,version:"0.1.0",target:"host",crateTypes:["lib"],outDir,extraFilename:"-1f",source:"/ws/"+crate+"/src/lib.rs",manifestDir:"/ws/"+crate,primary:false,buildScriptOutDir:null,externs:["/lane/b/deps/libdep-9.rmeta"]});
function converse(endpoint:string,lines:readonly object[],replies:number):Promise<Record<string,unknown>[]>{
 return new Promise((done,fail)=>{
  const socket:Socket=createConnection(endpoint),rows:Record<string,unknown>[]=[],timer=setTimeout(()=>{socket.destroy();done(rows);},3000);
  socket.on("error",fail);createInterface({input:socket}).on("line",line=>{rows.push(JSON.parse(line));if(rows.length===replies){clearTimeout(timer);socket.destroy();done(rows);}});
  socket.on("close",()=>{clearTimeout(timer);done(rows);});
  socket.on("connect",()=>{for(const line of lines)socket.write(JSON.stringify(line)+"\n");});
 });
}

test("cached units are recorded with their measurements, marked, excluded from history and not recallable; invalid reports are refused",async()=>{
 const broker=await startAdmissionBrokerV1({physicalBytes:64*2**30,history:new Map()});
 try{
  broker.registerPass({id:"p1",profile:"dev",target:"host"});
  const compiled=unit("alpha","/lane/a/deps");
  await converse(broker.endpoint,[{op:"admit",...compiled},{v:1,op:"done",exit:0,peakRssBytes:123456,userMs:40,sysMs:5,startMs:1000,endMs:1900,rmetaMs:1400}],1);
  await Bun.sleep(100);
  const recalled=await converse(broker.endpoint,[{v:1,op:"recall",pass:"p1",crate:"alpha",outDir:"/lane/a/deps",extraFilename:"-1f"}],1);
  expect(recalled[0]!.op).toBe("unit");const record=recalled[0]!.record as {peakRssBytes:number;startMs:number;endMs:number;rmetaMs:number};
  expect(record.peakRssBytes).toBe(123456);expect(record.endMs-record.startMs).toBe(900);expect(record.rmetaMs-record.startMs).toBe(400);
  expect((await converse(broker.endpoint,[{v:1,op:"recall",pass:"p1",crate:"missing",outDir:"/lane/a/deps",extraFilename:"-1f"}],1))[0]!.record).toBeNull();

  const cached={...unit("alpha","/lane/b/deps"),op:"cached",cacheKey:key,provenance:{peakRssBytes:record.peakRssBytes,userMs:40,sysMs:5,wallMs:900,rmetaOffsetMs:400}};
  expect(validate("CachedRequestV1")(cached),JSON.stringify(validate("CachedRequestV1").errors)).toBe(true);
  const verdict=await converse(broker.endpoint,[cached],1);expect(verdict[0]).toEqual({v:1,op:"verdict",admitted:true});
  const units=broker.units();expect(units.length).toBe(2);
  const served=units[1]!;expect(served.cache).toEqual({key,recorded:true});expect(served.exit).toBe(0);expect(served.peakRssBytes).toBe(123456);expect(served.predictedBytes).toBe(0);expect(served.endMs-served.startMs).toBe(0);
  expect(served.artifact.startsWith("/lane/b/deps/")).toBe(true);expect(served.dependencies[0]!.edge).toBe("rmeta");expect(units[0]!.cache??null).toBeNull();
  expect(unitObservationsV1(units,new Map(),new Map([[units[0]!.artifact,10]]),5).length,"history keeps real compiles only").toBe(1);
  expect((await converse(broker.endpoint,[{v:1,op:"recall",pass:"p1",crate:"alpha",outDir:"/lane/b/deps",extraFilename:"-1f"}],1))[0]!.record,"a cached unit is not a compile").toBeNull();

  const unrecorded=await converse(broker.endpoint,[{...unit("beta","/lane/b/deps"),op:"cached",cacheKey:key,provenance:null}],1);expect(unrecorded[0]!.admitted).toBe(true);expect(broker.units()[2]!.cache).toEqual({key,recorded:false});expect(broker.units()[2]!.peakRssBytes).toBe(0);
  const refused=await converse(broker.endpoint,[{...unit("gamma","/lane/b/deps"),op:"cached",cacheKey:"nothex",provenance:null}],1);expect(refused.length).toBe(0);expect(broker.units().length).toBe(3);
  const unknown=await converse(broker.endpoint,[{...unit("gamma","/lane/b/deps"),pass:"other",op:"cached",cacheKey:key,provenance:null}],1);expect(unknown.length).toBe(0);
  expect(broker.stats().running).toBe(0);
 }finally{await broker.close();}
});

test("the compiler chain of a build names the cache wrapper only when the environment names a cache",()=>{
 expect(compilerWrapperEnvironmentV1("/t/scheduler","/gen",{})).toEqual({});
 expect(compilerWrapperEnvironmentV1("/t/scheduler","/gen",{SEMIO_UNIT_CACHE_ROOT:"/m/u"})).toEqual({});
 expect(compilerWrapperEnvironmentV1("/t/scheduler","/gen",{SEMIO_UNIT_CACHE_ROOT:"/m/u",SEMIO_UNIT_CACHE_SCRIPT:"/s.ts",SEMIO_UNIT_CACHE:"off"})).toEqual({});
 const chain=compilerWrapperEnvironmentV1("/t/scheduler","/gen",{SEMIO_UNIT_CACHE_ROOT:"/m/u",SEMIO_UNIT_CACHE_SCRIPT:"/s.ts",RUSTC:"rustc-x",SEMIO_UNIT_CACHE_SLOTS:"3",SEMIO_UNIT_CACHE_TOOLCHAIN:"tc"});
 expect(chain.RUSTC_WRAPPER).toBe(process.execPath);expect(chain.RUSTC).toBe("/s.ts");expect(JSON.parse(chain.SEMIO_UNIT_CACHE_INNER)).toEqual(["/t/scheduler","rustc-x"]);expect(chain.SEMIO_UNIT_CACHE_LANE_ROOT).toBe("/gen");expect(chain.SEMIO_UNIT_CACHE_SLOTS).toBe("3");expect(chain.SEMIO_UNIT_CACHE_BUILD_SCRIPTS).toBe("on");expect(chain.SEMIO_UNIT_CACHE_BROKER_REPLAY).toBe("on");expect(chain.SEMIO_UNIT_CACHE_TOOLCHAIN).toBe("tc");
});
