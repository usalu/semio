/** 📦️ Models exact scope detachment while its original node owners remain retained. */
export interface LeaseScopeSource {keyed:boolean;active:number;documentActive:number;operation:boolean;document:boolean}
export interface LeaseDetachGrant {items:number;copy:number;capacity:number;release:number;depth:number}
export class LeaseDetachCursor{
 detached=false;
 operation=false;
 document=false;
 constructor(readonly source:LeaseScopeSource,readonly copyDemand:number){}
 step(grant:LeaseDetachGrant):boolean{
  if(this.detached||grant.items<1||grant.copy<this.copyDemand||grant.depth<1)return false;
  if(this.source.operation){this.operation=true;this.source.operation=false;this.source.active--;if(this.source.keyed){this.source.documentActive--;if(this.source.documentActive===0){this.document=true;this.source.document=false;}}}
  this.detached=true;return true;
 }
}
