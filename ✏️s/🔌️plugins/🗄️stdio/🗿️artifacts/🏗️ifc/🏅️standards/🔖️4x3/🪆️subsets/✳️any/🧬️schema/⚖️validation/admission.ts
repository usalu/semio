import {ifc4x3Catalog} from '../📚️catalog/🟦️.ts';
import type {IfcDiagnostic,IfcTypeExpression} from '../📚️catalog/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../📸️snapshot/🟦️.ts';
import type {ExpressControl} from './ast.ts';
import {ExpressEvaluator,ExpressEvaluationError,expressAggregate} from './evaluator.ts';
import type {ExpressValue} from './evaluator.ts';
import {captureExpressControlScope} from './control.ts';
import {ifcDomainTypeMatchesSteps} from './type-matching.ts';

/** 🛂 Controlled structural admission over an owned, shape-admitted graph. */
export async function validateIfc4x3Domains(snapshot:IfcSnapshot,control:ExpressControl={}):Promise<IfcDiagnostic[]>{
 const scope=captureExpressControlScope(control);control=scope.control;scope.check();
 const evaluator=new ExpressEvaluator(undefined,{...control,progress:row=>control.progress?.({...row,phase:'structure'})}),diagnostics:IfcDiagnostic[]=[];
 const entities=new Map<bigint,IfcSnapshot['entities'][number]>(),maximumDepth=control.maximumDepth??64;
 const tick=async(depth=0):Promise<void>=>{await evaluator.tick();if(depth>maximumDepth)throw new ExpressEvaluationError('depth','Structural type traversal exceeds depth limit');};
 const report=(code:string,message:string,entityId?:bigint,path:number[]=[])=>diagnostics.push({code,message,entityId,path});
 const schema=snapshot.header.fileSchema;await tick();
 if(schema.length!==1||schema[0]?.kind!=='aggregate'||schema[0].value.length!==1||schema[0].value[0]?.kind!=='string'||schema[0].value[0].value.toUpperCase()!==ifc4x3Catalog.schema)report('file-schema','FILE_SCHEMA must identify IFC4X3_ADD2.');
 for(const entity of snapshot.entities){await tick();if(entity.id<=0n||entity.id>18446744073709551615n)report('entity-id','Entity identifiers must be positive unsigned 64-bit integers.',entity.id);if(entities.has(entity.id))report('duplicate-id','Entity identifiers must be unique.',entity.id);entities.set(entity.id,entity);}
 const matches=async(value:IfcValue,expression:IfcTypeExpression,depth=0):Promise<boolean>=>{
  const steps=ifcDomainTypeMatchesSteps(value,expression,id=>entities.get(id)?.name,depth);try{for(;;){const step=steps.next();if(step.done)return step.value;await tick(step.value);}}finally{steps.return(false);}
 };
 const comparisonValue=async(value:IfcValue,expression:IfcTypeExpression,depth=0):Promise<ExpressValue>=>{
  await tick(depth);if(value.kind==='unset'||value.kind==='derived')return{kind:'unknown'};
  if(value.kind==='reference')return{kind:'entity',identity:value.value,types:[entities.get(value.value)?.name.toUpperCase()??'']};
  if(value.kind==='typedValue')return comparisonValue(value.value.items[0]!,{kind:'named',name:value.value.name.toUpperCase()},depth+1);
  if(value.kind==='aggregate'){let type=expression;while(type.kind==='named'&&ifc4x3Catalog.types[type.name!]?.typeExpr){await tick(++depth);type=ifc4x3Catalog.types[type.name!]!.typeExpr!;}const converted:ExpressValue[]=[];for(const item of value.value)converted.push(await comparisonValue(item,type.item??{kind:'named',name:'GENERIC'},depth+1));return expressAggregate(converted,type.aggregate,type.aggregate==='ARRAY'?BigInt(type.lower??1):1n);}
  if(value.kind==='real'){const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,value.value.bits,false);return{kind:'real',value:view.getFloat64(0,false)};}
  if(value.kind==='enum')return{kind:'enum',value:value.value.toUpperCase()};
  return{kind:value.kind,value:value.value};
 };
 const check=async(value:IfcValue,expression:IfcTypeExpression,entityId:bigint,path:number[],depth=0):Promise<void>=>{
  await tick(depth);const type=expression.kind==='named'?ifc4x3Catalog.types[expression.name!]:undefined;
  if(type?.kind==='defined'){await check(value.kind==='typedValue'?value.value.items[0]!:value,type.typeExpr!,entityId,path,depth+1);return;}
  if(type?.kind==='select'&&value.kind==='typedValue'){await check(value.value.items[0]!,{kind:'named',name:value.value.name.toUpperCase()},entityId,path,depth+1);return;}
  if(expression.kind!=='aggregate'||value.kind!=='aggregate')return;
  const {lower,upper}=expression,size=value.value.length,bounded=expression.aggregate==='ARRAY'?lower==null||upper==null||size===upper-lower+1:(lower==null||size>=lower)&&(upper==null||size<=upper);
  if(!bounded)report('aggregate-bounds','Aggregate cardinality violates its EXPRESS bounds.',entityId,path);
  if(expression.unique||expression.aggregate==='SET'){const previous:ExpressValue[]=[];let duplicate=false;for(const item of value.value){const converted=await comparisonValue(item,expression.item!);for(const earlier of previous)if((await evaluator.equal(converted,earlier,true)).value==='true'){duplicate=true;break;}if(duplicate)break;previous.push(converted);}if(duplicate)report('aggregate-unique','Aggregate instances must be unique.',entityId,path);}
  for(let index=0;index<value.value.length;index++)await check(value.value[index]!,expression.item!,entityId,[...path,index],depth+1);
 };
 const references=async(value:IfcValue,entityId:bigint,path:number[],depth=0):Promise<void>=>{await tick();if(depth>maximumDepth){report('value-depth','Value nesting exceeds the structural limit.',entityId,path);return;}if(value.kind==='reference'&&!entities.has(value.value))report('dangling-reference',`Reference #${value.value} has no target.`,entityId,path);const children=value.kind==='aggregate'?value.value:value.kind==='typedValue'?value.value.items:[];for(let index=0;index<children.length;index++)await references(children[index]!,entityId,[...path,index],depth+1);};
 for(const entity of snapshot.entities){await tick();for(let index=0;index<entity.args.length;index++)await references(entity.args[index]!,entity.id,[index]);const declaration=ifc4x3Catalog.entities[entity.name.toUpperCase()];if(!declaration){report('unknown-entity',`Unknown IFC4X3_ADD2 entity ${entity.name}.`,entity.id);continue;}if(entity.complex?.length){report('complex-instance','Structural admission requires normalized complex constituents.',entity.id);continue;}if(declaration.abstractEntity)report('abstract-entity',`${declaration.name} is abstract.`,entity.id);if(entity.args.length!==declaration.allAttributes.length)report('attribute-count',`Expected ${declaration.allAttributes.length} positional attributes.`,entity.id);
  for(let index=0;index<declaration.allAttributes.length;index++){await tick();const attribute=declaration.allAttributes[index]!,value=entity.args[index];if(!value)continue;if(attribute.derived){if(value.kind!=='derived')report('derived-attribute',`${attribute.name} requires a derived marker.`,entity.id,[index]);continue;}if(value.kind==='unset'){if(!attribute.optional)report('required-attribute',`${attribute.name} is required.`,entity.id,[index]);continue;}if(!await matches(value,attribute.typeExpr))report('attribute-type',`${attribute.name} violates its EXPRESS type.`,entity.id,[index]);else await check(value,attribute.typeExpr,entity.id,[index]);}
 }
 await tick();await scope.checkpoint({phase:'structure',completed:evaluator.work,total:evaluator.work});return diagnostics;
}
