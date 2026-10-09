/** 📋️ World-space clipboard placement and identity remapping are renderer-independent. */
import {parseDrawingLayerNode,type DrawingLayerNode,type DrawingImageAsset} from "../../🧬️schema/🟦️.ts";
export type ClipboardMatrix=[number,number,number,number,number,number];
export interface DrawingClipboard {schema:"drawing.clipboard.v1";roots:DrawingLayerNode[];selected:string[];assets:Record<string,DrawingImageAsset>}
/** 🔎️ Packet admission validates one physical node, dependency edge or selection per work unit. */
export class DrawingClipboardValidationJob {
 private phase:"layers"|"references"|"cycles"|"selection"|"assets"|"complete"="layers";
 private work=0;private at=0;private reference=0;private cancelled=false;private failure:Error|undefined;
 private nodes=new Map<string,DrawingLayerNode>();private order:DrawingLayerNode[]=[];private selection=new Set<string>();
 private physical:{node:DrawingLayerNode;child:number;depth:number}[]=[];private graph:{id:string;edge:number}[]=[];private colors=new Map<string,number>();private assetKeys:string[];
 constructor(private packet:DrawingClipboard){this.assetKeys=Object.keys(packet.assets);if(packet.schema!=="drawing.clipboard.v1"||!packet.roots.length||packet.roots.length>1024||!packet.selected.length||packet.selected.length>1024||this.assetKeys.length>1024)throw Error("Invalid drawing clipboard packet");}
 cancel():void{this.cancelled=true;}
 advance(grant:number):{done:boolean;phase:string;work:number}{if(!Number.isSafeInteger(grant)||grant<1||grant>4096)throw Error("Invalid clipboard work grant");if(this.failure)throw this.failure;if(this.cancelled)throw Error("Clipboard validation cancelled");try{for(let count=0;count<grant&&this.phase!=="complete";count++){this.step();this.work++;}}catch(error){this.failure=error instanceof Error?error:Error(String(error));throw this.failure;}return {done:this.phase==="complete",phase:this.phase,work:this.work};}
 result():void{if(this.failure)throw this.failure;if(this.cancelled||this.phase!=="complete")throw Error("Clipboard validation is incomplete");}
 private identity(id:string):void{if(!id||new TextEncoder().encode(id).length>4096)throw Error("Invalid clipboard identity");}
 private edge(node:DrawingLayerNode,index:number):string|undefined{return node.kind==="boolean"?node.children[index]:node.kind==="group"?node.children[index]?.id:undefined;}
 private step():void{
  if(this.phase==="layers"){
   if(!this.physical.length){if(this.at===this.packet.roots.length){this.phase="references";this.at=0;return;}this.physical.push({node:this.packet.roots[this.at++]!,child:-1,depth:1});return;}
   const frame=this.physical.at(-1)!,node=frame.node;
   if(frame.child===-1){this.identity(node.id);if(this.nodes.has(node.id)||this.nodes.size===1024)throw Error("Duplicate clipboard identity");this.nodes.set(node.id,node);this.order.push(node);frame.child=0;return;}
   if(node.kind==="group"&&frame.child<node.children.length){if(frame.depth===32)throw Error("Clipboard ancestry exceeds capacity");this.physical.push({node:node.children[frame.child++]!,child:-1,depth:frame.depth+1});return;}this.physical.pop();return;
  }
  if(this.phase==="references"){
   if(this.at===this.order.length){this.phase="cycles";this.at=0;return;}const node=this.order[this.at]!;
   if(node.kind==="boolean"){if(!["union","difference","intersection","xor"].includes(node.operation)||node.children.length>1024)throw Error("Unsupported clipboard Boolean operation");if(this.reference<node.children.length){const id=node.children[this.reference++]!;this.identity(id);if(!this.nodes.has(id))throw Error("Clipboard reference is missing");return;}}
   const asset=node.kind==="image"?node.imageKey:node.kind==="trace"?node.sourceKey:undefined;if(asset!==undefined){this.identity(asset);if(!this.packet.assets[asset])throw Error("Clipboard asset is missing");}this.at++;this.reference=0;return;
  }
  if(this.phase==="cycles"){
   if(!this.graph.length){if(this.at===this.order.length){this.phase="selection";this.at=0;return;}const id=this.order[this.at++]!.id;if(this.colors.get(id)===2)return;this.colors.set(id,1);this.graph.push({id,edge:0});return;}
   const frame=this.graph.at(-1)!,node=this.nodes.get(frame.id)!,id=this.edge(node,frame.edge);if(id===undefined){this.colors.set(frame.id,2);this.graph.pop();return;}frame.edge++;if(this.colors.get(id)===1)throw Error("Clipboard dependency cycle");if(this.colors.get(id)===2)return;this.colors.set(id,1);this.graph.push({id,edge:0});return;
  }
  if(this.phase==="selection"){if(this.at===this.packet.selected.length){this.phase="assets";this.at=0;return;}const id=this.packet.selected[this.at++]!;this.identity(id);if(!this.nodes.has(id)||this.selection.has(id))throw Error("Invalid clipboard selection");this.selection.add(id);return;}
  if(this.phase==="assets"){if(this.at===this.assetKeys.length){this.phase="complete";return;}const id=this.assetKeys[this.at++]!,asset=this.packet.assets[id]!;this.identity(id);if(!Number.isSafeInteger(asset.width)||!Number.isSafeInteger(asset.height)||asset.width<1||asset.height<1||asset.width*asset.height!==asset.samples.length)throw Error("Invalid clipboard image dimensions");}
 }
}
export function clipboardPasteMatrix(source:ClipboardMatrix,destination:ClipboardMatrix,offset:[number,number],boolean=false):ClipboardMatrix|null {
 const [a,b,c,d,x,y]=destination,determinant=a*d-b*c;
 if(determinant===0||![...source,...destination,...offset].every(Number.isFinite))return null;
 const multiply=(left:ClipboardMatrix,right:ClipboardMatrix):ClipboardMatrix=>[left[0]*right[0]+left[2]*right[1],left[1]*right[0]+left[3]*right[1],left[0]*right[2]+left[2]*right[3],left[1]*right[2]+left[3]*right[3],left[0]*right[4]+left[2]*right[5]+left[4],left[1]*right[4]+left[3]*right[5]+left[5]];
 const inverse:ClipboardMatrix=[d/determinant,-b/determinant,-c/determinant,a/determinant,(c*y-d*x)/determinant,(b*x-a*y)/determinant];
 const local=multiply(inverse,multiply([1,0,0,1,...offset],source));
 return boolean?multiply(local,destination):local;
}
export function remapDrawingClipboard(packet:DrawingClipboard,identities:ReadonlyMap<string,string>,assets:ReadonlyMap<string,string>):DrawingClipboard|null {
 const mapped=[...identities.values()];
 if(packet.schema!=="drawing.clipboard.v1"||mapped.length!==new Set(mapped).size||mapped.some(id=>!id))return null;
 const visit=(node:DrawingLayerNode):DrawingLayerNode=>{
  const next=parseDrawingLayerNode(structuredClone(node));
  const id=identities.get(node.id);if(!id)throw Error("Clipboard layer identity is missing");next.id=id;
  if(next.kind==="boolean")next.children=next.children.map(id=>{const mapped=identities.get(id);if(!mapped)throw Error("Clipboard reference identity is missing");return mapped;});
  if(next.kind==="group")next.children=next.children.map(visit);
  if(next.kind==="image"){const mapped=assets.get(next.imageKey);if(!mapped)throw Error("Clipboard image identity is missing");next.imageKey=mapped;}
  if(next.kind==="trace"){const mapped=assets.get(next.sourceKey);if(!mapped)throw Error("Clipboard source identity is missing");next.sourceKey=mapped;}
  return next;
 };
 try{return {schema:packet.schema,roots:packet.roots.map(visit),selected:packet.selected.map(id=>{const mapped=identities.get(id);if(!mapped)throw Error("Clipboard selected identity is missing");return mapped;}),assets:Object.fromEntries(Object.entries(packet.assets).map(([id,asset])=>{const mapped=assets.get(id);if(!mapped)throw Error("Clipboard asset identity is missing");return [mapped,structuredClone(asset)];}))};}catch{return null;}
}
