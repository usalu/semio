/** 📇️ Traverse already captured source records and cumulative values in their original arithmetic order. */
import type {IfcIoSteps} from './control.ts';
export function* ifcSourceEntriesSteps<T extends object>(source:T|undefined):IfcIoSteps<[Extract<keyof T,string>,T[keyof T]][]>{
 const result:[Extract<keyof T,string>,T[keyof T]][]=[];if(source)for(const key in source)if(Object.hasOwn(source,key)){yield;result.push([key,source[key]!]);}return result;
}
/** 📐️ Binary64 sums deliberately retain source order; addition is not reassociated by level sorting. */
export function* ifcLevelElevationSteps(rows:readonly {level:number;height:number}[],level:number):IfcIoSteps<number>{
 let result=0;for(const row of rows){yield;if(level>=0?row.level>=0&&row.level<level:row.level<0&&row.level>=level)result+=row.height;}return level>=0?result:-result;
}
/** 🏢️ Ordered groups cache only prefixes that exactly preserve original filtered addition order. */
export class IfcPreparedLevels {
 private constructor(readonly rows:readonly {level:number;height:number}[],readonly counts:ReadonlyMap<number,number>,readonly positive:ReadonlyMap<number,number>|undefined,readonly negative:ReadonlyMap<number,number>|undefined){}
 count(level:number):number{return Number.isNaN(level)?0:this.counts.get(level)??0;}
 *elevationSteps(level:number):IfcIoSteps<number>{const cache=level>=0?this.positive:this.negative;if(cache?.has(level)){yield;return cache.get(level)!;}return yield* ifcLevelElevationSteps(this.rows,level);}
 static *prepare(rows:readonly {level:number;height:number}[]):IfcIoSteps<IfcPreparedLevels>{
  const counts=new Map<number,number>(),positive=new Map<number,number>(),negative=new Map<number,number>();let valid=true,ascending=true,descending=true,lastPositive=-Infinity,lastNegative=Infinity,above=0,below=0;
  for(const row of rows){yield;counts.set(row.level,(counts.get(row.level)??0)+1);if(typeof row.level!=='number'||typeof row.height!=='number'||!Number.isFinite(row.level)||!Number.isFinite(row.height)){valid=false;continue;}if(row.level>=0){if(row.level<=lastPositive)ascending=false;lastPositive=row.level;positive.set(row.level,above);above+=row.height;}else{if(row.level>=lastNegative)descending=false;lastNegative=row.level;below+=row.height;negative.set(row.level,-below);}}
  return new IfcPreparedLevels(rows,counts,valid&&ascending?positive:undefined,valid&&descending?negative:undefined);
 }
}
/** 📐️ Prepare duplicate counts and safe prefix opportunities under the original traversal context. */
export function ifcPrepareLevelsSteps(rows:readonly {level:number;height:number}[]):IfcIoSteps<IfcPreparedLevels>{return IfcPreparedLevels.prepare(rows);}
