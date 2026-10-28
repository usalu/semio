/** 🗺️ Exchange consumers share one prepared borrowed relation index over their private graph. */
import {prepareIfcRelationsSteps,type IfcPreparedRelations,type IfcRelationPlan} from '../../../../../📚️catalog/🔎️query/relations/🟦️.ts';
import type {IfcGeometryGraph} from "./geometry.ts";
import type {IfcEntity,IfcValue} from '../🧬️schema/📸️snapshot/🟦️.ts';
import {ifc4x3Catalog} from '../🧬️schema/📚️catalog/🟦️.ts';
import {IfcStepError,runIfcIoSteps,type IfcIoControl,type IfcIoSteps} from './control.ts';
const family=(key:string,type:string,subtypes=false)=>({key,type,subtypes});
const plan:IfcRelationPlan={families:[
 family('projects','IFCPROJECT'),family('decomposition','IFCRELDECOMPOSES',true),family('spatial','IFCSPATIALSTRUCTUREELEMENT',true),family('storeys','IFCBUILDINGSTOREY'),family('typeRelations','IFCRELDEFINESBYTYPE'),
 family('analyses','IFCSTRUCTURALANALYSISMODEL'),family('groups','IFCRELASSIGNSTOGROUP',true),family('items','IFCSTRUCTURALITEM',true),family('loadGroups','IFCSTRUCTURALLOADGROUP',true),family('actions','IFCSTRUCTURALACTION',true),family('contexts','IFCGEOMETRICREPRESENTATIONCONTEXT'),family('materials','IFCMATERIAL'),family('materialProperties','IFCMATERIALPROPERTIES'),family('members','IFCSTRUCTURALCURVEMEMBER'),family('connections','IFCRELCONNECTSSTRUCTURALMEMBER',true),family('materialRelations','IFCRELASSOCIATESMATERIAL'),family('propertyRelations','IFCRELDEFINESBYPROPERTIES'),family('activityRelations','IFCRELCONNECTSSTRUCTURALACTIVITY',true)
],roles:[
 {key:'group',family:'groups',attribute:'RelatingGroup',owner:'IFCRELASSIGNSTOGROUP',cardinality:'one'},
 {key:'materialProperties',family:'materialProperties',attribute:'Material',owner:'IFCMATERIALPROPERTIES',cardinality:'one'},
 {key:'connections',family:'connections',attribute:'RelatingStructuralMember',owner:'IFCRELCONNECTSSTRUCTURALMEMBER',cardinality:'one'},
 {key:'materialRelations',family:'materialRelations',attribute:'RelatedObjects',owner:'IFCRELASSOCIATES',cardinality:'many'},
 {key:'propertyRelations',family:'propertyRelations',attribute:'RelatedObjects',owner:'IFCRELDEFINESBYPROPERTIES',cardinality:'many'},
 {key:'propertyDefinitions',family:'propertyRelations',attribute:'RelatingPropertyDefinition',owner:'IFCRELDEFINESBYPROPERTIES',cardinality:'one'},
 {key:'typeRelations',family:'typeRelations',attribute:'RelatedObjects',owner:'IFCRELDEFINESBYTYPE',cardinality:'many'},
 {key:'activityRelations',family:'activityRelations',attribute:'RelatedStructuralActivity',owner:'IFCRELCONNECTSSTRUCTURALACTIVITY',cardinality:'one'}
]};
const reader={reference(value:IfcValue|undefined):bigint|undefined{return value?.kind==='reference'?value.value:undefined;},aggregate(value:IfcValue|undefined):readonly IfcValue[]|undefined{return value?.kind==='aggregate'?value.value:undefined;}};
/** 🪢️ No payload resolution occurs until a source-ordered consumer requests its original relation predicate. */
export class IfcExchangeRelations {
 private constructor(readonly index:IfcPreparedRelations<IfcValue,bigint>,readonly control:Readonly<IfcIoControl>){}
 static async prepare(graph:IfcGeometryGraph,control:Readonly<IfcIoControl>):Promise<IfcExchangeRelations>{const source={rows:graph.snapshot.entities,indexed:(entity:typeof graph.snapshot.entities[number])=>{const indexed=graph.indexed(entity.id);return indexed?.entity===entity?indexed:undefined;}};const index=await runIfcIoSteps(prepareIfcRelationsSteps(ifc4x3Catalog,source,plan,reader),control,'exchange-relations');return new IfcExchangeRelations(index,control);}
 *valueSteps(entity:IfcEntity,attribute:string,owner:string):IfcIoSteps<IfcValue|undefined>{const query=yield* this.index.attributeSteps(entity,attribute,owner);if(query.status!=='unique')throw new IfcStepError('exchange.attribute',`Unresolved ${owner}.${attribute}: ${query.status}`);return query.matches[0]!.value;}
 value(entity:IfcEntity,attribute:string,owner:string):Promise<IfcValue|undefined>{return runIfcIoSteps(this.valueSteps(entity,attribute,owner),this.control,'exchange-attribute');}
 async rows(family:string):Promise<IfcEntity[]>{const rows=this.index.family(family);function* collect():IfcIoSteps<IfcEntity[]>{const result:IfcEntity[]=[];for(const row of rows){yield;result.push(row.entity as IfcEntity);}return result;}return runIfcIoSteps(collect(),this.control,'exchange-family');}
 *relatedSteps(role:string,target:bigint,select?:(row:IfcEntity)=>IfcIoSteps<boolean>):IfcIoSteps<IfcEntity[]>{
  const hits=yield* this.index.relatedSteps(role,target),result:IfcEntity[]=[];
  for(const hit of hits){yield;if(hit.issue==='aggregate')throw new IfcStepError('exchange.aggregate','Aggregate argument required');if(hit.issue)throw new IfcStepError('exchange.attribute',`Unresolved relation role ${role}: ${hit.issue}`);const row=hit.row.entity as IfcEntity;if(!select||(yield* select(row)))result.push(row);}
  return result;
 }
 related(role:string,target:bigint,select?:(row:IfcEntity)=>IfcIoSteps<boolean>):Promise<IfcEntity[]>{return runIfcIoSteps(this.relatedSteps(role,target,select),this.control,'exchange-relations-lookup');}
}
