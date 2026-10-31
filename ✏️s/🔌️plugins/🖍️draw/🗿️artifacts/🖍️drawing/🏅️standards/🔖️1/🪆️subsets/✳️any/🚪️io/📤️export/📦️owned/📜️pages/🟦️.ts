/** 📜️ Export byte pages keep real backing and one private output issue under original caller funding. */
import {RetainedWorkBuffer,RetainedWorkBytes,RetainedWorkRetirement,assertRetainedWorkTurn,type RetainedWorkLedger,type RetainedWorkTurn,type RetainedWorkDemand,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
const PAGE=3072;
class PageBody{
 readonly #auth=true;readonly pages=new RetainedWorkBuffer<RetainedWorkBytes>(8);at=PAGE;length=0;closing=false;closed=false;
 constructor(readonly maximumBytes:number){if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>67108864)throw RangeError("Invalid export byte page limit");}
 static authentic(value:unknown):value is PageBody{return value!==null&&typeof value==="object"&&#auth in value;}
 check(){if(this.closing||this.closed)throw Error("Export byte borrow expired");}
 page(index:number):Uint8Array|undefined{this.check();if(!Number.isSafeInteger(index)||index<0)return undefined;const value=this.pages.at(index);return value===undefined?undefined:value.bytes.subarray(0,Math.min(PAGE,this.length-index*PAGE));}
 close():RetainedWorkRetirementCursor{this.check();this.closing=true;let active:RetainedWorkBytes|null=null,finished=false;return new RetainedWorkRetirement((work,turn)=>{if(active!==null){if(!active.terminalIsEmpty()){active.retire(work,turn);return false;}if(!work.pop(this.pages,turn))return false;active=null;return false;}if(this.pages.length){if(!work.canStep())return false;active=this.pages.at(-1)!;turn.commitActualWork();return false;}if(!this.pages.retireOne(work,turn)||turn.actualWork()>0)return false;if(!finished){if(!work.canStep())return false;this.closed=true;finished=true;turn.commitActualWork();return false;}return true;});}
}
let issue:(body:PageBody)=>EncodedBytePages;
/** 🗝️ One private body moves from its writer to its output without a raw page-adoption door. */
export class EncodedBytePages implements RetainedWorkRetirementCursor{
 #body:PageBody;#close:RetainedWorkRetirementCursor|null=null;#last=0;
 private constructor(body:PageBody){if(!PageBody.authentic(body))throw Error("Authentic export body required");this.#body=body;}
 static{issue=body=>new EncodedBytePages(body);}
 get length():number{this.#body.check();return this.#body.pages.length;}
 get byteLength():number{this.#body.check();return this.#body.length;}
 at(index:number):Uint8Array|undefined{return this.#body.page(index);}
 lastTurnWork(){return this.#last;}
 nextDemand():RetainedWorkDemand{if(this.#close!==null)return this.#close.nextDemand();return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};}
 terminalIsEmpty(){return this.#close?.terminalIsEmpty()??false;}
 advance(items:number,work:RetainedWorkLedger){this.#last=0;if(!Number.isSafeInteger(items)||items<1)throw RangeError("Invalid export page close quantum");if(this.#close===null){const demand=this.nextDemand();const grant=work.childGrant;if(grant.maximumItems===0||grant.maximumCopyBytes<demand.copyBytes||grant.maximumCapacityBytes<demand.capacityBytes||grant.maximumReleaseBytes<demand.releaseBytes||grant.maximumDepth<demand.depth){work.requireChild(demand);return{done:false,work:0};}this.#close=this.#body.close();}try{return this.#close.advance(items,work);}finally{this.#last=this.#close.lastTurnWork();}}
}
/** 🧱️ Each publication row, backing birth and byte store is a separate defining effect. */
export class BytePageWriter{
 #body:PageBody|null;
 constructor(maximumBytes:number){this.#body=new PageBody(maximumBytes);}
 #get():PageBody{if(this.#body===null)throw Error("Export page writer already transferred");this.#body.check();return this.#body;}
 get byteLength():number{return this.#get().length;}
 get length():number{return this.#get().pages.length;}
 at(index:number):Uint8Array|undefined{return this.#get().page(index);}
 nextWriteDemand():RetainedWorkDemand{const body=this.#get();if(body.length===body.maximumBytes)return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};if(body.at===PAGE)return body.pages.nextSlotDemand();const page=body.pages.at(-1)!;return page.terminalIsEmpty()?page.nextAdmissionDemand(Math.min(PAGE,body.maximumBytes-(body.pages.length-1)*PAGE)):{copyBytes:1,capacityBytes:0,releaseBytes:0,depth:1};}
 write(value:number,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{assertRetainedWorkTurn(turn);const body=this.#get();if(!Number.isInteger(value)||value<0||value>255)throw RangeError("Invalid export byte value");if(body.length===body.maximumBytes)throw Error("Export output byte limit exceeded");const demand=this.nextWriteDemand(),grant=work.childGrant;if(grant.maximumItems===0||grant.maximumCopyBytes<demand.copyBytes||grant.maximumCapacityBytes<demand.capacityBytes||grant.maximumDepth<demand.depth){work.requireChild(demand);return false;}if(body.at===PAGE){if(!work.slot(body.pages,turn))return false;work.push(body.pages,new RetainedWorkBytes(),turn);body.at=0;return false;}const page=body.pages.at(-1)!;if(page.terminalIsEmpty()){page.admit(Math.min(PAGE,body.maximumBytes-(body.pages.length-1)*PAGE),work,turn);return false;}if(!page.set(body.at,value,work,turn))return false;body.at++;body.length++;if(body.at===page.bytes.length)body.at=PAGE;return true;}
 publish(work:RetainedWorkLedger,turn:RetainedWorkTurn):EncodedBytePages|null{assertRetainedWorkTurn(turn);const body=this.#get(),grant=work.childGrant;if(grant.maximumItems===0||grant.maximumDepth<1){work.requireChild({copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1});return null;}const output=issue(body);turn.commitActualWork();this.#body=null;return output;}
 intoRetirement():RetainedWorkRetirementCursor{const body=this.#get();this.#body=null;return body.close();}
}
