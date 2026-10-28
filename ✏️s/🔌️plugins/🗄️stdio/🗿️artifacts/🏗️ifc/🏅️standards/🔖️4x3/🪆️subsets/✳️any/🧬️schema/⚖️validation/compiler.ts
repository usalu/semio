import {ifc4x3Catalog} from "../📚️catalog/🟦️.ts";
import type {IfcTypeExpression} from "../📚️catalog/🟦️.ts";
import type {ExpressControl,ExpressDiagnostic,ExpressProgram} from "./ast.ts";
import {ExpressParser,parseExpressExpression} from "./parser.ts";
import {captureExpressControlScope,ExpressEvaluationError} from './control.ts';

/** 📚 The canonical program remains inspectable even when a retained construct cannot compile. */
export interface ExpressCompilation {readonly program:ExpressProgram;readonly diagnostics:readonly ExpressDiagnostic[];readonly complete:boolean}
const issuedCompilations=new WeakMap<ExpressProgram,ExpressCompilation>();

/** 🪪 Full-schema validation retains the exact compiler-issued completeness witness. */
export function retainedIfc4x3ExpressCompilation(program:ExpressProgram):ExpressCompilation|undefined{return issuedCompilations.get(program);}

/** 🏗️ Compile every retained schema expression and imperative declaration with caller-owned limits. */
export async function compileIfc4x3Express(control:ExpressControl={}):Promise<ExpressCompilation>{
  const scope=captureExpressControlScope(control);control=scope.control;scope.check();
  const program:{-readonly[Key in keyof ExpressProgram]:ExpressProgram[Key] extends readonly(infer Entry)[]?Entry[]:ExpressProgram[Key]}={schema:"IFC4X3_ADD2",functions:[],globalRules:[],rules:[],derived:[],unique:[],bounds:[],inverse:[]};
  const diagnostics:ExpressDiagnostic[]=[];
  const jobs:{owner:string;label:string;source:string;run:()=>void}[]=[];
  const add=(owner:string,label:string,source:string,run:()=>void)=>jobs.push({owner,label,source,run});
  const rule=(owner:string,source:string)=>{const parser=new ExpressParser(source,control.maximumDepth);const result=parser.rule(owner);parser.done();return result;};
  const bounds=(owner:string,label:string,type:IfcTypeExpression):void=>{
    if(type.kind!=="aggregate")return;
    for(const [side,source] of [["lower",type.lowerExpression],["upper",type.upperExpression]] as const){if(source&&source!=="?"&&!/^-?\d+$/.test(source))add(owner,`${label}.${side}`,source,()=>program.bounds.push({owner,label:`${label}.${side}`,expression:parseExpressExpression(source,control.maximumDepth)}));}
    if(type.item)bounds(owner,`${label}.item`,type.item);
  };
  const entities=Object.entries(ifc4x3Catalog.entities),types=Object.entries(ifc4x3Catalog.types),functions=Object.entries(ifc4x3Catalog.functions),rules=Object.entries(ifc4x3Catalog.rules),total=entities.length+types.length+functions.length+rules.length;let indexed=0;
  const index=async()=>{scope.check();if(indexed++%(control.batchSize??32)===0)await scope.checkpoint({phase:'compile-index',completed:indexed-1,total});};
  for(const [owner,entity] of entities){
    await index();
    for(const source of entity.whereRules)add(owner,source.split(":",1)[0]!,source,()=>program.rules.push(rule(owner,source)));
    for(const derived of entity.derived){add(owner,derived.name,derived.expression,()=>program.derived.push({owner,label:derived.name.toUpperCase(),expression:parseExpressExpression(derived.expression,control.maximumDepth)}));bounds(owner,derived.name.toUpperCase(),derived.typeExpr);}
    for(const source of entity.uniqueRules)add(owner,source.split(":",1)[0]!,source,()=>{const colon=source.indexOf(":");program.unique.push({owner,label:source.slice(0,colon).trim().toUpperCase(),expression:parseExpressExpression(`[${source.slice(colon+1)}]`,control.maximumDepth)});});
    for(const attribute of entity.attributes)bounds(owner,attribute.name.toUpperCase(),attribute.typeExpr);
    for(const source of entity.inverse)add(owner,source.split(":",1)[0]!,source,()=>{
      const match=/^(\w+)\s*:\s*(?:(SET|BAG)\s*(?:\[([^:]+):([^\]]+)\])?\s*OF\s+)?(\w+)\s+FOR\s+(\w+)$/i.exec(source.trim());
      if(!match)throw new Error("Unsupported inverse declaration");
      program.inverse.push({owner,name:match[1]!.toUpperCase(),type:match[5]!.toUpperCase(),attribute:match[6]!.toUpperCase(),aggregate:!!match[2],...(match[3]?{lower:parseExpressExpression(match[3],control.maximumDepth)}:{}),...(match[4]?{upper:parseExpressExpression(match[4],control.maximumDepth)}:{})});
    });
  }
  for(const [owner,type] of types){
    await index();
    for(const source of type.whereRules)add(owner,source.split(":",1)[0]!,source,()=>program.rules.push(rule(owner,source)));
    if(type.typeExpr)bounds(owner,"SELF",type.typeExpr);
  }
  for(const [owner,source] of functions){await index();add(owner,"FUNCTION",source,()=>program.functions.push(new ExpressParser(source,control.maximumDepth).function(owner)));}
  for(const [owner,source] of rules){await index();add(owner,"RULE",source,()=>program.globalRules.push(new ExpressParser(source,control.maximumDepth).global(owner)));}
  await scope.checkpoint({phase:'compile-index',completed:indexed,total});
  let work=0;
  for(let index=0;index<jobs.length;index++){
    scope.check();
    const job=jobs[index]!;work+=job.source.length;
    if(work>(control.maximumWork??20_000_000))throw new ExpressEvaluationError('work-budget','EXPRESS schema compilation work limit');
    try{job.run();}catch(error){diagnostics.push({code:"express.compile",owner:job.owner,rule:job.label,message:error instanceof Error?error.message:String(error),...(error&&typeof error==="object"&&"offset" in error?{offset:Number(error.offset)}:{})});}
    if(index%(control.batchSize??32)===0)await scope.checkpoint({phase:"compile",completed:index,total:jobs.length});
  }
  const compilation:ExpressCompilation={program,diagnostics,complete:diagnostics.length===0},pending:object[]=[compilation],seen=new WeakSet<object>();let sealed=0;
  await scope.checkpoint({phase:'compile-seal',completed:0,total:0});
  while(pending.length){scope.check();const value=pending.pop()!;if(seen.has(value))continue;seen.add(value);if(++work>(control.maximumWork??20_000_000))throw new ExpressEvaluationError('work-budget','EXPRESS schema sealing work limit');for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);Object.freeze(value);if(++sealed%(control.batchSize??32)===0)await scope.checkpoint({phase:'compile-seal',completed:sealed,total:0});}
  await scope.checkpoint({phase:'compile-seal',completed:sealed,total:sealed});
  await scope.checkpoint({phase:"compile",completed:jobs.length,total:jobs.length});
  issuedCompilations.set(program,compilation);return compilation;
}
