/** 🎛️ Authored tangent modes keep node positions and unrelated controls stable. */
import {parsePathGeometrySegment,type PathGeometrySegment} from "../../../🟦️.ts";
type Point=[number,number];
type Cubic=Extract<PathGeometrySegment,{kind:"cubic"}>;
function cubic(segment:PathGeometrySegment,from:Point):Cubic {
  const mix=(a:Point,b:Point,t:number):Point=>[a[0]*(1-t)+b[0]*t,a[1]*(1-t)+b[1]*t];
  if(segment.kind==="cubic")return parsePathGeometrySegment(segment) as Cubic;
  if(segment.kind==="line")return {kind:"cubic",ctrl1:mix(from,segment.to,1/3),ctrl2:mix(from,segment.to,2/3),to:segment.to};
  if(segment.kind==="quad")return {kind:"cubic",ctrl1:mix(from,segment.ctrl,2/3),ctrl2:mix(segment.to,segment.ctrl,2/3),to:segment.to};
  if(segment.kind==="arc")throw new Error("Convert adjacent arcs to cubic curves before editing tangents");
  throw new Error("Select an anchor with an adjacent segment");
}
export function editNode(source:readonly PathGeometrySegment[],index:number,mode:"corner"|"smooth"|"symmetric"):PathGeometrySegment[] {
  if(!Number.isSafeInteger(index)||index<0||!["corner","smooth","symmetric"].includes(mode))throw new Error("Invalid path node mode");
  const output=source.map(segment=>parsePathGeometrySegment(segment)),segment=output[index];
  if(!segment||segment.kind==="close")throw new Error("Select an anchor");
  const anchor=segment.to;
  let start=index;while(start>0&&source[start]?.kind!=="move")start--;
  let end=start+1;while(end<source.length&&source[end]?.kind!=="move"&&source[end-1]?.kind!=="close")end++;
  const closed=source[end-1]?.kind==="close";
  let incomingIndex=index,outgoingIndex=index+1;
  let incoming:Cubic|undefined,outgoing:Cubic|undefined;
  if(index===start&&closed&&end-start>2) {
    const previous=source[end-2]!;if(previous.kind==="close")throw Error("Missing previous anchor");
    const same=previous.to[0]===anchor[0]&&previous.to[1]===anchor[1];incomingIndex=same?end-2:end-1;
    const earlier=source[end-3]!;if(earlier.kind==="close")throw Error("Missing previous anchor");
    incoming=cubic(same?previous:{kind:"line",to:anchor},same?earlier.to:previous.to);
    if(same)output[incomingIndex]=incoming;else output.splice(incomingIndex,0,incoming);
  }else if(segment.kind!=="move") {
    const previous=output[index-1];
    if(!previous||previous.kind==="close")throw new Error("Missing previous anchor");
    incoming=cubic(segment,previous.to);output[index]=incoming;
  }
  let next=source[index+1];let insert=false;
  if(closed&&index===end-2) {
    const first=source[start]!;if(first.kind==="close")throw Error("Missing first anchor");
    if(anchor[0]===first.to[0]&&anchor[1]===first.to[1]){outgoingIndex=start+1;next=source[outgoingIndex];}
    else {next={kind:"line",to:first.to};insert=true;}
  }
  if(next&&next.kind!=="move"&&next.kind!=="close"){outgoing=cubic(next,anchor);if(insert)output.splice(outgoingIndex,0,outgoing);else output[outgoingIndex]=outgoing;}
  if(!incoming&&!outgoing)throw new Error("Select an anchor with an adjacent segment");
  const length=(p:Point)=>Math.hypot(p[0]-anchor[0],p[1]-anchor[1]);
  let left=incoming?length(incoming.ctrl2):0,right=outgoing?length(outgoing.ctrl1):0;
  const a:Point=incoming&&left>0?[(anchor[0]-incoming.ctrl2[0])/left,(anchor[1]-incoming.ctrl2[1])/left]:[0,0];
  const b:Point=outgoing&&right>0?[(outgoing.ctrl1[0]-anchor[0])/right,(outgoing.ctrl1[1]-anchor[1])/right]:[0,0];
  const sum:Point=[a[0]+b[0],a[1]+b[1]],norm=Math.hypot(...sum),direction:Point=norm>1e-12?[sum[0]/norm,sum[1]/norm]:right>0?b:a;
  if(mode==="corner"){left=0;right=0;}
  if(mode==="symmetric"&&incoming&&outgoing){const size=left/2+right/2;left=size;right=size;}
  if(incoming)incoming.ctrl2=[anchor[0]-direction[0]*left,anchor[1]-direction[1]*left];
  if(outgoing)outgoing.ctrl1=[anchor[0]+direction[0]*right,anchor[1]+direction[1]*right];
  return output.map(segment=>parsePathGeometrySegment(segment));
}
