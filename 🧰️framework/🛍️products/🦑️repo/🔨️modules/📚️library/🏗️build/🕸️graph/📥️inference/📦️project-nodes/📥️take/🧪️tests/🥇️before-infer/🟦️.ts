/** 🥇️ Law: with every owner stored in the fleet, a read performs zero local inferences and ends in look time. A farm model (a fixed number of children, each look takes L, each inference takes I) and a simulated fleet drive the real decision before an inference (`takeBeforeInferV1`) through the same owner loop shape the read runs (20 workers over the owners in order). The control is the old behaviour, an inference for every owner whose look has not been asked yet, and infers every owner. An owner the fleet lists but whose record is stale, missed, incomplete or faulted is inferred; an unlisted owner is inferred; an owner taken already is not looked at again; a look that is already in flight is the one the loop waits for.
 * @see ../../🟨️.mjs `takeBeforeInferV1` */
import{expect,test}from"bun:test";
import{createAdoptCountersV1,takeBeforeInferV1,takeAdmittedV1,takeFetchFirstV1,TAKE_POLICY_V1}from"../../🟨️.mjs";
import source from"../../../🟨️.mjs"with{type:"text"};

type Outcome="present"|"fetched"|"stale"|"miss"|"fault"|"incomplete";
const farm=(children:number,milliseconds:number)=>{let busy=0;const waiting:(()=>void)[]=[];return async<T,>(work:()=>T)=>{while(busy>=children)await new Promise<void>(resolve=>waiting.push(resolve));busy++;try{await Bun.sleep(milliseconds);return work();}finally{busy--;waiting.shift()?.();}};};
const simulate=async(options:{count:number;listed:(name:string)=>boolean;record:(name:string)=>Outcome;lookMilliseconds:number;inferMilliseconds:number;children:number;workers:number;useDecision:boolean}) =>{
 const names=Array.from({length:options.count},(_,at)=>"o"+at),taken=new Set<string>(),inflight=new Map<string,Promise<Outcome>>(),counters=createAdoptCountersV1(),looks:string[]=[],inferred:string[]=[],begun=performance.now(),runLook=farm(options.children,options.lookMilliseconds),runInfer=farm(options.children,options.inferMilliseconds);
 const look=(name:string):Promise<Outcome>=>{
  const running=inflight.get(name);if(running!==undefined)return running;
  counters.look("infer");looks.push(name);
  const work=runLook(()=>{const outcome=options.record(name);if(outcome==="present"||outcome==="fetched")taken.add(name);counters.outcome(outcome);return outcome;}).finally(()=>inflight.delete(name));
  inflight.set(name,work);return work;
 };
 let cursor=0;
 await Promise.all(Array.from({length:options.workers},async()=>{
  while(cursor<names.length){
   const name=names[cursor++]!;
   if(options.useDecision){const adopted=await takeBeforeInferV1({name,listed:options.listed,taken,look,counters});if(adopted!==undefined)continue;}
   else if(taken.has(name))continue;
   await runInfer(()=>{inferred.push(name);taken.add(name);});
  }
 }));
 return{elapsed:performance.now()-begun,looks,inferred,taken,counters:counters.counts()};
};

test("every owner stored in the fleet: zero local inferences and the read ends in look time; the old behaviour infers every owner",async()=>{
 const base={count:60,listed:()=>true,record:()=>"fetched" as Outcome,lookMilliseconds:20,inferMilliseconds:120,children:6,workers:20};
 const got=await simulate({...base,useDecision:true}),old=await simulate({...base,useDecision:false});
 expect(got.inferred.length).toBe(0);expect(got.taken.size).toBe(60);expect(got.looks.length).toBe(60);
 expect(got.counters.fetched).toBe(60);expect(got.counters.inferredListed).toBe(0);
 expect(got.elapsed,"60 looks x 20 ms / 6 children = 200 ms of look time").toBeLessThan(600);
 expect(old.inferred.length,"control: the old loop infers every owner nobody looked at").toBe(60);
 expect(old.elapsed,"60 inferences x 120 ms / 6 children = 1200 ms").toBeGreaterThan(got.elapsed*2);
});

test("an owner that is not listed, or whose listed record is stale, missed, incomplete or faulted, is inferred; an owner taken already is not looked at",async()=>{
 const records:Record<string,Outcome>={o1:"stale",o2:"miss",o3:"incomplete",o4:"fault"};
 const got=await simulate({count:10,listed:name=>name!=="o5"&&name!=="o6",record:name=>records[name]??"fetched",lookMilliseconds:5,inferMilliseconds:5,children:4,workers:10,useDecision:true});
 expect(new Set(got.inferred)).toEqual(new Set(["o1","o2","o3","o4","o5","o6"]));
 expect(got.looks.sort()).toEqual(["o0","o1","o2","o3","o4","o7","o8","o9"]);
 expect(got.counters.inferredListed,"the four listed owners whose record did not serve").toBe(4);
 expect(got.counters.stale+got.counters.miss+got.counters.incomplete+got.counters.fault).toBe(4);
 const taken=new Set(["x"]);let looked=0;
 expect(await takeBeforeInferV1({name:"x",listed:()=>true,taken,look:async()=>{looked++;return"present";}})).toBe("present");expect(looked).toBe(0);
 expect(await takeBeforeInferV1({name:"y",listed:()=>true,taken,look:async()=>{throw Error("cancelled");}})).toBeUndefined();
});

test("a look that is already in flight is the one the loop waits for: two askers, one look",async()=>{
 const got=await simulate({count:1,listed:()=>true,record:()=>"fetched",lookMilliseconds:30,inferMilliseconds:30,children:2,workers:2,useDecision:true});
 expect(got.looks.length).toBe(1);expect(got.inferred.length).toBe(0);
});

test("the adopt lane has a gate of its own: with the pass lane blocked at the memory gate for good, the adopt loop still takes the owners the fleet lists",async()=>{
 const names=Array.from({length:12},(_,at)=>"n"+at),decided=new Map<string,string>(),blocked={admit:()=>new Promise<never>(()=>{})};
 const look=async(name:string)=>{if(decided.has(name))return"present";await Bun.sleep(3);decided.set(name,"fetched");return"fetched";};
 const flight={flying:1},pass=takeAdmittedV1(look,()=>blocked,undefined,flight),adopt=takeAdmittedV1(look,()=>undefined,undefined,{flying:0});
 const policy={...TAKE_POLICY_V1,pollFirstMilliseconds:10,adoptRetryMilliseconds:50};
 void takeFetchFirstV1({lanes:[names],heavy:[],concurrency:2,attempt:pass,adoptAttempt:adopt,hot:()=>new Set(names),queue:()=>names.filter(name=>!decided.has(name)),arrived:name=>{decided.set(name,"fetched");},stop:()=>decided.size===names.length,policy,mark:()=>{}});
 const begun=performance.now();while(decided.size<names.length&&performance.now()-begun<1500)await Bun.sleep(10);
 expect(decided.size,"the pass lane never moved, the adopt lane took all twelve").toBe(12);expect(performance.now()-begun).toBeLessThan(1000);
 const control=new Map<string,string>(),stuck=takeAdmittedV1(async name=>{control.set(name,"x");return"fetched";},()=>blocked,undefined,{flying:1});void stuck("a");await Bun.sleep(50);expect(control.size,"control: the same look behind the blocked gate never starts").toBe(0);
});

test("the read wires it: the owner loop decides before every inference, the adopt and poll looks have no footprint gate, the pass counts its waits, the line is printed",()=>{
 const text=source as unknown as string;
 expect(text).toContain("takeBeforeInferV1({name:job.name,listed:name=>listedNames().has(name),taken,look:()=>look(job,\"infer\"),counters:adopt})");
 expect(text).toContain("adoptAttempt=takeAdmittedV1((name,active)=>look(byName.get(name),\"adopt\",active),()=>undefined,signal,{flying:0})");
 expect(text).toContain("attempt:takeAdmittedV1((name,active)=>look(byName.get(name),\"adopt\",active),()=>undefined,polling.signal,{flying:0})");
 expect(text).toContain("countedGate");expect(text).toContain("adopt.line(language)");expect(text).toContain("adopt.staleRow(looked.staleRow?.kind)");
 expect(text).toContain("outcomes:adopt.lookups()");expect(text).toContain("recordTakeLookFailureV1(error,adopt,signal.aborted||waveSignal.aborted)");
 expect(text).toContain("adopt.look(lane)");expect(text).toContain("adopt.outcome(outcome)");expect(text).toContain("adopt.skipInferring(job.name)");
 expect(text.includes("fleetPublishedNamesV1(process.env.SEMIO_FLEET_STORE_ROOT)"),"the store is read in one place, through the hot names").toBe(true);
 expect(text.split("fleetPublishedNamesV1(process.env.SEMIO_FLEET_STORE_ROOT)").length-1).toBe(1);
});
