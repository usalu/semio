/** 🧾️ One structural type matcher borrows original values, current declarations and caller-owned source lookup. */
import{ifc4x3Catalog,ifcIsSubtype,type IfcTypeExpression}from'../📚️catalog/🟦️.ts';
import type{IfcValue}from'../📸️snapshot/🟦️.ts';
export type IfcDomainTypeSteps=Generator<number,boolean,void>;
function* memberAccepts(member:string,actual:string,depth:number):IfcDomainTypeSteps{
 yield depth;if(member===actual||ifcIsSubtype(actual,member))return true;const type=ifc4x3Catalog.types[member];if(type?.kind==='select')for(const nested of type.members)if(yield* memberAccepts(nested,actual,depth+1))return true;return false;
}
/** 🔢️ Yield each original structural depth; the consuming driver owns cancellation, work and maximum-depth checks. */
export function* ifcDomainTypeMatchesSteps(value:IfcValue,expression:IfcTypeExpression,entityName:(id:bigint)=>string|undefined,depth=0):IfcDomainTypeSteps{
 yield depth;
 if(expression.kind==='aggregate'){if(value.kind!=='aggregate')return false;for(const item of value.value)if(!(item.kind==='unset'&&expression.optional)&&!(yield* ifcDomainTypeMatchesSteps(item,expression.item!,entityName,depth+1)))return false;return true;}
 const name=expression.name!,type=ifc4x3Catalog.types[name];
 if(ifc4x3Catalog.entities[name]){if(value.kind!=='reference')return false;const actual=entityName(value.value);return actual===undefined||ifcIsSubtype(actual,name);}
 if(type?.kind==='enum')return value.kind==='enum'&&type.members.includes(value.value.toUpperCase());
 if(type?.kind==='select'){
  if(value.kind==='reference'){const actual=entityName(value.value);if(actual===undefined)return true;for(const member of type.members)if(yield* memberAccepts(member,actual.toUpperCase(),0))return true;return false;}
  if(value.kind!=='typedValue'||value.value.items.length!==1)return false;let accepted=false;for(const member of type.members)if(yield* memberAccepts(member,value.value.name.toUpperCase(),0)){accepted=true;break;}
  return accepted&&(yield* ifcDomainTypeMatchesSteps(value.value.items[0]!,{kind:'named',name:value.value.name.toUpperCase()},entityName,depth+1));
 }
 if(value.kind==='typedValue'&&type&&value.value.name.toUpperCase()===name)return value.value.items.length===1&&!!type.typeExpr&&(yield* ifcDomainTypeMatchesSteps(value.value.items[0]!,type.typeExpr,entityName,depth+1));
 if(type?.typeExpr)return yield* ifcDomainTypeMatchesSteps(value,type.typeExpr,entityName,depth+1);
 if(name==='STRING'){if(value.kind!=='string')return false;if(expression.width==null)return true;let count=0;for(const character of value.value){yield 0;if(++count>expression.width)return false;}return !expression.fixed||count===expression.width;}
 if(name==='INTEGER')return value.kind==='integer';if(name==='REAL')return value.kind==='real';if(name==='NUMBER')return value.kind==='integer'||value.kind==='real';
 if(name==='BOOLEAN'||name==='LOGICAL')return value.kind==='enum'&&(name==='BOOLEAN'?['T','F']:['T','F','U']).includes(value.value.toUpperCase());
 if(name==='BINARY'){if(value.kind!=='binary'||!value.value.length||!/^[0-3]$/.test(value.value[0]!))return false;const unused=Number(value.value[0]);if(value.value.length===1&&unused!==0||value.value.length>1&&parseInt(value.value[1]!,16)>=2**(4-unused))return false;for(let at=1;at<value.value.length;at+=4096){yield 0;if(!/^[0-9A-F]+$/.test(value.value.slice(at,at+4096)))return false;}const length=(value.value.length-1)*4-unused;return expression.width==null||(expression.fixed?length===expression.width:length<=expression.width);}
 return false;
}
