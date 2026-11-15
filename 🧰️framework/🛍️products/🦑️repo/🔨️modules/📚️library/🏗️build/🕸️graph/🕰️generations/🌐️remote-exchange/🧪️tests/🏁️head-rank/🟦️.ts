import {expect,test} from "bun:test";
import Ajv from "ajv";
import {wantedServerPolicyV1} from "../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import {generationHeadScoreV1,rankGenerationHeadsV1,type GenerationNoteV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 🏁️ Contract: the head a reader takes is the one that leaves it the fewest stale owners, judged by the state of the tree it describes and by what its producer knew stale, never by the time of the upload; the same ranking is checked against a plain loop and against Ajv. */
const policy=schema.definitions.Policy.const,hex=(seed:string)=>seed.repeat(64).slice(0,64),minute=60000,now=1_800_000_000_000;
const note=(over:Partial<GenerationNoteV1>={}):GenerationNoteV1=>({version:1,generation:10,owners:5000,identity:hex("a"),manifest:hex("b"),signature:hex("c"),platform:"darwin-arm64",publishedMs:now,origin:"g1-server",stateMs:now,implementation:hex("d"),stale:0,...over});

test("the score is what the producer knew stale plus what the tree moved under since, and all of it when the code differs",()=>{
 expect(generationHeadScoreV1(note({stale:300}),now)).toBe(300);
 expect(generationHeadScoreV1(note({stale:300,stateMs:now-10*minute}),now)).toBe(300+10*policy.staleOwnersPerMinute);
 expect(generationHeadScoreV1(note({stale:undefined}),now)).toBe(policy.unmeasuredStale);
 expect(generationHeadScoreV1(note({stale:9000}),now)).toBe(5000);
 expect(generationHeadScoreV1(note({stale:0}),now,{implementation:hex("e")})).toBe(5000);
 expect(generationHeadScoreV1(note({stale:0}),now,{implementation:hex("d")})).toBe(0);
 expect(generationHeadScoreV1(note({stale:40,stateMs:now-600*minute}),now,{changedPaths:100})).toBe(40+100*policy.ownersPerChangedPath);
 expect(generationHeadScoreV1(note({stale:0,stateMs:now+5*minute}),now)).toBe(0);
});

test("an old state uploaded later never outranks a newer state, and the stale count decides between states of different age",()=>{
 const server=note({origin:"g1-server",stateMs:now-2*minute,stale:30,publishedMs:now-2*minute}),worker=note({origin:"g1-m03",stateMs:now-600*minute,stale:0,publishedMs:now}),workerNear=note({origin:"g1-m04",stateMs:now-1*minute,stale:900,publishedMs:now});
 const heads=[{note:worker},{note:server}];
 expect(rankGenerationHeadsV1(heads,now)[0]!.note.origin).toBe("g1-server");
 expect(rankGenerationHeadsV1([{note:workerNear},{note:server}],now)[0]!.note.origin).toBe("g1-server");
 const closeAndComplete=note({origin:"g1-m05",stateMs:now-7*minute,stale:0});
 expect(rankGenerationHeadsV1([{note:server},{note:closeAndComplete}],now)[0]!.note.origin).toBe("g1-m05");
 const equal=[{note:note({origin:"g1-m02",stateMs:now-5})},{note:note({origin:"g1-m01",stateMs:now-5})},{note:note({origin:"g1-m09",stateMs:now})}];
 expect(rankGenerationHeadsV1(equal,now).map(head=>head.note.origin)).toEqual(["g1-m09","g1-m01","g1-m02"]);
});

test("measured distances replace the estimate by age, and the code of the reader counts",()=>{
 const near=note({origin:"g1-a",stateMs:now-900*minute,stale:10}),far=note({origin:"g1-b",stateMs:now-1*minute,stale:10}),heads=[{note:near},{note:far}];
 expect(rankGenerationHeadsV1(heads,now)[0]!.note.origin).toBe("g1-b");
 expect(rankGenerationHeadsV1(heads,now,{changed:new Map([[heads[0]!,3],[heads[1]!,4000]])})[0]!.note.origin).toBe("g1-a");
 const mine=note({origin:"g1-mine",implementation:hex("e"),stale:200}),theirs=note({origin:"g1-theirs",implementation:hex("d"),stale:0}),both=[{note:theirs},{note:mine}];
 expect(rankGenerationHeadsV1(both,now,{implementation:hex("e")})[0]!.note.origin).toBe("g1-mine");
 expect(rankGenerationHeadsV1(both,now,{implementation:hex("d")})[0]!.note.origin).toBe("g1-theirs");
});

test("the ranking equals a plain minimum loop on random heads, in either input order",()=>{
 let seed=7;const random=(n:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%n;};
 for(let round=0;round<300;round++){
  const heads=Array.from({length:1+random(6)},(_,at)=>({note:note({origin:"g1-m"+String(at).padStart(2,"0"),owners:1000+random(5000),stateMs:now-random(900)*minute,stale:random(5)===0?undefined:random(1200),implementation:hex(random(3)===0?"e":"d")})}));
  const reader={implementation:hex("d")};let best=heads[0]!;
  const score=(head:{note:GenerationNoteV1})=>generationHeadScoreV1(head.note,now,reader);
  for(const head of heads){const a=score(head),b=score(best);if(a<b||a===b&&(head.note.stateMs>best.note.stateMs||head.note.stateMs===best.note.stateMs&&head.note.origin<best.note.origin))best=head;}
  expect(rankGenerationHeadsV1(heads,now,reader)[0]).toBe(best);
  expect(rankGenerationHeadsV1([...heads].reverse(),now,reader)[0]).toBe(best);
 }
});

test("policy, note and the serving size are what the schemas and the graph server declare (checked with Ajv)",()=>{
 const ajv=new Ajv({strict:false}),valid=ajv.compile(schema.definitions.Note);
 expect(valid(note()),JSON.stringify(valid.errors)).toBe(true);
 expect(valid(note({stale:undefined})),JSON.stringify(valid.errors)).toBe(true);
 const bare:Record<string,unknown>={...note()};delete bare.stateMs;expect(valid(bare)).toBe(false);
 delete bare.origin;expect(valid({...note(),origin:"G1 Server"})).toBe(false);
 expect(valid({...note(),stale:-1})).toBe(false);expect(valid({...note(),implementation:"x"})).toBe(false);
 const numbers=ajv.compile({type:"object",required:["unmeasuredStale","staleOwnersPerMinute","ownersPerChangedPath","shadowMarginOwners","servingTotalMegabytes","rankCandidates"],properties:{unmeasuredStale:{type:"integer",minimum:1},staleOwnersPerMinute:{type:"number",minimum:0},ownersPerChangedPath:{type:"number",minimum:0},shadowMarginOwners:{type:"integer",minimum:0},servingTotalMegabytes:{type:"integer",minimum:1},rankCandidates:{type:"integer",minimum:2,maximum:8}}});
 expect(numbers(policy),JSON.stringify(numbers.errors)).toBe(true);
 expect(policy.servingTotalMegabytes).toBe(wantedServerPolicyV1.serveTotalMegabytes);
});
