/** 👓 Contract: a project holds a command walk through the view of the closure recipe it reads it with. A target's own closure (target-sources, generic-target) drops every scheduling file, a Rust command's closure (native-command, native-target) keeps the native-build modules; so the `closure-graph` row such a project holds (the module graph's closure digest and the answer through the view) keeps holding after an edit that only moves files its view leaves out (a peer edit of scheduling code leaves product owners verified), and changes as soon as its view's roster or compiled modules move (an edit that changes a command's inputs still re-infers). A recipe the policy does not name (closure-of, the test discovery) keeps holding the whole walk. The real walk, the real `resolve` and the real judge of the row run under the real observation layer on a tiny workspace; an independent Python model (reading the scheduling table itself) states every verdict and roster, and esbuild's bundle graph states the whole roster. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️.json" with {type:"json"};
import policySchema from "../🧬️schema/🔣️.json" with {type:"json"};
import table from "../../../../../📥️source/🔏️closure/🗓️scheduling/🔣️.json" with {type:"json"};

setDefaultTimeout(120000);
type Verdict={whole:boolean;scheduling:boolean;native:boolean};
const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🟨️.mjs") as any,views=await import("../🟨️.mjs") as any;
const python=requireTestToolV1("python"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const oracle=():{rosters:string[][];verdicts:Verdict[]}=>{const run=spawnSync(python,["-I",join(import.meta.dir,"🐍️oracle","🐍️.py"),join(import.meta.dir,"🔣️.json"),join(import.meta.dir,"../../../../../📥️source/🔏️closure/🗓️scheduling/🔣️.json")],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const esbuildRoster=async(root:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,corpus.entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};
const target={options:{cwd:"proj",command:"repo ./📜️script.ts build"}};
const resolveRecipes=(root:string)=>observation.observeInferenceSnapshotV1(async()=>{
 const scripts=scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
 const own=scriptInputs.closureHandleOfV1(await api.genericTargetCommandInputs(target,"proj")),whole=scriptInputs.closureHandleOfV1(await api.closureOf([join(root,corpus.entry)],[],"test-discovery"));
 return{own:await api.resolve([[own.key,own.recipe]]),whole:await api.resolve([[whole.key,whole.recipe]])};
});
const walks=(resolved:any)=>resolved.rows.filter((row:any)=>row.kind==="global"&&JSON.parse(row.path)[0]==="closure-graph").map((row:any)=>({row}));
const viewed=(row:any,view:string|null)=>{const[name,root,segments]=JSON.parse(row.path);return{kind:"global",path:JSON.stringify([name,root,{...segments,view}]),fact:row.fact};};
const judged=(row:any)=>observation.observeInferenceSnapshotV1(async()=>{await observation.prepareGlobalFactsV1([row]);return observation.observationRowFreshV1(row) as boolean;});

test("the view policy is closed under its schema, the native view leaves out exactly the scheduling prefixes no native-keyed module lies in, and an unknown view is refused",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate(policySchema,policy)).toBe(true);expect(validateJsonSchemaSubset(policySchema,policy)).toEqual([]);
 expect(validateJsonSchemaSubset(policySchema,{...policy,recipes:{"target-sources":"everything"}}).length).toBeGreaterThan(0);
 const scheduling=[...new Set(table.domains.flatMap(domain=>domain.prefixes))].sort(),keyed=table.domains.flatMap(domain=>(domain as {nativeKeyed?:string[]}).nativeKeyed??[]);
 expect([...views.walkViewPrefixesV1("scheduling")]).toEqual(scheduling);
 expect([...views.walkViewPrefixesV1("native")]).toEqual(scheduling.filter(prefix=>!keyed.some(row=>row.startsWith(prefix)||prefix.startsWith(row))));
 expect(views.walkViewOfRecipeV1({fn:"target-sources"})).toBe("scheduling");expect(views.walkViewOfRecipeV1({fn:"native-command"})).toBe("native");expect(views.walkViewOfRecipeV1({fn:"closure-of",role:"test-discovery"})).toBeUndefined();
 expect(views.walkViewOfRecipeV1({fn:"closure-of",role:"artifact-command"})).toBe("scheduling");expect(views.walkViewOfRecipeV1({fn:"closure-of",role:"artifact-sources"})).toBeUndefined();expect(views.walkViewOfRecipeV1({fn:"script-provider",view:"native"})).toBe("native");expect(views.walkViewOfRecipeV1({fn:"tool-closure",name:"cargo-command",view:"native"})).toBe("native");
 expect(()=>views.walkViewOfRecipeV1({fn:"closure-of"})).toThrow();expect(()=>views.walkViewOfRecipeV1({fn:"closure-of",role:"cargo-lock-owner"})).toThrow();expect(()=>views.walkViewOfRecipeV1({fn:"script-provider",view:"everything"})).toThrow();
 expect(()=>views.walkViewPrefixesV1("everything")).toThrow();
 expect(views.walkViewAnswerV1(["{workspaceRoot}/a.ts"],["{workspaceRoot}/a.ts"],[],undefined)).not.toBe(views.walkViewAnswerV1(["{workspaceRoot}/a.ts"],["{workspaceRoot}/a.ts"],[],"scheduling"));
});

test("the corpus is closed under its schema and every authored verdict is the independent model's",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate(corpusSchema,corpus)).toBe(true);
 expect(oracle().verdicts).toEqual(corpus.mutations.map(mutation=>mutation.changed));
});

test("a target's own closure holds the walk through the scheduling view and the whole walk stays with closure-of; each held row follows exactly its view's roster across the edits",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"walk-views-")));roots.push(root);write(root,corpus.files);
 const first=await resolveRecipes(root),own=walks(first.own),whole=walks(first.whole);
 expect(own).toHaveLength(1);expect(whole).toHaveLength(1);
 expect(JSON.parse(own[0].row.path)[2].view).toBe("scheduling");expect(JSON.parse(whole[0].row.path)[2].view).toBeNull();
 const native=await observation.observeInferenceSnapshotV1(async()=>{const scripts=scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:true});scriptInputs.closureApiV1(scripts,root);const[,,segments]=JSON.parse(own[0].row.path);return scriptInputs.closureGraphAnswerV1(scripts,root,{roots:segments.roots.map((name:string)=>join(root,name.slice(16))),requiredSources:segments.requiredSources,resources:[],selectors:segments.selectors.map(([name,words]:[string,string[]])=>[join(root,name.slice(16)),words]),stops:segments.stops.map(([name,tool]:[string,string])=>[join(root,name.slice(16)),tool]),view:"native"});});
 const held={whole:whole[0].row,scheduling:own[0].row,native:{...viewed(own[0].row,"native"),fact:JSON.stringify(native)}} as Record<keyof Verdict,any>;
 for(const row of Object.values(held))expect(await judged(row)).toBe(true);
 const answers=Object.fromEntries(Object.entries(held).map(([name,row])=>[name,JSON.parse(row.fact).answer])) as Record<keyof Verdict,string>;
 expect(answers.scheduling).not.toBe(answers.whole);
 const model=oracle();
 for(const[index,mutation]of corpus.mutations.entries()){
  write(root,mutation.edit);
  const verdict={} as Verdict;
  for(const name of ["whole","scheduling","native"] as const)verdict[name]=!await judged(held[name]);
  expect(verdict,mutation.name).toEqual(mutation.changed);
  const fresh=await resolveRecipes(root),freshOwn=JSON.parse(walks(fresh.own)[0].row.fact),freshWhole=JSON.parse(walks(fresh.whole)[0].row.fact);
  expect(freshOwn.answer===answers.scheduling,mutation.name+": a held row holds exactly when a fresh inference answers the same through its view").toBe(!verdict.scheduling);
  expect(freshWhole.answer===answers.whole,mutation.name).toBe(!verdict.whole);
  held.whole=walks(fresh.whole)[0].row;held.scheduling=walks(fresh.own)[0].row;answers.whole=freshWhole.answer;answers.scheduling=freshOwn.answer;
  if(verdict.native){const again=await observation.observeInferenceSnapshotV1(async()=>{const scripts=scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:true});scriptInputs.closureApiV1(scripts,root);const[,,segments]=JSON.parse(held.scheduling.path);return scriptInputs.closureGraphAnswerV1(scripts,root,{roots:segments.roots.map((name:string)=>join(root,name.slice(16))),requiredSources:segments.requiredSources,resources:[],selectors:segments.selectors.map(([name,words]:[string,string[]])=>[join(root,name.slice(16)),words]),stops:segments.stops.map(([name,tool]:[string,string])=>[join(root,name.slice(16)),tool]),view:"native"});});held.native={...viewed(held.scheduling,"native"),fact:JSON.stringify(again)};}
  const roster=(fresh.whole.values.values().next().value as string[]).filter(path=>typeof path==="string").map(path=>path.replace("{workspaceRoot}/","")).sort();
  expect(roster,mutation.name).toEqual(model.rosters[index]!);expect(roster,mutation.name).toEqual(await esbuildRoster(root));
 }
});

test("an unrelated edit leaves the held answers valid: files the remote cache gains or loses in a command roster move neither the scheduling nor the native answer, a keyed file moves both, and the answers equal an independent digest",()=>{
 const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",cache=library+"🏗️build/🌐️remote-cache/",checkout=join(import.meta.dir,"../../../../../../../../../../../"),name=(path:string)=>"{workspaceRoot}/"+path;
 const domain=table.domains.find(row=>row.id==="remote-cache")!;
 expect(domain.prefixes).toEqual([cache]);
 for(const port of domain.ports){expect(port.startsWith(cache),port).toBe(true);expect(statSync(join(checkout,port)).isFile(),port).toBe(true);}
 const bootstrap=name(library+"⚡️caching/🚀️bootstrap/📜️script.ts"),route=name(cache+"🧶️nx/🟦️.ts"),added=name(cache+"⌨️command/🟦️.ts"),keyed=name(library+"🏗️build/📥️source/🔏️closure/🟦️.ts");
 const answer=(roster:string[],view?:string)=>views.walkViewAnswerV1(roster,roster,[],view);
 const independent=(roster:string[],view:string)=>{const prefixes=view==="scheduling"?[...new Set(table.domains.flatMap(row=>row.prefixes))]:[...new Set(table.domains.flatMap(row=>row.prefixes))].filter(prefix=>!table.domains.flatMap(row=>(row as {nativeKeyed?:string[]}).nativeKeyed??[]).some(row=>row.startsWith(prefix)||prefix.startsWith(row))),kept=roster.filter(path=>!prefixes.some(prefix=>path.slice("{workspaceRoot}/".length).startsWith(prefix)));return createHash("sha256").update(JSON.stringify([view,kept,kept,[],[]])).digest("hex");};
 const before=[bootstrap,route,keyed],grown=[bootstrap,route,added,keyed],shrunk=[bootstrap,keyed],moved=[bootstrap,route,keyed,name(library+"🏗️build/🕸️graph/📜️contract/🟦️.ts")];
 for(const view of ["scheduling","native"]){
  expect(views.walkViewPrefixesV1(view)).toContain(cache);
  expect(answer(grown,view),view+": a module the remote cache gains").toBe(answer(before,view));
  expect(answer(shrunk,view),view+": a module the remote cache loses").toBe(answer(before,view));
  expect(answer(moved,view),view+": a keyed module").not.toBe(answer(before,view));
  expect(answer(before,view)).toBe(independent(before,view));expect(answer(moved,view)).toBe(independent(moved,view));
 }
 expect(answer(grown),"the whole walk still names every file").not.toBe(answer(before));
});

test("a closure-graph row naming a view the policy does not carry never holds",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"walk-views-refuse-")));roots.push(root);write(root,corpus.files);
 const whole=walks((await resolveRecipes(root)).whole)[0];
 expect(await judged(whole.row)).toBe(true);
 expect(await judged(viewed(whole.row,"everything"))).toBe(false);
});
