/** 🏗️ PDF object tables own actual COS bodies, dictionary values and resource rows under caller turns. */
import {RetainedWorkBuffer,RetainedWorkRetirement,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import type {RetainedWorkProducerStep} from "../../../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {BytePageWriter} from "../../../../../../../../📦️owned/📜️pages/🟦️.ts";
export type PdfSpan={readonly from:number;readonly to:number;readonly left:number;readonly right:number};
export type PdfBody={readonly pages:BytePageWriter;used:number;length:number;readonly dict:RetainedWorkBuffer<string>;readonly stream:boolean};
export type PdfRecord={parent:number|null;constant:readonly number[]|null;readonly stops:RetainedWorkBuffer<number>;readonly spans:RetainedWorkBuffer<PdfSpan>;asset:number;gradient:boolean};
export class PdfObjectStore{
 #objects=new RetainedWorkBuffer<number|null>(8);#offsets=new RetainedWorkBuffer<number>(8);#bodies=new RetainedWorkBuffer<PdfBody>(40);#records=new RetainedWorkBuffer<PdfRecord>(48);#creatingId:number|null=null;#creatingIndex=0;#transferred=false;
 constructor(private readonly maximumBytes:number,readonly objectCount:number){}
 get length(){this.#check();return this.#objects.length;}
 get recordCount(){this.#check();return this.#records.length;}
 #check(){if(this.#transferred)throw Error("PDF object custody already transferred");}
 initialize(c:RetainedWorkProducerStep):boolean{this.#check();if(this.#offsets.length<this.#objects.length){if(!c.slot(this.#offsets))return false;c.push(this.#offsets,0);return false;}if(this.#objects.length<this.objectCount){if(!c.slot(this.#objects))return false;c.push(this.#objects,null);return false;}return true;}
 object(id:number):PdfBody|null{this.#check();const index=this.#objects.at(id);if(index===undefined)throw RangeError("PDF object identity outside initialized table");return index===null?null:this.#bodies.at(index)!;}
 maybeRecord(index:number):PdfRecord|undefined{this.#check();return this.#records.at(index);}
 record(index:number):PdfRecord{this.#check();const row=this.#records.at(index);if(row===undefined)throw Error("PDF resource row absent");return row;}
 offset(index:number):number{this.#check();const value=this.#offsets.at(index);if(value===undefined)throw Error("PDF offset row absent");return value;}
 setOffset(index:number,value:number,c:RetainedWorkProducerStep):boolean{this.#check();return c.write(this.#offsets,index,value);}
 create(id:number,dict:string,stream:boolean,c:RetainedWorkProducerStep):boolean{this.#check();const existing=this.#objects.at(id);if(existing===undefined)throw RangeError("PDF object identity outside initialized table");if(existing!==null){if(this.#creatingId===id){this.#creatingId=null;c.commitControlWork();return false;}return true;}if(this.#creatingId===null){if(!c.slot(this.#bodies))return false;const value:PdfBody={pages:new BytePageWriter(this.maximumBytes),used:3072,length:0,dict:new RetainedWorkBuffer<string>(Math.max(2,dict.length*2)),stream};c.push(this.#bodies,value);this.#creatingId=id;this.#creatingIndex=this.#bodies.length-1;return false;}if(this.#creatingId!==id)throw Error("PDF object creation source changed");const body=this.#bodies.at(this.#creatingIndex)!;if(body.dict.elementBytes!==Math.max(2,dict.length*2)||body.stream!==stream)throw Error("PDF dictionary source changed");if(body.dict.length===0){if(!c.slot(body.dict))return false;c.push(body.dict,dict);return false;}c.write(this.#objects,id,this.#creatingIndex);return false;}
 setRecord<K extends "parent"|"constant"|"asset"|"gradient">(index:number,key:K,value:PdfRecord[K],c:RetainedWorkProducerStep):boolean{this.#check();const row=this.record(index);if(Object.is(row[key],value))return true;if(!c.work.copyBytes(8))return false;row[key]=value;c.retainedTurn.commitActualWork();c.work.notePhysicalEffect();return true;}
 appendRecord(c:RetainedWorkProducerStep):boolean{this.#check();if(!c.slot(this.#records))return false;c.push(this.#records,{parent:null,constant:null,stops:new RetainedWorkBuffer<number>(8),spans:new RetainedWorkBuffer<PdfSpan>(32),asset:-1,gradient:false});return true;}
 intoRetirement():RetainedWorkRetirementCursor{this.#check();this.#transferred=true;let active:RetainedWorkRetirementCursor|null=null,phase=0,body:PdfBody|null=null,record:PdfRecord|null=null,finished=false;return new RetainedWorkRetirement((work,turn)=>{
  if(active!==null){if(!active.terminalIsEmpty()){try{active.advance(1,work);}finally{turn.commitActualWork(active.lastTurnWork());}return false;}if(!work.canStep())return false;active=null;turn.commitActualWork();return false;}
  if(phase===0){if(body===null&&this.#bodies.length){if(!work.canStep())return false;body=this.#bodies.at(-1)!;active=body.pages.intoRetirement();turn.commitActualWork();return false;}if(body!==null){if(!body.dict.retireOne(work,turn)||turn.actualWork()>0)return false;if(!work.pop(this.#bodies,turn))return false;body=null;return false;}if(!this.#bodies.retireOne(work,turn)||turn.actualWork()>0)return false;phase++;}
  if(phase===1){if(record===null&&this.#records.length){if(!work.canStep())return false;record=this.#records.at(-1)!;turn.commitActualWork();return false;}if(record!==null){if(!record.stops.retireOne(work,turn)||turn.actualWork()>0)return false;if(!record.spans.retireOne(work,turn)||turn.actualWork()>0)return false;if(!work.pop(this.#records,turn))return false;record=null;return false;}if(!this.#records.retireOne(work,turn)||turn.actualWork()>0)return false;phase++;}
  if(phase===2){if(!this.#objects.retireOne(work,turn)||turn.actualWork()>0)return false;phase++;}
  if(phase===3){if(!this.#offsets.retireOne(work,turn)||turn.actualWork()>0)return false;phase++;}
  if(!finished){if(!work.canStep())return false;this.#creatingId=null;finished=true;turn.commitActualWork();return false;}return true;
 });}
}
