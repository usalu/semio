/** 🪞️ Contract: unmirror deletes the path-preserving re-export layers a carve leaves behind (mirror modules, explicit and glob re-exports of other carved crates), re-points every consumer path, import and glob to the owning crate, keeps designed API roots and own namespaces, leaves multi-mounted and non-editable files alone and reports what it cannot resolve; the corpus is language-agnostic, tree-sitter's Rust grammar checks every edited file and the resolver of a second universe built from the edited sources must resolve every surviving name to the very item it named before. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {Universe,type Hit} from "../🔍️resolver/🟦️.ts";
import {Unmirror} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const textsOf=(files:object):[string,string][]=>Object.entries(files).filter((entry):entry is[string,string]=>typeof entry[1]==="string");
const universeOf=(row:any,files:object):Universe=>{const u=new Universe("/virtual",new Map(textsOf(files)));for(const c of row.crates)u.addCrate(c.name,c.dir,c.root);for(const c of row.crates)u.root(c.name.replaceAll("-","_"));return u;};
const run=(row:any)=>{
 const u=universeOf(row,row.files),editable=row.editable===null?()=>true:(path:string)=>row.editable.includes(path),unmirror=new Unmirror(u,{apiRoots:new Set<string>(row.apiRoots),targets:new Set<string>(row.targets),editable});
 const paths=[...u.files.keys()].sort(),audit:Record<string,string[]>={};
 for(const path of paths){const failures=unmirror.audit(path);if(failures.length)audit[path]=failures;}
 for(const path of paths)unmirror.processFile(path);
 const edits:Record<string,string>={};for(const[path,text]of[...unmirror.report.edits].sort())edits[path]=text;
 const r=unmirror.report;
 return{edits,report:{removedLeaves:r.removedLeaves,repointedLeaves:r.repointedLeaves,repointedChains:r.repointedChains,addedImports:r.addedImports,removedModules:r.removedModules,unresolvedMirrors:[...r.unresolvedMirrors],multiMounted:[...r.multiMounted],invisible:[...r.invisible]},audit};
};
const witness=(row:{defect:string})=>row.defect?test.failing:test;
for(const row of corpus.cases)witness(row)("unmirror: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{expect<unknown>(run(row)).toEqual(row.expect);expect(run(row)).toEqual(run(row));});
test("every edited file is valid Rust (tree-sitter) and every file that was not reported stays byte-identical",async()=>{
 const parser=await parsed();
 for(const row of corpus.cases){
  for(const[path,text]of textsOf(row.expect.edits)){expect(parser.parse(text).rootNode.hasError(),row.name+": "+path).toBe(false);expect(text===new Map(textsOf(row.files)).get(path),row.name+": "+path).toBe(false);}
  for(const path of Object.keys(row.expect.edits))expect(Object.keys(row.files),path).toContain(path);
  if(row.editable!==null)for(const path of Object.keys(row.expect.edits))expect<unknown>(row.editable,path).toContain(path);
 }
});
const identity=(hit:Hit|undefined):string|null=>hit===undefined?null:hit.kind==="module"?"module "+hit.node.crate.ident+":"+hit.node.file+":"+hit.node.name:"item "+hit.node.crate.ident+":"+hit.node.file+":"+hit.name;
test("metamorphic: every explicit import that survives resolves, in a universe built from the edited sources, to the item or module it named before",()=>{
 for(const row of corpus.cases){
  const before=universeOf(row,row.files),after=universeOf(row,{...row.files,...row.expect.edits});
  for(const path of Object.keys(row.expect.edits)){
   const oldInfo=before.files.get(path)!,newInfo=after.files.get(path)!,old=new Map<string,string|null>();
   for(const node of oldInfo.nodes)for(const bound of node.uses){if(bound.leaf.glob)continue;const hit=before.leafHits(bound)[0],name=bound.leaf.alias??bound.leaf.path.at(-1)!;if(hit&&identity(hit)!==null)old.set(name,identity(hit));}
   for(const node of newInfo.nodes)for(const bound of node.uses){
    if(bound.leaf.glob)continue;const name=bound.leaf.alias??bound.leaf.path.at(-1)!,expected=old.get(name);if(expected===undefined)continue;
    const hit=after.leafHits(bound)[0];expect(identity(hit),row.name+": "+path+" "+bound.leaf.path.join("::")).toBe(expected);
   }
  }
 }
});
test("an unmirrored crate set audits clean in the edited universe (no leaf or path of a target is left unresolved)",()=>{
 for(const row of corpus.cases.filter(item=>Object.keys(item.expect.edits).length>0&&item.expect.report.unresolvedMirrors.length===0&&Object.keys(item.expect.audit).length===0)){
  const after=universeOf(row,{...row.files,...row.expect.edits}),unmirror=new Unmirror(after,{apiRoots:new Set<string>(row.apiRoots),targets:new Set<string>(row.targets),editable:()=>true});
  for(const path of Object.keys(row.expect.edits))expect(unmirror.audit(path),row.name+": "+path).toEqual([]);
 }
});
test("a second run over the edited sources finds nothing more to change",()=>{
 for(const row of corpus.cases.filter(item=>Object.keys(item.expect.edits).length>0)){
  const files={...row.files,...row.expect.edits},again=run({...row,files});expect(Object.keys(again.edits),row.name).toEqual([]);
 }
});

test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
