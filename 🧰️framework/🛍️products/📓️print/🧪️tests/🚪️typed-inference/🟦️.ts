/** 🚪️ Logical inference rows retain coordinates and group identities without native JSON bodies. */
import {expect,test} from "bun:test";
import * as d3Array from "d3-array";
import * as d3Hierarchy from "d3-hierarchy";
import * as d3Polygon from "d3-polygon";
import {inferVizLayerTable} from "../../🧬️schema/💡️inferences/🧮transform/🟦️.ts";
import type {VizChartSpecification,VizRow} from "../../🧬️schema/📸️snapshot/📊️chart/🟦️.ts";
import fixture from "./🔣️.json";
import ownership from "../../🚪️io/🧫️fixtures/🏛️ownership/🔣️.json";
import Ajv from "ajv/dist/2020.js";
import chartSchema from "../../🧬️schema/📸️snapshot/📊️chart/🔣️.json";
import snapshotSchema from "../../🧬️schema/📸️snapshot/🔣️.json";
import catalogSchema from "../../🧬️schema/🔣️.json";
import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
import Parser from "web-tree-sitter";

test("native chart representation definitions belong to IO according to the neutral owner declaration",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));const root=resolve(import.meta.dir,"../..");let checked=0;
 try{
  for(const path of ownership.semanticOwners){const source=readFileSync(resolve(root,path),"utf8"),ast=parser.parse(source)!;expect(ast.rootNode.hasError()).toBe(false);const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="attribute_item")expect(node.text).not.toMatch(/\b(?:ToValue|FromValue|DslRecord|value_codec|record_binding)\b|#\[\s*(?:value|dsl)\s*\(/);if(node.type==="impl_item")expect(node.childForFieldName("trait")?.text??"").not.toMatch(/(?:ToValue|FromValue|DslRecord)$/);for(const child of node.namedChildren)visit(child);};visit(ast.rootNode);ast.delete();checked++;}
  for(const row of ownership.representationOwners){const source=readFileSync(resolve(root,row.path),"utf8"),ast=parser.parse(source)!;expect(ast.rootNode.hasError()).toBe(false);for(const type of row.types)expect(source).toMatch(new RegExp("(?:for\\s+|struct\\s+)"+type+"\\b"));if(row.record)expect(source).toMatch(/record_binding!\s*\{/);ast.delete();checked++;}
 }finally{parser.delete();}
 await import("../../🚪️io/🧪️tests/🏛️ownership/🟦️.ts");
 console.log(`[DEBUG] Print native codec ownership sources=${checked} plainExamples=true independentTreeSitter=true independentMinimatch=true runtime=false`);
});

function specification(rows:readonly VizRow[]):VizChartSpecification{return{width:100,height:80,language:"en",margin:{left:0,right:0,top:0,bottom:0},tables:[{name:"data",columns:[...new Set(rows.flatMap(Object.keys))],rows}],layers:[]};}

test("native inference entry functions belong to binary IO and the real service selects that owner",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));const root=resolve(import.meta.dir,"../.."),law=ownership.transportEntry;
 try{const source=readFileSync(resolve(root,law.semanticOwner),"utf8"),ast=parser.parse(source)!;const functions=ast.rootNode.namedChildren.filter(node=>node.type==="function_item").map(node=>node.childForFieldName("name")?.text);for(const name of law.functions)expect(functions).not.toContain(name);expect(source).toContain("io::binary::inferences::execute_chart_inference");ast.delete();const transport=parser.parse(readFileSync(resolve(root,law.owner),"utf8"))!;expect(transport.rootNode.hasError()).toBe(false);const owned=transport.rootNode.namedChildren.filter(node=>node.type==="function_item").map(node=>node.childForFieldName("name")?.text);for(const name of law.functions)expect(owned).toContain(name);transport.delete();}finally{parser.delete();}
 console.log("[DEBUG] Print real inference service selects binary IO entry; independentTreeSitter=true nativeRuntimeRequired=true");
});

test("semantic validation reads authored chart fields without projecting a native artifact",async()=>{
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));const law=ownership.pureValidation,source=readFileSync(resolve(import.meta.dir,"../..",law.owner),"utf8"),ast=parser.parse(source)!;
 try{const method=ast.rootNode.namedChildren.find(node=>node.type==="function_item"&&node.childForFieldName("name")?.text===law.method);expect(method).toBeDefined();let calls=0;const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="call_expression"){calls++;const callable=node.childForFieldName("function");expect(callable?.childForFieldName("field")?.text??callable?.text).not.toBe(law.forbiddenCall);}for(const child of node.namedChildren)visit(child);};visit(method!);expect(calls).toBeGreaterThan(0);}finally{ast.delete();parser.delete();}
 const validate=new Ajv({strict:false,allowUnionTypes:true}).addSchema([snapshotSchema,catalogSchema]).compile(chartSchema);
 for(const row of ownership.validation)expect(Boolean(validate(row.chart))).toBe(row.valid);
 console.log(`[DEBUG] Print semantic validation owns the authored chart value; independentTreeSitter=true independentAjvCases=${ownership.validation.length} nativeRuntimeRequired=true`);
});

test("bundled paths are ordered coordinate rows adjudicated by D3",()=>{
 const rows=fixture.bundle.rows,spec=specification(rows),actual=inferVizLayerTable(spec,{mark:"line",data:"data",layout:{algorithm:"bundling",options:{width:100,height:80}}}).rows;
 const root=d3Hierarchy.stratify<typeof rows[number]>().id(row=>row.id).parentId(row=>row.parent)(rows),laid=d3Hierarchy.cluster<typeof rows[number]>().size([100,80])(root);
 const oracle=laid.links().flatMap(({source,target},detail)=>target.path(source).map((node,order)=>({detail,order,x:node.x,y:node.y})));
 expect(actual).toEqual(fixture.bundle.expected);expect(actual).toEqual(oracle);expect(actual.every(row=>!Object.hasOwn(row,"points"))).toBe(true);
 console.log("[DEBUG] Print bundling emits four typed coordinate rows; independent D3 agrees");
});

test("spatial polygons contain typed vertices adjudicated by D3",()=>{
 const rows=fixture.polygon.rows,actual=inferVizLayerTable(specification(rows),{mark:"polygon",data:"data",layout:{algorithm:"hull"}}).rows;
 const sorted=(points:readonly(readonly number[])[])=>[...points].sort((a,b)=>a[0]!-b[0]!||a[1]!-b[1]!);
 expect(sorted(actual.map(row=>[Number(row.x),Number(row.y)]))).toEqual(fixture.polygon.expected);
 expect(sorted(actual.map(row=>[Number(row.x),Number(row.y)]))).toEqual(sorted(d3Polygon.polygonHull(rows.map(row=>[row.x,row.y]))!));
 expect(actual.every(row=>!Object.hasOwn(row,"points"))).toBe(true);
 console.log("[DEBUG] Print hull emits three typed vertices; independent D3 agrees");
});

test("group identities compare owned scalar values without textual coercion",()=>{
 const rows=fixture.group.rows.map((row,sourceIndex)=>({...row,sourceIndex})),actual=inferVizLayerTable(specification(rows),{mark:"point",data:"data",transform:[{kind:"group",options:{columns:["a","b"]}}]}).rows;
 const ids=new Map<typeof rows[number],number>(),leaves=d3Array.groups(rows,row=>row.a,row=>row.b).flatMap(([,groups])=>groups.map(([,values])=>values)).sort((a,b)=>rows.indexOf(a[0]!)-rows.indexOf(b[0]!));
 for(const[group,values]of leaves.entries())for(const row of values)ids.set(row,group);
 expect(actual.map(row=>row.group)).toEqual(actual.map(row=>fixture.group.expected[Number(row.sourceIndex)]));expect(actual.map(row=>row.group)).toEqual(actual.map(row=>ids.get(rows[Number(row.sourceIndex)]!)));
 console.log("[DEBUG] Print grouping retains four exact scalar tuple identities; independent D3 agrees");
});

test("Print original ownership examples retain no whole-trial schema authority",()=>{
 expect(existsSync(resolve(import.meta.dir,"../../🚪️io/🧫️fixtures/🏛️ownership/🧬️schema/🔣️.json"))).toBe(false);
 console.log("[DEBUG] Print plain ownership examples remain testing only; actual native representation/TreeSitter/D3 laws retained");
});
