import {readFileSync} from "node:fs";
import {dirname,resolve,join} from "node:path";
import assert from "node:assert/strict";
import Parser from "web-tree-sitter";
export type BindingFixture={version:number;owner:string;entry:string;manifest:string;binding:string;types:string[];fields:Record<string,{id:number;key:string;shape:string;optional:boolean}[]>;controls:{initialCancel:string;metadataZero:string;projection:string;construction:string};cases:{id:string;type:string;value:Record<string,unknown>;expectedFields:[number,unknown][]}[];sourceCases:{id:string;accepted:boolean}[]};
export type BindingReader=(path:string)=>string;
/** 🪆️ Checks executable binding ownership and direct canonical trait authority without private implementation digests. */
export async function inspectNeutralBindingOwnership(root:string,fixturePath:string,read:BindingReader=path=>readFileSync(resolve(root,path),"utf8")):Promise<{types:number;directRecord:boolean}>{
 const fixture=JSON.parse(read(fixturePath)) as BindingFixture;
 const manifest=Bun.TOML.parse(read(fixture.manifest)) as {package:{name:string};dependencies:Record<string,{path?:string;package?:string}>};
 assert.equal(manifest.package.name,fixture.owner);const dep=manifest.dependencies["semio-framework-dsl-record"];assert.ok(dep?.path);const canonical=resolve(root,dirname(fixture.manifest),dep.path,"Cargo.toml"),owned=resolve(root,"🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml");assert.equal(canonical,owned);
 for(const [name,dependency]of Object.entries(manifest.dependencies)){assert.equal(name.includes("os-kernel"),false);if(dependency.path)assert.equal(dependency.path.includes("🛍️products"),false);}
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")));const trees:Parser.Tree[]=[];
 const ast=(path:string)=>{const tree=parser.parse(read(path));trees.push(tree);assert.equal(tree.rootNode.hasError(),false,path);return tree.rootNode;};
 const attrs=(node:Parser.SyntaxNode)=>{const found:Parser.SyntaxNode[]=[];for(let previous=node.previousNamedSibling;previous;previous=previous.previousNamedSibling){if(previous.type.endsWith("comment"))continue;if(previous.type!=="attribute_item")break;found.unshift(previous.namedChildren[0]!);}return found;};
 const active=(node:Parser.SyntaxNode)=>{assert.equal(attrs(node).some(a=>["cfg","cfg_attr"].includes(a.namedChildren[0]?.text??"")),false);};
 try{const entry=ast(fixture.entry),mounts=entry.namedChildren.filter(node=>node.type==="mod_item"&&attrs(node).some(attribute=>attribute.namedChildren[0]?.text==="path"&&resolve(root,dirname(fixture.entry),JSON.parse(attribute.childForFieldName("value")!.text))===resolve(root,fixture.binding)));assert.equal(mounts.length,1);active(mounts[0]!);
 const binding=ast(fixture.binding),impls=binding.namedChildren.filter(node=>node.type==="impl_item"&&node.childForFieldName("trait")?.text==="DslField");assert.deepEqual(impls.map(node=>node.childForFieldName("type")!.text),fixture.types);for(const node of impls)active(node);assert.ok(binding.namedChildren.some(node=>node.type==="use_declaration"&&node.text.includes("semio_framework_dsl_record")));
 return{types:impls.length,directRecord:true};}finally{for(const tree of trees)tree.delete();parser.delete();}
}