export type MountedFaultGrant={items:number;copy:number;capacity:number;release:number;depth:number};
/** ⚠️ Holds the original fault through refused admission, active child and separate frame release. */
export class MountedFaultCustody<T>{
 private original:T|null;
 private active:T|null=null;
 private bodyEmpty=false;
 private captured=false;
 private refusal:T|null=null;
 constructor(value:T,public report:string|null,private readonly frameBytes:number,private readonly reportBytes:number){this.original=value;}
 get pending(){return this.original!==null||this.active!==null||this.refusal!==null;}
 get source(){return this.original??this.active;}
 capture(grant:MountedFaultGrant,code:(value:T)=>string){if(grant.items<1||grant.depth<1||grant.copy<(this.report===null?this.reportBytes:0))return false;if(!this.captured){this.report??=code(this.original!);this.captured=true;}return true;}
 admit(grant:MountedFaultGrant,accept:(value:T)=>boolean){if(!this.captured||this.active!==null||this.original===null||grant.items<1||grant.capacity<this.frameBytes||grant.depth<2)return false;if(!accept(this.original))return false;this.active=this.original;this.original=null;return true;}
 observeBodyTerminal(){if(this.active===null||this.refusal!==null)return false;this.bodyEmpty=true;return true;}
 retainRefusal(value:T){this.refusal=value;}
 releaseFrame(grant:MountedFaultGrant){if(!this.bodyEmpty||this.refusal!==null||grant.items<1||grant.release<this.frameBytes||grant.depth<1)return false;this.active=null;return true;}
}
