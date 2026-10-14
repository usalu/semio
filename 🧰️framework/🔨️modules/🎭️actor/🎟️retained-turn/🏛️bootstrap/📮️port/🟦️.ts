import {OriginalBootstrapOwner} from "../🟦️.ts";
import type {OriginalOperationAdmission,OriginalOperationReceipt} from "../../🟦️.ts";
/** 📨️ Closed command identity routes only original owner admission, actual receiving and final custody. */
type OriginalBootstrapCommand=({readonly command:"admit"}|{readonly command:"receive";readonly operation:bigint;readonly generation:bigint;readonly receipt:OriginalOperationReceipt}|{readonly command:"hold";readonly operation:bigint;readonly generation:bigint;readonly refusal:unknown}|{readonly command:"close";readonly operation:bigint;readonly generation:bigint;readonly sourceClosed:boolean});
export type OriginalBootstrapRequest={readonly kind:"original-stock-request";readonly version:1;readonly request:bigint}&OriginalBootstrapCommand;
export type OriginalBootstrapResponse={readonly kind:"original-stock-result";readonly request:bigint}&({readonly accepted:true;readonly admission:OriginalOperationAdmission|undefined}|{readonly accepted:false;readonly refusal:unknown});
/** 🌉️ One frame route retains the genuine parent admissions and replies from that same root owner. */
export class OriginalBootstrapRootPort {
 readonly #originals=new Map<bigint,OriginalOperationAdmission>();#lastRequest=0n;
 constructor(readonly owner:OriginalBootstrapOwner,readonly reply:(response:OriginalBootstrapResponse)=>void){}
 async receive(request:OriginalBootstrapRequest):Promise<void>{if(request.version!==1||typeof request.request!=="bigint"||request.request<=0n||request.request>0xffffffffffffffffn||request.request<=this.#lastRequest)return;this.#lastRequest=request.request;try{if(request.command==="admit"){const original=await this.owner.admit();this.#originals.set(original.operation,original);this.reply({kind:"original-stock-result",request:request.request,accepted:true,admission:original});return}const original=this.#originals.get(request.operation);if(!original||original.generation!==request.generation)throw Error("original-bootstrap.foreign-admission");if(request.command==="receive")await this.owner.receive(original,request.receipt);else if(request.command==="hold")await this.owner.hold(original,request.refusal);else{await this.owner.returnClosed(original,request.sourceClosed);this.#originals.delete(original.operation)}this.reply({kind:"original-stock-result",request:request.request,accepted:true,admission:undefined});}catch(refusal){this.reply({kind:"original-stock-result",request:request.request,accepted:false,refusal});}}
 get heldAdmissions():number{return this.#originals.size}
}
/** 🧵️ The frame never creates stock: every child draws and returns through its captured parent route. */
export class RemoteOriginalBootstrapOwner extends OriginalBootstrapOwner {
 #next=1n;readonly #pending=new Map<bigint,{resolve:(response:OriginalBootstrapResponse)=>void;reject:(refusal:unknown)=>void}>();readonly #originals=new Map<bigint,OriginalOperationAdmission>();
 constructor(readonly post:(request:OriginalBootstrapRequest)=>void){super()}
 receiveResponse(response:OriginalBootstrapResponse):void{if(response.kind!=="original-stock-result")return;const pending=this.#pending.get(response.request);if(!pending)return;this.#pending.delete(response.request);if(response.accepted)pending.resolve(response);else pending.reject(response.refusal);}
 #request(command:OriginalBootstrapCommand):Promise<OriginalBootstrapResponse>{if(this.#next>=0xffffffffffffffffn)throw Error("original-bootstrap.sequence-exhausted");const request=this.#next++;return new Promise((resolve,reject)=>{this.#pending.set(request,{resolve,reject});try{this.post({kind:"original-stock-request",version:1,request,...command} as OriginalBootstrapRequest)}catch(refusal){reject(refusal)}})}
 #original(original:OriginalOperationAdmission):void{if(this.#originals.get(original.operation)!==original)throw Error("original-bootstrap.foreign-admission")}
 async admit():Promise<OriginalOperationAdmission>{const response=await this.#request({command:"admit"});if(!response.accepted||!response.admission)throw Error("original-bootstrap.admission-refused");const original=response.admission;this.#originals.set(original.operation,original);return original}
 async receive(original:OriginalOperationAdmission,receipt:OriginalOperationReceipt):Promise<void>{this.#original(original);await this.#request({command:"receive",operation:original.operation,generation:original.generation,receipt})}
 async hold(original:OriginalOperationAdmission,refusal:unknown):Promise<void>{this.#original(original);await this.#request({command:"hold",operation:original.operation,generation:original.generation,refusal})}
 async returnClosed(original:OriginalOperationAdmission,sourceClosed:boolean):Promise<void>{this.#original(original);await this.#request({command:"close",operation:original.operation,generation:original.generation,sourceClosed});this.#originals.delete(original.operation)}
}
