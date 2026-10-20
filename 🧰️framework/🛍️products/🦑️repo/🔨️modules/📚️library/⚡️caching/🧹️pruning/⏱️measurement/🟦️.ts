import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
export const cacheScanTimingPrefixV1=schema.definitions.Protocol.const.prefix;
/** ⏱️ Labels one of the two fresh observations without changing its custody or coverage. */
export type CacheScanTimingPhaseV1="initial"|"recensus";
/** 📊️ Retains fixed scalar metadata-operation costs without per-file payloads. */
export type CacheScanTimingV1=Readonly<{version:1;phase:CacheScanTimingPhaseV1;scanner:"build"|"incremental";status:"complete"|"failed";root:string;elapsedMilliseconds:number;discoveryMilliseconds:number;measurementMilliseconds:number;enumerationMilliseconds:number;units:number;maximumInFlight:number;operations:Readonly<Record<"stat"|"measurement"|"entries"|"continuation",Readonly<{calls:number;milliseconds:number}>>>}>;
/** 🧾️ Formats only the closed finite summary; acceptance supplies no runtime authority. */
export function formatCacheScanTimingV1(value:unknown):string{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed finite cache scan timing required");const text=JSON.stringify(value);if(typeof text!=="string"||Buffer.byteLength(text)+Buffer.byteLength(cacheScanTimingPrefixV1)+1>schema.definitions.Protocol.const.maximumSummaryBytes)throw Error("Bounded cache scan timing required");return text;}

type OperationV1=keyof CacheScanTimingV1["operations"];
type StageV1="discovery"|"measurement"|"enumeration";
/** 🔬️ Accounts only for this actual serial pass; it never starts work or grants authority. */
export class CacheScanTimingRecorderV1{
 private readonly started=performance.now();
 private stage:StageV1="discovery";
 private stageStarted=this.started;
 private active=0;
 private maximumInFlight=0;
 private readonly stages={discovery:0,measurement:0,enumeration:0};
 private readonly operations={stat:{calls:0,milliseconds:0},measurement:{calls:0,milliseconds:0},entries:{calls:0,milliseconds:0},continuation:{calls:0,milliseconds:0}};
 private readonly root:string;private readonly phase:CacheScanTimingPhaseV1;private readonly scanner:CacheScanTimingV1["scanner"];
 constructor(root:string,phase:CacheScanTimingPhaseV1,scanner:CacheScanTimingV1["scanner"]){this.root=root;this.phase=phase;this.scanner=scanner;if(phase!=="initial"&&phase!=="recensus")throw Error("Explicit cache timing phase required");}
 enter(stage:StageV1):void{const now=performance.now();this.stages[this.stage]+=now-this.stageStarted;this.stageStarted=now;this.stage=stage;}
 begin(kind:OperationV1):number{this.operations[kind].calls++;if(kind!=="continuation")this.maximumInFlight=Math.max(this.maximumInFlight,++this.active);return performance.now();}
 end(kind:OperationV1,started:number):void{this.operations[kind].milliseconds+=performance.now()-started;if(kind!=="continuation")this.active--;}
 finish(status:CacheScanTimingV1["status"],units:number):CacheScanTimingV1{const now=performance.now();return{version:1,phase:this.phase,scanner:this.scanner,status,root:this.root,elapsedMilliseconds:now-this.started,discoveryMilliseconds:this.stages.discovery+(this.stage==="discovery"?now-this.stageStarted:0),measurementMilliseconds:this.stages.measurement+(this.stage==="measurement"?now-this.stageStarted:0),enumerationMilliseconds:this.stages.enumeration+(this.stage==="enumeration"?now-this.stageStarted:0),units,maximumInFlight:this.maximumInFlight,operations:this.operations};}
}

/** 🔌️ Enables observations only by the closed authored switch, without granting storage authority. */
export function cacheScanTimingEnabledV1(environment:Readonly<Record<string,string|undefined>>):boolean{return environment[schema.definitions.Protocol.const.switch]==="1";}
