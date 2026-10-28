/** 🗓️ Authored ordered phase names remain distinct from the singular IFC context phase. */
import type {IfcEntity,IfcValue} from '../../../🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcGeometryGraph} from '../../geometry.ts';
import type {IfcExchangeRelations} from '../../relations.ts';
import {IfcStepError,type IfcIoSteps} from '../../control.ts';
const unset:IfcValue={kind:'unset'},text=(value:string):IfcValue=>({kind:'string',value}),aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value});
const invalid=():never=>{throw new IfcStepError('bim.phase-definition','Distinct complete authored BIM phase count and typed text list required');};
interface Builder {add(name:string,args:IfcValue[]):IfcValue;root(name:string):IfcValue[];project:IfcValue;}
/** 🧺 Empty lists use an explicit zero count; nonempty lists preserve every original string. */
export function* appendBimPhasesSteps(builder:Builder,phases:readonly string[]):IfcIoSteps<void>{
 const items:IfcValue[]=[];
 for(const phase of phases){yield;if(typeof phase!=='string')invalid();items.push({kind:'typedValue',value:{name:'IFCTEXT',items:[text(phase)]}});}
 yield;const count=builder.add('IFCPROPERTYSINGLEVALUE',[text('PhaseCount'),unset,{kind:'typedValue',value:{name:'IFCINTEGER',items:[{kind:'integer',value:BigInt(items.length)}]}},unset]),properties=[count];
 if(items.length){yield;properties.push(builder.add('IFCPROPERTYLISTVALUE',[text('PhaseNames'),unset,aggregate(items),unset]));}
 yield;const set=builder.add('IFCPROPERTYSET',[...builder.root('semio_BimProject'),aggregate(properties)]);
 yield;builder.add('IFCRELDEFINESBYPROPERTIES',[...builder.root(''),aggregate([builder.project]),set]);
}
/** 🔎 Source-owned relation queries retain original row order and exact attribute owners. */
export function* readBimPhasesSteps(graph:IfcGeometryGraph,relations:IfcExchangeRelations,project:IfcEntity):IfcIoSteps<string[]>{
 const definitions=yield* relations.relatedSteps('propertyRelations',project.id,function*(row){
  const set=graph.ref(yield* relations.valueSteps(row,'RelatingPropertyDefinition','IFCRELDEFINESBYPROPERTIES')),name=yield* relations.valueSteps(set,'Name','IFCROOT');
  return name?.kind==='string'&&name.value==='semio_BimProject';
 });
 if(!definitions.length){const phase=yield* relations.valueSteps(project,'Phase','IFCCONTEXT');if(phase?.kind==='unset')return[];if(phase?.kind!=='string')return invalid();return[phase.value];}
 if(definitions.length!==1)return invalid();
 const set=graph.ref(yield* relations.valueSteps(definitions[0]!,'RelatingPropertyDefinition','IFCRELDEFINESBYPROPERTIES'));
 if(set.name!=='IFCPROPERTYSET')return invalid();
 const properties=yield* relations.valueSteps(set,'HasProperties','IFCPROPERTYSET');
 if(properties?.kind!=='aggregate')return invalid();
 let count:bigint|undefined,names:IfcEntity|undefined;
 for(const ref of properties.value){
  yield;const property=graph.ref(ref),name=yield* relations.valueSteps(property,'Name','IFCPROPERTY');
  if(name?.kind!=='string')return invalid();
  if(name.value==='PhaseCount'){
   if(count!==undefined||property.name!=='IFCPROPERTYSINGLEVALUE')return invalid();
   const nominal=yield* relations.valueSteps(property,'NominalValue','IFCPROPERTYSINGLEVALUE'),unit=yield* relations.valueSteps(property,'Unit','IFCPROPERTYSINGLEVALUE');
   if(unit?.kind!=='unset'||nominal?.kind!=='typedValue'||nominal.value.name!=='IFCINTEGER'||nominal.value.items.length!==1)return invalid();
   const value=nominal.value.items[0];if(value?.kind!=='integer'||value.value<0n)return invalid();count=value.value;
  }else if(name.value==='PhaseNames'){
   if(names||property.name!=='IFCPROPERTYLISTVALUE')return invalid();names=property;
  }else return invalid();
 }
 if(count===undefined)return invalid();
 if(count===0n){if(names)return invalid();return[];}
 if(!names)return invalid();
 const list=yield* relations.valueSteps(names,'ListValues','IFCPROPERTYLISTVALUE'),unit=yield* relations.valueSteps(names,'Unit','IFCPROPERTYLISTVALUE');
 if(unit?.kind!=='unset'||list?.kind!=='aggregate'||BigInt(list.value.length)!==count)return invalid();
 const result:string[]=[];
 for(const item of list.value){yield;if(item.kind!=='typedValue'||item.value.name!=='IFCTEXT'||item.value.items.length!==1)return invalid();const value=item.value.items[0];if(value?.kind!=='string')return invalid();result.push(value.value);}
 return result;
}

