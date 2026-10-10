/** 🫴️ Logical scoped children forward into one original decoder and explicit recipients. */
import {NativeDecodeControl} from "../🟦️.ts";
import {ValueError} from "../../⚠️refusal/🟦️.ts";
export class NativeDecodeRetirementRecipient{
 reserved=false;
 owner:object|undefined;
 get empty(){return !this.reserved&&this.owner===undefined;}
 take(items:number){if(items<=0||this.reserved)return undefined;const owner=this.owner;this.owner=undefined;return owner;}
}
type Root={control:NativeDecodeControl,depth:number};
export class NativeDecodeChildLoan{
 private active=true;
 private constructor(private root:Root,private recipient:NativeDecodeRetirementRecipient,private depth:number){}
 static original(control:NativeDecodeControl,recipient:NativeDecodeRetirementRecipient){if(!recipient.empty)throw new ValueError("ownershipLimit","native decode needs an empty original recipient");return new NativeDecodeChildLoan({control,depth:0},recipient,0);}
 private enter(){if(!this.active||this.depth!==this.root.depth)throw new ValueError("ownershipLimit","native decoder loan is not exclusive or has returned");}
 get ledgerIdentity(){this.enter();return this.root.control;}
 async charge(bytes:number){this.enter();await this.root.control.charge(bytes);}
 async checkpoint(){this.enter();await this.root.control.checkpoint();}
 async beginStage(total:number){this.enter();await this.root.control.beginStage(total);}
 async advance(units:number){this.enter();await this.root.control.advance(units);}
 async scopedStage<T>(operation:(loan:NativeDecodeChildLoan)=>Promise<T>){this.enter();return this.root.control.scopedStage(()=>operation(this));}
 async withOwner<T>(wrapper:number,operation:(loan:NativeDecodeChildLoan)=>Promise<{value?:T,error?:unknown,owner?:object}>){this.enter();if(!this.recipient.empty)throw new ValueError("ownershipLimit","native decode has no available original slot");await this.charge(wrapper);await this.checkpoint();this.recipient.reserved=true;try{const outcome=await operation(this);this.recipient.owner=outcome.owner;if(outcome.error!==undefined)throw outcome.error;return outcome.value;}finally{this.recipient.reserved=false;}}
 async withChild<T>(recipient:NativeDecodeRetirementRecipient,operation:(loan:NativeDecodeChildLoan)=>Promise<T>){this.enter();if(!this.recipient.reserved||!recipient.empty)throw new ValueError("ownershipLimit","native child needs active parent and empty original recipient");this.root.depth++;const child=new NativeDecodeChildLoan(this.root,recipient,this.root.depth);try{return await operation(child);}finally{child.active=false;this.root.depth--;}}
}
