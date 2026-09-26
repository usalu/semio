/** ✏️ Pure node editing in path-local coordinates. */
import { parsePathSegment, type PathSegment } from "../../🟦️.ts";
import { splitCubic, arcGeometry, arcPoint } from "../🟦️.ts";

export type PathEdit =
  | { kind: "coordinate"; index: number; point: "anchor" | "control1" | "control2"; axis: "x" | "y"; value: number }
  | { kind: "split"; index: number; t: number }
  | { kind: "convert"; index: number; target: "line" | "cubic" }
  | { kind: "join"; index: number; other: number }
  | { kind: "delete" | "close" | "open"; index: number }
  | { kind: "reverse" };

/** 🧭 Checks contour boundaries before applying an edit. */
function contours(segments: PathSegment[]): [number, number][] {
  const ranges: [number, number][] = [];
  let start = -1;
  for (let index = 0; index < segments.length; index++) {
    const item = segments[index]!;
    if (item.kind === "move") {
      if (start >= 0) ranges.push([start, index]);
      start = index;
    } else {
      if (start < 0) throw new Error("A contour must start with a move");
      if (item.kind === "close") { ranges.push([start, index + 1]); start = -1; }
    }
  }
  if (start >= 0) ranges.push([start, segments.length]);
  return ranges;
}

/** 🪢 Returns a replacement vector; the source remains untouched on failure. */
export function editPath(source: readonly PathSegment[], operation: PathEdit): PathSegment[] {
  const segments = source.map(item => parsePathSegment(item));
  const ranges = contours(segments);
  if (operation.kind === "join") {
    if (operation.index === operation.other) throw new Error("Choose two different endpoints");
    const range = (index:number):[number,number] => {
      if (!Number.isSafeInteger(index) || index<0) throw new Error("Missing endpoint");
      const found=ranges.find(([start,end])=>(index===start || index===end-1) && segments[end-1]!.kind!=="close");
      if (!found) throw new Error("Choose endpoints of open contours");
      return found;
    };
    const a=range(operation.index),b=range(operation.other);
    if (a===b) return editPath(segments,{kind:"close",index:operation.index});
    const first=segments.slice(...a),second=segments.slice(...b);
    const joined=operation.index===a[0]?editPath(first,{kind:"reverse"}):first;
    const target=operation.other===b[0]?second:editPath(second,{kind:"reverse"});
    const end=joined.at(-1)!,start=target[0]!;
    if (end.kind==="close" || start.kind!=="move") throw new Error("Missing endpoint");
    if (end.to[0]!==start.to[0] || end.to[1]!==start.to[1]) joined.push({kind:"line",to:[...start.to]});
    for(let index=1;index<target.length;index++) joined.push(target[index]!);
    const output:PathSegment[]=[];
    for(const [start,end] of ranges) {
      if(start===Math.min(a[0],b[0])) for(const segment of joined) output.push(segment);
      if(start!==a[0] && start!==b[0]) for(let index=start;index<end;index++) output.push(segments[index]!);
    }
    return output;
  }
  if (operation.kind === "reverse") {
    const output: PathSegment[] = [];
    for (const [start, end] of ranges) {
      const closed = segments[end - 1]!.kind === "close";
      const last = end - (closed ? 2 : 1);
      const endpoint = segments[last]!;
      if (endpoint.kind === "close") throw new Error("Missing endpoint");
      output.push({ kind: "move", to: [...endpoint.to] });
      for (let index = last; index > start; index--) {
        const item = segments[index]!;
        const previous = segments[index - 1]!;
        if (previous.kind === "close" || item.kind === "close" || item.kind === "move") throw new Error("Invalid contour");
        const to: [number, number] = [...previous.to];
        if (item.kind === "cubic") output.push({ kind: "cubic", ctrl1: [...item.ctrl2], ctrl2: [...item.ctrl1], to });
        else if (item.kind === "arc") output.push({ ...item, sweep: !item.sweep, to });
        else output.push({ ...item, to });
      }
      if (closed) output.push({ kind: "close" });
    }
    return output;
  }
  const index = operation.index;
  if (!Number.isSafeInteger(index) || index < 0 || index >= segments.length) throw new Error("Missing path node");
  const item = segments[index]!;
  const [start, end] = ranges.find(([start, end]) => index >= start && index < end)!;
  if (operation.kind === "convert") {
    const previous = segments[index-1];
    if (!previous || previous.kind === "close" || item.kind === "move" || item.kind === "close") throw new Error("Select a segment to convert");
    const from = previous.to, to = item.to;
    const mix = (a:[number,number],b:[number,number],t:number):[number,number] => [a[0]*(1-t)+b[0]*t,a[1]*(1-t)+b[1]*t];
    let curves:PathSegment[];
    if (operation.target === "line") curves=[{kind:"line",to}];
    else if (operation.target !== "cubic") throw new Error("Invalid segment type");
    else if (item.kind === "cubic") curves=[item];
    else if (item.kind === "quad") curves=[{kind:"cubic",ctrl1:mix(from,item.ctrl,2/3),ctrl2:mix(to,item.ctrl,2/3),to}];
    else {
      const arc = item.kind === "arc" ? arcGeometry(from,[item.rx,item.ry],item.rotation,item.largeArc,item.sweep,to) : null;
      curves=[];
      if (arc) {
        let ratio=Math.abs(arc.sweep)/(Math.PI/2);
        if (Math.abs(1-ratio)<1e-7) ratio=1;
        const count=Math.max(1,Math.ceil(ratio)),delta=arc.sweep/count,k=4/3*Math.tan(delta/4),sr=Math.sin(arc.rotation),cr=Math.cos(arc.rotation);
        const map=(x:number,y:number):[number,number]=>[arc.center[0]+x*arc.radii[0]*cr-y*arc.radii[1]*sr,arc.center[1]+x*arc.radii[0]*sr+y*arc.radii[1]*cr];
        let angle=arc.start;
        for(let part=0;part<count;part++) {
          const end=angle+delta,c1=Math.cos(angle),s1=Math.sin(angle),c2=Math.cos(end),s2=Math.sin(end);
          curves.push({kind:"cubic",ctrl1:map(c1-s1*k,s1+c1*k),ctrl2:map(c2+s2*k,s2-c2*k),to:part===count-1?to:map(c2,s2)});
          angle=end;
        }
      } else curves.push({kind:"cubic",ctrl1:mix(from,to,1/3),ctrl2:mix(from,to,2/3),to});
    }
    segments.splice(index,1,...curves);
  } else if (operation.kind === "open" || operation.kind === "close") {
    const closed = segments[end - 1]!.kind === "close";
    if (operation.kind === "open" && closed) segments.splice(end - 1, 1);
    if (operation.kind === "close" && !closed) {
      if (end - start < 2) throw new Error("A contour needs at least two anchors");
      segments.splice(end, 0, { kind: "close" });
    }
  } else if (operation.kind === "delete") {
    if (item.kind === "close") throw new Error("Select an anchor to delete");
    if (item.kind === "move") {
      const next = segments[index + 1];
      if (!next || next.kind === "move" || next.kind === "close") segments.splice(start, end - start);
      else { segments[index + 1] = { kind: "move", to: [...next.to] }; segments.splice(index, 1); }
    } else segments.splice(index, 1);
  } else if (operation.kind === "coordinate") {
    if (!Number.isFinite(operation.value) || !["x", "y"].includes(operation.axis)) throw new Error("Invalid coordinate");
    if (item.kind === "close") throw new Error("Select an anchor");
    const axis = operation.axis === "x" ? 0 : 1;
    if (operation.point === "anchor") {
      const delta = operation.value - item.to[axis];
      item.to[axis] = operation.value;
      if (item.kind === "cubic") item.ctrl2[axis] += delta;
      if (item.kind === "quad") item.ctrl[axis] += delta;
      const next = segments[index + 1];
      if (next?.kind === "cubic") next.ctrl1[axis] += delta;
      if (next?.kind === "quad") next.ctrl[axis] += delta;
    } else if (item.kind === "cubic") {
      if (operation.point === "control1") item.ctrl1[axis] = operation.value;
      else if (operation.point === "control2") item.ctrl2[axis] = operation.value;
      else throw new Error("Invalid handle");
    } else if (item.kind === "quad" && operation.point === "control1") item.ctrl[axis] = operation.value;
    else throw new Error("This node has no selected handle");
  } else if (operation.kind === "split") {
    if (!(operation.t > 0 && operation.t < 1)) throw new Error("Split position must be between zero and one");
    const previous = segments[index - 1];
    if (!previous || previous.kind === "close" || item.kind === "move" || item.kind === "close") throw new Error("Select a segment to split");
    const t = operation.t;
    const mix = (a: [number,number], b: [number,number]): [number,number] => [a[0]*(1-t)+b[0]*t,a[1]*(1-t)+b[1]*t];
    if (item.kind === "line") segments.splice(index, 1, { kind: "line", to: mix(previous.to, item.to) }, item);
    else if (item.kind === "quad") {
      const a = mix(previous.to, item.ctrl), b = mix(item.ctrl, item.to);
      segments.splice(index, 1, { kind: "quad", ctrl: a, to: mix(a,b) }, { kind: "quad", ctrl: b, to: item.to });
    } else if (item.kind === "cubic") {
      const halves = splitCubic([previous.to,item.ctrl1,item.ctrl2,item.to],t);
      if (!halves) throw new Error("Invalid split geometry");
      segments.splice(index, 1, ...halves.map(points => ({ kind: "cubic" as const, ctrl1: points[1], ctrl2: points[2], to: points[3] })));
    } else {
      if (item.rx === 0 || item.ry === 0 || (previous.to[0]===item.to[0] && previous.to[1]===item.to[1])) {
        segments.splice(index, 1, { kind: "line", to: mix(previous.to,item.to) }, { kind: "line", to: item.to });
      } else {
        const arc = arcGeometry(previous.to,[item.rx,item.ry],item.rotation,item.largeArc,item.sweep,item.to);
        if (!arc) throw new Error("Invalid arc geometry");
        const center = arcPoint(arc,t);
        segments.splice(index, 1, { ...item, rx: arc.radii[0], ry: arc.radii[1], largeArc: Math.abs(arc.sweep*t)>Math.PI, to: center }, { ...item, rx: arc.radii[0], ry: arc.radii[1], largeArc: Math.abs(arc.sweep*(1-t))>Math.PI });
      }
    }
  } else throw new Error("Unknown path operation");
  return segments.map(item => parsePathSegment(item));
}
