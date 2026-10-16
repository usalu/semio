import type {DagSelectionDomain,DagSelectionSource} from "../../../../🧬️schema/🎯️dag-input/🟦️.ts";
/** ⛽️ Admits one physical census or emitted-byte unit. */
export interface DagSelectionTextGrant {readonly fuel:number;readonly nowMilliseconds:number;readonly deadlineMilliseconds:number;readonly cancelled:boolean;readonly interrupted:boolean}
/** 🚧️ Identifies a refused retained text step without changing its continuation. */
export type DagSelectionTextFault="Cancelled"|"Interrupted"|"Deadline"|"NoFuel"|"Limit"|"Sealed";
/** 🚨️ Owns the physical cursor refusal identity. */
export class DagSelectionTextError extends Error {constructor(readonly fault:DagSelectionTextFault){super(fault);this.name="DagSelectionTextError"}}
/** 📬️ Publishes preflight size, one byte, progress or completion. */
export type DagSelectionTextStep={readonly kind:"progress";readonly completed:number;readonly total:number}|{readonly kind:"census";readonly bytes:number}|{readonly kind:"byte";readonly byte:number}|{readonly kind:"complete"};
type Phase="censusNode"|"censusText"|"open"|"seek"|"separator"|"quoteOpen"|"text"|"escape"|"quoteClose"|"close"|"complete";
const maximumOutput=65536;
function refuse(fault:DagSelectionTextFault):never{throw new DagSelectionTextError(fault)}
const shortEscapes:Readonly<Record<number,number>>=Object.freeze({8:98,12:102,10:110,13:114,9:116});
function escapeLength(byte:number):number{return byte===34||byte===92||shortEscapes[byte]!==undefined?2:byte<32?6:1}


/** 🧾️ Retains exact JSON preflight and one-unit publication over borrowed selection facts. */
export class DagSelectionJsonCursor {
 #phase:Phase="censusNode";#candidate=0;#selected=0;#emitted=0;#census=2;#output=0;#units=0;#utf8=[0,0,0,0];#utf8Length=0;#utf8Cursor=0;#escape=[0,0,0,0,0,0];#escapeLength=0;#escapeCursor=0;
 constructor(readonly domain:DagSelectionDomain){if(domain!=="nodes"&&domain!=="edges")refuse("Limit")}
 #resetText():void{this.#units=0;this.#utf8Length=0;this.#utf8Cursor=0}
 #byte(text:string):number|undefined{
  if(this.#utf8Cursor===this.#utf8Length){if(this.#units===text.length)return undefined;let point=text.charCodeAt(this.#units++);if(point>=0xd800&&point<=0xdbff){const next=text.charCodeAt(this.#units++);if(!(next>=0xdc00&&next<=0xdfff))refuse("Limit");point=0x10000+((point-0xd800)<<10)+(next-0xdc00)}else if(point>=0xdc00&&point<=0xdfff)refuse("Limit");
  this.#utf8Cursor=0;if(point<0x80){this.#utf8[0]=point;this.#utf8Length=1}else if(point<0x800){this.#utf8[0]=0xc0|(point>>6);this.#utf8[1]=0x80|(point&63);this.#utf8Length=2}else if(point<0x10000){this.#utf8[0]=0xe0|(point>>12);this.#utf8[1]=0x80|((point>>6)&63);this.#utf8[2]=0x80|(point&63);this.#utf8Length=3}else{this.#utf8[0]=0xf0|(point>>18);this.#utf8[1]=0x80|((point>>12)&63);this.#utf8[2]=0x80|((point>>6)&63);this.#utf8[3]=0x80|(point&63);this.#utf8Length=4}}
  return this.#utf8[this.#utf8Cursor++]!;
 }
 #prepareEscape(byte:number):void{this.#escapeLength=escapeLength(byte);this.#escape[0]=92;if(this.#escapeLength===2)this.#escape[1]=shortEscapes[byte]??byte;else{const hex="0123456789abcdef";this.#escape[1]=117;this.#escape[2]=48;this.#escape[3]=48;this.#escape[4]=hex.charCodeAt(byte>>4);this.#escape[5]=hex.charCodeAt(byte&15)}}
 #progress(total:number):DagSelectionTextStep{return {kind:"progress",completed:this.#candidate,total}}
 #emit(byte:number):DagSelectionTextStep{this.#output++;return {kind:"byte",byte}}
 /** 📤️ Refuses caller cancellation before inspecting or advancing borrowed source facts. */
 step(source:DagSelectionSource,grant:DagSelectionTextGrant):DagSelectionTextStep {
  if(grant.cancelled)refuse("Cancelled");if(grant.interrupted)refuse("Interrupted");if(grant.nowMilliseconds>=grant.deadlineMilliseconds)refuse("Deadline");if(!Number.isSafeInteger(grant.fuel)||grant.fuel<1)refuse("NoFuel");
  if(!Number.isSafeInteger(grant.nowMilliseconds)||grant.nowMilliseconds<0||!Number.isSafeInteger(grant.deadlineMilliseconds)||grant.deadlineMilliseconds<0)refuse("Limit");
  const count=source.selectionCandidateCount(this.domain);if(!Number.isSafeInteger(count)||count<0)refuse("Limit");const item=()=>source.selectionCandidateId(this.domain,this.#candidate);
  switch(this.#phase){
   case "censusNode":if(this.#candidate===count){if(this.#census>maximumOutput)refuse("Limit");this.#candidate=0;this.#resetText();this.#phase="open";return {kind:"census",bytes:this.#census}}if(item()!==undefined){this.#census+=2+Number(this.#selected!==0);this.#phase="censusText"}else this.#candidate++;return this.#progress(count);
   case "censusText":{const text=item();if(text===undefined)refuse("Limit");const byte=this.#byte(text);if(byte===undefined){this.#selected++;this.#candidate++;this.#resetText();this.#phase="censusNode"}else this.#census+=escapeLength(byte);return this.#progress(count)}
   case "open":this.#phase="seek";return this.#emit(91);
   case "seek":if(this.#candidate===count)this.#phase="close";else if(item()!==undefined)this.#phase=this.#emitted===0?"quoteOpen":"separator";else this.#candidate++;return this.#progress(count);
   case "separator":this.#phase="quoteOpen";return this.#emit(44);
   case "quoteOpen":this.#phase="text";return this.#emit(34);
   case "text":{const text=item();if(text===undefined)refuse("Limit");const byte=this.#byte(text);if(byte===undefined){this.#phase="quoteClose";return {kind:"progress",completed:this.#output,total:this.#census}}if(escapeLength(byte)===1)return this.#emit(byte);this.#prepareEscape(byte);this.#escapeCursor=0;this.#phase="escape";return {kind:"progress",completed:this.#output,total:this.#census}}
   case "escape":{const byte=this.#escape[this.#escapeCursor++]!;if(this.#escapeCursor===this.#escapeLength)this.#phase="text";return this.#emit(byte)}
   case "quoteClose":this.#emitted++;this.#candidate++;this.#resetText();this.#phase="seek";return this.#emit(34);
   case "close":this.#phase="complete";return this.#emit(93);
   case "complete":if(this.#output!==this.#census)refuse("Limit");return {kind:"complete"};
  }
 }
}
