/** 📈️ Explicit source-step outcomes keep semantic work, actual CPU and physical evidence independent. */
import {RetainedWorkLedger,RetainedWorkTurnCounter,assertRetainedWorkTurn,type RetainedWorkBuffer,type RetainedWorkTurn} from "../🟦️.ts";
export interface RetainedWorkProducerChild<P>{advance(items:number,work:RetainedWorkLedger):P;lastTurnWork():number;lastTurnSemanticWork():number;settlementPending():boolean;failurePending():boolean}
export interface RetainedWorkProducerOwner<P>{validate():void;stopped():boolean;settlementPending():boolean;failurePending():boolean;semanticWork():number;maximumWork:number;workLimitMessage:string;recordSemanticWork(units:number):void;step(context:RetainedWorkProducerStep):void;progress():P}
/** 🧭️ Only defining probes, mutations and actual child getters commit a source step. */
export class RetainedWorkProducerStep{
 #semantic=0;private readonly initialWork:number;
 constructor(readonly work:RetainedWorkLedger,private readonly turn:RetainedWorkTurn,private readonly maximum:number,private readonly before:number,private readonly limitMessage:string){this.initialWork=assertRetainedWorkTurn(turn);}
 get semanticWork():number{return this.#semantic;}
 get retainedTurn():RetainedWorkTurn{assertRetainedWorkTurn(this.turn);return this.turn;}
 slot<T>(values:RetainedWorkBuffer<T>):boolean{return this.work.slot(values,this.turn);}
 contiguous<T>(values:RetainedWorkBuffer<T>,target:number):boolean{return this.work.contiguous(values,target,this.turn);}
 push<T>(values:RetainedWorkBuffer<T>,value:T):void{this.work.push(values,value,this.turn);}
 write<T>(values:RetainedWorkBuffer<T>,index:number,value:T):boolean{return this.work.write(values,index,value,this.turn);}
 pop<T>(values:RetainedWorkBuffer<T>):boolean{return this.work.pop(values,this.turn);}

 requireSemanticWork(units=1):void{assertRetainedWorkTurn(this.turn);if(!Number.isSafeInteger(units)||units<0||this.before+this.#semantic+units>this.maximum)throw Error(this.limitMessage);}
 postSemanticWork():void{this.requireSemanticWork();this.#semantic++;}
 commitSemanticWork():void{this.postSemanticWork();this.turn.commitActualWork();}
 commitControlWork():void{this.turn.commitActualWork();}
 hasActualWork():boolean{return this.turn.actualWork()>this.initialWork;}
 advanceChild<P>(child:RetainedWorkProducerChild<P>):P{if(!child.settlementPending()&&!child.failurePending())this.requireSemanticWork();let result:P|undefined,failures:unknown[]|null=null;try{result=child.advance(1,this.work);}catch(error){(failures??=[]).push(error);}try{this.turn.commitActualWork(child.lastTurnWork());}catch(error){(failures??=[]).push(error);}try{const semantic=child.lastTurnSemanticWork();if(!Number.isSafeInteger(semantic)||semantic<0)throw Error("Invalid actual retained semantic work");this.#semantic+=semantic;if(this.before+this.#semantic>this.maximum)throw Error(this.limitMessage);}catch(error){(failures??=[]).push(error);}if(failures!==null)throw failures.length===1?failures[0]:new AggregateError(failures,"Actual retained child failures");return result!;}
}
/** ⏱️ Paid preparation is evidenced by actual owner effects and never charged as a semantic source step. */
export class RetainedWorkProducerRuntime{
 #counter=new RetainedWorkTurnCounter();#last=0;#semantic=0;
 lastTurnWork():number{return this.#last;}
 lastTurnSemanticWork():number{return this.#semantic;}
 advance<P>(items:number,work:RetainedWorkLedger,owner:RetainedWorkProducerOwner<P>):P{this.#last=0;this.#semantic=0;if(!Number.isSafeInteger(items)||items<1)throw Error("Invalid retained producer quantum");owner.validate();if(owner.stopped()||work.blocked)return owner.progress();const entered=work.enter();try{while(entered!==null&&this.#last<items&&work.remainingItems>0&&!owner.stopped()){const semanticBefore=owner.semanticWork();if(semanticBefore>=owner.maximumWork&&!owner.settlementPending()&&!owner.failurePending())throw Error(owner.workLimitMessage);const turn=this.#counter.beginTurn(),context=new RetainedWorkProducerStep(work,turn,owner.maximumWork,semanticBefore,owner.workLimitMessage);let failures:unknown[]|null=null,actual=0;try{owner.step(context);}catch(error){(failures??=[]).push(error);}finally{actual=this.#counter.endTurn(turn);this.#last+=actual;this.#semantic+=context.semanticWork;try{owner.recordSemanticWork(context.semanticWork);}catch(error){(failures??=[]).push(error);}try{for(let unit=0;unit<actual;unit++)work.noteSource(entered);}catch(error){(failures??=[]).push(error);}}if(failures!==null)throw failures.length===1?failures[0]:new AggregateError(failures,"Actual retained source and posting failures");if(work.blocked||actual===0)break;}return owner.progress();}finally{if(entered!==null)work.exit();}}
}
