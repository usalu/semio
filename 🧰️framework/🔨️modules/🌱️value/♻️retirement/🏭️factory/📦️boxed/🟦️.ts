/** 📦️ Structural custody mirrors native boxed payload and issuer retirement. */
export interface FactoryPayloadClose{closeStep():boolean;terminalIsEmpty():boolean}
export interface FactoryBoxedIssuer<T>{retireOwned(original:T):FactoryPayloadClose;closeStep():boolean;terminalIsEmpty():boolean}
export type FactoryBox<T>={value:T};
/** 🧳️ Refused payload admission keeps the original value and captured issuer. */
export class FactoryBoxedValue<T>{
 private boxed:FactoryBox<T>|null;private original:FactoryBox<T>|null=null;private child:FactoryPayloadClose|null=null;private complete=false;private issuer:FactoryBoxedIssuer<T>|null;
 constructor(original:FactoryBox<T>,issuer:FactoryBoxedIssuer<T>){this.boxed=original;this.issuer=issuer;}
 restorePublication(original:FactoryBox<T>,issuer:FactoryBoxedIssuer<T>):void{if(!this.complete||this.boxed||this.original||this.child||this.issuer)throw new Error("Publication carrier no longer has transferred custody");this.boxed=original;this.issuer=issuer;this.complete=false;}
 originalBox():FactoryBox<T>|null{return this.boxed;}
 originalValue():T|null{return this.boxed?this.boxed.value:this.original?this.original.value:null;}
 terminalIsEmpty():boolean{return this.complete;}
 takeForPublication():{value:T;retirement:FactoryBoxedPublication<T>}{if(!this.boxed||this.original||this.child||this.complete)throw new Error("Boxed publication requires untouched original custody");const original=this.boxed,issuer=this.issuer!;this.boxed=null;this.issuer=null;this.complete=true;return{value:original.value,retirement:new FactoryBoxedPublication(original,this,issuer)};}
 closeStep(maximumItems:number):boolean{if(!Number.isSafeInteger(maximumItems)||maximumItems<0)throw new RangeError("Invalid boxed retirement item grant");for(let at=0;at<maximumItems&&!this.complete;at++){if(this.boxed){this.original={value:this.boxed.value};this.boxed=null;}else if(this.original!==null){this.child=this.issuer!.retireOwned(this.original.value);this.original=null;}else if(this.child){const done=this.child.closeStep();if(done&&!this.child.terminalIsEmpty())throw new Error("Boxed payload close falsely reported completion");if(done)this.child=null;}else{const done=this.issuer!.closeStep();if(done&&!this.issuer!.terminalIsEmpty())throw new Error("Boxed issuer close falsely reported completion");this.complete=done;if(done)this.issuer=null;}}return this.complete;}
}
/** 📭️ Published values keep their original carrier and issuer until explicit close. */
export class FactoryBoxedPublication<T>{
 constructor(private payload:FactoryBox<T>|null,private carrier:FactoryBoxedValue<T>|null,private issuer:FactoryBoxedIssuer<T>|null){}
 restore(value:T):FactoryBoxedValue<T>{if(!this.payload||!this.carrier||!this.issuer)throw new Error("Rejected publication requires untouched original metadata");const payload=this.payload,carrier=this.carrier,issuer=this.issuer;payload.value=value;carrier.restorePublication(payload,issuer);this.payload=null;this.carrier=null;this.issuer=null;return carrier;}
 terminalIsEmpty():boolean{return!this.payload&&!this.carrier&&!this.issuer;}
 closeStep(maximumItems:number):boolean{if(!Number.isSafeInteger(maximumItems)||maximumItems<0)throw new RangeError("Invalid boxed publication item grant");for(let at=0;at<maximumItems&&!this.terminalIsEmpty();at++){if(this.payload)this.payload=null;else if(this.carrier)this.carrier=null;else{const issuer=this.issuer!,done=issuer.closeStep();if(done&&!issuer.terminalIsEmpty())throw new Error("Publication issuer falsely reported completion");if(done)this.issuer=null;}}return this.terminalIsEmpty();}
}
