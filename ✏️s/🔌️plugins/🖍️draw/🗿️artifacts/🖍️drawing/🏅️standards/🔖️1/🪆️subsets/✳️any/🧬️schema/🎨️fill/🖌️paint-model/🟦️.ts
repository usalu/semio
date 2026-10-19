/** 🖌️ The drawing fill model sampled by the gradient ramp and edited by the fill editor: colors, gradient stops and fills. */

export type Color = [number,number,number,number];

export interface Stop { offset:number; color:Color }

export type Fill = {kind:"solid";color:Color} | {kind:"linearGradient";x1:number;y1:number;x2:number;y2:number;stops:Stop[]} | {kind:"radialGradient";cx:number;cy:number;r:number;stops:Stop[]};
