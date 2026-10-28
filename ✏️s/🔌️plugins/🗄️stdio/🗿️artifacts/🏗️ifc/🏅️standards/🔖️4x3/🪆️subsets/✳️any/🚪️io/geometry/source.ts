/** 🔎️ Geometry consumers borrow catalog-qualified values from the canonical captured source index. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {ifc4x3Catalog} from '../../🧬️schema/📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps} from '../../../../../../📚️catalog/🔎️query/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import type {IfcGeometryGraph} from '../geometry.ts';
import {ifcDomainTypeMatchesSteps} from '../../🧬️schema/⚖️validation/type-matching.ts';

/** 🏷️ Exact simple geometric roles retain their current complete catalog arity. */
export function ifcGeometryEntity(entity:IfcEntity,names:readonly string[]):void{if(!names.includes(entity.name)||entity.complex?.length||entity.args.length!==ifc4x3Catalog.entities[entity.name]?.allAttributes.length)throw new IfcStepError('geometry.declaration','An exact simple '+names.join(' or ')+' declaration is required');}

/** 📍️ Resolve one owned declaration without guessed slots or a second source ordinal index. */
export function* ifcGeometryFieldSteps(graph:IfcGeometryGraph,entity:IfcEntity,owner:string,name:string):IfcIoSteps<IfcValue>{
 yield;const indexed=graph.indexed(entity.id);if(!indexed||indexed.entity!==entity)throw new IfcStepError('geometry.source','Original captured entity and ordinal required');const query=yield* ifcAttributeQuerySteps(ifc4x3Catalog,indexed,name,owner),match=query.matches[0];if(query.status!=='unique'||!match||match.part!==0||match.value===undefined)throw new IfcStepError('geometry.source','Unique declared '+owner+'.'+name+' source required');
 const value=match.value,refuse=()=>{throw new IfcStepError('geometry.domain',owner+'.'+name+' requires its declared source value domain');};
 if(match.declaration.derived){if(value.kind!=='derived')refuse();return value;}if(value.kind==='unset'){if(!match.declaration.optional)refuse();return value;}
 const steps=ifcDomainTypeMatchesSteps(value,match.type,id=>graph.lookup(id)?.name);try{for(;;){const step=steps.next();if(step.done){if(!step.value)refuse();return value;}yield;if(step.value>256)throw new IfcStepError('geometry.depth','Source type traversal exceeds the geometry depth limit');}}finally{steps.return(false);}
}
