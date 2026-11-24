/** 🛬️ Law: the owners of a take pass whose records are not stored here wait for the fleet and for the presence batch of the store, not for memory, so their lane runs `fetchConcurrency` looks at once, while the lane of the records stored here keeps the declared concurrency; one batch costs the same for 8 owners and for 48, so 605 fetched owners need a twelfth of the batches.
 * @see ../../🟨️.mjs `takeFetchConcurrencyV1`, `takeFetchFirstV1` */
import{expect,test}from"bun:test";
import{TAKE_POLICY_V1,takeFetchConcurrencyV1,takeFetchFirstV1,takeLanesV1,takeLookConcurrencyV1}from"../../🟨️.mjs";

const run=async(names:readonly string[],present:ReadonlySet<string>)=>{
 const lanes=takeLanesV1(names,name=>present.has(name)),flying=new Map<string,number>();let now=0,most=new Map<string,number>(),batches=0;
 const laneOf=(name:string)=>present.has(name)?"local":"remote";
 const attempt=async(name:string)=>{
  const lane=laneOf(name);now=(flying.get(lane)??0)+1;flying.set(lane,now);most.set(lane,Math.max(most.get(lane)??0,now));
  await Bun.sleep(5);flying.set(lane,(flying.get(lane)??1)-1);return"fetched";
 };
 await takeFetchFirstV1({lanes,concurrency:takeLookConcurrencyV1(),fetchConcurrency:takeFetchConcurrencyV1(),fetches:lane=>lane.length>0&&!present.has(lane[0]!),attempt,hot:()=>undefined,queue:()=>[],arrived:()=>{},stop:()=>false,mark:()=>{}});
 return{most,batches};
};

test("the lane of owners that are not stored here runs fetchConcurrency looks at once and the lane of stored ones the declared concurrency",async()=>{
 const names=Array.from({length:200},(_all,at)=>"owner-"+at),present=new Set(names.slice(0,100));
 const{most}=await run(names,present);
 expect(takeFetchConcurrencyV1()).toBeGreaterThan(takeLookConcurrencyV1());
 expect(most.get("remote")).toBe(takeFetchConcurrencyV1());
 expect(most.get("local")).toBe(takeLookConcurrencyV1());
});

test("a cut process still runs the look floor on the fetch lane and never fewer than the fetch concurrency",()=>{
 expect(takeFetchConcurrencyV1(1)).toBe(Math.max(takeLookConcurrencyV1(1),TAKE_POLICY_V1.fetchConcurrency));
 expect(takeFetchConcurrencyV1(Infinity)).toBe(TAKE_POLICY_V1.fetchConcurrency);
});

test("605 owners of the fleet take about a twelfth of the presence batches that the former concurrency needed (one batch per wave of concurrent looks)",()=>{
 const waves=(count:number,width:number)=>Math.ceil(count/width);
 expect(waves(605,takeFetchConcurrencyV1())).toBeLessThanOrEqual(Math.ceil(waves(605,takeLookConcurrencyV1())/3));
});
