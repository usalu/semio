/** 🎬️ Contract (the table is the declared one, or the candidate table of `SEMIO_BUILD_IMPLEMENTATION_SCOPES` for the proof before a class is flipped live): a narrow implementation scope exists only for an action class whose runtime trace is a subset of the scope's closure, and the closure is derived twice (TypeScript compiler API and an independent Python walk) before any action key may rely on it; an unrelated build-tool edit leaves the key of a scoped class unchanged and an edit inside the closure changes it; a class without a row keeps the full closure; the bounds live in `./🔣️.json`. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,readdirSync,readFileSync,statSync,writeFileSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,normalize,relative,resolve,sep} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bazelActionClassV1,bazelScopeOfV1,bazelScopesFromEnvironmentV1,bazelScopesV1} from "../🟦️.ts";
import {buildSourceClosureIdentityV1,type BuildSourceClosureV1} from "../../../../📥️source/🔏️closure/🟦️.ts";
import {buildSchedulingPortsV1,buildSchedulingPrefixesV1,buildSchedulingV1,splitBuildSourceClosureV1} from "../../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import declared from "../🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const python=requireTestToolV1("python"),checkout=resolve(import.meta.dir,corpus.checkoutFromTests),entry=resolve(import.meta.dir,corpus.entryFromTests),oracleFile=resolve(import.meta.dir,corpus.executorOracleFromTests),schedulingOracleFile=resolve(import.meta.dir,corpus.schedulingOracleFromTests);

const regular=(path:string):boolean=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");
const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const schemasBeside=(path:string):string[]=>schemaFiles(join(dirname(path),"🧬️schema"));
const staticSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];for(const node of file.statements){if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);}return found;};
const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;
const compilerClosure=(roots:readonly string[],leaves:readonly string[]):Set<string>=>{
 const seen=new Set<string>(),pending=[normalize(entry),...roots.map(root=>normalize(join(checkout,root)))];
 while(pending.length){const path=pending.pop()!;if(seen.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemasBeside(path));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}
 for(const leaf of leaves)if(regular(normalize(join(checkout,leaf))))seen.add(normalize(join(checkout,leaf)));
 return seen;
};
const dynamicSpecifiers=(path:string):string[]=>{const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.arguments.length>=1&&ts.isStringLiteralLike(node.arguments[0]!)){const callee=node.expression;if(callee.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(callee)&&["require","__require"].includes(callee.text))found.push((node.arguments[0] as ts.StringLiteral).text);}ts.forEachChild(node,visit);};visit(file);return found;};
const closureAdding=(target:string,base:ReadonlySet<string>):number=>{const seen=new Set<string>(),pending=[normalize(target)];while(pending.length){const path=pending.pop()!;if(seen.has(path)||base.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;pending.push(...schemasBeside(path));for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));}return seen.size;};
const cheapLazyEdges=():string[]=>{const base=compilerClosure([],[]),found=new Set<string>();for(const path of base){if(path.endsWith(".json"))continue;for(const specifier of dynamicSpecifiers(path)){if(!specifier.startsWith("."))continue;const target=normalize(join(dirname(path),specifier));if(!base.has(target)&&regular(target)&&closureAdding(target,base)<=corpus.lazyEdgeBound)found.add(relative(checkout,target));}}return [...found].sort();};
const identity=(paths:Set<string>,edits:Readonly<Record<string,string>>={}):string=>{const digest=createHash("sha256");for(const path of [...paths].sort())digest.update(relative(checkout,path)+"\0").update(edits[relative(checkout,path)]??readFileSync(path)).update("\0");return digest.digest("hex");};
const rows=bazelScopesFromEnvironmentV1(process.env);

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,maximumFiles:0})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,unrelated:[]})).toBe(false);
});

test("the declared table is closed, unique per class and refuses forged tables",()=>{
 expect(bazelScopesV1().length).toBe(declared.scopes.length);
 const first=declared.scopes[0];if(!first)return;
 const refusal=(table:unknown):unknown=>{try{bazelScopesV1(table);}catch(error){return (error as {key?:string}).key;}return undefined;};
 expect(refusal({...declared,scopes:[first,{...first,id:"other"}]})).toBe("build.bazel.scope-unique-required");
 expect(refusal({...declared,scopes:[{...first,classes:[first.classes[0],first.classes[0]]}]})).toBe("build.bazel.scope-unique-required");
 expect(refusal({...declared,extra:true})).toBe("build.bazel.scope-table-required");
 expect(refusal({...declared,scopes:[{...first,leaves:"x"}]})).toBe("build.bazel.scope-table-required");
});

test("an action class selects its scope by the five facts the executor trace records, and an unlisted class keeps the full closure",()=>{
 const trace=readFileSync(resolve(import.meta.dir,"../../🎬️executor/🟦️.ts"),"utf8");
 for(const field of corpus.traceFields)expect(trace,field).toContain(field);
 const task={definition:{execution:{kind:"command"},metadata:{semio:{execution:{kind:"source"}}}}},none={definition:{execution:{kind:"command"}}};
 expect(bazelActionClassV1(task,"task","generate")).toEqual({command:"action",workerKind:"generate",executionKind:"command",semioKind:"source",owner:"none"});
 expect(bazelActionClassV1(none,"launch","service")).toEqual({command:"launch",workerKind:"service",executionKind:"command",semioKind:"none",owner:"none"});
 const native={definition:{execution:{kind:"command",nativeOwnerCommand:{}}}};
 expect(bazelActionClassV1(native,"task","build").owner).toBe("native");
 const row={id:"action-generate-command-source",roots:[],leaves:[],classes:[{select:bazelActionClassV1(task,"task","generate"),traced:{processes:1,modules:["a"],reads:[],physical:[]}}]},table=bazelScopesV1({version:1,scopes:[row]});
 expect(bazelScopeOfV1(bazelActionClassV1(task,"task","generate"),table)?.id).toBe(row.id);
 expect(bazelScopeOfV1(bazelActionClassV1(task,"test","generate"),table)).toBeUndefined();
 expect(bazelScopeOfV1(bazelActionClassV1(none,"task","generate"),table)).toBeUndefined();
 expect(bazelScopeOfV1(bazelActionClassV1({definition:{execution:{kind:"command",nativeOwnerCommand:{}},metadata:task.definition.metadata}},"task","generate"),table),"a native owner command never inherits the scope of a plain command").toBeUndefined();
});

for(const row of rows){
 test(`scope ${row.id}: roots and leaves exist, the compiler API and the independent Python walk derive one closure, and it stays bounded and inside the executor's world`,()=>{
  for(const path of [...row.scope.roots,...row.scope.leaves])expect(regular(join(checkout,path)),path).toBe(true);
  expect(new Set([...row.scope.roots,...row.scope.leaves]).size).toBe(row.scope.roots.length+row.scope.leaves.length);
  const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({entry,roots:row.scope.roots.map(path=>join(checkout,path)),leaves:row.scope.leaves.map(path=>join(checkout,path))}),encoding:"utf8",maxBuffer:1<<26});
  expect(run.status,run.stderr).toBe(0);
  const actual=compilerClosure(row.scope.roots,row.scope.leaves),oracle=new Set((JSON.parse(run.stdout) as string[]).map(path=>normalize(path)));
  expect([...actual].sort()).toEqual([...oracle].sort());
  expect(actual.size).toBeLessThanOrEqual(corpus.maximumFiles);
  for(const forbidden of corpus.forbidden)expect([...actual].filter(path=>path.includes(forbidden.fragment)),forbidden.reason).toEqual([]);
 });

 test(`scope ${row.id}: everything the runtime trace of each of its classes opened is inside its closure`,()=>{
  const closure=compilerClosure(row.scope.roots,row.scope.leaves);
  for(const item of row.classes){for(const path of [...item.traced.modules,...item.traced.reads,...item.traced.physical])expect(closure.has(normalize(join(checkout,path))),item.select.command+" "+item.select.workerKind+" "+path).toBe(true);expect(item.traced.processes).toBeGreaterThan(0);}
 });

 test(`scope ${row.id}: no walked file names a bare package specifier, so no module the runtime trace opens can hide from the static walk (a workspace name resolves through an installed link the walk cannot follow)`,()=>{
  const leaves=new Set(row.scope.leaves.map(path=>normalize(join(checkout,path)))),walked=[...compilerClosure(row.scope.roots,row.scope.leaves)].filter(path=>!leaves.has(path)&&!path.endsWith(".json")),isBare=(specifier:string)=>!specifier.startsWith(".")&&!/^(?:node:|bun:)/u.test(specifier)&&specifier!=="bun";
  const actual=walked.flatMap(path=>[...staticSpecifiers(path),...dynamicSpecifiers(path)].filter(isBare).map(specifier=>relative(checkout,path)+" -> "+specifier)).sort();
  const run=spawnSync(python,["-I",schedulingOracleFile],{input:JSON.stringify({entry,roots:row.scope.roots.map(path=>join(checkout,path)),leaves:row.scope.leaves.map(path=>join(checkout,path)),prefixes:[],ports:[]}),encoding:"utf8",maxBuffer:1<<26});
  expect(run.status,run.stderr).toBe(0);
  expect((JSON.parse(run.stdout) as {bare:string[][]}).bare.map(([path,specifier])=>relative(checkout,path!)+" -> "+specifier).sort()).toEqual(actual);
  expect(actual).toEqual([]);
 });

 test(`scope ${row.id}: every cheap dynamic edge of the executor is a declared root, so a rare path never meets an uncaptured module`,()=>{
  for(const path of cheapLazyEdges())expect(row.scope.roots,path).toContain(path);
 });

 test(`scope ${row.id}: an unrelated build-tool edit leaves the implementation identity unchanged and an edit inside the closure changes it`,()=>{
  const closure=compilerClosure(row.scope.roots,row.scope.leaves),before=identity(closure);
  for(const path of corpus.unrelated){expect(closure.has(normalize(join(checkout,path))),path).toBe(false);expect(identity(closure,{[path]:"edited"})).toBe(before);}
  const inside=relative(checkout,[...closure].find(path=>path.endsWith(".ts"))!);
  expect(identity(closure,{[inside]:"edited"})).not.toBe(before);
 });
}

const rowOf=(closure:ReadonlySet<string>,path:string,edit?:string):BuildSourceClosureV1["files"][number]=>{
 const local=relative(checkout,path),bytes=Buffer.from(edit??readFileSync(path)),imports:Record<string,string>={};
 if(!path.endsWith(".json"))for(const specifier of staticSpecifiers(path)){if(!specifier.startsWith("."))continue;const target=normalize(join(dirname(path),specifier));if(closure.has(target))imports[specifier]=relative(checkout,target);}
 return {path:local,sha256:createHash("sha256").update(bytes).digest("hex"),base64:bytes.toString("base64"),imports};
};
const snapshotOf=(closure:ReadonlySet<string>,edits:Readonly<Record<string,string>>={}):BuildSourceClosureV1=>({version:1,root:checkout,entry:relative(checkout,entry),files:[...closure].sort().map(path=>rowOf(closure,path,edits[relative(checkout,path)]))});
const sample=(paths:readonly string[],producers:readonly string[]):string[]=>{const step=Math.max(1,Math.ceil(paths.length/16));return paths.filter((path,index)=>index%step===0||producers.some(fragment=>path.includes(fragment)));};

for(const row of rows.filter(candidate=>candidate.scheduling.length)){
 const policy=buildSchedulingV1(),prefixes=buildSchedulingPrefixesV1(policy,row.scheduling),ports=buildSchedulingPortsV1(policy,row.scheduling);

 test(`scope ${row.id}: the scheduling split equals the independent Python oracle, and keyed code reaches scheduling code only through declared ports`,()=>{
  const closure=compilerClosure(row.scope.roots,row.scope.leaves),{keyed,overlay}=splitBuildSourceClosureV1(snapshotOf(closure),row.scheduling);
  const run=spawnSync(python,["-I",schedulingOracleFile],{input:JSON.stringify({entry,roots:row.scope.roots.map(path=>join(checkout,path)),leaves:row.scope.leaves.map(path=>join(checkout,path)),prefixes:prefixes.map(prefix=>join(checkout,prefix)),ports:[...ports].map(port=>join(checkout,port))}),encoding:"utf8",maxBuffer:1<<26});
  expect(run.status,run.stderr).toBe(0);
  const oracle=JSON.parse(run.stdout) as {keyed:string[];overlay:string[];violations:string[][]};
  expect(keyed.files.map(file=>file.path)).toEqual(oracle.keyed.map(path=>relative(checkout,path)).sort());
  expect((overlay?.files??[]).map(file=>file.path)).toEqual(oracle.overlay.map(path=>relative(checkout,path)).sort());
  expect(oracle.violations.map(([from,to])=>relative(checkout,from!)+" -> "+relative(checkout,to!))).toEqual([]);
  const own=new Set(keyed.files.map(file=>file.path)),stray=keyed.files.flatMap(file=>Object.values(file.imports).filter(target=>!own.has(target)&&!ports.has(target)).map(target=>file.path+" -> "+target));
  expect(stray).toEqual([]);
 });

 test(`scope ${row.id}: the modules that produce action bytes stay keyed, the scheduling samples leave the key, and the keyed part stays bounded`,()=>{
  const closure=compilerClosure(row.scope.roots,row.scope.leaves),{keyed,overlay}=splitBuildSourceClosureV1(snapshotOf(closure),row.scheduling),keyedPaths=keyed.files.map(file=>file.path),overlayPaths=(overlay?.files??[]).map(file=>file.path);
  for(const producer of corpus.keyedProducers)expect(keyedPaths.filter(path=>path.includes(producer.fragment)).length,producer.reason).toBeGreaterThan(0);
  for(const producer of corpus.keyedProducers)expect(overlayPaths.filter(path=>path.includes(producer.fragment)),producer.reason).toEqual([]);
  for(const sample of corpus.schedulingSamples)expect(overlayPaths.filter(path=>path.includes(sample.fragment)).length,sample.reason).toBeGreaterThan(0);
  expect(keyedPaths.length).toBeLessThanOrEqual(corpus.maximumKeyedFiles);
  expect(keyedPaths.length+overlayPaths.length).toBe(closure.size);
  expect(keyedPaths).toContain(relative(checkout,entry));
 });

 test(`scope ${row.id}: an edit of a scheduling file leaves the keyed identity unchanged and an edit of an output-producing file changes it`,()=>{
  const closure=compilerClosure(row.scope.roots,row.scope.leaves),base=snapshotOf(closure),identity=(snapshot:BuildSourceClosureV1)=>buildSourceClosureIdentityV1(splitBuildSourceClosureV1(snapshot,row.scheduling).keyed),before=identity(base);
  const edited=(path:string):BuildSourceClosureV1=>({...base,files:base.files.map(file=>file.path===path?rowOf(closure,join(checkout,path),"edited"):file)});
  const split=splitBuildSourceClosureV1(base,row.scheduling),scheduling=(split.overlay?.files??[]).map(file=>file.path),keyed=split.keyed.files.map(file=>file.path);
  expect(scheduling.length).toBeGreaterThan(0);
  for(const path of sample(scheduling,corpus.schedulingSamples.map(item=>item.fragment)))expect(identity(edited(path)),path).toBe(before);
  for(const path of sample(keyed,corpus.keyedProducers.map(item=>item.fragment)))expect(identity(edited(path)),path).not.toBe(before);
 },120000);
}
