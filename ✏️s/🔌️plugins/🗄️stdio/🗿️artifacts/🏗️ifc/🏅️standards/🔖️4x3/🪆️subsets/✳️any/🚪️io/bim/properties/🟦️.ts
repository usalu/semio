/** 🏷️ Exact dimensionless occurrence values share original source controls and declared-owner queries. */
import type {IfcEntity,IfcValue} from '../../../🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcGeometryGraph,IfcExchangeDiagnostic} from '../../geometry.ts';
import type {IfcExchangeRelations} from '../../relations.ts';
import {ifcReal,ifcNumber} from '../../step.ts';
import {IfcStepError,type IfcIoSteps} from '../../control.ts';
import type {PropertySet,PropertyValue} from '../../../../../../../../../../🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts';
import {ifc4x3Catalog} from '../../../🧬️schema/📚️catalog/🟦️.ts';
import {ifcSubtypeSteps} from '../../../../../../../📚️catalog/🟦️.ts';
const unset:IfcValue={kind:'unset'},text=(value:string):IfcValue=>({kind:'string',value}),aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),reserved=new Set(['semio_BimStorey','semio_BimProject']);
const invalid=(message:string):never=>{throw new IfcStepError('bim.occurrence-property',message);};
const record=(value:unknown):Record<string,unknown>=>{if(!value||typeof value!=='object'||Array.isArray(value))return invalid('Occurrence property maps require records');return value as Record<string,unknown>;};
function* textSteps(value:string,type:string):IfcIoSteps<string>{
 const width=ifc4x3Catalog.types[type]!.typeExpr?.width;let count=0;
 for(const scalar of value){yield;const code=scalar.charCodeAt(0);if(scalar.length===1&&code>=0xd800&&code<=0xdfff)return invalid('Occurrence text requires Unicode scalar values');if(width!=null&&++count>width)return invalid('Occurrence text exceeds its declared IFC string width');}
 return value;
}
function* nameSteps(value:IfcValue|undefined,type:string):IfcIoSteps<string>{if(value?.kind!=='string'||!value.value.length)return invalid('Explicit nonempty occurrence set and property names required');return yield* textSteps(value.value,type);}
function* nominalSteps(value:unknown):IfcIoSteps<IfcValue>{
 const row=record(value);let kind:string|undefined;
 for(const key in row){yield;if(Object.hasOwn(row,key)){if(kind!==undefined)return invalid('Exactly one declared BIM property variant required');kind=key;}}
 if(!kind)return invalid('Explicit BIM property variant required');
 const scalar=record(row[kind]);let fields=0;for(const key in scalar){yield;if(Object.hasOwn(scalar,key)){if(key!=='value')return invalid('Closed BIM property scalar required');fields++;}}
 if(fields!==1)return invalid('One explicit BIM property scalar required');
 const valueScalar=scalar.value;let type:string,item:IfcValue;
 if(kind==='Text'&&typeof valueScalar==='string'){type='IFCTEXT';item=text(yield* textSteps(valueScalar,type));}
 else if(kind==='Real'&&typeof valueScalar==='number'&&Number.isFinite(valueScalar)){type='IFCREAL';item=ifcReal(valueScalar);}
 else if(kind==='Integer'&&typeof valueScalar==='number'&&Number.isInteger(valueScalar)&&valueScalar>=-2147483648&&valueScalar<=2147483647){type='IFCINTEGER';item={kind:'integer',value:BigInt(valueScalar)};}
 else if(kind==='Boolean'&&typeof valueScalar==='boolean'){type='IFCBOOLEAN';item={kind:'enum',value:valueScalar?'T':'F'};}
 else return invalid('Only Text, finite Real, canonical signed32-bit Integer and Boolean dimensionless BIM properties are supported');
 yield;return{kind:'typedValue',value:{name:type,items:[item]}};
}
interface Builder {add(name:string,args:IfcValue[]):IfcValue;root(name:string):IfcValue[];}
function* occurrenceObjectSteps(entity:IfcEntity):IfcIoSteps<void>{
 let object=false,type=false;
 for(let part=0;part<= (entity.complex?.length??0);part++){yield;const name=part?entity.complex![part-1]!.name:entity.name;if(!ifc4x3Catalog.entities[name])return invalid('Unknown occurrence relation target constituent');object=(yield* ifcSubtypeSteps(ifc4x3Catalog,name,'IFCOBJECTDEFINITION'))||object;type=(yield* ifcSubtypeSteps(ifc4x3Catalog,name,'IFCTYPEOBJECT'))||type;}
 if(!object||type)return invalid('Occurrence property assignments require non-type IfcObjectDefinition targets');
}
/** 📤 Empty or unsupported authored records refuse before any partial artifact is returned. */
export function* appendBimOccurrencePropertiesSteps(builder:Builder,properties:unknown,holders:ReadonlyMap<string,IfcValue|null>):IfcIoSteps<void>{
 if(properties===undefined)return;
 const entries=record(properties);
 for(const holder in entries){
  yield;if(!Object.hasOwn(entries,holder))continue;const target=holders.get(holder);if(!target)return invalid('Occurrence properties require one supported spatial holder');
  let sets=0;const source=record(entries[holder]);
  for(const setName in source){
   yield;if(!Object.hasOwn(source,setName))continue;if(!setName||reserved.has(setName))return invalid('Nonempty unreserved occurrence set name required');yield* textSteps(setName,'IFCLABEL');const values=record(source[setName]),items:IfcValue[]=[];
   for(const property in values){yield;if(!Object.hasOwn(values,property))continue;if(!property)return invalid('Nonempty occurrence property name required');yield* textSteps(property,'IFCIDENTIFIER');const nominal=yield* nominalSteps(values[property]);yield;items.push(builder.add('IFCPROPERTYSINGLEVALUE',[text(property),unset,nominal,unset]));}
   if(!items.length)return invalid('An authored occurrence property set must not be empty');
   yield;const set=builder.add('IFCPROPERTYSET',[...builder.root(setName),aggregate(items)]);yield;builder.add('IFCRELDEFINESBYPROPERTIES',[...builder.root(''),aggregate([target]),set]);sets++;
  }
  if(!sets)return invalid('An authored occurrence holder property map must not be empty');
 }
}
function* readNominalSteps(value:IfcValue|undefined,unit:IfcValue|undefined,diagnostics:IfcExchangeDiagnostic[],id:bigint):IfcIoSteps<PropertyValue>{
 if(unit?.kind!=='unset'||value?.kind!=='typedValue'||value.value.items.length!==1)return invalid('One typed dimensionless nominal value and unset Unit required');
 const item=value.value.items[0]!,type=value.value.name;
 if(['IFCTEXT','IFCLABEL','IFCIDENTIFIER'].includes(type)&&item.kind==='string'){
  yield* textSteps(item.value,type);
  if(type!=='IFCTEXT')diagnostics.push({code:'bim.property-string-type',severity:'warning',message:`${type} occurrence nominal projects to BIM Text; its IFC nominal subtype is retained only in the source graph`,entityIds:[id]});
  return{Text:{value:item.value}};
 }
 if(type==='IFCREAL'&&item.kind==='real'){const number=ifcNumber(item);if(Number.isFinite(number))return{Real:{value:number}};}
 if(type==='IFCINTEGER'&&item.kind==='integer'&&item.value>=-2147483648n&&item.value<=2147483647n)return{Integer:{value:Number(item.value)}};
 if(type==='IFCBOOLEAN'&&item.kind==='enum'&&['T','F'].includes(item.value))return{Boolean:{value:item.value==='T'}};
 return invalid('Unsupported or inexact occurrence nominal type/value');
}
/** 📥 Exact source identities deduplicate repeated assignments; authored names never overwrite collisions. */
export function* readBimOccurrencePropertiesSteps(graph:IfcGeometryGraph,relations:IfcExchangeRelations,holder:IfcEntity,diagnostics:IfcExchangeDiagnostic[]):IfcIoSteps<PropertySet|undefined>{
 const rows=yield* relations.relatedSteps('propertyRelations',holder.id),sets:PropertySet=Object.create(null),seen=new Set<bigint>();let count=0;
 for(const row of rows){
  yield;const set=graph.ref(yield* relations.valueSteps(row,'RelatingPropertyDefinition','IFCRELDEFINESBYPROPERTIES'));if(set.name!=='IFCPROPERTYSET')return invalid('Only explicit occurrence IfcPropertySet definitions are supported');
  const setName=yield* nameSteps(yield* relations.valueSteps(set,'Name','IFCROOT'),'IFCLABEL');
  if(reserved.has(setName)){if(setName==='semio_BimStorey'&&holder.name==='IFCBUILDINGSTOREY')continue;return invalid('Reserved authored convention assigned to the wrong holder');}
  if(seen.has(set.id))continue;seen.add(set.id);if(Object.hasOwn(sets,setName))return invalid('Distinct same-name occurrence property sets cannot be represented');
  const properties=yield* relations.valueSteps(set,'HasProperties','IFCPROPERTYSET');if(properties?.kind!=='aggregate'||!properties.value.length)return invalid('Nonempty occurrence property set required');
  const values:Record<string,PropertyValue>=Object.create(null);
  for(const reference of properties.value){
   yield;const property=graph.ref(reference);if(property.name!=='IFCPROPERTYSINGLEVALUE')return invalid('Only occurrence IfcPropertySingleValue is supported');const propertyName=yield* nameSteps(yield* relations.valueSteps(property,'Name','IFCPROPERTY'),'IFCIDENTIFIER');if(Object.hasOwn(values,propertyName))return invalid('Distinct occurrence property names required');
   const nominal=yield* relations.valueSteps(property,'NominalValue','IFCPROPERTYSINGLEVALUE'),unit=yield* relations.valueSteps(property,'Unit','IFCPROPERTYSINGLEVALUE');yield;values[propertyName]=yield* readNominalSteps(nominal,unit,diagnostics,property.id);
  }
  yield;sets[setName]=values;count++;
  const definitions=yield* relations.relatedSteps('propertyDefinitions',set.id),targets=new Set<bigint>();
  for(const definition of definitions){const related=yield* relations.valueSteps(definition,'RelatedObjects','IFCRELDEFINESBYPROPERTIES');if(related?.kind!=='aggregate'||!related.value.length)return invalid('Nonempty occurrence relation objects required');const unique=new Set<bigint>();for(const target of related.value){yield;const actual=graph.ref(target);yield* occurrenceObjectSteps(actual);if(unique.has(actual.id))return invalid('Distinct occurrence relation objects required');unique.add(actual.id);targets.add(actual.id);}}
  if(targets.size>1)diagnostics.push({code:'bim.property-identity',severity:'warning',message:'Shared IFC property-set identity projects to separate authored occurrence maps; original identity remains in the source graph',entityIds:[set.id]});
 }
 const inherited=yield* relations.relatedSteps('typeRelations',holder.id);if(inherited.length)diagnostics.push({code:'bim.property-inheritance',severity:'warning',message:'IFC type assignments and inherited properties are not flattened into authored occurrence values',entityIds:[holder.id]});
 return count?sets:undefined;
}
