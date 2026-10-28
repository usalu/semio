/** 🧮️ Logical numeric storage capacity; reservations convey no source or native allocation certificate. */
export interface SolidValueReservation{readonly values:number;}
/** 🏕️ Private identity grants retain exact safe-integer live capacity across owned producer phases. */
export class SolidValueWorkspace{
 readonly #limit:number;readonly #leases=new Map<SolidValueReservation,number>();#used=0;#peak=0;#closed=false;#failure?:Error;
 constructor(maxValues:number){if(!Number.isSafeInteger(maxValues)||maxValues<1)throw Error('boolean-workspace-invalid-limit');this.#limit=maxValues;}
 get used():number{return this.#used;}
 get peak():number{return this.#peak;}
 get closed():boolean{return this.#closed;}
 #refuse(reason:string):never{this.#failure??=Error('boolean-workspace-'+reason);throw this.#failure;}
 #ready():void{if(this.#failure)throw this.#failure;if(this.#closed)this.#refuse('closed');}
 #values(values:number):void{if(!Number.isSafeInteger(values)||values<0)this.#refuse('invalid-values');}
 #owned(lease:SolidValueReservation):number{const values=this.#leases.get(lease);if(values===undefined)return this.#refuse('reservation');return values;}
 #lease(values:number):SolidValueReservation{const owner=this,lease:SolidValueReservation=Object.freeze({get values():number{return owner.#owned(lease);}});this.#leases.set(lease,values);return lease;}
 #admit(values:number):void{if(values>this.#limit-this.#used)this.#refuse('limit');this.#used+=values;this.#peak=Math.max(this.#peak,this.#used);}
 reserve(values:number):SolidValueReservation{this.#ready();this.#values(values);this.#admit(values);return this.#lease(values);}
 grow(lease:SolidValueReservation,values:number):void{this.#ready();const current=this.#owned(lease);this.#values(values);this.#admit(values);this.#leases.set(lease,current+values);}
 shrink(lease:SolidValueReservation,values:number):void{this.#ready();const current=this.#owned(lease);this.#values(values);if(values>current)this.#refuse('invalid-values');this.#leases.set(lease,current-values);this.#used-=values;}
 split(lease:SolidValueReservation,values:number):SolidValueReservation{this.#ready();const current=this.#owned(lease);this.#values(values);if(values>current)this.#refuse('invalid-values');this.#leases.set(lease,current-values);return this.#lease(values);}
 merge(target:SolidValueReservation,source:SolidValueReservation):void{this.#ready();const left=this.#owned(target),right=this.#owned(source);if(target===source)this.#refuse('reservation');this.#leases.set(target,left+right);this.#leases.delete(source);}
 release(lease:SolidValueReservation):void{const values=this.#owned(lease);this.#leases.delete(lease);this.#used-=values;}
 close():void{this.#leases.clear();this.#used=0;this.#closed=true;}
}
