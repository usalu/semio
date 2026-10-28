import blockCases from'./csg-block-cases.json';
import roundCases from'./csg-round-cases.json';
import rectangleCases from'./rectangle-extrusion-cases.json';
import circleCases from'./circle-extrusion-cases.json';
import roundedCases from'./rounded-rectangle-cases.json';
import{ifcBlockFixture,type IfcBlockCase}from'./csg-block-fixtures.ts';
import{ifcRoundFixture,type IfcRoundSource}from'./csg-round-fixtures.ts';
import{ifcRectangleFixture,type IfcRectangleCase}from'./rectangle-extrusion-fixtures.ts';
import{ifcCircleFixture,type IfcCircleCase}from'./circle-extrusion-fixtures.ts';
import{ifcRoundedRectangleFixture,type IfcRoundedRectangleCase}from'./rounded-rectangle-fixtures.ts';
import{IfcGeometryGraph,ifcIdentity,type IfcMatrix}from'../geometry.ts';
export const meshWriterPolicy={absoluteChordErrorMetres:0,relativeChordError:.001,maxSegments:4096};
/** 🧾️ Original declared fixtures are shared by word baselines, direct sinks and raw output routes. */
export async function meshWriterFixture(kind:string,reflected=false){
 const fixture=kind==='block'?ifcBlockFixture(blockCases.cases[0]as IfcBlockCase):kind==='cone'||kind==='cylinder'?ifcRoundFixture(roundCases.cases.find(row=>row.id===kind+'-default')as{id:string;source:IfcRoundSource}):kind==='rectangle'?ifcRectangleFixture(rectangleCases.cases[0]as IfcRectangleCase):kind==='circle'?ifcCircleFixture(circleCases.cases[0]as IfcCircleCase):ifcRoundedRectangleFixture(roundedCases.cases[0]as IfcRoundedRectangleCase),source=fixture.snapshot,captured=await IfcGeometryGraph.capture(source,{batchSize:4096}),graph=captured.artifact!,reference='primitive'in fixture?fixture.primitive:'block'in fixture?fixture.block:fixture.solid;
 if(!('id'in reference)&&reference.kind!=='reference')throw Error('Original source reference required');
 return{source,graph,entity:graph.lookup('id'in reference?reference.id:reference.kind==='reference'?reference.value:0n)!,world:(reflected?[-2,0,0,0,0,3,0,0,0,0,4,0,5,-7,11,1]:ifcIdentity())as IfcMatrix,scale:1,policy:meshWriterPolicy};
}
