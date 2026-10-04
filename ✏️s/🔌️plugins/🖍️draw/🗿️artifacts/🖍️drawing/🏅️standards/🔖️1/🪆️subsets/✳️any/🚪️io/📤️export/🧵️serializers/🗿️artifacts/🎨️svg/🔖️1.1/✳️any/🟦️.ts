/** 🎨️ SVG scene serializer twin preserving paint, text and affine geometry. */
import { pathSegmentsToSvgD, drawingTextLines, DRAWING_TEXT_LINE_HEIGHT, type PathGeometrySegment, type FillStyle, type StrokeStyle } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";

import {PreparedFill} from "../../../../../../../🧬️schema/🎨️fill/🎨️sampling/🟦️.ts";

export interface DrawingSvgNode {
  readonly id: string;
  readonly groups?: readonly {readonly id:string;readonly opacity:number;readonly blendMode:string}[];
  readonly transform: readonly number[];
  readonly segments: readonly PathGeometrySegment[];
  readonly fill?: FillStyle;
  readonly stroke?: StrokeStyle;
  readonly opacity: number;
  readonly blendMode: string;
  readonly visible: boolean;
  readonly fillRule?: string;
  readonly text?: { readonly content: string; readonly size: number };
  readonly image?: { readonly src: string; readonly width: number; readonly height: number };
}

const escape = (value: string | number): string => String(value).replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
const element = (name: string, attrs: Record<string, string | number>, content = ""): string => `<${name}${Object.entries(attrs).map(([key,value]) => ` ${key}="${escape(value)}"`).join("")}>${content}</${name}>`;
const finiteNumbers = (value: unknown): boolean => typeof value === "number" ? Number.isFinite(value)
  : value !== null && typeof value === "object" ? Object.values(value).every(finiteNumbers) : true;
const rgb = (color: readonly number[]): string => `rgb(${color.slice(0,3).map(value => value * 255).join(",")})`;
const blendModes: Readonly<Record<string,string>> = {normal:"normal",multiply:"multiply",screen:"screen",overlay:"overlay",darken:"darken",lighten:"lighten",colorDodge:"color-dodge",colorBurn:"color-burn",hardLight:"hard-light",softLight:"soft-light",difference:"difference",exclusion:"exclusion",hue:"hue",saturation:"saturation",color:"color",luminosity:"luminosity"};

export function drawingSceneToSvg(nodes: readonly DrawingSvgNode[], viewBox: readonly [number,number,number,number]): string {
  if (!viewBox.every(Number.isFinite) || viewBox[2] <= 0 || viewBox[3] <= 0) throw new Error("SVG view box must have finite coordinates and positive dimensions");
  const defs: string[] = [], children: string[] = [];
  let active:NonNullable<DrawingSvgNode["groups"]>=[];
  const opened=new Set<string>();
  for (const [index,node] of nodes.entries()) {
    if (!node.visible) continue;
    if (!finiteNumbers(node)) throw new Error(`SVG layer ${node.id} geometry and paint must be finite`);
    if (node.opacity <= 0) continue;
    if (node.transform.length !== 6 || !node.transform.every(Number.isFinite)) throw new Error("SVG layer transform must be finite");
    const groups=node.groups??[];
    let common=0;
    while(common<active.length && common<groups.length && active[common]!.id===groups[common]!.id && active[common]!.opacity===groups[common]!.opacity && active[common]!.blendMode===groups[common]!.blendMode) common++;
    for(let i=active.length;i>common;i--) children.push("</g>");
    for(const group of groups.slice(common)) {
      if(typeof group.id!=="string" || !group.id || opened.has(group.id) || !Number.isFinite(group.opacity) || group.opacity<0 || group.opacity>1 || !Object.hasOwn(blendModes,group.blendMode)) throw new Error("Invalid scene compositing hierarchy");
      opened.add(group.id);
      const attrs={"data-group-id":group.id,opacity:group.opacity,style:`isolation:isolate;mix-blend-mode:${blendModes[group.blendMode]}`};
      children.push(`<g${Object.entries(attrs).map(([key,value])=>` ${key}="${escape(value)}"`).join("")}>`);
    }
    active=groups;
    const paint: Record<string,string | number> = {fill:"none",stroke:"none","fill-rule":node.fillRule ?? "evenodd"};
    const constant=node.fill ? new PreparedFill(node.fill).constantColor() : null;
    if (constant) { paint.fill=rgb(constant);paint["fill-opacity"]=constant[3]; }
    else if (node.fill) {
      const fill = node.fill, id = `draw-gradient-${index}`;
      const stops = [...fill.stops].sort((a,b) => a.offset - b.offset).map(stop => element("stop",{offset:Math.min(1,Math.max(0,stop.offset)),"stop-color":rgb(stop.color),"stop-opacity":stop.color[3]})).join("");
      const attrs: Record<string,string | number> = fill.kind === "linearGradient" ? {x1:fill.x1,y1:fill.y1,x2:fill.x2,y2:fill.y2} : {cx:fill.cx,cy:fill.cy,r:fill.r,fx:fill.cx,fy:fill.cy};
      defs.push(element(fill.kind,{id,gradientUnits:"userSpaceOnUse",...attrs},stops));
      paint.fill = `url(#${id})`;
    }
    if (node.stroke && Number.isFinite(node.stroke.width) && node.stroke.width > 0) {
      const stroke = node.stroke;
      Object.assign(paint,{stroke:rgb(stroke.color),"stroke-opacity":stroke.color[3],"stroke-width":stroke.width,"stroke-linecap":stroke.cap,"stroke-linejoin":stroke.join});
      if (stroke.dash) paint["stroke-dasharray"] = stroke.dash.join(" ");
    }
    let leaf: string;
    if (node.text) {
      const text = node.text;
      const lines = [...drawingTextLines(text.content)].map((line,index) => line ? element("tspan",{x:0,y:text.size + index * text.size * DRAWING_TEXT_LINE_HEIGHT},escape(line)) : "").join("");
      leaf = element("text",{...paint,"font-size":text.size,"font-family":"ui-sans-serif, system-ui, sans-serif","xml:space":"preserve"},lines);
    } else if (node.image) {
      leaf = element("image",{x:0,y:0,width:node.image.width,height:node.image.height,preserveAspectRatio:"none",href:node.image.src,"xlink:href":node.image.src});
    } else leaf = element("path",{...paint,d:pathSegmentsToSvgD(node.segments)});
    const wrapper: Record<string,string | number> = {transform:`matrix(${node.transform.join(" ")})`,opacity:node.opacity,"data-layer-id":node.id};
    if (node.blendMode !== "normal") wrapper.style = `mix-blend-mode:${blendModes[node.blendMode] ?? "normal"}`;
    children.push(element("g",wrapper,leaf));
  }
  for(const _group of active) children.push("</g>");
  return element("svg",{version:"1.1",xmlns:"http://www.w3.org/2000/svg","xmlns:xlink":"http://www.w3.org/1999/xlink",width:viewBox[2],height:viewBox[3],viewBox:viewBox.join(" ")},(defs.length ? element("defs",{},defs.join("")) : "") + children.join(""));
}
