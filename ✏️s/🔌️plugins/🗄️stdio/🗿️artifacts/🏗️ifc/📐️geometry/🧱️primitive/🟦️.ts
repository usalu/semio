import type {Point,Triangle} from "../🟦️.ts";
export const BLOCK_TRIANGLES:readonly Triangle[]=[[0,2,1],[0,3,2],[4,5,6],[4,6,7],[0,1,5],[0,5,4],[1,2,6],[1,6,5],[2,3,7],[2,7,6],[3,0,4],[3,4,7]];
/** 🧱️ One positive-axis block corner; one actual emission unit for either driver. */
export function blockPoint(lengths:Point,index:number):Point{if(!Number.isInteger(index)||index<0||index>7)throw Error("invalid-block-corner");return [((index^(index>>1))&1)?lengths[0]:0,(index&2)?lengths[1]:0,(index&4)?lengths[2]:0];}
/** ▭️ One centered rectangle corner in original profile coordinates. */
export function rectanglePoint(x:number,y:number,index:number):Point{if(!Number.isInteger(index)||index<0||index>3)throw Error("invalid-rectangle-corner");return [((index^(index>>1))&1)?x/2:-x/2,index>=2?y/2:-y/2,0];}
