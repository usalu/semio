import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import ts from "typescript";
import fixture from "../🧫️fixtures/🔣️.json";

const readsProduct=(source:string)=>source.includes("/🛍️products/");
const compilerReadsProduct=(source:string)=>{
 const tree=ts.createSourceFile("ownership.ts",source,ts.ScriptTarget.Latest,true);let found=false;
 const visit=(node:ts.Node)=>{if(ts.isStringLiteralLike(node)&&node.text.split("/").includes("🛍️products"))found=true;ts.forEachChild(node,visit);};visit(tree);return found;
};

test("General diagnostic laws remain independent of product sources",()=>{
 expect(fixture.version).toBe(1);
 for(const row of fixture.cases){expect(readsProduct(row.source)).toBe(row.readsProduct);expect(compilerReadsProduct(row.source)).toBe(row.readsProduct);}
 const source=readFileSync(new URL("../../🧪️tests/🟦️.ts",import.meta.url),"utf8");
 expect(compilerReadsProduct(source)).toBe(fixture.generalReadsProduct);expect(readsProduct(source)).toBe(fixture.generalReadsProduct);
});

test("General diagnostic propagation uses one neutral authored field",()=>{
 const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⚠️kind/🔣️.json",import.meta.url),"utf8"));
 const db=new Database(":memory:");try{expect(db.query("SELECT count(*) AS n FROM json_each(?) WHERE key=?").get(JSON.stringify(law),fixture.parentPropagationField)).toEqual({n:1});}finally{db.close();}
 expect(Object.hasOwn(law,fixture.parentPropagationField)).toBe(true);
 expect(Object.keys(law).filter(key=>key.startsWith("parent"))).toEqual([fixture.parentPropagationField]);
});
