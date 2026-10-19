/** 🪢️ Semantic reference keeps original chunk custody under independent work currencies. */
export type OriginalVectorConcatGrant={maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number};
export type OriginalVectorConcatDemand={copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number};
export type OriginalVector<T>={rows:T[]};
const fits=(d:OriginalVectorConcatDemand,g:OriginalVectorConcatGrant)=>g.maximumItems>0&&g.maximumCopyBytes>=d.copyBytes&&g.maximumCapacityBytes>=d.capacityBytes&&g.maximumReleaseBytes>=d.releaseBytes&&g.maximumDepth>=d.depth;
/** 🧳️ Owned original row directories advance by index instead of shifting or copying payloads. */
export class OriginalVectorConcat<T>{
 private readonly chunks:{rows:(T|undefined)[];index:number}[]=[];
 private output:T[]|undefined;
 private index=0;
 private total=0;
 private sealed=false;
 private closing=false;
 constructor(readonly rowHeaderBytes:number){if(!Number.isSafeInteger(rowHeaderBytes)||rowHeaderBytes<0)throw Error("invalid original row header size");}
 get ready():boolean{return this.sealed&&!this.closing&&this.index===this.chunks.length&&this.output!==undefined;}
 get terminal():boolean{return this.closing&&this.chunks.length===0&&this.output===undefined;}
 private quote(capacityBytes=0,releaseBytes=0):OriginalVectorConcatDemand{return{copyBytes:128,capacityBytes,releaseBytes,depth:1};}
 pushDemand():OriginalVectorConcatDemand{if(this.sealed||this.closing)throw Error("original concatenation is settled");return this.quote(24);}
 push(input:OriginalVector<T>,grant:OriginalVectorConcatGrant):boolean{if(!fits(this.pushDemand(),grant))return false;this.total+=input.rows.length;this.chunks.push({rows:input.rows,index:0});input.rows=[];return true;}
 finishDemand():OriginalVectorConcatDemand{if(this.sealed||this.closing)throw Error("original concatenation is settled");return this.quote(this.total*this.rowHeaderBytes);}
 finish(grant:OriginalVectorConcatGrant):boolean{if(!fits(this.finishDemand(),grant))return false;this.sealed=true;this.output=[];return true;}
 demand():OriginalVectorConcatDemand{if(!this.sealed||this.closing||this.ready)throw Error("original concatenation is not advancing");const chunk=this.chunks[this.index]!;return this.quote(0,chunk.index===chunk.rows.length?chunk.rows.length*this.rowHeaderBytes:0);}
 advance(grant:OriginalVectorConcatGrant):boolean{if(!fits(this.demand(),grant))return false;const chunk=this.chunks[this.index]!;if(chunk.index<chunk.rows.length){const value=chunk.rows[chunk.index]!;chunk.rows[chunk.index++]=undefined;this.output!.push(value);}else{chunk.rows=[];this.index++;}return true;}
 takeDemand():OriginalVectorConcatDemand{if(!this.ready)throw Error("original concatenation output is not ready");return this.quote();}
 take(grant:OriginalVectorConcatGrant):T[]|undefined{if(!fits(this.takeDemand(),grant))return undefined;const output=this.output!;this.output=undefined;return output;}
 beginClose():void{this.closing=true;}
 closeDemand():OriginalVectorConcatDemand{if(!this.closing)throw Error("original concatenation close has not begun");return this.quote();}
 closeStep(grant:OriginalVectorConcatGrant):boolean{if(!fits(this.closeDemand(),grant))return false;const chunk=this.chunks.at(-1);if(chunk){if(chunk.index<chunk.rows.length)chunk.rows[chunk.index++]=undefined;else this.chunks.pop();}else if(this.output?.length)this.output.pop();else this.output=undefined;return true;}
}
