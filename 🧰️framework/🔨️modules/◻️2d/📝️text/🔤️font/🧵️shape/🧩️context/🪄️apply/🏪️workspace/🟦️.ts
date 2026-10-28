import {FontWorkLedger,FontBuffer,type FontRetainedDemand} from "../../../../🎟️work/🟦️.ts";
/** 🏬️ An exclusive glyph array moves its gap one actual original scalar per work turn. */
import type {FontClusterMapping,FontMappedScalar} from "../../../📇️face/🟦️.ts";
export class FontWorkingMapping{
 private slots=new FontBuffer<FontMappedScalar|undefined>(64);private admitted=false;private gapStart=0;private gapCount:number;
 constructor(readonly face:number,readonly maximum:number){if(!Number.isSafeInteger(maximum)||maximum<0)throw Error("Invalid glyph gap capacity");this.gapCount=maximum;}
 admit(work:FontWorkLedger):boolean{if(this.admitted)return true;if(!work.contiguous(this.slots,this.maximum))return false;this.slots.length=this.maximum;this.admitted=true;return true;}
 get length():number{return this.admitted?this.slots.length-this.gapCount:0;}
 get(index:number):FontMappedScalar{if(!Number.isSafeInteger(index)||index<0||index>=this.length)throw Error("Glyph gap read exceeds original authority");return this.slots[index+(index>=this.gapStart?this.gapCount:0)]!;}
 insert(value:FontMappedScalar,work:FontWorkLedger):boolean{if(this.gapCount===0)throw Error("Glyph gap capacity exceeded");if(!this.admitted)throw Error("Glyph gap is not admitted");if(!work.copyBytes(64))return false;this.slots[this.gapStart++]=value;this.gapCount--;return true;}
 delete(count:number):void{if(!Number.isSafeInteger(count)||count<0||count>this.length-this.gapStart)throw Error("Glyph gap deletion exceeds original authority");this.gapCount+=count;}
 moveStep(to:number,work:FontWorkLedger):boolean{if(!Number.isSafeInteger(to)||to<0||to>this.length)throw Error("Glyph gap position exceeds authority");if(this.gapStart!==to&&!work.copyBytes(64))return false;if(this.gapStart<to){this.slots[this.gapStart]=this.get(this.gapStart);this.gapStart++;}else if(this.gapStart>to){const value=this.get(this.gapStart-1);this.gapStart--;this.slots[this.gapStart+this.gapCount]=value;}return this.gapStart===to;}
 compactStep(work:FontWorkLedger):boolean{if(!this.moveStep(this.length,work))return false;if(this.gapCount){if(!work.copyBytes(this.slots.elementBytes))return false;this.slots.pop();this.gapCount--;return false;}return true;}
 take():FontClusterMapping{if(this.gapCount!==0||this.gapStart!==this.slots.length)throw Error("Original glyph gap is not compacted");const glyphs=this.slots as FontMappedScalar[];this.slots=new FontBuffer<FontMappedScalar|undefined>(64);this.admitted=false;this.gapStart=0;return{face:this.face,glyphs};}
 nextCloseDemand():FontRetainedDemand{return this.slots.nextRetireDemand();}
 closeStep(work:FontWorkLedger):boolean{if(!this.slots.retireOne(work))return false;this.gapStart=0;this.gapCount=0;this.admitted=false;return true;}
}
