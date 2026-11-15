/** 🎯️ Contract: a project whose target runs a script holds the command walk it reuses by its derived fact alone. The raw rows the walk read (the files, their analyses, the resolutions) are the support of that fact and belong to no project, so the edit of a tool file stales a project exactly when it moves the answer of the walk: an edit of a body, a comment or a rewritten specifier re-infers nothing, and a file that joins or leaves the roster re-infers exactly the projects that hold the fact. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import fixture from"../🔣️.json";

const inference=await import("../../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../../🧭️script-inputs/🟨️.mjs") as any,rowLog=await import("../../../../🕰️generations/🗃️row-log/🟨️.mjs") as any,rows=await import("../../../../🕰️generations/🧮️fact-rows/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string|undefined>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const target={options:{cwd:".",command:"repo ./tools/📜️script.ts run"}};
const workspace=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"target-rows-"));roots.push(root);write(root,{...fixture.files,"tools/📜️script.ts":'import "./entry.mjs";\nexport {};\n'});return root;};
const resolveTarget=(root:string,log?:any)=>observation.observeInferenceSnapshotV1(async()=>{
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.genericTargetCommandInputs(target,"."),key=scriptInputs.closureHandleOfV1(list).key;
 return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]],log);
});
const keysOf=(list:any)=>new Set<string>(Array.isArray(list)?list.map((row:any)=>rows.rowKeyV1(row)):list.keys());

test("the rows of a script-backed target and the support of its walk fact share no row",async()=>{
 const root=workspace(),resolved=await resolveTarget(root);
 expect(resolved.derived).toHaveLength(1);
 const support=keysOf(resolved.derived[0].support),own=[...keysOf(resolved.rows)];
 expect(support.size).toBeGreaterThan(64);
 expect(own.filter(key=>support.has(key))).toEqual([]);
 expect(resolved.rows.length).toBeGreaterThan(0);
},120000);

test("the row log path of the generation build excludes the support the same way",async()=>{
 const root=workspace(),directory=mkdtempSync(join(realpathSync(tmpdir()),"target-rows-log-"));roots.push(directory);
 const log=rowLog.createRowLogV1({path:join(directory,"rows.log")});
 try{
  const resolved=await resolveTarget(root,log),plain=await resolveTarget(root);
  const support=keysOf(resolved.derived[0].support),own=[...keysOf(resolved.rows)];
  expect(own.filter(key=>support.has(key))).toEqual([]);
  expect(new Set(own)).toEqual(keysOf(plain.rows));
 }finally{log.close();}
},120000);

test("an edit of a body or a comment of a walked file leaves every own row and the derived fact unchanged",async()=>{
 const root=workspace(),resolved=await resolveTarget(root),baseline=resolved.derived[0].row.fact as string;
 write(root,{"tools/m5.mjs":'import "./m6.mjs";\n// a comment\nexport const m5 = 55;\nexport function more(a){return a+1;}\n'});
 const outcome=await observation.observeInferenceSnapshotV1(async()=>({
  holds:observation.observationRowsHoldV1(resolved.rows),
  fact:(await scriptInputs.deriveClosureFactV1(resolved.derived[0].row,createScriptInputCache({workspaceRoot:root}))).fact,
 }));
 expect(outcome.fact).toBe(baseline);
 expect(outcome.holds).toBe(true);
},120000);

test("a file that joins the walk moves the derived fact while a body edit does not",async()=>{
 const root=workspace(),resolved=await resolveTarget(root),baseline=resolved.derived[0].row.fact as string;
 const again=()=>observation.observeInferenceSnapshotV1(async()=>(await scriptInputs.deriveClosureFactV1(resolved.derived[0].row,createScriptInputCache({workspaceRoot:root}))).fact);
 write(root,{"tools/m5.mjs":'import "./m6.mjs";\nexport const m5 = 6;\n'});
 expect(await again()).toBe(baseline);
 write(root,{"tools/m5.mjs":'import "./m6.mjs";\nimport "./extra.mjs";\nexport const m5 = 5;\n'});
 expect(await again()).not.toBe(baseline);
},120000);
