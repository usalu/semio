/** 🔗️ Original read capabilities return to their exact issuer before witness closure. */
export interface ReturnedReadOwner{closeStep(maximumItems:number):boolean;terminalIsEmpty():boolean}
export interface OriginalReadOwner<T>{value():T;tryReturn():ReturnedReadOwner|undefined}
export class OriginalReadRetirement<T>{
 private original:OriginalReadOwner<T>|undefined;
 private returned:ReturnedReadOwner|undefined;
 constructor(original:OriginalReadOwner<T>){this.original=original;}
 originalValue():T|undefined{return this.original?.value();}
 closeStep(maximumItems:number):boolean{if(this.terminalIsEmpty())return true;if(maximumItems<1)return false;if(this.original){const returned=this.original.tryReturn();if(returned){this.returned=returned;this.original=undefined;}return false;}if(this.returned){this.returned.closeStep(1);if(this.returned.terminalIsEmpty())this.returned=undefined;return false;}return this.terminalIsEmpty();}
 terminalIsEmpty():boolean{return this.original===undefined&&this.returned===undefined;}
}
