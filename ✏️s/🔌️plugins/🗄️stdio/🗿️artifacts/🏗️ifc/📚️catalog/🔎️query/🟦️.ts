/** 🔎️ Catalog-owned queries locate borrowed source slots without admitting or publishing values. */
import {ifcNameSteps,type IfcCatalog,type IfcCatalogSteps,type IfcAttributeDeclaration,type IfcTypeExpression} from '../🟦️.ts';
export interface IfcQueryPart<V>{readonly name:string;readonly args:readonly V[]}
export interface IfcQueryEntity<V> extends IfcQueryPart<V>{readonly complex?:readonly IfcQueryPart<V>[]}
export interface IfcIndexedQueryEntity<V>{readonly entity:IfcQueryEntity<V>;readonly ordinal:number}
export interface IfcAttributeMatch<V>{readonly value:V|undefined;readonly declaration:IfcAttributeDeclaration;readonly type:IfcTypeExpression;readonly part:number;readonly index:number;readonly pointer:string}
export interface IfcAttributeQuery<V>{readonly status:'missing'|'unique'|'ambiguous'|'unknown';readonly matches:IfcAttributeMatch<V>[]}

/** 🪢️ Already captured simple rows use flattened slots; complex constituents use declared owner slots. */
export function* ifcAttributeQuerySteps<V>(catalog:IfcCatalog,indexed:IfcIndexedQueryEntity<V>,name:string,owner?:string):IfcCatalogSteps<IfcAttributeQuery<V>>{
 if(!Number.isSafeInteger(indexed.ordinal)||indexed.ordinal<0)throw new RangeError('A source query requires its nonnegative safe entity ordinal');
 const expected=yield* ifcNameSteps(name),qualified=owner===undefined?undefined:yield* ifcNameSteps(owner),matches:IfcAttributeMatch<V>[]=[],entity=indexed.entity,complex=entity.complex??[];
 if(!expected||owner!==undefined&&!qualified)return{status:'unknown',matches};
 let unknown=false;
 for(let part=0;part<=complex.length;part++){
  yield;const row=part?complex[part-1]!:entity,key=yield* ifcNameSteps(row.name),declaration=key?catalog.entities[key]:undefined;
  if(!declaration){unknown=true;continue;}
  if(matches.length===2)continue;
  const attributes=complex.length?declaration.attributes:declaration.allAttributes;
  for(let index=0;index<attributes.length;index++){
   yield;const attribute=attributes[index]!,attributeName=yield* ifcNameSteps(attribute.name);if(attributeName!==expected)continue;
   const declaredBy=yield* ifcNameSteps(attribute.declaredBy);if(qualified&&declaredBy!==qualified)continue;
   matches.push({value:row.args[index],declaration:attribute,type:attribute.typeExpr,part,index,pointer:'/entities/'+indexed.ordinal+'/'+(part?'complex/'+(part-1)+'/':'')+'args/'+index});if(matches.length===2)break;
  }
 }
 return{status:unknown?'unknown':matches.length===2?'ambiguous':matches.length?'unique':'missing',matches};
}
