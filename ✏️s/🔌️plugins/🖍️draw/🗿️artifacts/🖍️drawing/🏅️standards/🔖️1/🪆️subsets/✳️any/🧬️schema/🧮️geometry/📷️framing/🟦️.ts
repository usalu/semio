/** 📷️ Visible world bounds shared by Drawing surfaces. */
import { segmentBounds,type Matrix,type Point } from "../🟦️.ts";
import type { DrawingArtboard,PathSegment } from "../../🟦️.ts";
export interface FramingNode {
  transform: Matrix;
  segments: PathSegment[];
  visible: boolean;
  opacity: number;
  stroke?: { width: number };
  image?: { width: number;height: number };
  text?: { content: string;size: number };
}
export function drawingSceneBounds(artboard: DrawingArtboard | undefined,nodes: readonly FramingNode[]): [number,number,number,number] {
  let bounds: [number,number,number,number] | undefined = artboard && artboard.width>0 && artboard.height>0 ? [0,0,artboard.width,artboard.height] : undefined;
  for (const node of nodes) {
    if (!node.visible || node.opacity<=0) continue;
    const rectangle = node.image ? [0,0,node.image.width,node.image.height] : node.text ? [0,-node.text.size,Array.from(node.text.content).length*node.text.size*0.6,node.text.size*1.2] : null;
    const segments: PathSegment[] = rectangle ? [{ kind: "move",to: [rectangle[0]!,rectangle[1]!] },{ kind: "line",to: [rectangle[0]!+rectangle[2]!,rectangle[1]!] },{ kind: "line",to: [rectangle[0]!+rectangle[2]!,rectangle[1]!+rectangle[3]!] },{ kind: "line",to: [rectangle[0]!,rectangle[1]!+rectangle[3]!] },{ kind: "close" }] : node.segments;
    let current: Point = [0,0],start: Point = current;
    let started = false;
    const radius = (node.stroke?.width ?? 0)/2;
    const dx = radius*Math.hypot(node.transform[0],node.transform[2]),dy = radius*Math.hypot(node.transform[1],node.transform[3]);
    for (const segment of segments) {
      if (segment.kind === "close" && !started) continue;
      started = true;
      const [x,y,w,h] = segmentBounds(segment,current,start,node.transform);
      const next: [number,number,number,number] = [x-dx,y-dy,x+w+dx,y+h+dy];
      bounds = bounds ? [Math.min(bounds[0],next[0]),Math.min(bounds[1],next[1]),Math.max(bounds[2],next[2]),Math.max(bounds[3],next[3])] : next;
      if (segment.kind === "move") start = segment.to;
      current = "to" in segment ? segment.to : start;
    }
  }
  return bounds ?? [0,0,1024,1024];
}
