/** 🎯️ Contract: a project whose target runs a script holds the command walk it reuses by its `closure-graph` row alone. The raw rows the walk read (the files, their analyses, the resolutions) are no row of any project: the module graph answers for them, so the edit of a tool file stales a project exactly when it moves the answer of the walk: an edit of a body, a comment or a rewritten specifier re-infers nothing, and a file that joins or leaves the roster re-infers exactly the projects that hold the row. */
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
const graphRows=(rows:any[])=>rows.filter((row:any)=>row.kind==="global"&&JSON.parse(row.path)[0]==="closure-graph");
const judged=(row:any)=>observation.observeInferenceSnapshotV1(async()=>{await observation.prepareGlobalFactsV1([row]);return observation.observationRowFreshV1(row) as boolean;});
const keysOf=(list:any)=>new Set<string>(Array.isArray(list)?list.map((row:any)=>rows.rowKeyV1(row)):list.keys());

test("the rows of a script-backed target name its walk by one closure-graph row and hold no row the walk read",async()=>{
 const root=workspace(),resolved=await resolveTarget(root);
 expect(resolved.derived).toEqual([]);
 const rows=graphRows(resolved.rows);
 expect(rows).toHaveLength(1);expect(JSON.parse(rows[0].path)[2].view).toBe("scheduling");
 expect(resolved.rows.filter((row:any)=>row.kind==="analysis"&&String(row.fact?.provider).startsWith("build:command-imports")),"no row the walk read of a module is a row of the project").toEqual([]);
},120000);

test("the row log path of the generation build holds the same rows",async()=>{
 const root=workspace(),directory=mkdtempSync(join(realpathSync(tmpdir()),"target-rows-log-"));roots.push(directory);
 const log=rowLog.createRowLogV1({path:join(directory,"rows.log")});
 try{
  const resolved=await resolveTarget(root,log),plain=await resolveTarget(root);
  expect(new Set([...keysOf(resolved.rows)])).toEqual(keysOf(plain.rows));
  expect(resolved.derived).toEqual([]);
 }finally{log.close();}
},120000);

test("an edit of a body or a comment of a walked file leaves every own row and the closure-graph row holding",async()=>{
 const root=workspace(),resolved=await resolveTarget(root);
 write(root,{"tools/m5.mjs":'import "./m6.mjs";\n// a comment\nexport const m5 = 55;\nexport function more(a){return a+1;}\n'});
 const others=resolved.rows.filter((row:any)=>!graphRows([row]).length);
 expect(await observation.observeInferenceSnapshotV1(async()=>observation.observationRowsHoldV1(others))).toBe(true);
 expect(await judged(graphRows(resolved.rows)[0])).toBe(true);
},120000);

test("a file that joins the walk fails the closure-graph row while a body edit does not",async()=>{
 const root=workspace(),resolved=await resolveTarget(root),row=graphRows(resolved.rows)[0];
 write(root,{"tools/m5.mjs":'import "./m6.mjs";\nexport const m5 = 6;\n'});
 expect(await judged(row)).toBe(true);
 write(root,{"tools/m5.mjs":'import "./m6.mjs";\nimport "./extra.mjs";\nexport const m5 = 5;\n'});
 expect(await judged(row)).toBe(false);
},120000);
