/** 🎨️ Visible initial paint for newly authored artwork. */
import type { StrokeCap, StrokeJoin } from "../../../🧬️schema/🖊️stroke/🟦️.ts";
export interface InitialAppearance {
  fillRule:import("../../../🧬️schema/🎨️fill/🌀️rule/🟦️.ts").FillRule;
  fill:{kind:"solid";color:[number,number,number,number]}|null;
  stroke:{color:[number,number,number,number];width:number;cap:StrokeCap;join:StrokeJoin;dash:number[]|null}|null;
}
export function initialAppearance(kind:string):InitialAppearance {
  const filled=["shape:rect","shape:ellipse","shape:polygon","boolean","trace"].includes(kind);
  const outlined=kind==="path" || kind.startsWith("shape:");
  return {
    fillRule:"evenodd",
    fill:filled?{kind:"solid",color:[0.2,0.7,0.65,1]}:kind==="text"?{kind:"solid",color:[0,0,0,1]}:null,
    stroke:outlined?{color:[0.1,0.15,0.2,1],width:2,cap:"round",join:"round",dash:null}:null,
  };
}
