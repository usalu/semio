/** 🏛️ One neutral vocabulary owner is independent of the OS binding and preserves literal wire data. */
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Parser from "web-tree-sitter";
import {parseArtifactRef} from "../../../../🧬️schema/🗿️artifact-reference/🟦️.ts";
import vocabulary from "../../🔣️.json";
import references from "../../../../🧬️schema/🗿️artifact-reference/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
const owner=resolve(import.meta.dir,"../..");
const pkg=join(owner,"📦️packages/🦀️rust");
const sorted=(value:object)=>Object.fromEntries(Object.entries(value).sort(([a],[b])=>a.localeCompare(b)));
const namespaces=new Set(["crate","self","super","std","core","alloc","bool","char","str","u8","u16","u32","u64","u128","usize","i8","i16","i32","i64","i128","isize","f32","f64"]);
const ownerSources=async(entry:string,dependencies:readonly string[]):Promise<{paths:string[];unbound:{path:string;namespace:string}[]}>=>{
  await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out","tree-sitter-rust.wasm")));
  const seen=new Set<string>(),unbound:{path:string;namespace:string}[]=[],pending=[entry],owned=new Set(dependencies.map(value=>value.replaceAll("-","_")));
  while(pending.length){const path=resolve(pending.pop()!);if(seen.has(path))continue;seen.add(path);const source=await Bun.file(path).text(),tree=parser.parse(source);expect(tree.rootNode.hasError()).toBe(false);const nodes:Parser.SyntaxNode[]=[];const visit=(node:Parser.SyntaxNode):void=>{nodes.push(node);for(const child of node.namedChildren)visit(child);};visit(tree.rootNode);
    const modules=nodes.filter(node=>node.type==="mod_item"),local=new Set(modules.map(node=>node.childForFieldName("name")?.text).filter((value):value is string=>value!==undefined));
    const root=(node:Parser.SyntaxNode):string=>{const parent=node.childForFieldName("path");return parent?root(parent):node.namedChildren.length?root(node.namedChildren[0]):node.text;};
    for(const node of nodes.filter(value=>["scoped_identifier","scoped_type_identifier","scoped_use_list","use_wildcard"].includes(value.type))){const name=root(node);if(!namespaces.has(name)&&!owned.has(name)&&!local.has(name)&&/^[a-z_][a-z_0-9]*$/.test(name)&&!unbound.some(value=>value.path===path&&value.namespace===name))unbound.push({path,namespace:name});}
    for(const module of modules.filter(node=>!node.childForFieldName("body"))){let sibling=module.previousNamedSibling,authored:string|undefined;while(sibling?.type==="attribute_item"){const match=sibling.text.match(/^#\[path\s*=\s*("(?:[^"\\]|\\.)*")\]$/);if(match)authored=JSON.parse(match[1]) as string;sibling=sibling.previousNamedSibling;}const name=module.childForFieldName("name")!.text,candidates=authored?[resolve(dirname(path),authored)]:[join(dirname(path),name+".rs"),join(dirname(path),name,"mod.rs")];const next=(await Promise.all(candidates.map(async value=>await Bun.file(value).exists()?value:null))).find(value=>value!==null);if(!next)throw Error("module source absent "+path+" "+name);pending.push(next);}
  }return{paths:[...seen],unbound};
};


test("neutral vocabulary references match production contracts and independent literal SQLite rows",()=>{
  const ajv=new Ajv({strict:false});ajv.addSchema(vocabulary);ajv.addSchema(references);const admit=ajv.getSchema(`${references.$id}#/$defs/ArtifactRef`)!;
  
  const db=new Database(":memory:");db.run("CREATE TABLE identity(artifact_id TEXT,kind TEXT,standard TEXT,subset TEXT)");
  try{for(const value of corpus.references){expect(admit(value)).toBe(true);const actual=parseArtifactRef(value);db.run("DELETE FROM identity");db.run("INSERT INTO identity VALUES(?,?,?,?)",[actual.artifactId,actual.dialect.artifactKind,actual.dialect.standard,actual.dialect.subset]);expect(db.query("SELECT artifact_id AS artifactId,kind AS artifactKind,standard,subset FROM identity").get()).toEqual({artifactId:value.artifactId,...value.dialect});}}finally{db.close();}
  console.error("[DEBUG] Three literal vocabulary identities retained under independent schema and SQLite");
});

test("actual package has only owned general dependencies and an explicit complete export surface",async()=>{
  const text=await Bun.file(join(pkg,"Cargo.toml")).text();
  const first=Bun.TOML.parse(text) as {package:{name:string};dependencies:Record<string,{path:string}>};
  const oracle=toml.parse(text) as unknown as typeof first;
  expect(sorted(first.dependencies)).toEqual(sorted(oracle.dependencies));expect(first.package.name).toBe(corpus.packageName);
  expect(Object.keys(first.dependencies).sort()).toEqual([...corpus.normalDependencies].sort());
  for(const dependency of Object.values(first.dependencies)){const path=resolve(pkg,dependency.path,"Cargo.toml");expect(await Bun.file(path).exists()).toBe(true);expect(path.includes("/🛍️products/")).toBe(false);}
  const config=await Bun.file(join(pkg,"📋️project.json")).json() as {namedInputs:{default:string[]}};
  for(const dependency of Object.values(first.dependencies)){const domain=relative(resolve(owner,"../../../.."),dirname(dirname(resolve(pkg,dependency.path)))).replaceAll("\\","/");expect(config.namedInputs.default.some(input=>{const root=input.replace("{workspaceRoot}/","").replace("/**/*","");return domain===root||domain.startsWith(root+"/");})).toBe(true);}
  const entry=await Bun.file(join(pkg,"🦀️.rs")).text();await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out","tree-sitter-rust.wasm")));
  expect(parser.parse(entry).rootNode.hasError()).toBe(false);expect(entry.includes("::*")).toBe(false);
  const exports=entry.slice(entry.indexOf("pub use vocabulary::{")+"pub use vocabulary::{".length,entry.lastIndexOf("};")).split(",").map(value=>value.trim()).filter(Boolean);
  expect(exports.sort()).toEqual([...corpus.exports].sort());console.error("[DEBUG] Twelve explicit IO vocabulary exports and six first-party dependency owners");
});

test("neutral vocabulary namespace closure resolves only its actual owned dependencies",async()=>{
  const graph=await ownerSources(join(pkg,"🦀️.rs"),[...corpus.normalDependencies,"serde_json"]);
  expect(graph.unbound).toEqual([]);console.error("[DEBUG] Neutral vocabulary actual module closure "+graph.paths.length+" files, zero unbound external namespaces");
});
