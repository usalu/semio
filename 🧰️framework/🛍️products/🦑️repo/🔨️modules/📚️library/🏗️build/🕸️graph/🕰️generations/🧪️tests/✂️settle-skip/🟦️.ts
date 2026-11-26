/** ✂️ Contract: a derived question is asked again only while a project that is not rebuilt anyway holds it; when every holder is dirty already (its own stale facts, a changed source, a stale config) the answer cannot move the dirty set, so the read does not pay for it before the inference pays for it again. */
import{expect,test}from"bun:test";
import{createFactIndexV1}from"../../🟨️.mjs";
import{derivedOwnerNameV1}from"../../✂️cutoff/🟨️.mjs";
import{createGenerationPlanV1}from"../../🧭️plan/🟨️.mjs";

const file=(path:string)=>({kind:"file",path:"/ws/"+path,fact:{sha256:"0".repeat(64),size:1,mtimeMs:1,ctimeMs:1,ino:1}});
const answer=(name:string)=>({kind:"derived",path:"closure:"+name,fact:{answer:name}});
const owner=(name:string)=>({root:name,kind:"project",fragment:"f-"+name,bytes:1,edgesEnv:"e",targets:[]});

const setup=(holders:Record<string,string[]>,questions:string[])=>{
 const asked:string[][]=[];
 const rows=new Map<string,unknown[]>();
 for(const name of Object.keys(holders))rows.set(name,[file(name+"/own.ts"),...questions.filter(question=>holders[name].includes(question)).map(answer)]);
 for(const question of questions)rows.set(derivedOwnerNameV1(answer(question) as never),[answer(question),file(question+"/raw.ts")]);
 const index=createFactIndexV1(rows as never) as never;
 const manifest={identity:"i",global:"g",factIndex:"x",owners:Object.fromEntries(Object.keys(holders).map(name=>[name,owner(name)]))};
 const plan=createGenerationPlanV1({
  readIndexView:async()=>index,fragmentNames:async()=>["g.json.br",...Object.keys(holders).map(name=>"f-"+name+".json.br")],now:()=>1,
  settler:{settle:async(input:{pseudoRows:Map<string,unknown>})=>{asked.push([...input.pseudoRows.keys()].sort());return{changed:new Map(),held:new Map(),kept:new Set(),failed:0,reused:0};},release:async()=>undefined} as never,
  derive:(()=>undefined) as never,hold:(async(list:{path:string}[])=>!list.some(item=>item.path.endsWith("/raw.ts")||item.path.endsWith("/dirty.ts"))) as never,workspaceRoot:undefined,say:()=>undefined,
 });
 const run=(changes:{path:string;kind:string}[]=[],only?:Set<string>)=>plan({current:{manifest} as never,identity:"i",changes,only,verifyAll:true} as never);
 return{run,asked,name:(question:string)=>derivedOwnerNameV1(answer(question) as never)};
};

test("a question held only by projects that are dirty anyway is not asked again",async()=>{
 const state=setup({a:["q"],b:["q"]},["q"]);
 const plan=await state.run([{path:"a/x.ts",kind:"modified"},{path:"b/x.ts",kind:"modified"}]);
 expect(state.asked).toEqual([[]]);
 expect([...plan.dirty].sort()).toEqual(["a","b"]);
 expect(plan.cutoff?.questions).toBe(0);
 expect(plan.cutoff?.skipped).toBe(1);
},30_000);

test("a question that one clean project holds is still asked again",async()=>{
 const state=setup({a:["q"],b:["q"]},["q"]);
 const plan=await state.run([{path:"a/x.ts",kind:"modified"}]);
 expect(state.asked).toEqual([[state.name("q")]]);
 expect(plan.cutoff?.questions).toBe(1);
 expect(plan.cutoff?.skipped).toBe(0);
 expect([...plan.dirty]).toEqual(["a"]);
},30_000);

test("only the questions with a clean holder are asked, as the independent model names them",async()=>{
 const holders={a:["q1","q2"],b:["q2"],c:["q3"],d:["q4"]},questions=["q1","q2","q3","q4"],dirty=new Set(["a","c"]);
 const state=setup(holders,questions);
 await state.run([{path:"a/x.ts",kind:"modified"},{path:"c/x.ts",kind:"modified"}]);
 const model=questions.filter(question=>Object.entries(holders).some(([name,list])=>list.includes(question)&&!dirty.has(name)));
 expect(state.asked).toEqual([model.map(state.name).sort()]);
 expect(model).toEqual(["q2","q4"]);
},30_000);

test("a read scoped to other projects does not count a dirty holder outside its scope as rebuilt",async()=>{
 const state=setup({a:["q"],b:["q"]},["q"]);
 await state.run([{path:"a/x.ts",kind:"modified"}],new Set(["b"]));
 expect(state.asked).toEqual([[state.name("q")]]);
},30_000);
