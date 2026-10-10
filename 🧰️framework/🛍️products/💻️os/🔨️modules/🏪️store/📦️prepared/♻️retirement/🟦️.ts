/** 📦️ Portable candidate custody mirrors native edit, root, seal and metadata ownership. */
import {FactoryBoxedValue,type FactoryBoxedIssuer} from "../../../../../../🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🟦️.ts";
export interface PreparedCandidate<E,P,A>{edit:E;post:P;authority:A;identities:string[]}
export interface PreparedCandidateIssuers<E,P,A>{edit:FactoryBoxedIssuer<E>;post:FactoryBoxedIssuer<P>;authority:FactoryBoxedIssuer<A>}
export class PreparedCandidateRetirement<E,P,A>{
 private edit:FactoryBoxedValue<E>|null;private post:FactoryBoxedValue<P>|null;private authority:FactoryBoxedValue<A>|null;private original:PreparedCandidate<E,P,A>|null;
 constructor(original:PreparedCandidate<E,P,A>,issuers:PreparedCandidateIssuers<E,P,A>){this.original=original;this.edit=new FactoryBoxedValue({value:original.edit},issuers.edit);this.post=new FactoryBoxedValue({value:original.post},issuers.post);this.authority=new FactoryBoxedValue({value:original.authority},issuers.authority);}
 terminalIsEmpty():boolean{return!this.original&&!this.edit&&!this.post&&!this.authority;}
 closeStep(maximumItems:number):boolean{if(!Number.isSafeInteger(maximumItems)||maximumItems<0)throw new RangeError("Invalid prepared retirement grant");for(let at=0;at<maximumItems&&!this.terminalIsEmpty();at++){if(this.edit){if(this.edit.closeStep(1))this.edit=null;}else if(this.post){if(this.post.closeStep(1))this.post=null;}else if(this.authority){if(this.authority.closeStep(1))this.authority=null;}else if(this.original!.identities.length)this.original!.identities.pop();else this.original=null;}return this.terminalIsEmpty();}
}