import { test,expect } from "bun:test";

import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { dirname,join,resolve } from "node:path";
import Parser from "web-tree-sitter";
import fixture from "../🧫️fixtures/🪆️record-owner/🔣️.json";

const owner=resolve(import.meta.dir,"../.."), dependencyLoader=createRequire(join(owner,"🛂️manifest/📦️packages/🦀️rust/Cargo.toml")), graphCrates=["⏯️layout-run","⚙️engine","🖊️drawing","🗣️dsl","🛂️manifest","🧮️algorithms"];

test("canonical Record binding ownership agrees with independent closed schema verdicts",()=>{
 
 for(const row of fixture.bindings){expect(row.traitOwner===row.implementationOwner||row.typeOwner===row.implementationOwner).toBe(row.legal);expect(row.traitOwner==="record").toBe(row.canonical);const hostile=structuredClone(fixture) as unknown as {bindings:Record<string,unknown>[]};hostile.bindings[0]!.unknown=true;}
});

test("Graph names every canonical Record Value Diagnostic and Replication provider directly",()=>{
 const dependencies:Record<string,unknown>={};
 for(const crate of graphCrates){const source=readFileSync(join(owner,crate,"📦️packages/🦀️rust/Cargo.toml"),"utf8"), first=Bun.TOML.parse(source) as {dependencies:Record<string,unknown>}, third=dependencyLoader("@iarna/toml").parse(source);expect(first).toEqual(third);Object.assign(dependencies,first.dependencies);}
 const packages={record:"semio-framework-dsl-record",value:"semio-framework-value",diagnostic:"semio-framework-diagnostic",replication:"semio-framework-replication"} as const;
 for(const row of fixture.directEdges.filter(row=>row.consumer==="graph"))expect(dependencies[packages[row.provider as keyof typeof packages]]).toBeDefined();
 expect(dependencies["semio-framework-os-kernel"]).toBeUndefined();
});

test("Graph package syntax refuses the former OS dsl_core alias without reading any product",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",owner)),"out/tree-sitter-rust.wasm")));
 try{for(const crate of graphCrates){const tree=parser.parse(readFileSync(join(owner,crate,"🦀️.rs"),"utf8"));
  if(!tree)throw Error("Missing Graph package syntax tree");
  try{expect(tree.rootNode.hasError(),crate).toBe(false);for(const node of tree.rootNode.namedChildren.filter(node=>node.type==="extern_crate_declaration")){expect(node.text.includes("semio_framework_os_kernel")).toBe(false);expect(node.text.includes("dsl_core")).toBe(false);}}
  finally{tree.delete();}}}
 finally{parser.delete();}
});
