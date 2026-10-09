export type CargoProgress=Readonly<{stage:string;path:string;completed:number;ownedBytes:number}>;
export interface CargoControl {readonly signal:AbortSignal;readonly maximumUnits:number;readonly maximumOwnedBytes:number;readonly maximumDepth:number;remainingMilliseconds():number;onProgress(progress:CargoProgress):void;yieldContinuation():Promise<void>;}
export interface CargoAccounting {completed:number;ownedBytes:number;}
export interface CargoRetirementOperation extends CargoControl {readonly accounting:CargoAccounting;}

const yieldMarks=new WeakMap<CargoAccounting,number>();

/** 🎛️ Admits cumulative work and ownership before continuing, retaining real cancellation and yielding. */
export class CargoController {
 constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){
  if(typeof operation.signal?.aborted!=="boolean"||typeof operation.remainingMilliseconds!=="function"||typeof operation.onProgress!=="function"||typeof operation.yieldContinuation!=="function")throw Error("Cargo control requires its actual signal, deadline and progress continuation");
  for(const value of[operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth,accounting.completed,accounting.ownedBytes])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");
  if(accounting.completed>operation.maximumUnits||accounting.ownedBytes>operation.maximumOwnedBytes)throw Error("Cargo accounting exceeds its explicit authority");
  if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);
 }
 check():void{if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");const remaining=this.operation.remainingMilliseconds();if(!Number.isFinite(remaining)||remaining<=0)throw Error("Cargo operation deadline refused");}
 async step(stage:string,path:string,units=1,bytes=0):Promise<void>{
  this.check();
  if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(bytes)||bytes<0)throw Error("Cargo accounting requires finite nonnegative admissions");
  if(units>this.operation.maximumUnits-this.accounting.completed||bytes>this.operation.maximumOwnedBytes-this.accounting.ownedBytes)throw Error("Cargo operation budget refused");
  this.accounting.completed+=units;this.accounting.ownedBytes+=bytes;
  if(this.accounting.completed-yieldMarks.get(this.accounting)!>=128||units===0){
   this.operation.onProgress({stage,path,completed:this.accounting.completed,ownedBytes:this.accounting.ownedBytes});this.check();await this.operation.yieldContinuation();this.check();yieldMarks.set(this.accounting,this.accounting.completed);
  }
 }
 async finish(stage:string,path:string):Promise<void>{await this.step(stage,path,0);}
}
