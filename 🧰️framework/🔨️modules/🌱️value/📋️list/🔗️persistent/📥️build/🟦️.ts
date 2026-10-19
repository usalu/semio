/** 📥️ Original owned rows form fixed-fanout pages through separate carry and finishing turns. */
type Page<T>={kind:"leaf";rows:readonly T[];length:number;height:number}|{kind:"branch";children:readonly Page<T>[];length:number;height:number};
const FANOUT=16,LEVELS=14;
const issuer=Symbol("original persistent page producer");
export type PersistentGrant={maximumItems:number;maximumCopyBytes:number;maximumDepth:number};
export class BuiltPersistentList<T> implements Iterable<T>{
 #page:Page<T>|null;
 #identity:object;
 constructor(page:Page<T>|null,authority:symbol,original?:object){if(authority!==issuer)throw Error("original persistent page producer required");this.#page=page;this.#identity=original??{};}
 get originalAuthority():object{return this.#identity;}
 project(grant:PersistentGrant):BuiltPersistentList<T>|undefined{if(grant.maximumItems<1||grant.maximumCopyBytes<256||grant.maximumDepth<1)return;return new BuiltPersistentList(this.#page,issuer,this.#identity);}
 get length():number{return this.#page?.length??0;}
 get height():number|null{return this.#page?.height??null;}
 get(index:number):T|undefined{if(!Number.isSafeInteger(index)||index<0||index>=this.length)return;let page=this.#page;while(page?.kind==="branch"){let next:Page<T>|undefined;for(const child of page.children){if(index<child.length){next=child;break;}index-=child.length;}page=next??null;}return page?.kind==="leaf"?page.rows[index]:undefined;}
 rankDemand(index:number):{copyBytes:number;depth:number}{if(!Number.isSafeInteger(index)||index<0||index>=this.length)throw Error("rank is outside original persistent pages");const depth=(this.height??0)+1;return{copyBytes:depth*256,depth};}
 observeRank(index:number,grant:PersistentGrant):T|undefined{const d=this.rankDemand(index);if(grant.maximumItems<1||grant.maximumCopyBytes<d.copyBytes||grant.maximumDepth<d.depth)return;let page=this.#page;while(page?.kind==="branch"){let next:Page<T>|undefined;for(const child of page.children){if(index<child.length){next=child;break;}index-=child.length;}page=next??null;}return page?.kind==="leaf"?page.rows[index]:undefined;}
 beginSet(index:number,row:T):PersistentListSet<T>{this.rankDemand(index);return new PersistentListSet(this.#page!,index,row,issuer);}
 *[Symbol.iterator]():Iterator<T>{if(!this.#page)return;const stack:Page<T>[]=[this.#page];while(stack.length){const page=stack.pop()!;if(page.kind==="leaf")yield*page.rows;else for(let i=page.children.length-1;i>=0;i--)stack.push(page.children[i]!);}}
 close():void{this.#page=null;}
}
/** 🪡️ The actual producer replaces one immutable page path and borrows every unaddressed row. */
export class PersistentListSet<T>{
 #root:Page<T>|null;#current:Page<T>|null;#rank:number;#row:T|undefined;#path:{page:Extract<Page<T>,{kind:"branch"}>;slot:number}[]=[];#slots:(T|Page<T>)[]=[];#slot=0;#candidate:Page<T>|null=null;#phase:"descend"|"copy"|"seal"|"ascend"|"ready"|"spent"|"closed"="descend";
 constructor(root:Page<T>,rank:number,row:T,authority:symbol){if(authority!==issuer)throw Error("original changed-path producer required");this.#root=root;this.#current=root;this.#rank=rank;this.#row=row;}
 get ready():boolean{return this.#phase==="ready";}
 get empty():boolean{return this.#root===null&&this.#current===null&&this.#path.length===0&&this.#slots.length===0&&this.#candidate===null&&this.#row===undefined;}
 demand():{copyBytes:number;depth:number}{if(["spent","closed"].includes(this.#phase))throw Error("original changed-path producer is terminal");return{copyBytes:512,depth:1};}
 advance(grant:PersistentGrant):boolean{const d=this.demand();if(grant.maximumItems<1||grant.maximumCopyBytes<d.copyBytes||grant.maximumDepth<d.depth)return false;const page=this.#current!;
  if(this.#phase==="descend"){if(page.kind==="leaf"){this.#phase="copy";return true;}let rank=this.#rank,slot=0;while(slot<page.children.length&&rank>=page.children[slot]!.length){rank-=page.children[slot]!.length;slot++;}this.#path.push({page,slot});this.#current=page.children[slot]!;this.#rank=rank;return true;}
  if(this.#phase==="copy"){const target=page.kind==="leaf"?this.#rank:this.#path[this.#path.length-1]!.slot;const slots=page.kind==="leaf"?page.rows:page.children;this.#slots.push(this.#slot===target?(page.kind==="leaf"?this.#row!:this.#candidate!):slots[this.#slot]!);if(++this.#slot===slots.length)this.#phase="seal";return true;}
  if(this.#phase==="seal"){this.#candidate=page.kind==="leaf"?{kind:"leaf",rows:this.#slots as T[],length:page.length,height:0}:{kind:"branch",children:this.#slots as Page<T>[],length:page.length,height:page.height};this.#slots=[];this.#slot=0;this.#phase="ascend";return true;}
  if(this.#phase==="ascend"){if(page.kind==="branch")this.#path.pop();if(this.#path.length){this.#current=this.#path[this.#path.length-1]!.page;this.#phase="copy";}else{this.#phase="ready";this.#root=null;this.#current=null;this.#row=undefined;}return true;}
  return true;
 }
 take():BuiltPersistentList<T>{if(!this.ready)throw Error("original changed-path root is not ready");const page=this.#candidate;this.#candidate=null;this.#phase="spent";return new BuiltPersistentList(page,issuer);}
 close():void{this.#root=null;this.#current=null;this.#row=undefined;this.#path=[];this.#slots=[];this.#candidate=null;this.#phase="closed";}
}
export class PersistentListBuilder<T>{
 #expected:number;#adopted=0;#rows:T[]=[];#levels:Page<T>[][]=Array.from({length:LEVELS},()=>[]);#carry:Page<T>|null=null;#output:Page<T>|null=null;#level=0;#phase:"wait"|"leaf"|"carry"|"branch"|"scan"|"ready"|"spent"|"closed"="wait";#finishing=false;
 constructor(expected:number){if(!Number.isSafeInteger(expected)||expected<0)throw Error("original persistent cardinality must be a nonnegative safe integer");this.#expected=expected;}
 get acceptsRow():boolean{return this.#phase==="wait"&&!this.#finishing;}
 get ready():boolean{return this.#phase==="ready";}
 get empty():boolean{return this.#phase==="closed"&&this.#rows.length===0&&this.#levels.every(rows=>rows.length===0)&&this.#carry===null&&this.#output===null;}
 adopt(row:T):void{if(!this.acceptsRow||this.#adopted>=this.#expected)throw Error("original persistent row is outside its admitted stream");this.#rows.push(row);this.#adopted++;if(this.#rows.length===FANOUT)this.#phase="leaf";}
 finish():void{if(this.#finishing||this.#adopted!==this.#expected||["spent","closed"].includes(this.#phase))throw Error("original persistent stream has not supplied its exact cardinality");this.#finishing=true;if(this.#phase==="wait")this.#phase=this.#rows.length?"leaf":"scan";}
 advance():void{
  if(this.#phase==="leaf"){const rows=this.#rows;this.#rows=[];this.#carry={kind:"leaf",rows,length:rows.length,height:0};this.#level=0;this.#phase="carry";return;}
  if(this.#phase==="carry"){const level=this.#levels[this.#level];if(!level||!this.#carry)throw Error("original persistent carry exceeds ranked address space");level.push(this.#carry);this.#carry=null;this.#phase=level.length===FANOUT?"branch":this.#finishing?"scan":"wait";return;}
  if(this.#phase==="branch"){const children=this.#levels[this.#level];if(!children?.length)throw Error("original persistent branch has no children");this.#levels[this.#level]=[];this.#carry={kind:"branch",children,length:children.reduce((length,child)=>length+child.length,0),height:this.#level+1};this.#level++;this.#phase="carry";return;}
  if(this.#phase==="scan"){const level=this.#levels.findIndex(rows=>rows.length>0);if(level<0){this.#phase="ready";return;}const highest=this.#levels.findLastIndex(rows=>rows.length>0);const rows=this.#levels[level]!;if(level===highest&&rows.length===1){this.#output=rows[0]!;this.#levels[level]=[];this.#phase="ready";return;}this.#level=level;this.#phase="branch";return;}
  throw Error("original persistent builder has no pending preparation turn");
 }
 take():BuiltPersistentList<T>{if(!this.ready)throw Error("original persistent candidate is not ready");const output=this.#output;this.#output=null;this.#phase="spent";return new BuiltPersistentList(output,issuer);}
 close():void{this.#rows=[];for(let i=0;i<this.#levels.length;i++)this.#levels[i]=[];this.#carry=null;this.#output=null;this.#phase="closed";}
}
