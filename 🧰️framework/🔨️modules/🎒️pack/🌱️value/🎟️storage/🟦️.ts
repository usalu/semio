/** 🎟️ Logical twin of original stack storage with independently declared native layout dimensions. */
export interface PackStackGrant{items:number;copy:number;capacity:number;release:number;depth:number}
export interface PackStackLayout{vector:number;frame:number;length:number;pending:number}
export class PackStackStorage{
 private allocated=0;private roots=0;private pending=false;private closed=false;
 constructor(private readonly frames:number,private readonly layout:PackStackLayout){}
 admissionDemand():PackStackGrant{return this.allocated===0?{items:1,copy:this.layout.vector,capacity:this.frames*this.layout.frame,release:0,depth:1}:PackStackStorage.zero()}
 admit(grant:PackStackGrant):PackStackGrant{const demand=this.admissionDemand();if(!PackStackStorage.funded(grant,demand)||demand.items===0)return PackStackStorage.zero();this.allocated=demand.capacity;return demand}
 initialize(roots:number,pending:boolean):void{if(this.allocated===0||roots<0||roots>2||(pending&&roots!==2))throw Error("Invalid original stack roots");this.roots=roots;this.pending=pending}
 stage():string{return this.pending?"pending":this.roots>0?"frame":this.allocated!==0?"backing":"complete"}
 retirementDemand():PackStackGrant{if(this.closed)return PackStackStorage.zero();const stage=this.stage();return{items:1,copy:stage==="pending"?this.layout.pending+1:stage==="frame"?this.layout.frame+this.layout.length+2:stage==="backing"?this.layout.vector+2:3,capacity:0,release:stage==="backing"?this.allocated:0,depth:1}}
 retire(grant:PackStackGrant):PackStackGrant{const demand=this.retirementDemand();if(demand.items===0||!PackStackStorage.funded(grant,demand))return PackStackStorage.zero();if(this.pending)this.pending=false;else if(this.roots>0)this.roots--;else if(this.allocated!==0)this.allocated=0;else this.closed=true;return demand}
 terminal():boolean{return this.closed}
 private static zero():PackStackGrant{return{items:0,copy:0,capacity:0,release:0,depth:0}}
 private static funded(grant:PackStackGrant,demand:PackStackGrant):boolean{return grant.items>=demand.items&&grant.copy>=demand.copy&&grant.capacity>=demand.capacity&&grant.release>=demand.release&&grant.depth>=demand.depth}
}

/** 📋️ Eager logical page custody progresses independently of unconsumed reserved entries. */
export class PackEagerCapacity{
 private admitted=0;private metadata=false;private original:object|undefined;
 constructor(private readonly pageItems:number,private readonly maximum:number){}
 capacity():number{return this.admitted}
 capture(original:object):void{if(this.original!==undefined||this.admitted===0)throw Error("Missing original reserved capacity");this.original=original}
 borrowed():object|undefined{return this.original}
 next(target:number):"metadata"|"payload"|undefined{if(target<0||target>this.maximum)throw Error("Original capacity limit");if(target<=this.admitted)return undefined;return this.metadata?"payload":"metadata"}
 reserve(target:number,grant:PackStackGrant):boolean{const stage=this.next(target);if(stage===undefined||grant.items<1||grant.copy<1||grant.capacity<1||grant.depth<1)return false;if(stage==="metadata")this.metadata=true;else{this.admitted=Math.min(this.maximum,this.admitted+this.pageItems);this.metadata=false;}return true}
}

/** 🏷️ Logical original Unicode scalars and spans remain private through cancellation. */
export class PackSymbolStorage{
 private readonly scalars:string[]=[];private readonly spans:{start:number;count:number;utf8:number}[]=[];private published=0;private pendingBytes=0;private closed=false;
 constructor(private readonly maximumSymbols:number,private readonly maximumScalars:number,private readonly maximumBytes:number){}
 append(character:string):void{if(this.closed||Array.from(character).length!==1||this.scalars.length===this.maximumScalars)throw Error("Original scalar refusal");const bytes=new TextEncoder().encode(character).length;if(this.pendingBytes+bytes+this.spans.reduce((sum,span)=>sum+span.utf8,0)>this.maximumBytes)throw Error("Original UTF8 refusal");this.scalars.push(character);this.pendingBytes+=bytes}
 publish():void{if(this.closed||this.spans.length===this.maximumSymbols)throw Error("Original span refusal");this.spans.push({start:this.published,count:this.scalars.length-this.published,utf8:this.pendingBytes});this.published=this.scalars.length;this.pendingBytes=0}
 symbolCount():number{return this.spans.length}
 scalarCount():number{return this.scalars.length}
 symbolChar(symbol:number,ordinal:number):string|undefined{const span=this.spans[symbol];return span!==undefined&&ordinal>=0&&ordinal<span.count?this.scalars[span.start+ordinal]:undefined}
 close(grant:PackStackGrant):boolean{if(grant.items<1||grant.copy<1||grant.depth<1)return false;if(this.scalars.length!==0)this.scalars.pop();else if(this.spans.length!==0)this.spans.pop();else{this.published=0;this.pendingBytes=0;this.closed=true;}return true}
 terminal():boolean{return this.closed}
}
