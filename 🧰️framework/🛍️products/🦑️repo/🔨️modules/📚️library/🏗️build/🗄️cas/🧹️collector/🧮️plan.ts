import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";

/** ✂️ Where the least-recently-accessed entries stop being kept: everything older than `access` goes, and of the entries accessed exactly then, in scan order, as many as `tieBytes` still needs. `access` is Infinity when even all candidates cannot cover the need. */
export type CasCutV1=Readonly<{access:number;tieBytes:number}>;
/** 🏷️ Why an entry is evicted: its age, its area cap or the host cap. */
export type CasReasonV1="age"|"area"|"host";
/** 📐️ The declared bounds of one area: its own byte cap and idle age (null is unbounded). */
export type CasAreaBoundsV1=Readonly<{capBytes:number|null;maxAgeMs:number|null}>;
/** 📐️ What one planner needs: the policy, the bounds per area id (the area `store` is the host store), bytes held by tool-owned images, and the clock. */
export type CasPlannerConfigV1=Readonly<{policy:Pick<CasPolicyV1,"capBytes"|"highWatermark"|"lowWatermark">;areas:readonly (CasAreaBoundsV1&Readonly<{id:string}>)[];externalBytes:number;nowMs:number}>;
/** 📊️ The planner's totals after the first pass. */
export type CasPlanTotalsV1=Readonly<{totalBytes:number;protectedBytes:number;candidateBytes:number;plannedBytes:number;keptBytes:number;shortfallBytes:number;needBytes:number}>;
/** 🔎️ The evaluator of the second pass; call `decide` for every entry in the same scan order as `add`. */
export type CasEvaluatorV1=Readonly<{decide(area:string,access:number,size:number):CasReasonV1|null}>;
/** 🧮️ A streaming planner: `add` every entry in scan order (first pass), `finish` for totals and the evaluator. */
export type CasPlannerV1=Readonly<{add(area:string,access:number,size:number,protectedEntry:boolean):void;finish():Readonly<{totals:CasPlanTotalsV1;evaluator():CasEvaluatorV1}>}>;

class Columns{
 access=new Float64Array(1024);size=new Float64Array(1024);area=new Uint16Array(1024);n=0;
 push(area:number,access:number,size:number):void{
  if(this.n===this.access.length){const grow=this.n*2;for(const key of ["access","size"] as const){const next=new Float64Array(grow);next.set(this[key]);this[key]=next;}const next=new Uint16Array(grow);next.set(this.area);this.area=next;}
  this.access[this.n]=access;this.size[this.n]=size;this.area[this.n]=area;this.n++;
 }
}

/** ✂️ The cut over `indices` (scan order) that frees at least `need` bytes oldest-first; null when `need` is not positive. */
export function casCutV1(access:Float64Array,size:Float64Array,indices:Uint32Array,need:number):CasCutV1|null{
 if(!(need>0))return null;
 const order=Uint32Array.from(indices).sort((left,right)=>access[left]!-access[right]!||left-right);let sum=0;
 for(const index of order){
  if(sum+size[index]!>=need){
   const at=access[index]!;let below=0;
   for(const other of order){if(access[other]!>=at)break;below+=size[other]!;}
   return{access:at,tieBytes:need-below};
  }
  sum+=size[index]!;
 }
 return{access:Infinity,tieBytes:0};
}

class Rule{
 private left:number;
 constructor(private readonly cut:CasCutV1){this.left=cut.tieBytes;}
 test(access:number,size:number):boolean{
  if(access<this.cut.access)return true;
  if(access===this.cut.access&&this.left>0){this.left-=size;return true;}
  return false;
 }
}

/** 🧮️ Creates the streaming least-recently-accessed planner. Stage 1 drops entries idle longer than their area's age; stage 2 cuts an area above its cap down to the low watermark share; stage 3 cuts the host total above the high watermark down to the low watermark, oldest first across all areas, never touching protected entries (pinned, leased, linked or too young), which count toward every total. Memory is 20 bytes per candidate entry, no strings. */
export function createCasPlannerV1(config:CasPlannerConfigV1):CasPlannerV1{
 const ids=config.areas.map(area=>area.id),index=new Map(ids.map((id,position)=>[id,position])),columns=new Columns(),areaProtected=new Float64Array(ids.length);let protectedBytes=0;
 const {capBytes,highWatermark,lowWatermark}=config.policy;
 return Object.freeze({
  add(area,access,size,protectedEntry){
   const position=index.get(area);if(position===undefined)throw Error("Declared cache area required / Deklarierter Cache-Bereich erforderlich: "+area);
   if(protectedEntry){protectedBytes+=size;areaProtected[position]!+=size;}else columns.push(position,access,size);
  },
  finish(){
   const {n,access,size,area}=columns,live=new Uint8Array(n).fill(1);let reasonsAge=0;
   const ageBefore=config.areas.map(row=>row.maxAgeMs===null?-Infinity:config.nowMs-row.maxAgeMs);
   for(let i=0;i<n;i++)if(access[i]!<ageBefore[area[i]!]!){live[i]=0;reasonsAge+=size[i]!;}
   const areaRules:(CasCutV1|null)[]=[];let planned=reasonsAge;
   for(let a=0;a<ids.length;a++){
    const cap=config.areas[a]!.capBytes;let bytes=areaProtected[a]!;const members:number[]=[];
    for(let i=0;i<n;i++)if(area[i]===a&&live[i]){bytes+=size[i]!;members.push(i);}
    const cut=cap!==null&&bytes>cap?casCutV1(access,size,Uint32Array.from(members),bytes-cap*lowWatermark):null;areaRules.push(cut);
    if(cut){const rule=new Rule(cut);for(const i of members)if(rule.test(access[i]!,size[i]!)){live[i]=0;planned+=size[i]!;}}
   }
   let after=config.externalBytes+protectedBytes;const survivors:number[]=[];
   for(let i=0;i<n;i++)if(live[i]){after+=size[i]!;survivors.push(i);}
   const need=after>capBytes*highWatermark?after-capBytes*lowWatermark:0,hostCut=casCutV1(access,size,Uint32Array.from(survivors),need);
   let hostBytes=0,short=0;
   if(hostCut){const rule=new Rule(hostCut);for(const i of survivors)if(rule.test(access[i]!,size[i]!))hostBytes+=size[i]!;short=Math.max(0,need-hostBytes);}
   let candidate=0;for(let i=0;i<n;i++)candidate+=size[i]!;
   const total=config.externalBytes+protectedBytes+candidate,removed=planned+hostBytes;
   return{
    totals:Object.freeze({totalBytes:total,protectedBytes,candidateBytes:candidate,plannedBytes:removed,keptBytes:total-removed,shortfallBytes:short,needBytes:need}),
    evaluator():CasEvaluatorV1{
     const rules=areaRules.map(cut=>cut?new Rule(cut):null),host=hostCut?new Rule(hostCut):null;
     return{decide(areaId,at,bytes){
      const position=index.get(areaId)!;
      if(at<ageBefore[position]!)return "age";
      if(rules[position]?.test(at,bytes))return "area";
      if(host?.test(at,bytes))return "host";
      return null;
     }};
    },
   };
  },
 });
}
