/** ✂️ Contract: a project that reuses a command walk names it by one `closure-graph` row (the question the walk answered with the closure digest of the module graph and the answer through its view) instead of by the rows it read. Judging the row again against the live files holds while the roster, the compiled modules and the generated outputs are the same, whatever the walk's files changed in between (comments, imports of files the walk already reaches, rewritten specifiers), and fails as soon as one of them moves or a reached file no longer parses; the rosters equal esbuild's independent bundle graph. A TypeScript program and a repository walk keep their derived facts. */
import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import fixture from"./🔣️.json";

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
const graphRows=(rows:any[])=>rows.filter((row:any)=>row.kind==="global"&&JSON.parse(row.path)[0]==="closure-graph");
const judged=(row:any)=>observation.observeInferenceSnapshotV1(async()=>{await observation.prepareGlobalFactsV1([row]);return observation.observationRowFreshV1(row) as boolean;});
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};

test("a reused walk is named by one closure-graph row and judging it again follows the live files",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-walk-"));roots.push(root);write(root,fixture.files);
 const entry=join(root,fixture.entry);
 const first=await observation.observeInferenceSnapshotV1(async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.closureOf([entry],[],"test-discovery"),key=scriptInputs.closureHandleOfV1(list).key;
  return{resolved:await api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]),key};
 });
 expect(first.resolved.derived.filter((fact:any)=>JSON.parse(fact.row.path).walk===1)).toEqual([]);
 const[row,...more]=graphRows(first.resolved.rows);
 expect(more).toEqual([]);
 const fact=JSON.parse(row.fact),segments=JSON.parse(row.path)[2];
 expect(fact.digest).toMatch(/^[a-f0-9]{64}$/u);expect(fact.answer).toMatch(/^[a-f0-9]{64}$/u);
 expect(segments.roots).toEqual(["{workspaceRoot}/"+fixture.entry]);expect(segments.view).toBeNull();
 expect(first.resolved.rows.filter((item:any)=>item.kind==="analysis").length,"no module of the walk is a row of the project").toBe(0);
 for(const step of fixture.steps){
  write(root,step.edit);
  const holds=await judged(row);
  if(step.closureSize===null){expect(holds,step.name).toBe(false);continue;}
  expect(holds,step.name).toBe(step.same);
  const fresh=await observation.observeInferenceSnapshotV1(async()=>{const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:false});return(await scriptInputs.relativeScriptInputs([entry],root,scripts)) as string[];});
  const roster=fresh.map((path:string)=>path.replace("{workspaceRoot}/","")).sort();
  expect(roster.length,step.name).toBe(step.closureSize);
  expect(roster,step.name).toEqual(await esbuildClosure(root,fixture.entry));
 }
},120000);

test("a caller that is no project (the test discovery) holds the same closure-graph row for the same walk, with the same roster, and the project path shares its entry",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-walk-caller-"));roots.push(root);write(root,fixture.files);
 const entry=join(root,fixture.entry);
 const walk=await observation.observeInferenceSnapshotV1(()=>scriptInputs.resolveCommandWalkV1(root,[entry],createScriptInputCache({workspaceRoot:root,lateClosures:true})));
 const project=await observation.observeInferenceSnapshotV1(async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.closureOf([entry],[],"test-discovery"),key=scriptInputs.closureHandleOfV1(list).key;
  return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);
 });
 expect(graphRows(walk.rows)).toHaveLength(1);expect(graphRows(walk.rows)[0]).toEqual(graphRows(project.rows)[0]);expect(walk.derived).toEqual([]);
 const direct=await observation.observeInferenceSnapshotV1(async()=>(await scriptInputs.relativeScriptInputs([entry],root,createScriptInputCache({workspaceRoot:root}))) as string[]);
 expect(walk.value).toEqual(direct);
 expect(()=>scriptInputs.resolveCommandWalkV1(root,[entry],createScriptInputCache({workspaceRoot:root}))).toThrow();
 const row=graphRows(walk.rows)[0];
 for(const step of fixture.steps){
  write(root,step.edit);
  const holds=await judged(row);
  if(step.closureSize===null){expect(holds,step.name).toBe(false);continue;}
  expect(holds,step.name).toBe(step.same);
 }
},120000);

test("a derived walk question is not answered (a command walk is a closure-graph row), and a closure-graph row naming paths outside the checkout or an unknown view never holds",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-walk-refuse-"));roots.push(root);write(root,fixture.files);
 const cache=createScriptInputCache({workspaceRoot:root});
 await expect(scriptInputs.deriveClosureFactV1({kind:"derived",path:JSON.stringify({walk:2}),fact:"x"},cache)).rejects.toBeDefined();
 await expect(scriptInputs.deriveClosureFactV1({kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:[],requiredSources:false,resources:[],selectors:[],provider:null}),fact:"x"},cache)).rejects.toBeDefined();
 const segments={roots:["{workspaceRoot}/"+fixture.entry],requiredSources:false,resources:[],selectors:[],stops:[],view:null},row=(changed:object)=>({kind:"global",path:JSON.stringify(["closure-graph",root,{...segments,...changed}]),fact:JSON.stringify({digest:"0".repeat(64),answer:"0".repeat(64)})});
 for(const changed of[{roots:["{workspaceRoot}/../outside.ts"]},{view:"everything"},{stops:[["{workspaceRoot}/a.ts",3]]}])expect(await judged(row(changed))).toBe(false);
});

test("a TypeScript program reused by a project is named by one derived fact too, and asking it again follows the files the compiler reads",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-program-"));roots.push(root);write(root,fixture.program.files);
 const target={options:{cwd:"proj",command:"repo ./📜️script.ts check proj"},metadata:{semio:{closure:{typescript:["{projectRoot}/tsconfig.json"]}}}};
 const first=await observation.observeInferenceSnapshotV1(async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.genericTargetCommandInputs(target,fixture.program.projectRoot),key=scriptInputs.closureHandleOfV1(list).key;
  return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);
 });
 const programs=first.derived.filter((fact:any)=>JSON.parse(fact.row.path).program===1);
 expect(programs).toHaveLength(1);
 const derived=programs[0];
 expect(derived.support.length).toBeGreaterThan(64);expect(derived.row.fact).toMatch(/^[a-f0-9]{64}$/u);
 expect(first.rows.filter((row:any)=>derived.support.includes(row)).map((row:any)=>[row.kind,row.path]),"the command walk reads its configuration through the module graph, so no own row of the record is a row of the program").toEqual([]);
 expect(first.rows.filter((row:any)=>row.kind==="global"&&JSON.parse(row.path)[0]==="closure-graph").length).toBeGreaterThan(0);
 const baseline=derived.row.fact as string,independent=()=>{const ts=createRequire(import.meta.url)("typescript"),config=ts.readConfigFile(join(root,"proj/tsconfig.json"),ts.sys.readFile),parsed=ts.parseJsonConfigFileContent(config.config,ts.sys,join(root,"proj")),program=ts.createProgram(parsed.fileNames,parsed.options);return program.getSourceFiles().map((file:any)=>file.fileName).filter((name:string)=>name.startsWith(root)&&!name.includes("/node_modules/")).sort();};
 const firstFiles=independent();
 for(const step of fixture.program.steps){
  for(const[path,text]of Object.entries(step.edit as Record<string,string|null>)){if(text===null)rmSync(join(root,path),{force:true});else write(root,{[path]:text});}
  const again=await observation.observeInferenceSnapshotV1(async()=>scriptInputs.deriveClosureFactV1(derived.row,createScriptInputCache({workspaceRoot:root})));
  expect(again.fact===baseline,step.name).toBe(step.same);expect(again.rows.length,step.name).toBeGreaterThan(0);
  expect(JSON.stringify(independent())===JSON.stringify(firstFiles),step.name+": the compiler's own program agrees").toBe(step.same);
}
},120000);
