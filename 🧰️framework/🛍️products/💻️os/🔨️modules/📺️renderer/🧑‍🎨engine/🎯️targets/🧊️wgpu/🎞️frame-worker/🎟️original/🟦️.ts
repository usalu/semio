import { OriginalOperationSource, originalReceiptFromWit, type OriginalOperationAdmission, type OriginalOperationTurn, type OriginalWireReceipt } from "../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts";
import type { OriginalBootstrapOwner } from "../../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";
/** 🎟️ The actual browser native owner consumes only the initiating scalar input and returns its inline raw tuple. */
export interface OriginalBrowserFrameNative {
 admitOriginalFrame(operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number):boolean;
 originalFrameReceiptScalar(index:number):bigint;
 originalFrameReceiptRefusal():number;
 originalFrameOutputLength():number;
 originalFrameOutputByte(index:number):number;
 acknowledgeOriginalFrameOutput(operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number,fuel:bigint,deadline:bigint):boolean;
 tick(timestamp:number,sequence:bigint,presentation:bigint,operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number,fuel:bigint,deadline:bigint,cancelled:boolean):number;
 assetDecodeStep(operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number,fuel:bigint,deadline:bigint):number;
 closeOriginalFrame(operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number,fuel:bigint,deadline:bigint):boolean;
 closeOriginalFrameContext(operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number):boolean;
}
export interface OriginalBrowserFrameFailure {readonly kind:"failure";readonly format:"pack";readonly bytes:Uint8Array}
/** 🧵️ One same-worker source retains original business authority independently of input presentation counters. */
export class OriginalBrowserFrameSource {
 readonly source:OriginalOperationSource;#returned:Promise<void>=Promise.resolve();#nativeFailure:unknown;#receiptFailure:unknown;#outputAcknowledgement=false;#closed=false;
 constructor(readonly native:OriginalBrowserFrameNative,readonly owner:OriginalBootstrapOwner,readonly admission:OriginalOperationAdmission,clock:()=>bigint){this.source=new OriginalOperationSource(admission,clock)}
 get nativeFailure():unknown{return this.#nativeFailure}
 get receiptFailure():unknown{return this.#receiptFailure}
 operationClockUs():bigint{return this.source.operationClockUs()}
 #input(turn:OriginalOperationTurn):[bigint,bigint,bigint,number,number,number,number,number]{const input=turn.input,g=input.grant;return[input.operation,input.generation,input.epoch,g.maximumItems,g.maximumCopyBytes,g.maximumCapacityBytes,g.maximumReleaseBytes,g.maximumDepth]}
 #receive():void{
  const raw:OriginalWireReceipt={input:{operation:this.native.originalFrameReceiptScalar(0),generation:this.native.originalFrameReceiptScalar(1),epoch:this.native.originalFrameReceiptScalar(2),grant:{maximumItems:this.native.originalFrameReceiptScalar(3),maximumCopyBytes:this.native.originalFrameReceiptScalar(4),maximumCapacityBytes:this.native.originalFrameReceiptScalar(5),maximumReleaseBytes:this.native.originalFrameReceiptScalar(6),maximumDepth:this.native.originalFrameReceiptScalar(7)}},spent:{copiedItems:this.native.originalFrameReceiptScalar(8),copiedBytes:this.native.originalFrameReceiptScalar(9),retainedCapacityBytes:this.native.originalFrameReceiptScalar(10),releasedBytes:this.native.originalFrameReceiptScalar(11)}};
  const receipt=originalReceiptFromWit(raw,this.native.originalFrameReceiptScalar(12));try{this.source.receive(receipt)}catch(original){this.#receiptFailure=original;throw original}this.#returned=this.#returned.then(()=>this.owner.receive(this.admission,receipt));
  if(this.native.originalFrameReceiptRefusal()!==0){this.#receiptFailure=raw;throw Error("original-frame.native-authority-refused")}
 }
 #call<T>(produce:(turn:OriginalOperationTurn)=>T):T{const turn=this.source.issue();try{return produce(turn)}catch(original){if(this.#nativeFailure===undefined)this.#nativeFailure=original;throw original}finally{this.#receive()}}
 async admit():Promise<void>{for(;;){const ready=this.#call(turn=>this.native.admitOriginalFrame(...this.#input(turn)));await this.#returned;if(ready)return;await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 tick(timestamp:number,sequence:bigint,presentation:bigint):string|OriginalBrowserFrameFailure|undefined{
  if(this.#closed||this.#nativeFailure!==undefined||this.#receiptFailure!==undefined)throw Error("original-frame.source-held");
  if(this.#outputAcknowledgement){const complete=this.#call(turn=>this.native.acknowledgeOriginalFrameOutput(...this.#input(turn),BigInt(turn.fuel),turn.deadlineUs));if(complete)this.#outputAcknowledgement=false;return undefined}
  const status=this.#call(turn=>this.native.tick(timestamp,sequence,presentation,...this.#input(turn),BigInt(turn.fuel),turn.deadlineUs,turn.cancelled));if(status===0)return undefined;if(status!==1&&status!==2)throw Error("original-frame.output-state-refused");
  const length=this.native.originalFrameOutputLength();if(!Number.isSafeInteger(length)||length<0)throw Error("original-frame.output-length-refused");const bytes=new Uint8Array(length);for(let index=0;index<length;index++){const byte=this.native.originalFrameOutputByte(index);if(!Number.isInteger(byte)||byte<0||byte>255)throw Error("original-frame.output-byte-refused");bytes[index]=byte;}
  this.#outputAcknowledgement=true;if(status===2)return{kind:"failure",format:"pack",bytes};return new TextDecoder("utf-8",{fatal:true}).decode(bytes);
 }

 /** 📡️ Decoder polls spend the same admitted source and return fixed semantic scalar status. */
 assetDecode():{kind:"pending"|"idle"|"published"|"cancelled"|"fault";pending:boolean}|undefined{
  if(this.#closed||this.#nativeFailure!==undefined||this.#receiptFailure!==undefined)throw Error("original-frame.source-held");
  if(this.#outputAcknowledgement)return undefined;
  const raw=this.#call(turn=>this.native.assetDecodeStep(...this.#input(turn),BigInt(turn.fuel),turn.deadlineUs));
  if(!Number.isSafeInteger(raw)||raw<0||raw>12||(raw&7)>4)throw Error("original-frame.decoder-status-refused");
  const kinds=["pending","idle","published","cancelled","fault"] as const;
  return{kind:kinds[raw&7]!,pending:(raw&8)!==0};
 }

 /** ♻️ Original child aliases, failure publication and native authorities finish before this exact parent admission returns. */
 async close():Promise<void>{if(this.#closed)return;this.source.cancel();for(;;){const complete=this.#call(turn=>this.native.closeOriginalFrame(...this.#input(turn),BigInt(turn.fuel),turn.deadlineUs));await this.#returned;if(complete)break;await new Promise<void>(resolve=>setTimeout(resolve,0));}for(;;){const complete=this.#call(turn=>this.native.closeOriginalFrameContext(...this.#input(turn)));await this.#returned;if(complete)break;await new Promise<void>(resolve=>setTimeout(resolve,0));}this.source.close();await this.owner.returnClosed(this.admission,true);this.#closed=true;}
}
