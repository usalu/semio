/** 🖼️ Completed stroke polygons transfer their genuine point pages without flattening or adopting caller arrays. */
import type {Vec2} from "../../../📐️drawing-contract/🟦️.ts";
import {RetainedWorkRetirement,retireRetainedWorkArchive,type RetainedWorkBuffer,type RetainedWorkInputSequence,type RetainedWorkRetirementCursor} from "../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
export interface StrokeContours extends RetainedWorkInputSequence<RetainedWorkInputSequence<Vec2>>{intoRetirement():RetainedWorkRetirementCursor}
class OwnedStrokeContours implements StrokeContours{
 constructor(private rows:RetainedWorkBuffer<RetainedWorkBuffer<Vec2>>|null){}
 get length():number{if(this.rows===null)throw Error("Stroke output ownership transferred");return this.rows.length;}
 at(index:number):RetainedWorkInputSequence<Vec2>|undefined{if(this.rows===null)throw Error("Stroke output ownership transferred");return this.rows.at(index);}
 intoRetirement():RetainedWorkRetirementCursor{if(this.rows===null)throw Error("Stroke output ownership transferred");let rows:RetainedWorkBuffer<RetainedWorkBuffer<Vec2>>|null=this.rows;this.rows=null;return new RetainedWorkRetirement((work,turn)=>{if(rows===null)return true;if(!retireRetainedWorkArchive(rows,work,turn,(points,work,turn)=>points.retireOne(work,turn)))return false;if(!work.canStep())return false;rows=null;turn.commitActualWork();return false;});}
}
/** 📦️ The defining stroke producer consumes its already funded polygon page owner at completed publication. */
export function publishStrokeContours(rows:RetainedWorkBuffer<RetainedWorkBuffer<Vec2>>):StrokeContours{return new OwnedStrokeContours(rows);}
