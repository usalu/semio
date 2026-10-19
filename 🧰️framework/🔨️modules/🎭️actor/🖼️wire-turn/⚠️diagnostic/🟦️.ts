import type {TypedOperationDiagnosticFragment,TypedOperationToken} from "../🔖️turn-result/🟦️.ts";
import {decodeJsonSyntaxSegments,type JsonSyntaxControl,type JsonSyntaxNode} from "../../../🎒️pack/🔤️json/📥️decode/🟦️.ts";
import {JsonMemberPolicy} from "@semio-tech/framework-pack-json-ts/members";
import {decodeDiagnosticControlled,type Diagnostic} from "@semio-tech/framework-diagnostic-ts/controlled";
import type {ValueRefusalCodecControl} from "@semio-tech/value-ts/refusal/codec";
import type {IntrinsicValue,IntrinsicMember} from "@semio-tech/value-ts/schema/intrinsic";

/** 🎟️ Syntax and ownership authority are supplied by the exact receiving command. */
export interface OperationDiagnosticControl {readonly syntax:JsonSyntaxControl;readonly ownership:ValueRefusalCodecControl}
function refuse(message:string):never {throw Error("typed-operation diagnostic: "+message);}
function sameToken(a:TypedOperationToken,b:TypedOperationToken):boolean {return a.receiver===b.receiver&&a.operation===b.operation&&a.generation===b.generation&&a.sequence===b.sequence&&a.attempt===b.attempt;}
async function intrinsic(node:JsonSyntaxNode,c:ValueRefusalCodecControl):Promise<IntrinsicValue>{
 await c.charge(64);
 if(node.kind==="string")return{kind:"text",value:node.value};
 if(node.kind==="number"){if(!/^(0|[1-9][0-9]*)$/.test(node.text)||node.text.length>10)refuse("span requires u32");const value=BigInt(node.text);if(value>4294967295n)refuse("span requires u32");return{kind:"unsigned",value};}
 if(node.kind==="null"||node.kind==="boolean")return node;
 return c.scopedStage(async()=>{
  const rows=node.kind==="array"?node.items:node.members;await c.beginStage(rows.length);await c.charge(rows.length*64);
  if(node.kind==="array"){const items:IntrinsicValue[]=[];for(const row of node.items){items.push(await intrinsic(row,c));await c.step();}return{kind:"array",items};}
  const members:IntrinsicMember[]=[];for(const row of node.members){members.push({name:row.name,value:await intrinsic(row.value,c)});await c.step();}return{kind:"object",members};
 });
}

/** 📬️ Receives original bounded fragments once; terminal publication requires a complete original row. */
export class OperationDiagnosticReceiver {
 private identity:TypedOperationToken|null=null;
 private previous:TypedOperationDiagnosticFragment|null=null;
 private decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});
 private parts:string[]=[];
 private bytes=0;
 private row=0;
 private stopped=false;
 private receiving=false;
 constructor(readonly control:OperationDiagnosticControl){}
 async receive(fragment:TypedOperationDiagnosticFragment):Promise<Diagnostic|null>{
  if(this.stopped||this.receiving||this.control.syntax.cancelled())refuse("receiver cancelled or busy");
  const previous=this.previous,token=fragment.token;
  if(previous&&sameToken(previous.token,token)){
   if(previous.row!==fragment.row||previous.complete!==fragment.complete||previous.bytes.length!==fragment.bytes.length||previous.bytes.some((value,index)=>value!==fragment.bytes[index]))refuse("replayed original page changed");
   return null;
  }
  if(fragment.bytes.length>4091||(!fragment.complete&&fragment.bytes.length===0)||fragment.row!==this.row)refuse("invalid original row frontier");
  if(this.identity&&(token.receiver!==this.identity.receiver||token.operation!==this.identity.operation||token.generation!==this.identity.generation))refuse("foreign original command");
  if(previous&&token.sequence!==previous.token.sequence+1)refuse("original page sequence gap");
  if(fragment.bytes.length>this.control.syntax.maximumBytes-this.bytes)refuse("original row exceeds byte authority");
  this.receiving=true;
  try{
   await this.control.ownership.charge(64+fragment.bytes.length*2);
   if(this.stopped||this.control.syntax.cancelled())refuse("receiver cancelled");
   const text=this.decoder.decode(fragment.bytes,{stream:!fragment.complete});
   if(text.length)this.parts.push(text);
   this.bytes+=fragment.bytes.length;this.identity??=token;this.previous=fragment;
   if(!fragment.complete)return null;
   const syntax=await decodeJsonSyntaxSegments(this.parts,JsonMemberPolicy.Reject,this.control.syntax);
   const diagnostic=await decodeDiagnosticControlled(await intrinsic(syntax,this.control.ownership),this.control.ownership);
   if(this.stopped||this.control.syntax.cancelled())refuse("receiver cancelled");
   this.parts=[];this.bytes=0;this.row++;this.decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});
   return diagnostic;
  }catch(error){this.stopped=true;throw error;}finally{this.receiving=false;}
 }
 /** 🏁️ Checks the original continuation before a terminal page may be accepted. */
 finish():void {if(this.stopped||this.receiving||this.bytes!==0||this.parts.length!==0)refuse("terminal precedes complete diagnostic custody");}
 /** 🛑️ Cancels the exact receiver and releases its host-owned text references. */
 cancel():void {this.stopped=true;this.parts=[];this.previous=null;this.identity=null;this.bytes=0;}
}

/** 🧾️ A decoded row remains attributable to the exact original receiver and operation generation. */
export interface OperationDiagnosticReceipt {readonly token:TypedOperationToken;readonly diagnostic:Diagnostic}
/** 🏘️ Original diagnostic streams outlive host polling slices and end only at their own terminal or cancellation. */
export class OperationDiagnosticInbox {
 private readonly streams=new Map<string,OperationDiagnosticReceiver>();
 private admitting:string|null=null;
 private admissionCancelled=false;
 private stopped=false;
 constructor(readonly maximumStreams:number,readonly control:(token:TypedOperationToken)=>OperationDiagnosticControl){if(!Number.isSafeInteger(maximumStreams)||maximumStreams<1)refuse("invalid stream capacity");}
 private key(token:TypedOperationToken):string{return `${token.receiver}/${token.operation}/${token.generation}`;}
 async receive(fragment:TypedOperationDiagnosticFragment):Promise<OperationDiagnosticReceipt|null>{
  if(this.stopped)refuse("inbox cancelled");
  const key=this.key(fragment.token);let receiver=this.streams.get(key);
  if(!receiver){
   if(this.admitting!==null)refuse("original stream admission busy");
   if(this.streams.size>=this.maximumStreams)refuse("original stream capacity exhausted");
   this.admitting=key;this.admissionCancelled=false;
   try{const original=this.control(fragment.token);await original.ownership.charge(96);if(this.stopped||this.admissionCancelled)refuse("original stream admission cancelled");receiver=new OperationDiagnosticReceiver(original);this.streams.set(key,receiver);}
   finally{this.admitting=null;this.admissionCancelled=false;}
  }
  const diagnostic=await receiver.receive(fragment);return diagnostic?{token:fragment.token,diagnostic}:null;
 }
 /** 🏁️ Admits terminal custody before removing the original operation stream. */
 finish(token:TypedOperationToken):void {const key=this.key(token);if(this.admitting===key)refuse("terminal precedes original stream admission");const receiver=this.streams.get(key);if(receiver){receiver.finish();receiver.cancel();this.streams.delete(key);}}
 /** 🛑️ Receiver loss cancels every original operation independently of any polling call. */
 cancelAll():void {this.stopped=true;this.admissionCancelled=true;for(const receiver of this.streams.values())receiver.cancel();this.streams.clear();}
 /** 🧯️ A failed original operation ends its incomplete receiving stream. */
 cancel(token:TypedOperationToken):void {const key=this.key(token);if(this.admitting===key)this.admissionCancelled=true;this.streams.get(key)?.cancel();this.streams.delete(key);}
}
