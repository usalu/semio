/** 📬️ A window-local address and its issued payload retain original custody on rejection. */
import {FactoryBoxedValue,type FactoryBoxedIssuer,type FactoryBoxedPublication} from "../../../../../../🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🟦️.ts";
export type WindowMutationLane="config"|"transient";
export class WindowIssuedMutation<T>{
 constructor(readonly lane:WindowMutationLane,readonly windowId:string,readonly windowKindId:string,private owner:FactoryBoxedValue<T>){if(!windowId||!windowKindId)throw new Error("Window mutation requires its exact address");}
 static of<T>(lane:WindowMutationLane,windowId:string,windowKindId:string,value:T,issuer:FactoryBoxedIssuer<T>):WindowIssuedMutation<T>{return new WindowIssuedMutation(lane,windowId,windowKindId,new FactoryBoxedValue({value},issuer));}
 originalValue():T|null{return this.owner.originalValue();}
 takeForPublication():WindowIssuedPublication<T>{const moved=this.owner.takeForPublication();return new WindowIssuedPublication(this,moved.value,moved.retirement);}
 closeStep(maximumItems:number):boolean{return this.owner.closeStep(maximumItems);}
 terminalIsEmpty():boolean{return this.owner.terminalIsEmpty();}
 restore(retirement:FactoryBoxedPublication<T>,value:T):this{const owner=retirement.restore(value);if(owner!==this.owner)throw new Error("Window rejection changed the original issued carrier");return this;}
}
export class WindowIssuedPublication<T>{
 constructor(private original:WindowIssuedMutation<T>|null,readonly value:T,private retirement:FactoryBoxedPublication<T>){}
 reject():WindowIssuedMutation<T>{if(!this.original)throw new Error("Window publication has already transferred its original address");const original=this.original.restore(this.retirement,this.value);this.original=null;return original;}
 closeStep(maximumItems:number):boolean{const done=this.retirement.closeStep(maximumItems);if(done)this.original=null;return done;}
 terminalIsEmpty():boolean{return this.retirement.terminalIsEmpty()&&!this.original;}
}