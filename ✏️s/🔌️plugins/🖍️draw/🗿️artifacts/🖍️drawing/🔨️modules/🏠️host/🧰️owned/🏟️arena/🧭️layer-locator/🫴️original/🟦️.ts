import type {DrawingArtifact,DrawingLayerNode} from "../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts";
export interface DrawingLookupGrant{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export interface DrawingLookupProgress{copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number}
const empty=():DrawingLookupProgress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
const metadata=256,maximumDepth=64;
function funded(g:DrawingLookupGrant,copy:number,capacity=0,release=0){return g.maximumItems>0&&g.maximumCopyBytes>=copy&&g.maximumCapacityBytes>=capacity&&g.maximumReleaseBytes>=release&&g.maximumDepth>0}
function octet(text:string,offset:number,byte:number):readonly[number|undefined,number]{const cp=text.codePointAt(offset);if(cp===undefined)return[undefined,0];const width=cp<0x80?1:cp<0x800?2:cp<0x10000?3:4;return[byte===0?(width===1?cp:(width===2?0xc0:width===3?0xe0:0xf0)|(cp>>(6*(width-1)))):0x80|((cp>>(6*(width-1-byte)))&63),width]}
/** 🧭️ Logical portable custody retains genuine Drawing nodes and one immutable original target value until close. */
export class DrawingOriginalLookup{
 private root=0;private depth=0;private path:Int32Array|undefined;private phases:Uint8Array|undefined;private children:Int32Array|undefined;private result:DrawingLayerNode|undefined;private completed=false;private closing=false;private left=0;private right=0;private bytes=0;
 private constructor(private original:DrawingArtifact|undefined,private identity:Readonly<{value:string}>|undefined,private target:string|undefined){}
 static demand(){return{copyBytes:metadata,capacityBytes:maximumDepth*9,depth:1}}
 static admit(snapshot:DrawingArtifact,target:Readonly<{value:string}>,g:DrawingLookupGrant):{owner?:DrawingOriginalLookup;progress:DrawingLookupProgress}{if(!funded(g,metadata,maximumDepth*9))return{progress:empty()};const owner=new DrawingOriginalLookup(snapshot,target,target.value);owner.path=new Int32Array(maximumDepth);owner.phases=new Uint8Array(maximumDepth);owner.children=new Int32Array(maximumDepth);return{owner,progress:{copiedItems:1,copiedBytes:metadata,retainedCapacityBytes:maximumDepth*9,releasedBytes:0}}}
 private current():DrawingLayerNode|undefined{let node=this.original?.layers[this.root];for(let i=0;node&&i<this.depth;i++)node=node.kind==='group'?node.children[this.path![i]]:undefined;return node}
 advance(snapshot:DrawingArtifact,target:Readonly<{value:string}>,g:DrawingLookupGrant):DrawingLookupProgress{
  if(this.closing||snapshot!==this.original||target!==this.identity)throw Error('Drawing original lookup source changed');if(this.completed||!funded(g,metadata+2))return empty();const node=this.current();if(!node){this.completed=true;return{...empty(),copiedItems:1,copiedBytes:metadata}}
  let compared=0;if(this.phases![this.depth]===0){const[a,aw]=octet(node.id,this.left,this.bytes),[b]=octet(this.target!,this.right,this.bytes);compared=2;if(a===b&&a!==undefined){this.bytes++;if(this.bytes===aw){this.left+=node.id.codePointAt(this.left)!>0xffff?2:1;this.right+=this.target!.codePointAt(this.right)!>0xffff?2:1;this.bytes=0}}else if(a===b){this.result=node;this.completed=true}else{this.phases![this.depth]=1;this.left=0;this.right=0;this.bytes=0}}
  else if(node.kind==='group'&&this.children![this.depth]<node.children.length){if(this.depth+1>=maximumDepth)throw Error('Drawing original lookup depth exceeded');const index=this.children![this.depth]++;this.path![this.depth]=index;this.depth++;this.phases![this.depth]=0;this.children![this.depth]=0}
  else if(this.depth===0){this.root++;this.phases![0]=0;this.children![0]=0}else this.depth--;
  return{...empty(),copiedItems:1,copiedBytes:metadata+compared}
 }
 ready(){return!this.closing&&this.completed}
 node(){if(!this.ready())throw Error('Drawing original lookup is incomplete');return this.result}
 beginClose(){this.closing=true}
 close(g:DrawingLookupGrant):DrawingLookupProgress{const bytes=this.path?maximumDepth*9:0;if(!funded(g,metadata,0,bytes))return empty();this.path=undefined;this.phases=undefined;this.children=undefined;this.original=undefined;this.identity=undefined;this.target=undefined;this.result=undefined;this.closing=true;return{copiedItems:1,copiedBytes:metadata,retainedCapacityBytes:0,releasedBytes:bytes}}
 terminalIsEmpty(){return this.original===undefined&&this.identity===undefined&&this.path===undefined}
}
