/** 🎣️ Law: a read adopts what the fleet publishes while its own take pass still runs. With a fleet that publishes every owner within T, a read whose own looks are slow finishes in about T (not in the time of its own looks), looks locally only at what the fleet had not delivered when the look began, and an owner decided by a record from the fleet is never looked at again by the pass.
 * @see ../../🟨️.mjs `takeFetchFirstV1` */
import{expect,test}from"bun:test";
import{TAKE_POLICY_V1,takeFetchFirstV1}from"../../🟨️.mjs";
const source=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text();

const POLICY={...TAKE_POLICY_V1,pollFirstMilliseconds:20,hotRetryMilliseconds:40,adoptRetryMilliseconds:40};
const simulate=async(count:number,publishWithin:number,localMilliseconds:number,concurrency=2)=>{
 const names=Array.from({length:count},(_,at)=>"n"+at),begun=performance.now(),decided=new Map<string,string>(),local:string[]=[];
 const publishedAt=(name:string)=>publishWithin*(Number(name.slice(1))+1)/count;
 const published=()=>new Set(names.filter(name=>performance.now()-begun>=publishedAt(name)));
 const attempt=async(name:string)=>{
  if(decided.has(name))return"present";
  if(published().has(name)){await Bun.sleep(2);decided.set(name,"fetched");return"fetched";}
  local.push(name);await Bun.sleep(localMilliseconds);return"miss";
 };
 await takeFetchFirstV1({lanes:[names],heavy:[],concurrency,attempt,hot:published,queue:()=>names.filter(name=>!decided.has(name)),arrived:name=>{decided.set(name,"fetched");},stop:()=>decided.size===count,policy:POLICY,mark:()=>{}});
 return{elapsed:performance.now()-begun,decided,local};
};

test("a fleet that publishes every owner within T ends the read in about T while the own looks alone would take five times longer",async()=>{
 const{elapsed,decided,local}=await simulate(30,400,150);
 expect(decided.size).toBe(30);expect([...decided.values()].every(outcome=>outcome==="fetched")).toBe(true);
 expect(elapsed,"own looks alone: 30 x 150 ms / 2 = 2250 ms").toBeLessThan(1100);
 expect(local.length,"only owners that were not yet published were looked at here").toBeLessThan(30);
});

test("a fleet that publishes nothing leaves every owner to the own looks and the read does not hang",async()=>{
 const{decided,local,elapsed}=await simulate(6,1e9,10);
 expect(decided.size).toBe(0);expect(local.length).toBe(6);expect(elapsed).toBeLessThan(500);
});

test("an owner the pass missed and the fleet delivered later is adopted by the poll without waiting for the pass",async()=>{
 const{decided,elapsed}=await simulate(4,300,400,1);
 expect(decided.size).toBe(4);expect(elapsed).toBeLessThan(900);
});

test("the read starts the adopt loop with the take pass and counts an owner once whoever decided it",()=>{
 expect(source).toContain("await takeFetchFirstV1({lanes,heavy:split.heavy");
 expect(source).toContain("counted=name=>{if(!countedNames.has(name))");
 expect(source).not.toContain("pass(split.heavy,1)");
});
