/** 🧮️ Contract: a pool starts children one by one while the machine is loaded and does not drain a child before its start has paid; under memory pressure it still sheds at once. Oracle: the situations measured on g1-server (14 children of 20 s to 5 minutes at 3 to 6 percent cpu, target 0). */
import{expect,test}from"bun:test";
import{OWNER_POOL_POLICY_V1 as P}from"../../🟦️.ts";
import{ownerPoolDrainPickV1,ownerPoolMayStartV1,ownerPoolStartDelayV1}from"../../🧮️lifecycle/🟦️.ts";

const poll=2000,child=(over:Record<string,unknown>={})=>({startedMs:0,ready:true,running:0,drained:false,...over});

test("no child starts within the spacing of the last start or while the allowed number of children is still loading",()=>{
 const idleBut=[child({ready:true,running:0,startedMs:15000}),child({ready:true,running:1,startedMs:15000})];
 expect(ownerPoolMayStartV1({children:[],survivors:0,target:9,lastStartMs:-Infinity,now:1000,pollMilliseconds:poll})).toBe(true);
 expect(ownerPoolMayStartV1({children:idleBut.slice(0,1),survivors:0,target:9,lastStartMs:0,now:20000,pollMilliseconds:poll}),"an idle young child blocks a start").toBe(false);
 expect(ownerPoolMayStartV1({children:idleBut.slice(1),survivors:0,target:1,lastStartMs:0,now:20000,pollMilliseconds:poll}),"live reaches the target").toBe(false);
 expect(ownerPoolMayStartV1({children:[],survivors:2,target:2,lastStartMs:0,now:20000,pollMilliseconds:poll}),"survivors count").toBe(false);
 expect(ownerPoolMayStartV1({children:[],survivors:0,target:9,lastStartMs:1000,now:1000+P.startSpacingPolls*poll-1,pollMilliseconds:poll})).toBe(false);
 expect(ownerPoolMayStartV1({children:[],survivors:0,target:9,lastStartMs:1000,now:1000+P.startSpacingPolls*poll,pollMilliseconds:poll})).toBe(true);
 const loading=Array.from({length:P.startingMaximum},()=>child({ready:false,startedMs:10000}));
 expect(ownerPoolMayStartV1({children:loading,survivors:0,target:9,lastStartMs:0,now:20000,pollMilliseconds:poll})).toBe(false);
 expect(ownerPoolMayStartV1({children:[...loading.slice(1),child({running:1})],survivors:0,target:9,lastStartMs:0,now:20000,pollMilliseconds:poll})).toBe(true);
 expect(ownerPoolMayStartV1({children:loading,survivors:0,target:9,lastStartMs:0,now:10000+P.startWarmupPolls*poll,pollMilliseconds:poll}),"a child that never says ready stops blocking").toBe(true);
 expect(ownerPoolMayStartV1({children:[child({ready:false,drained:true,startedMs:10000}),...loading.slice(1)],survivors:0,target:9,lastStartMs:0,now:20000,pollMilliseconds:poll})).toBe(true);
});

test("a young child is not drained unless memory is short; an old one is, one without an owner in flight first, the newest of those first",()=>{
 const now=P.childMinimumAgePolls*poll*2,young=child({startedMs:now-1000}),old=child({startedMs:now-P.childMinimumAgePolls*poll}),older=child({startedMs:now-P.childMinimumAgePolls*poll-5000}),busy=child({startedMs:now-P.childMinimumAgePolls*poll-9000,running:1});
 expect(ownerPoolDrainPickV1({children:[young],reason:"yielding",now,pollMilliseconds:poll})).toBeUndefined();
 expect(ownerPoolDrainPickV1({children:[young],reason:null,now,pollMilliseconds:poll})).toBeUndefined();
 expect(ownerPoolDrainPickV1({children:[young],reason:"pressure",now,pollMilliseconds:poll})).toBe(young);
 expect(ownerPoolDrainPickV1({children:[busy,older,old,young],reason:"yielding",now,pollMilliseconds:poll})).toBe(old);
 expect(ownerPoolDrainPickV1({children:[busy,child({startedMs:now-P.childMinimumAgePolls*poll-9000,drained:true})],reason:"yielding",now,pollMilliseconds:poll})).toBe(busy);
 expect(ownerPoolDrainPickV1({children:[],reason:"pressure",now,pollMilliseconds:poll})).toBeUndefined();
});

test("live children, draining ones and survivors included, never pass the target, not even when every start fails or the target falls to zero",()=>{
 type Live={startedMs:number;ready:boolean;running:number;drained:boolean;endsAt:number};
 let children:Live[]=[],last=-Infinity,most=0,starts=0,seed=7;
 const random=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 for(let tick=0;tick<600;tick++){
  const now=tick*poll,survivors=tick%100<20?2:0,target=tick<300?(tick%50<25?6:3):0;
  children=children.filter(row=>row.endsAt>now);
  const mayStart=ownerPoolMayStartV1({children,survivors,target,lastStartMs:last,now,pollMilliseconds:poll});
  if(mayStart){starts++;last=now;children.push({startedMs:now,ready:false,running:0,drained:false,endsAt:now+(random()<0.7?poll:poll*20*(1+Math.floor(random()*5)))});}
  for(const row of children)if(!row.ready&&now-row.startedMs>=poll*2)row.ready=true;
  const have=children.filter(row=>!row.drained);
  if(have.length+survivors>target){const pick=ownerPoolDrainPickV1({children:have,reason:"pressure",now,pollMilliseconds:poll});if(pick!==undefined){pick.drained=true;pick.endsAt=Math.min(pick.endsAt,now+poll*15);}}
  if(mayStart)expect(children.length+survivors,"a start never lifts the live children past the target").toBeLessThanOrEqual(Math.max(target,1));
  most=Math.max(most,children.length);
  if(target===0)expect(mayStart,"nothing starts at target zero").toBe(false);
 }
 expect(starts).toBeGreaterThan(5);expect(most).toBeLessThanOrEqual(6);
});

test("without the ramp only the target bounds the starts (rehearsals and tests that fill a pool at once)",()=>{
 const idle=[child({running:0,startedMs:0,ready:false})];
 expect(ownerPoolMayStartV1({children:idle,survivors:0,target:3,lastStartMs:0,now:10,pollMilliseconds:poll,ramp:false})).toBe(true);
 expect(ownerPoolMayStartV1({children:idle,survivors:0,target:1,lastStartMs:0,now:10,pollMilliseconds:poll,ramp:false})).toBe(false);
});

test("the delay to the next start is the number that holds it: the spacing, the age of an idle child, the warm-up of a loading child, and zero exactly when a start is allowed",()=>{
 const at=(children:ReturnType<typeof child>[],lastStartMs:number,now:number)=>ownerPoolStartDelayV1({children,lastStartMs,now,pollMilliseconds:poll});
 expect(at([],-Infinity,1000)).toBe(0);
 expect(at([],1000,1000+3000)).toBe(P.startSpacingPolls*poll-3000);
 expect(at([child({startedMs:5000})],-Infinity,10000)).toBe(P.childMinimumAgePolls*poll-5000);
 const loading=Array.from({length:P.startingMaximum},(_all,index)=>child({ready:false,startedMs:10000-index*1000}));
 expect(at(loading,-Infinity,20000)).toBe(Math.min(...loading.map(row=>P.startWarmupPolls*poll-(20000-row.startedMs))));
 for(const now of[0,999,20000,100000])for(const children of[[],[child({startedMs:now-4000})],loading])for(const lastStartMs of[-Infinity,now-1000,now-P.startSpacingPolls*poll]){
  const allowed=ownerPoolMayStartV1({children,survivors:0,target:99,lastStartMs,now,pollMilliseconds:poll});
  expect(at(children,lastStartMs,now)===0,JSON.stringify({now,children,lastStartMs})).toBe(allowed);
 }
});
