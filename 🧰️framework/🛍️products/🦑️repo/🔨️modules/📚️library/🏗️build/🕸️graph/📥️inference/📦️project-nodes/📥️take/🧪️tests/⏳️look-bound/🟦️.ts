/** ⏳️ Contract: one owner whose look never answers does not hold its lane or the pass: after `lookMaximumMilliseconds` the lane sets it aside as missing (the look goes on, the inference joins it) and takes the next owner, so 600 owners with one hanging look end in bounded time with 599 taken; the set-aside looks are bounded, and beyond the bound a lane waits for its look again. The outcomes equal an independent model in SQLite.
 * @see ../../🟨️.mjs `takeBoundedLookV1`, `takePassV1`, `takeFetchFirstV1` */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{TAKE_POLICY_V1,takeBoundedLookV1,takeFetchFirstV1,takePassV1}from"../../🟨️.mjs";

const names=Array.from({length:600},(_,at)=>"owner-"+at);
const model=(hanging:ReadonlySet<string>)=>{const db=new Database(":memory:");try{db.run("create table o(name text primary key)");db.run("create table h(name text)");const add=db.prepare("insert into o values (?)"),hang=db.prepare("insert into h values (?)");for(const name of names)add.run(name);for(const name of hanging)hang.run(name);return new Map((db.query("select o.name, case when h.name is null then 'present' else 'miss' end as outcome from o left join h on h.name=o.name").all() as {name:string;outcome:string}[]).map(row=>[row.name,row.outcome]));}finally{db.close();}};

test("600 owners with one look that never answers end within the bound, 599 present and the hanging one missing",async()=>{
 const hanging=new Set(["owner-7"]),abandoned={count:0},began=performance.now();
 const outcomes=await takePassV1({names,concurrency:8,lookMaximumMilliseconds:200,abandoned,attempt:name=>hanging.has(name)?new Promise<never>(()=>{}):Bun.sleep(1).then(()=>"present" as const)});
 expect(performance.now()-began).toBeLessThan(5000);
 expect<ReadonlyMap<string,string>>(new Map([...outcomes].sort())).toEqual(new Map([...model(hanging)].sort()));
 expect(abandoned.count).toBe(1);
});

test("a look that answers late but within the bound keeps its outcome, a set-aside look frees its count when it ends",async()=>{
 const state={count:0};
 expect(await takeBoundedLookV1(Bun.sleep(30).then(()=>"fetched" as const),200,state,4)).toBe("fetched");expect(state.count).toBe(0);
 let finish:(value:"present")=>void=()=>{};const slow=new Promise<"present">(done=>{finish=done;});
 expect(await takeBoundedLookV1(slow,20,state,4)).toBe("miss");expect(state.count).toBe(1);
 finish("present");await slow;await Bun.sleep(0);expect(state.count).toBe(0);
 const failing=Promise.reject(Error("refused"));failing.catch(()=>undefined);
 await expect(takeBoundedLookV1(failing,200,state,4)).rejects.toThrow("refused");
});

test("beyond the bound of set-aside looks a lane waits for its look again",async()=>{
 const state={count:2};
 expect(await takeBoundedLookV1(Bun.sleep(80).then(()=>"present" as const),10,state,2)).toBe("present");expect(state.count).toBe(2);
 expect(TAKE_POLICY_V1.abandonedLooksMaximum).toBeGreaterThan(0);expect(TAKE_POLICY_V1.lookMaximumMilliseconds).toBeGreaterThan(0);
});

test("the lanes of a fetch-first pass share the bound: hanging looks in both lanes do not hold the pass",async()=>{
 const hanging=new Set(["owner-3","owner-450"]),marked=new Map<string,string>(),began=performance.now();
 await takeFetchFirstV1({lanes:[names.slice(0,300),names.slice(300)],concurrency:8,fetchConcurrency:16,fetches:lane=>lane[0]==="owner-300",attempt:name=>hanging.has(name)?new Promise<never>(()=>{}):Bun.sleep(1).then(()=>"present" as const),hot:()=>undefined,queue:()=>[],arrived:()=>{},stop:()=>false,mark:(name,outcome)=>{marked.set(name,outcome);},policy:{...TAKE_POLICY_V1,lookMaximumMilliseconds:150}});
 expect(performance.now()-began).toBeLessThan(5000);
 expect(new Map([...marked].sort())).toEqual(new Map([...model(hanging)].sort()));
});
