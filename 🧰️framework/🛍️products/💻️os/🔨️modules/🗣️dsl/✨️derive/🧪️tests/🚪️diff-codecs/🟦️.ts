/** 🚪️ Neutral diff representation contracts validated with Ajv and independently parsed Rust grammar. */
import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import fixture from "../../🧫️fixtures/🚪️diff-codecs/🔣️.json";
const owner=join(import.meta.dir,"../..");
const schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8"));
const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(schema);
const validate=ajv.getSchema(schema.$id+"#/$defs/DiffRepresentations")!;
assert.equal(validate(fixture),true);
const invalid=structuredClone(fixture);invalid.representations[0]!.name="invalid";assert.equal(validate(invalid),false);
await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",owner)),"out/tree-sitter-rust.wasm")));
const tree=parser.parse(readFileSync(join(owner,"🦀️.rs"),"utf8"));assert(tree);assert.equal(tree.rootNode.hasError(),false);
try {
 const functions=tree.rootNode.namedChildren.filter(node=>node.type==="function_item");
 const names=(node:any):string[]=>["identifier","type_identifier","field_identifier"].includes(node.type)?[node.text]:node.namedChildren.flatMap(names);
 for(const contract of fixture.representations){const generator=functions.find(node=>node.childForFieldName("name")?.text===contract.generator);assert(generator);const found=new Set(names(generator));assert.equal(found.has(contract.contract),true);for(const method of contract.methods)assert.equal(found.has(method),true);for(const method of contract.forbidden)assert.equal(found.has(method),false);}
 assert.equal(functions.some(node=>node.childForFieldName("name")?.text==="expand_dsl_diff"),false);
} finally {tree.delete();parser.delete();}
