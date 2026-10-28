/** 👁️ Original post custody is issued by the selected host and returned before mutable access. */
export type PostMode='original'|'mutable';
export interface PostDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface PostGrant extends PostDemand{items:number}
export interface PostProgress{items:number;copyBytes:number;capacityBytes:number;releaseBytes:number}
export interface OriginalPostRef<S>{readonly value:S;readonly identity:object;readonly projection:number}
export interface OriginalPostAuthority<S>{borrow():OriginalPostRef<S>;takeDemand(body:number):PostDemand;take(grant:PostGrant):{kind:'pending';progress:PostProgress}|{kind:'ready';value:S;progress:PostProgress};terminalIsEmpty():boolean}
export interface OriginalPostIssuer<S>{openingDemand(value:S):PostDemand;admit(value:S,grant:PostGrant):{source:OriginalPostAuthority<S>;progress:PostProgress}}
export interface OriginalPostSlot<S>{value:S|undefined}
const zero=():PostProgress=>({items:0,copyBytes:0,capacityBytes:0,releaseBytes:0});
const fits=(d:PostDemand,g:PostGrant)=>g.items>0&&g.copyBytes>=d.copyBytes&&g.capacityBytes>=d.capacityBytes&&g.releaseBytes>=d.releaseBytes&&g.depth>=d.depth;
export class OriginalPostRead<S>{
 private source:OriginalPostAuthority<S>|undefined;
 static readonly logicalMovementBytes=128;
 constructor(private readonly issuer:OriginalPostIssuer<S>){}
 demand(post:Readonly<OriginalPostSlot<S>>,mode:PostMode,body:number):PostDemand|undefined{
  if((post.value!==undefined)===(this.source!==undefined))throw Error('Exactly one original post owner is required');
  const child=mode==='original'&&this.source===undefined?this.issuer.openingDemand(post.value!):mode==='mutable'&&this.source!==undefined?this.source.takeDemand(Math.max(0,body-OriginalPostRead.logicalMovementBytes)):undefined;
  return child===undefined?undefined:{...child,copyBytes:child.copyBytes+OriginalPostRead.logicalMovementBytes,depth:child.depth+1};
 }
 prepare(post:OriginalPostSlot<S>,mode:PostMode,body:number,grant:PostGrant):PostProgress|undefined{
  const d=this.demand(post,mode,body);if(d===undefined)return undefined;if(!fits(d,grant))return zero();
  const child={...grant,copyBytes:grant.copyBytes-OriginalPostRead.logicalMovementBytes,depth:grant.depth-1};let progress:PostProgress;
  if(mode==='original'){const admitted=this.issuer.admit(post.value!,child);this.source=admitted.source;post.value=undefined;progress=admitted.progress;}
  else{const result=this.source!.take(child);progress=result.progress;if(result.kind==='ready'){if(!this.source!.terminalIsEmpty())throw Error('Original post source has outstanding custody');post.value=result.value;this.source=undefined;}}
  return{...progress,items:Math.max(1,progress.items),copyBytes:progress.copyBytes+OriginalPostRead.logicalMovementBytes};
 }
 original(post:Readonly<OriginalPostSlot<S>>):OriginalPostRef<S>{if(this.demand(post,'original',0)!==undefined)throw Error('Original post opening is not funded');return this.source!.borrow();}
 mutable(post:Readonly<OriginalPostSlot<S>>):S{if(this.demand(post,'mutable',0)!==undefined)throw Error('Original post reads must close before mutable access');return post.value!;}
 terminalIsEmpty():boolean{return this.source===undefined;}
}
