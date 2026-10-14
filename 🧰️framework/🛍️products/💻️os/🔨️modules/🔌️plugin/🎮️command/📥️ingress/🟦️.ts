/** 🎟️ Linear original parser slots remain owned until their caller acknowledges paid retirement. */
export interface ActionIngressAdmission {readonly capacity:number;readonly requiredItems:number;readonly requiredDepth:number;}
export interface ActionIngressSource<A,F>{args:A|null;fault:F|null;prepared?:A;}
export interface ActionIngressReceiving<A,F>{value:ActionIngressSource<A,F>|undefined;}
export class ActionIngressOwner<A,F>{
 #source:ActionIngressSource<A,F>|undefined;
 #ready=false;
 private constructor(source:ActionIngressSource<A,F>){this.#source=source;}
 static admit<A,F>(receiving:ActionIngressReceiving<A,F>,admission:ActionIngressAdmission):ActionIngressOwner<A,F>|undefined{if(!Number.isSafeInteger(admission.capacity)||!Number.isSafeInteger(admission.requiredItems)||!Number.isSafeInteger(admission.requiredDepth)||admission.capacity<0||admission.requiredItems!==1||admission.requiredDepth!==1)throw Error("Explicit finite original ingress admission required");if(admission.capacity===0||receiving.value===undefined)return undefined;const source=receiving.value;receiving.value=undefined;return new ActionIngressOwner(source);}
 borrow():ActionIngressSource<A,F>{if(!this.#source)throw Error("Original ingress source is terminal");return this.#source;}
 prepare(original:A):void{const source=this.borrow();if(source.prepared!==undefined)throw Error("Prepared original source is already retained");source.prepared=original;}
 parse<C>(receiver:(source:ActionIngressSource<A,F>)=>C):C{return receiver(this.borrow());}
 close(retire:(original:ActionIngressSource<A,F>)=>boolean):boolean{if(!this.#source)return true;if(!retire(this.#source))return false;this.#source=undefined;return true;}
 finishParser(terminal:boolean):void{this.#ready=terminal;}
 maintenance(retire:(original:ActionIngressSource<A,F>)=>boolean):boolean{return this.#ready?this.close(retire):false;}
 terminal():boolean{return this.#source===undefined;}
}

/** 📦️ Original envelope fields stay received while typed projections wait for their actual control. */
export interface OriginalParsedCommandControl{readonly maximumWork:number;projection(kind:"action"|"command",source:unknown):{readonly value:unknown}|undefined;checkpoint():boolean;}
/** 🪪️ Deferred lease admission precedes parser donation and retains the exact declared operation key. */
export class OriginalPreviewAdmission<K,L>{
 readonly key:K;
 #lease:L|undefined;
 constructor(key:K){this.key=key;}
 admit(receive:(key:K)=>L|undefined):boolean{if(this.#lease===undefined)this.#lease=receive(this.key);return this.#lease!==undefined;}
 admitted():boolean{return this.#lease!==undefined;}
 lease():L|undefined{return this.#lease;}
}
export class OriginalParsedCommandSource{
 readonly kind:"command"|"artifact";
 readonly envelope:Record<string,unknown>;
 args:unknown=undefined;
 address:unknown=undefined;
 #extracted=false;
 constructor(kind:"command"|"artifact",envelope:Record<string,unknown>){this.kind=kind;this.envelope=envelope;}
 prepare(control:OriginalParsedCommandControl):boolean{
  if(!Number.isSafeInteger(control.maximumWork)||control.maximumWork<128)return false;
  const key=this.kind==="artifact"?"args":"arguments",head=this.kind==="artifact"?"action":"address",keys=Object.keys(this.envelope);
  if(keys.some(name=>name!==head&&name!==key)||!keys.includes(head)||(this.kind==="command"&&!keys.includes(key)))throw Error("Closed original command envelope required");
  if(this.kind==="artifact"&&typeof this.envelope.action!=="string")throw Error("Original artifact action must be a string");
  if(this.kind==="command"){
   if(!this.#extracted&&(typeof this.envelope.arguments!=="object"||this.envelope.arguments===null||Array.isArray(this.envelope.arguments)))throw Error("Original command arguments must be an object");
   if(!originalCommandAddress(this.envelope.address))throw Error("Closed original command address required");
  }
  if(this.kind==="command"&&this.address===undefined){const source=this.envelope.address;if(typeof source!=="object"||source===null||Array.isArray(source))throw Error("Original command address required");const kind="windowInstanceId"in source?"action":"command";const projected=control.projection(kind,source);if(projected===undefined)return false;this.address=projected.value;}
  if(!this.#extracted){if(!control.checkpoint())return false;this.args=this.envelope[key];if(key in this.envelope)this.envelope[key]=null;this.#extracted=true;}
  return true;
 }
}

function originalFields(value:unknown,keys:readonly string[],strings:boolean):value is Record<string,unknown>{return typeof value==="object"&&value!==null&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key)&&(!strings||typeof (value as Record<string,unknown>)[key]==="string"));}
function originalCommandAddress(value:unknown):boolean{
 if(originalFields(value,["pluginId","appId","modeId","windowKindId","windowInstanceId","actionId"],true))return true;
 if(!originalFields(value,["owner","commandId"],false)||typeof value.commandId!=="string")return false;
 const owner=value.owner;if(owner==="os")return true;
 if(typeof owner!=="object"||owner===null||Array.isArray(owner)||Object.keys(owner).length!==1)return false;
 for(const [tag,keys] of [["plugin",["pluginId"]],["app",["pluginId","appId"]],["mode",["pluginId","appId","modeId"]]] as const)if(Object.hasOwn(owner,tag))return originalFields((owner as Record<string,unknown>)[tag],keys,true);
 return false;
}
