/** 🚪️ Logical inference rows retain coordinates and group identities without native JSON bodies. */
import {expect,test} from "bun:test";
import * as d3Array from "d3-array";
import * as d3Hierarchy from "d3-hierarchy";
import * as d3Polygon from "d3-polygon";
import {inferVizLayerTable} from "../../🧬️schema/💡️inferences/🧮transform/🟦️.ts";
import type {VizChartSpecification,VizRow} from "../../🧬️schema/📸️snapshot/📊️chart/🟦️.ts";
import fixture from "./🔣️.json";

function specification(rows:readonly VizRow[]):VizChartSpecification{return{width:100,height:80,language:"en",margin:{left:0,right:0,top:0,bottom:0},tables:[{name:"data",columns:[...new Set(rows.flatMap(Object.keys))],rows}],layers:[]};}

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
