import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { dirname,join } from "node:path";

import Parser from "web-tree-sitter";
import fixture from "../../🧫️fixtures/🪆️record-owner/🔣️.json";

const owner = join(import.meta.dir, "../..");
assert.equal(new Set(fixture.functions.map(row=>row.name)).size,3);
for(const mutate of [(value:any)=>value.extra=true,(value:any)=>value.exports.registeredDerives.push("DslRecord"),(value:any)=>value.functions[0].unknown=true]){const changed=structuredClone(fixture);mutate(changed);}
await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",owner)),"out/tree-sitter-rust.wasm")));
const tree=parser.parse(readFileSync(join(owner,"🦀️.rs"),"utf8"));if(!tree)throw Error("Missing product macro tree");
try {
 assert.equal(tree.rootNode.hasError(),false,"Product macro grammar refuses syntax errors");
 const functions=tree.rootNode.namedChildren.filter(node=>node.type==="function_item");
 const identifiers=(node:any):string[]=>["identifier","type_identifier","field_identifier"].includes(node.type)?[node.text]:node.namedChildren.flatMap(identifiers);
 for(const contract of fixture.functions){const matches=functions.filter(node=>node.childForFieldName("name")?.text===contract.name);assert.equal(matches.length,1);const names=new Set(identifiers(matches[0]!.childForFieldName("body")));for(const name of contract.required)assert.equal(names.has(name),true,"Missing "+name);for(const name of contract.forbidden)assert.equal(names.has(name),false,"Duplicate generic identity "+name);}
 for(const name of fixture.retired)assert.equal(functions.some(node=>node.childForFieldName("name")?.text===name),false,"Retired "+name);
 for(const name of ["expand_derive_composite_mutation","expand_mutation_leaf","expand_derive_mutations"])assert.equal(functions.some(node=>node.childForFieldName("name")?.text===name),true,"Required product behavior "+name);
} finally {tree.delete();parser.delete();}
