/** 🛟️ Contract: a withheld owner of a sealed generation whose failure came from a transient cause is inferred again by a new read after the declared cool-down, at most the declared attempts, while a withholding that does not change is otherwise kept until one of its rows changes. */
import{expect,test}from"bun:test";
import{createFactIndexV1}from"../../🟨️.mjs";
import{createGenerationPlanV1}from"../../🧭️plan/🟨️.mjs";
import policy from"../../../📥️load/🛟️owner-isolation/🧬️schema/🔣️.json";

const limits=policy.$defs.Policy.const;
const row=(path:string)=>({kind:"file",path:"/ws/"+path,fact:{sha256:"0".repeat(64),size:1,mtimeMs:1,ctimeMs:1,ino:1}});
const setup=(reason?:()=>unknown)=>{
 let book:unknown,clock=5_000_000;const lines:string[]=[];
 const index=createFactIndexV1(new Map([["held",[row("h/c.ts")]],["plain",[row("p/c.ts")]]]) as never) as never;
 const manifest={identity:"i",global:"g",factIndex:"x",owners:{held:{root:"h",kind:"unavailable",config:"h/c.ts",fragment:"f1",bytes:1,edgesEnv:"e",targets:[]},plain:{root:"p",kind:"project",fragment:"f2",bytes:1,edgesEnv:"e",targets:[]}}};
 const plan=createGenerationPlanV1({
  readIndexView:async()=>index,fragmentNames:async()=>["g.json.br","f1.json.br","f2.json.br"],now:()=>clock,
  settler:{settle:async()=>({changed:new Map(),held:new Set(),kept:new Set(),failed:0,reused:0}),release:async()=>undefined} as never,
  derive:(()=>undefined) as never,hold:(async()=>true) as never,workspaceRoot:undefined,
  ledger:{read:async()=>book,write:async(next:unknown)=>{book=next;}},...reason?{reasonOf:async()=>reason()}:{},say:text=>lines.push(text.en+" / "+text.de),
 });
 const run=(only?:Set<string>,implementation?:string)=>plan({current:{manifest} as never,identity:"i",changes:[],only,verifyAll:true,implementation} as never);
 return{run,advance:(milliseconds:number)=>{clock+=milliseconds;},lines,manifest};
};

test("a withheld owner is inferred again once per cool-down, at most the declared attempts, then stays",async()=>{
 const state=setup();
 const first=await state.run();
 expect(first.kind).toBe("clean");
 expect(first.revalidated).toEqual([]);
 state.advance(limits.revalidateCooldownMilliseconds-1);
 expect((await state.run()).revalidated).toEqual([]);
 state.advance(1);
 const second=await state.run();
 expect(second.kind).toBe("delta");
 expect(second.revalidated).toEqual(["held"]);
 expect([...second.dirty]).toEqual(["held"]);
 expect(state.lines.length).toBe(1);
 expect(state.lines[0]).toContain("held");
 for(let attempt=2;attempt<=limits.revalidateAttempts;attempt++){state.advance(limits.revalidateCooldownMilliseconds);expect((await state.run()).revalidated).toEqual(["held"]);}
 state.advance(10*limits.revalidateCooldownMilliseconds);
 const spent=await state.run();
 expect(spent.revalidated).toEqual([]);
 expect(spent.kind).toBe("clean");
},30_000);

test("a read that selects other owners neither infers nor counts an unrelated withheld owner",async()=>{
 const state=setup();
 await state.run();
 state.advance(limits.revalidateCooldownMilliseconds);
 const scoped=await state.run(new Set(["plain"]));
 expect(scoped.revalidated).toEqual([]);
 expect([...scoped.dirty]).toEqual([]);
 expect((await state.run(new Set(["held"]))).revalidated).toEqual(["held"]);
},30_000);

test("an exhausted withholding is inferred again when the inference implementation changed, and while its recorded reason is transient",async()=>{
 const state=setup(()=>"Owner held failed: JavaScript heap out of memory / Speicher erschöpft");
 await state.run(undefined,"i1");
 for(let attempt=1;attempt<=limits.revalidateAttempts;attempt++){state.advance(limits.revalidateCooldownMilliseconds);expect((await state.run(undefined,"i1")).revalidated).toEqual(["held"]);}
 state.advance(limits.revalidateCooldownMilliseconds-1);
 expect((await state.run(undefined,"i1")).revalidated).toEqual([]);
 state.advance(1);
 expect((await state.run(undefined,"i1")).revalidated).toEqual(["held"]);
 expect((await state.run(undefined,"i2")).revalidated).toEqual(["held"]);
 const fixed=setup(()=>"Cannot find module './x.ts'");
 await fixed.run(undefined,"i1");
 for(let attempt=1;attempt<=limits.revalidateAttempts;attempt++){fixed.advance(limits.revalidateCooldownMilliseconds);await fixed.run(undefined,"i1");}
 fixed.advance(10*limits.revalidateCooldownMilliseconds);
 expect((await fixed.run(undefined,"i1")).revalidated).toEqual([]);
 expect((await fixed.run(undefined,"i2")).revalidated).toEqual(["held"]);
},30_000);
