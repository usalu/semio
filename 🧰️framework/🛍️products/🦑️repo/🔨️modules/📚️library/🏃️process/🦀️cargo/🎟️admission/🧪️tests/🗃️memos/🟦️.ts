/** 🗃️ Contract: repository cargo tests (print-rs:test and every `runRepositoryCargoTests` caller) admit their sources through the same memos as `bun repo cargo`: the foreign-source digest book and the per-unit admission memo under the store's unit cache. Oracles: a second memo instance and a second inventory over real files (behaviour), the TypeScript AST of the three call sites (wiring). Run from the repository root. */
import {expect,test} from "bun:test";
import ts from "typescript";
import {mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {readFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as routing from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../🏗️build/📥️caller/🟦️.ts";
import {repositoryCargoPlanningPortV1,repositoryCargoSourceMemoV1} from "../../../🚪️launcher/🟦️.ts";
import {unitCacheRootV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";

const files=(directory:string):number=>{let count=0;try{for(const entry of readdirSync(directory,{withFileTypes:true}))count+=entry.isDirectory()?files(join(directory,entry.name)):1;}catch{}return count;};
const past=(path:string)=>{const at=new Date(Date.now()-3_600_000);utimesSync(path,at,at);};

test("the source memo of a store recalls what another instance remembered, keyed by the toolchain and kept under the unit cache",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"rcm-"))),source=join(root,"src.rs");
 try{
  writeFileSync(source,"fn a(){}\n");past(source);
  const unit={packageId:"p 0.1.0",packageName:"p",name:"p",kind:["lib"],entry:source},policy={maximumFiles:32},result={inputs:[{path:source,digest:"d".repeat(64),bytes:9}],cfg:["unix"],environment:null,probed:[source],startedMs:Date.now()+3_000};
  repositoryCargoSourceMemoV1({root,toolchain:{digest:"t1"}}).remember(unit,policy,result);
  expect(files(unitCacheRootV1(root))).toBeGreaterThan(0);
  expect(repositoryCargoSourceMemoV1({root,toolchain:{digest:"t1"}}).recall({...unit},policy)?.inputs).toEqual(result.inputs);
  expect(repositoryCargoSourceMemoV1({root,toolchain:{digest:"t2"}}).recall({...unit},policy)).toBeUndefined();
  writeFileSync(source,"fn b(){}\n");
  expect(repositoryCargoSourceMemoV1({root,toolchain:{digest:"t1"}}).recall({...unit},policy)).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the planning port of a store inventories a foreign crate through the digest book under the unit cache, and a second inventory is equal",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"rcp-"))),crate=join(root,"registry","dep-1.0.0");
 try{
  mkdirSync(join(crate,"src"),{recursive:true});for(const[path,text]of[["Cargo.toml","[package]\nname=\"dep\"\n"],["src/lib.rs","pub fn f(){}\n"],["src/m.rs","pub fn g(){}\n"]] as const){writeFileSync(join(crate,path),text);past(join(crate,path));}
  await Bun.sleep(2_100);const before=files(unitCacheRootV1(root));
  const [first,second]=await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"cargo:memo-law",maximumElapsedMilliseconds:60_000},createNativeBuildCallerCapabilitiesV1(root,["run","fixture:test"],"bazel",{}),Date.now()),async(original:Parameters<typeof routing.createIssuedResourceControlV1>[0])=>{const control=routing.createIssuedResourceControlV1(original,"cargo:memo-law",0);return[await repositoryCargoPlanningPortV1({root}).sourceInputs(join(crate,"Cargo.toml"),control),await repositoryCargoPlanningPortV1({root}).sourceInputs(join(crate,"Cargo.toml"),control)];});
  expect(before).toBe(0);expect(files(unitCacheRootV1(root))).toBeGreaterThan(0);
  expect(second).toEqual(first);expect(first.length).toBe(3);
  expect(statSync(unitCacheRootV1(root)).isDirectory()).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},60_000);

const parse=async(path:string)=>{const text=await readFile(join(process.cwd(),path),"utf8");return ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true);};
const calls=(node:ts.Node,name:string):ts.CallExpression[]=>{const found:ts.CallExpression[]=[];const visit=(child:ts.Node)=>{if(ts.isCallExpression(child)&&child.expression.getText()===name)found.push(child);ts.forEachChild(child,visit);};visit(node);return found;};
const declaration=(tree:ts.SourceFile,name:string)=>{const node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);if(!node)throw Error("declaration required: "+name);return node;};
const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/";

test("wiring: repository tests compose both memos, the process admission hands the memo to source admission, and the launcher uses the same composition",async()=>{
 const admission=await parse(library+"🎟️admission/🟦️.ts"),execute=declaration(admission,"execute");
 const properties=new Map<string,string>();const visit=(node:ts.Node)=>{if(ts.isPropertyAssignment(node))properties.set(node.name.getText(),node.initializer.getText());ts.forEachChild(node,visit);};visit(execute);
 expect(properties.get("planningPort")).toBe("repositoryCargoPlanningPortV1(store)");expect(properties.get("sourceMemo")).toBe("repositoryCargoSourceMemoV1(store)");
 const testing=await parse("🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts"),admit=calls(declaration(testing,"admitCargoExecutionV1"),"admitCargoSourceBoundariesV1");
 expect(admit.length).toBe(1);expect(admit[0]!.arguments.length).toBe(6);expect(admit[0]!.arguments[5]!.getText()).toBe("admission.sourceMemo");
 const launcher=await parse(library+"🚪️launcher/🟦️.ts");
 expect(calls(launcher,"repositoryCargoPlanningPortV1").length).toBe(1);expect(calls(launcher,"repositoryCargoSourceMemoV1").length).toBe(1);
 expect(calls(launcher,"unitAdmissionMemoV1").length,"one composition only").toBe(1);expect(calls(launcher,"foreignDigestMemoV1").length,"one composition only").toBe(1);
});
