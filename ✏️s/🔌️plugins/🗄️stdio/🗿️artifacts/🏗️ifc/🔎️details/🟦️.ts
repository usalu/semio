import {checkIfcValueChange,type IfcValueChange} from "../📚️catalog/⚖️domain/🟦️.ts";
import {randomIfcGlobalId} from "../🪪️identity/🟦️.ts";
import {ifcAttributes,ifcSubtype,ifcSubtypeSteps,ifcResolveType,type IfcCatalog,type IfcAttributeDeclaration,type IfcTypeExpression} from "../📚️catalog/🟦️.ts";
import type {IfcSnapshot,IfcEntity,IfcValue} from "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {ifcMutationInverseRows,type IfcMutation} from "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
export type DetailsPath=readonly(string|number)[];
interface Context {entity:IfcEntity;argument:number;attribute:IfcAttributeDeclaration;expression:IfcTypeExpression;value:IfcValue;payload:boolean;rootLength:number}
export type {IfcDetailsControl} from "./control.ts";
import {declarationControl,detailsTypeLeaves,type IfcDetailsControl} from "./control.ts";
import {enumerationContext,type IfcEnumerationContext} from './contextual.ts';
function fail(reason:string):never{throw new Error(reason);}
export interface IfcDetailsWorkCapacity {invertibleItems:number;rows:number;}
/** 🧮️ Price bounded Details batches in the same forward-plus-inverse rows as native publication. */
export function detailsWorkCapacity(catalog:IfcCatalog):IfcDetailsWorkCapacity{
 const attributes=Object.values(catalog.entities).reduce((maximum,entity)=>Math.max(maximum,entity.allAttributes.length),0),invertibleItems=attributes*2+3;
 return {invertibleItems,rows:invertibleItems*2};
}
/** 🧾️ Count primitive forward rows and their declared maximum undo rows. */
export function detailsPublicationRows(mutations:readonly IfcMutation[]):number{return mutations.reduce((rows,mutation)=>rows+1+ifcMutationInverseRows(mutation),0);}
function checkPublicationCapacity(catalog:IfcCatalog,mutations:readonly IfcMutation[]):void{
 const capacity=detailsWorkCapacity(catalog);
 if(mutations.length>capacity.invertibleItems||detailsPublicationRows(mutations)>capacity.rows)fail("IFC edit exceeds its declared publication row capacity");
}
const named=(name:string):IfcTypeExpression=>({kind:"named",name:name.toUpperCase()});
function resolve(catalog:IfcCatalog,expression:IfcTypeExpression):IfcTypeExpression{return ifcResolveType(catalog,expression)??expression;}
function context(snapshot:IfcSnapshot,catalog:IfcCatalog,path:DetailsPath):Context|undefined{
 if(path.length<4||path.length>128||path[0]!=="entities"||typeof path[1]!=="number")return undefined;
 const entity=snapshot.entities[path[1]];if(!entity)return undefined;let name=entity.name,args=entity.args,start=3;
 if(path[2]==="complex"&&typeof path[3]==="number"&&path[4]==="args"){const part=entity.complex?.[path[3]];if(!part)return undefined;name=part.name;args=part.args;start=5;}else if(path[2]!=="args")return undefined;
 const index=path[start];if(typeof index!=="number")return undefined;
 const attributes=(start===5||entity.complex?.length)?catalog.entities[name.toUpperCase()]?.attributes:ifcAttributes(catalog,name);let attribute=attributes?.[index];if(!attribute)return undefined;
 if(entity.complex?.length&&[entity.name,...entity.complex.map(part=>part.name)].some(name=>catalog.entities[name.toUpperCase()]?.allAttributes.some(candidate=>candidate.name===attribute!.name&&candidate.declaredBy===attribute!.declaredBy&&candidate.derived)))attribute={...attribute,derived:true};
 let expression=attribute.typeExpr,value=args[index],cursor=start+1,payload=false;if(!value)return undefined;
 while(cursor<path.length){if((path[cursor]==="reference"||path[cursor]==="appendReference")&&cursor===path.length-1)break;if(path[cursor++]!=="value")return undefined;payload=true;if(cursor===path.length)break;
  if(value.kind==="aggregate"){const index=path[cursor++];if(typeof index!=="number")return undefined;value=value.value[index];const aggregate=resolve(catalog,expression);if(!aggregate.item||!value)return undefined;expression={...aggregate.item,optional:Boolean(aggregate.item.optional||aggregate.optional)};payload=false;}
  else if(value.kind==="typedValue"&&path[cursor++]==="items"){expression=named(value.value.name);if(cursor===path.length)break;const index=path[cursor++];if(typeof index!=="number")return undefined;value=value.value.items[index];if(!value)return undefined;payload=false;}else return undefined;
 }
 return {entity,argument:index,attribute,expression,value,payload,rootLength:start+1};
}
const leaves=detailsTypeLeaves;
function hasEntity(catalog:IfcCatalog,expression:IfcTypeExpression):boolean{return [...leaves(catalog,expression)].some(name=>Boolean(catalog.entities[name]));}
function accepts(catalog:IfcCatalog,expression:IfcTypeExpression,name:string):boolean{return [...leaves(catalog,expression)].some(expected=>ifcSubtype(catalog,name,expected));}
/** 🧭️ Describe a real IFC value with its selected schema's inherited declaration and control domain. */
export function detailsControl(snapshot:IfcSnapshot,catalog:IfcCatalog,path:DetailsPath,contextual?:IfcEnumerationContext):IfcDetailsControl|undefined{
 const found=context(snapshot,catalog,path);if(!found)return undefined;const control=declarationControl(catalog,found.attribute,found.expression,found.value.kind==="aggregate"?found.value.value.length:0,Boolean(found.expression.optional||found.attribute.optional&&path.length<=found.rootLength+Number(found.payload)));if(contextual){const choices=enumerationContext(snapshot,path,contextual);if(choices)control.contextualChoices=choices;}return control;
}
/** 🔗️ Parse the full positive u64 domain without binary64 truncation. */
export function parseReferenceId(decimal:string):bigint{if(typeof decimal!=="string"||!/^[0-9]{1,20}$/.test(decimal))return fail("reference-id");const id=BigInt(decimal);return id>0n&&id<=18446744073709551615n?id:fail("reference-id");}
function cloneValue(value:IfcValue,tick:()=>void,depth=0):IfcValue{
 tick();if(depth>128)return fail("details-value-depth");if(value.kind==="aggregate")return {kind:"aggregate",value:value.value.map(item=>cloneValue(item,tick,depth+1))};if(value.kind==="typedValue")return {kind:"typedValue",value:{name:value.value.name,items:value.value.items.map(item=>cloneValue(item,tick,depth+1))}};return {...value};
}
function replace(root:IfcValue,path:DetailsPath,value:unknown):IfcValue{if(!path.length)return value as IfcValue;let target:unknown=root;for(const key of path.slice(0,-1))target=(target as Record<string|number,unknown>)[key];(target as Record<string|number,unknown>)[path[path.length-1]]=value;return root;}
/** ✍️ Plan exact declared reference edits through the existing stable-id argument mutation. */
export function referenceMutation(snapshot:IfcSnapshot,catalog:IfcCatalog,path:DetailsPath,decimal:string,tick:()=>void=()=>{}):Extract<IfcMutation,{mutation:"setEntityArg"|"setEntityComplex"}>{
 tick();const found=context(snapshot,catalog,path)??fail("reference-path"),last=path[path.length-1],append=last==="appendReference",virtual=last==="reference";if(found.attribute.derived)fail("reference-derived");
 const domain=resolve(catalog,found.expression),expression=(append?domain.item:found.expression)??fail("reference-domain");if(!hasEntity(catalog,expression))fail("reference-domain");
 if(append&&(found.value.kind!=="aggregate"||domain.aggregate==="ARRAY"||domain.upper!==undefined&&domain.upper!==null&&found.value.value.length>=domain.upper))fail("reference-capacity");
 const id=virtual&&decimal===""?undefined:parseReferenceId(decimal);
 if(id===undefined){if(!(found.expression.optional||found.attribute.optional&&path.length===found.rootLength+1))fail("reference-required");}
 else {let target:IfcEntity|undefined;for(const entity of snapshot.entities){tick();if(entity.id===id){target=entity;break;}}const record=target??fail("reference-target");let admitted=false;for(const name of [record.name,...(record.complex??[]).map(part=>part.name)]){tick();if(accepts(catalog,expression,name)){admitted=true;break;}}if(!admitted)fail("reference-target");}

 const part=path[2]==="complex"?Number(path[3]):undefined,args=part===undefined?found.entity.args:found.entity.complex?.[part]?.args??fail("reference-path");
 const argument=cloneValue(args[found.argument],tick),relative=path.slice(found.rootLength);
 let value:IfcValue;if(append){let target:unknown=argument;for(const key of relative.slice(0,-1))target=(target as Record<string|number,unknown>)[key];const aggregate=target as Extract<IfcValue,{kind:"aggregate"}>;aggregate.value.push({kind:"reference",value:id!});value=argument;}
 else if(virtual)value=replace(argument,relative.slice(0,-1),id===undefined?{kind:"unset"}:{kind:"reference",value:id});
 else {if(!found.payload||found.value.kind!=="reference"||id===undefined)fail("reference-path");value=replace(argument,relative,id);}
 const normalized=append?[...path.slice(0,-1),"value",found.value.kind==="aggregate"?found.value.value.length:0]:virtual?path.slice(0,-1):path;
 return declaredMutation(snapshot,catalog,normalized,value,append?"insert":"set",tick);
}

function sameValue(a:IfcValue,b:IfcValue,tick:()=>void,depth=0):boolean{
 tick();if(depth>128)fail("details-value-depth");if(a.kind!==b.kind)return false;if(a.kind==="real"&&b.kind==="real")return a.value.bits===b.value.bits;if(a.kind==="aggregate"&&b.kind==="aggregate")return a.value.length===b.value.length&&a.value.every((item,index)=>sameValue(item,b.value[index],tick,depth+1));if(a.kind==="typedValue"&&b.kind==="typedValue")return a.value.name===b.value.name&&a.value.items.length===b.value.items.length&&a.value.items.every((item,index)=>sameValue(item,b.value.items[index],tick,depth+1));return !("value"in a)&&!("value"in b)||"value"in a&&"value"in b&&Object.is(a.value,b.value);
}
function template(snapshot:IfcSnapshot,catalog:IfcCatalog,expression:IfcTypeExpression,tick:()=>void,depth=0):IfcValue|undefined{
 tick();if(depth>=64)return undefined;const domain=resolve(catalog,expression),name=domain.name?.toUpperCase();
 if(domain.aggregate){const count=domain.aggregate==="ARRAY"&&domain.upper!==null&&domain.upper!==undefined&&domain.lower!==null&&domain.lower!==undefined?domain.upper-domain.lower+1:domain.lower??0;if(count<0||count>256||domain.unique&&count>1||!domain.item)return undefined;const value=template(snapshot,catalog,domain.item,tick,depth+1);return value?{kind:"aggregate",value:Array.from({length:count},()=>cloneValue(value,tick))}:undefined;}
 if(!name)return undefined;if(catalog.entities[name]){for(const entity of snapshot.entities.slice(0,256)){tick();if(accepts(catalog,expression,entity.name))return {kind:"reference",value:entity.id};}return undefined;}
 const declaration=catalog.types[name];if(declaration?.kind==="enum")return declaration.members[0]?{kind:"enum",value:declaration.members[0]}:undefined;if(declaration?.kind==="select"){for(const member of declaration.members){const value=template(snapshot,catalog,named(member),tick,depth+1);if(value)return value.kind==="reference"||value.kind==="typedValue"?value:{kind:"typedValue",value:{name:member,items:[value]}};}return undefined;}
 switch(name){case "STRING":return {kind:"string",value:""};case "BINARY":return {kind:"binary",value:"0"};case "INTEGER":return {kind:"integer",value:0n};case "REAL":case "NUMBER":return {kind:"real",value:{bits:0n}};case "BOOLEAN":case "LOGICAL":return {kind:"enum",value:"F"};default:return undefined;}
}
function freshGlobalId(snapshot:IfcSnapshot,tick:()=>void):string{
 const occupied=new Set<string>();for(const entity of snapshot.entities){tick();for(const args of [entity.args,...(entity.complex??[]).map(part=>part.args)]){tick();if(args[0]?.kind==="string")occupied.add(args[0].value);}}
 for(;;){tick();const result=randomIfcGlobalId();if(!occupied.has(result))return result;}
}
/** 🔀️ Plan a catalog-declared entity variant as one ordered batch for the existing diff and inverse authority. */
export function typeSwitchMutations(snapshot:IfcSnapshot,catalog:IfcCatalog,id:bigint,target:string,tick:()=>void=()=>{}):IfcMutation[]{
 tick();let source:IfcEntity|undefined;for(const entity of snapshot.entities){tick();if(entity.id===id){source=entity;break;}}const current=source??fail("details-entity-missing"),name=target.toUpperCase(),declaration=catalog.entities[name]??fail("details-type-unknown"),partial=Boolean(current.complex?.length);if(declaration.abstractEntity&&!partial)fail("details-type-abstract");const previous=catalog.entities[current.name.toUpperCase()]??fail("details-type-unknown"),attributes=partial?declaration.attributes:declaration.allAttributes,old=partial?previous.attributes:previous.allAttributes,derived=new Set<string>();
 for(const type of [name,...(current.complex??[]).map(part=>part.name)]){tick();for(const attribute of catalog.entities[type]?.allAttributes??[])if(attribute.derived)derived.add(attribute.declaredBy+"."+attribute.name);}
 const make=(attribute:IfcAttributeDeclaration,value:IfcValue|undefined):IfcValue=>{tick();if(attribute.derived||derived.has(attribute.declaredBy+"."+attribute.name))return {kind:"derived"};if(value&&value.kind!=="derived"&&(value.kind!=="unset"||attribute.optional))return cloneValue(value,tick);if(attribute.optional)return {kind:"unset"};if(attribute.name==="GlobalId")return {kind:"string",value:freshGlobalId(snapshot,tick)};return template(snapshot,catalog,attribute.typeExpr,tick)??{kind:"unset"};};
 const args=attributes.map(attribute=>{const index=old.findIndex(candidate=>candidate.name===attribute.name&&JSON.stringify(candidate.typeExpr)===JSON.stringify(attribute.typeExpr));return make(attribute,index<0?undefined:current.args[index]);}),mutations:IfcMutation[]=[];if(name!==current.name)mutations.push({mutation:"setEntityName",id,name});for(let index=current.args.length-1;index>=args.length;index--){tick();mutations.push({mutation:"removeEntityArg",id,index});}
 for(const [index,value]of args.entries()){tick();if(index>=current.args.length)mutations.push({mutation:"insertEntityArg",id,index,value});else if(!sameValue(value,current.args[index],tick))mutations.push({mutation:"setEntityArg",id,index,value});}
 let changed=false;const complex=(current.complex??[]).map(part=>{tick();const attributes=catalog.entities[part.name.toUpperCase()]?.attributes;return {name:part.name,args:part.args.map((value,index)=>{const attribute=attributes?.[index],next=attribute?make(attribute,value):cloneValue(value,tick);if(!sameValue(next,value,tick))changed=true;return next;})};});if(changed)mutations.push({mutation:"setEntityComplex",id,complex});checkPublicationCapacity(catalog,mutations);return mutations;
}

export type IfcDeclaredValueEdit={kind:"set";path:DetailsPath;value:unknown}|{kind:"insert";path:DetailsPath;value:unknown}|{kind:"remove";path:DetailsPath}|{kind:"move";path:DetailsPath;from:DetailsPath};
function* referenceLookup(snapshot:IfcSnapshot,catalog:IfcCatalog,id:bigint,expected:string):Generator<void,boolean,void>{let count=0,accepted=false;for(const entity of snapshot.entities){yield;if(entity.id===id){count++;yield;accepted ||= yield* ifcSubtypeSteps(catalog,entity.name,expected);for(const part of entity.complex??[]){yield;accepted ||= yield* ifcSubtypeSteps(catalog,part.name,expected);}}}return count===1&&accepted;}
function valueRoute(before:IfcValue,path:DetailsPath):number[]{const route:number[]=[];let value=before,cursor=0;while(cursor<path.length){if(path[cursor++]!=="value")fail("details-value-path");if(cursor===path.length)break;let index:number;if(value.kind==="aggregate"){const segment=path[cursor++];index=segment==="-"?value.value.length:Number(segment);}else if(value.kind==="typedValue"){const field=path[cursor++];if(field==="name"&&cursor===path.length)break;if(field!=="items")fail("details-value-path");if(cursor===path.length)break;index=Number(path[cursor++]);}else return fail("details-value-path");if(!Number.isSafeInteger(index)||index<0||route.length===128)fail("details-value-path");route.push(index);if(cursor===path.length)break;value=(value.kind==="aggregate"?value.value:value.kind==="typedValue"?value.value.items:[])[index]??fail("details-value-path");}return route;}
function declaredMutation(snapshot:IfcSnapshot,catalog:IfcCatalog,path:DetailsPath,value:IfcValue,operation:IfcValueChange,tick:()=>void):Extract<IfcMutation,{mutation:"setEntityArg"|"setEntityComplex"}>{
 const complex=path[2]==="complex",length=complex?6:4,root=path.slice(0,length),found=context(snapshot,catalog,root)??fail("details-value-path");if(found.attribute.derived)fail("details-value-derived");checkIfcValueChange(catalog,found.attribute.typeExpr,found.value,value,valueRoute(found.value,path.slice(length)),operation,{optional:found.attribute.optional,tick,reference:(id,expected)=>referenceLookup(snapshot,catalog,id,expected)});
 if(complex){const part=Number(path[3]);return {mutation:"setEntityComplex",id:found.entity.id,complex:found.entity.complex!.map((record,index)=>({name:record.name,args:record.args.map((current,argument)=>index===part&&argument===found.argument?value:cloneValue(current,tick))}))};}return {mutation:"setEntityArg",id:found.entity.id,index:found.argument,value};
}
/** 🛂️ Plan one typed argument edit through the existing stable primitive and declaration authority. */
export function declaredValueMutation(snapshot:IfcSnapshot,catalog:IfcCatalog,event:IfcDeclaredValueEdit,tick:()=>void=()=>{}):Extract<IfcMutation,{mutation:"setEntityArg"|"setEntityComplex"}>{
 tick();const length=event.path[2]==="complex"?6:4,found=context(snapshot,catalog,event.path.slice(0,length))??fail("details-value-path");let value=cloneValue(found.value,tick);const relative=event.path.slice(length);
 if(event.kind==="set")value=replace(value,relative,event.value);
 else {if(!relative.length)fail("details-value-path");const parentPath=relative.slice(0,-1);let target:unknown=value;for(const key of parentPath)target=(target as Record<string|number,unknown>)[key];if(!Array.isArray(target))fail("details-value-path");const index=relative.at(-1)==="-"?target.length:Number(relative.at(-1));if(!Number.isSafeInteger(index)||index<0||index>target.length)fail("details-value-path");if(event.kind==="insert")target.splice(index,0,event.value);else if(event.kind==="remove"){if(index>=target.length)fail("details-value-path");target.splice(index,1);}else{const origin=event.from.slice(length);if(origin.length!==relative.length||origin.slice(0,-1).some((key,index)=>key!==parentPath[index]))fail("details-value-path");const from=Number(origin.at(-1)),to=index===target.length?target.length-1:index;if(!Number.isSafeInteger(from)||from<0||from>=target.length||to>=target.length)fail("details-value-path");target.splice(to,0,target.splice(from,1)[0]);}}
 return declaredMutation(snapshot,catalog,event.path,value,event.kind,tick);
}
