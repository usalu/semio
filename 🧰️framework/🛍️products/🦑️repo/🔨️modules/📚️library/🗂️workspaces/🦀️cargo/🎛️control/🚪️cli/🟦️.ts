import {CargoDiscoveryWorkspace,closeCargoDiscoveryOwners,type CargoDiscoveryOperation} from "../../📁️physical/🟦️.ts";
import {CargoController,type CargoRetirementOperation,type CargoProgress} from "../🟦️.ts";

export type CargoOperationLimits=Readonly<{maximumUnits:number;maximumOwnedBytes:number;maximumDepth:number}>;
export type CargoCliPolicy=Readonly<{schemaVersion:1;discovery:CargoOperationLimits;retirement:CargoOperationLimits;progressIntervalMilliseconds:number}>;
export interface CargoCliProgressPort {remainingMilliseconds():number;publish(progress:CargoProgress):void;yieldContinuation():Promise<void>;}
const table=(value:unknown):value is Record<string,unknown>=>Boolean(value)&&typeof value==="object"&&!Array.isArray(value);
function limits(value:unknown):CargoOperationLimits{if(!table(value)||Object.keys(value).sort().join(",")!=="maximumDepth,maximumOwnedBytes,maximumUnits")throw Error("Cargo CLI requires complete explicit limits");const maximumUnits=value.maximumUnits,maximumOwnedBytes=value.maximumOwnedBytes,maximumDepth=value.maximumDepth;if(typeof maximumUnits!=="number"||typeof maximumOwnedBytes!=="number"||typeof maximumDepth!=="number"||![maximumUnits,maximumOwnedBytes,maximumDepth].every(number=>Number.isSafeInteger(number)&&number>=0)||maximumDepth>4096)throw Error("Cargo CLI limits are not finite");return{maximumUnits,maximumOwnedBytes,maximumDepth};}

/** 🎛️ Admits the complete closed policy without implicit budgets or field defaults. */
export function parseCargoCliPolicy(value:unknown):CargoCliPolicy{if(!table(value)||Object.keys(value).sort().join(",")!=="discovery,progressIntervalMilliseconds,retirement,schemaVersion"||value.schemaVersion!==1||typeof value.progressIntervalMilliseconds!=="number"||!Number.isSafeInteger(value.progressIntervalMilliseconds)||value.progressIntervalMilliseconds<0||value.progressIntervalMilliseconds>60000)throw Error("Cargo CLI policy refused");return{schemaVersion:1,discovery:limits(value.discovery),retirement:limits(value.retirement),progressIntervalMilliseconds:value.progressIntervalMilliseconds};}

/** 🚪️ Owns the real command signal and a separately funded retirement operation. */
export class CargoCliOperationOwner {
 readonly policy:CargoCliPolicy;readonly workspace:CargoDiscoveryWorkspace;readonly operation:CargoDiscoveryOperation;
 private readonly discoverySignal=new AbortController();private readonly retirementSignal=new AbortController();private readonly retirement:CargoRetirementOperation;
 private attached=true;private lastProgress=Number.NEGATIVE_INFINITY;
 private readonly onInterrupt=()=>this.discoverySignal.abort();
 constructor(policy:CargoCliPolicy,private readonly progress:CargoCliProgressPort){
  this.policy=parseCargoCliPolicy(policy);if(typeof progress.remainingMilliseconds!=="function"||typeof progress.publish!=="function"||typeof progress.yieldContinuation!=="function")throw Error("Cargo CLI requires real deadline, progress and yielding ports");this.workspace=new CargoDiscoveryWorkspace();
  this.operation={workspace:this.workspace,signal:this.discoverySignal.signal,...this.policy.discovery,remainingMilliseconds:()=>progress.remainingMilliseconds(),onProgress:row=>this.publish(row),yieldContinuation:()=>progress.yieldContinuation()};
  this.retirement={accounting:{completed:0,ownedBytes:0},signal:this.retirementSignal.signal,...this.policy.retirement,remainingMilliseconds:()=>progress.remainingMilliseconds(),onProgress:row=>progress.publish(row),yieldContinuation:()=>progress.yieldContinuation()};
  process.once("SIGINT",this.onInterrupt);process.once("SIGTERM",this.onInterrupt);
 }
 private publish(row:CargoProgress):void{const now=performance.now();if(now-this.lastProgress>=this.policy.progressIntervalMilliseconds){this.lastProgress=now;this.progress.publish(row);}}
 cancelDiscovery():void{this.discoverySignal.abort();}
 cancelRetirement():void{this.retirementSignal.abort();}
 get retirementAborted():boolean{return this.retirementSignal.signal.aborted;}
 get retirementCompleted():number{return this.retirement.accounting.completed;}
 detachSignals():void{if(this.attached){process.off("SIGINT",this.onInterrupt);process.off("SIGTERM",this.onInterrupt);this.attached=false;}}
 async closeSystemOwners():Promise<void>{try{await closeCargoDiscoveryOwners(this.workspace,this.retirement);}finally{this.detachSignals();}}
}

/** ⚠️ Retains the exact interrupted owner and both command/retirement refusals. */
export class CargoCommandFailure extends Error {constructor(readonly owner:CargoCliOperationOwner,readonly commandCause:unknown,readonly retirementCause:unknown){super("Cargo command or owned retirement refused");this.name="CargoCommandFailure";}}

/** 🧾️ Executes the real command and closes its owners without turning a failure into acceptance. */
export async function runCargoCliCommand<T>(owner:CargoCliOperationOwner,body:()=>Promise<T>):Promise<T>{let value:T|undefined,commandCause:unknown=null,retirementCause:unknown=null;try{await new CargoController(owner.operation,owner.workspace).step("command-entry","",0);value=await body();}catch(error){commandCause=error;}owner.cancelDiscovery();try{await owner.closeSystemOwners();}catch(error){retirementCause=error;}if(commandCause!==null||retirementCause!==null)throw new CargoCommandFailure(owner,commandCause,retirementCause);return value!;}
