/** 🪢️ One cooperative borrowed-source preparation serves schema-qualified relation consumers. */
import {ifcNameSteps,ifcSubtypeSteps,type IfcCatalog,type IfcCatalogSteps} from '../../🟦️.ts';
import {ifcAttributeQuerySteps,type IfcAttributeQuery,type IfcIndexedQueryEntity,type IfcQueryEntity} from '../🟦️.ts';
/** 🧷️ The source owner lends one already-indexed row without scans, allocation or payload reads. */
export interface IfcRelationSource<V>{readonly rows:readonly IfcQueryEntity<V>[];indexed(entity:IfcQueryEntity<V>):IfcIndexedQueryEntity<V>|undefined}
export interface IfcRelationFamily {readonly key:string;readonly type:string;readonly subtypes:boolean}
export interface IfcRelationRole {readonly key:string;readonly family:string;readonly attribute:string;readonly owner:string;readonly cardinality:'one'|'many'}
export interface IfcRelationPlan {readonly families:readonly IfcRelationFamily[];readonly roles:readonly IfcRelationRole[]}
/** 👁️ Accessors must cheaply borrow one private immutable slot; they do not resolve or validate referenced payloads. */
export interface IfcRelationValueReader<V,K> {reference(value:V|undefined):K|undefined;aggregate(value:V|undefined):readonly V[]|undefined}
export interface IfcRelationHit<V> {readonly row:IfcIndexedQueryEntity<V>;readonly query:IfcAttributeQuery<V>;readonly issue:null|'aggregate'|'missing'|'ambiguous'|'unknown'}
interface RoleIndex<V,K> {readonly targets:Map<K,IfcRelationHit<V>[]>;readonly issues:IfcRelationHit<V>[]}
/** 🗂️ Lookup authority retains exact source objects and ordinals; it does not admit a graph or publish domain data. */
export class IfcPreparedRelations<V,K> {
 readonly #families=new Map<string,IfcIndexedQueryEntity<V>[]>();readonly #roles=new Map<string,RoleIndex<V,K>>();
 private constructor(readonly owner:IfcRelationSource<V>,readonly catalog:IfcCatalog){}
 get source():readonly IfcQueryEntity<V>[]{return this.owner.rows;}
 /** 📇️ Borrow the completed source-order roster; consuming arbitrary-length results requires caller continuation. */
 family(key:string):readonly IfcIndexedQueryEntity<V>[]{const rows=this.#families.get(key);if(!rows)throw new RangeError('Unknown relation family');return rows;}
 /** 🧷️ Locate only an exact original source row, never a same-content replacement. */
 *attributeSteps(entity:IfcQueryEntity<V>,attribute:string,owner:string):IfcCatalogSteps<IfcAttributeQuery<V>>{const indexed=this.owner.indexed(entity),ordinal=indexed?.ordinal;if(!indexed||indexed.entity!==entity||ordinal===undefined||!Number.isSafeInteger(ordinal)||ordinal<0||this.source[ordinal]!==entity)throw new RangeError('Attribute query requires its prepared source row');return yield* ifcAttributeQuerySteps(this.catalog,{entity,ordinal},attribute,owner);}
 /** 🧵️ Merge deferred unresolved evidence with target matches before the domain consumer inspects payloads. */
 *relatedSteps(role:string,target:K):IfcCatalogSteps<readonly IfcRelationHit<V>[]>{
  const index=this.#roles.get(role);if(!index)throw new RangeError('Unknown relation role');const rows=index.targets.get(target)??[],issues=index.issues,result:IfcRelationHit<V>[]=[];let a=0,b=0;
  while(a<rows.length||b<issues.length){yield;if(b===issues.length||a<rows.length&&rows[a]!.row.ordinal<issues[b]!.row.ordinal)result.push(rows[a++]!);else result.push(issues[b++]!);}
  return result;
 }
 /** 🫳️ Preparation keeps every partial index private until the complete source has been traversed. */
 static *prepare<V,K>(catalog:IfcCatalog,source:IfcRelationSource<V>,plan:IfcRelationPlan,reader:IfcRelationValueReader<V,K>):IfcCatalogSteps<IfcPreparedRelations<V,K>>{
  if(!plan.families.length||plan.families.length>32||plan.roles.length>32)throw new RangeError('Relation plan exceeds its closed bounds');
  const prepared=new IfcPreparedRelations<V,K>(source,catalog),families=new Map<string,IfcRelationFamily>(),roles=new Map<string,IfcRelationRole[]>(),names=new Map<string,Map<string,boolean>>();
  for(const family of plan.families){yield;if(!family.key||families.has(family.key)||!(yield* ifcNameSteps(family.type))||!catalog.entities[family.type]||typeof family.subtypes!=='boolean')throw new RangeError('Invalid relation family');families.set(family.key,family);prepared.#families.set(family.key,[]);roles.set(family.key,[]);}
  for(const role of plan.roles){yield;if(!role.key||prepared.#roles.has(role.key)||!families.has(role.family)||!role.attribute||!(yield* ifcNameSteps(role.owner))||!catalog.entities[role.owner]||role.cardinality!=='one'&&role.cardinality!=='many')throw new RangeError('Invalid relation role');roles.get(role.family)!.push(role);prepared.#roles.set(role.key,{targets:new Map(),issues:[]});}
  function* accepts(actual:string,family:IfcRelationFamily):IfcCatalogSteps<boolean>{let cache=names.get(actual);if(cache?.has(family.key)){yield;return cache.get(family.key)!;}const accepted=family.subtypes?yield* ifcSubtypeSteps(catalog,actual,family.type):(yield* ifcNameSteps(actual))===family.type;if(!cache){cache=new Map();names.set(actual,cache);}cache.set(family.key,accepted);yield;return accepted;}
  for(let ordinal=0;ordinal<source.rows.length;ordinal++){
   yield;const entity=source.rows[ordinal]!,row=source.indexed(entity);if(!row||row.entity!==entity||row.ordinal!==ordinal)throw new RangeError('Prepared relation source requires its exact original indexed row');
   for(const family of plan.families){
    let accepted=false;for(let part=0;part<= (entity.complex?.length??0);part++){yield;const name=part?entity.complex![part-1]!.name:entity.name;if(yield* accepts(name,family)){accepted=true;break;}}
    if(!accepted)continue;prepared.#families.get(family.key)!.push(row);yield;
    for(const role of roles.get(family.key)!){
     const query=yield* ifcAttributeQuerySteps(catalog,row,role.attribute,role.owner),index=prepared.#roles.get(role.key)!;
     if(query.status!=='unique'){index.issues.push({row,query,issue:query.status});yield;continue;}
     const value=query.matches[0]!.value,hit:IfcRelationHit<V>={row,query,issue:null};
     if(role.cardinality==='one'){yield;const target=reader.reference(value);if(target!==undefined){yield;let entries=index.targets.get(target);if(!entries){entries=[];index.targets.set(target,entries);}entries.push(hit);}continue;}
     yield;const items=reader.aggregate(value);if(!items){index.issues.push({row,query,issue:'aggregate'});yield;continue;}
     const seen=new Set<K>();for(const item of items){yield;const target=reader.reference(item);if(target===undefined||seen.has(target))continue;seen.add(target);yield;let entries=index.targets.get(target);if(!entries){entries=[];index.targets.set(target,entries);}entries.push(hit);}
    }
   }
  }
  return prepared;
 }
}
/** 🧭️ The same raw continuation supports reference drains and original-controlled host scheduling. */
export function prepareIfcRelationsSteps<V,K>(catalog:IfcCatalog,source:IfcRelationSource<V>,plan:IfcRelationPlan,reader:IfcRelationValueReader<V,K>):IfcCatalogSteps<IfcPreparedRelations<V,K>>{return IfcPreparedRelations.prepare(catalog,source,plan,reader);}
