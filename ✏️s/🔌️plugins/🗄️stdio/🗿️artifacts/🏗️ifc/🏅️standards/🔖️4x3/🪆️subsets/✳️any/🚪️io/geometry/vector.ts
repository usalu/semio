/** 🧭️ Bounded coordinate readers admit actual point/direction REAL domains before numeric conversion. */
import type{IfcValue}from'../../🧬️schema/📸️snapshot/🟦️.ts';
import{IfcStepError,type IfcIoSteps}from'../control.ts';
import{ifcNumber}from'../step.ts';
import type{IfcGeometryGraph}from'../geometry.ts';
import{ifcGeometryEntity,ifcGeometryFieldSteps}from'./source.ts';
/** 📍️ Exact2D/3D cardinality bounds source traversal before catalog-qualified leaf admission. */
export function* ifcGeometryVectorSteps(graph:IfcGeometryGraph,value:IfcValue|undefined,expected?:'IFCCARTESIANPOINT'|'IFCDIRECTION',dimension?:2|3):IfcIoSteps<[number,number,number]>{
 yield;const entity=graph.ref(value);ifcGeometryEntity(entity,expected?[expected]:['IFCCARTESIANPOINT','IFCDIRECTION']);const items=entity.args[0];
 if(items?.kind!=='aggregate'||items.value.length<2||items.value.length>3||dimension!==undefined&&items.value.length!==dimension)throw new IfcStepError('exchange.vector','Cartesian coordinate dimension must match its declared role');
 yield* ifcGeometryFieldSteps(graph,entity,entity.name,entity.name==='IFCCARTESIANPOINT'?'Coordinates':'DirectionRatios');
 return[ifcNumber(items.value[0]),ifcNumber(items.value[1]),items.value.length===3?ifcNumber(items.value[2]):0];
}
