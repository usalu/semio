/** 🏬️ An exclusive glyph array moves its gap one actual original scalar per work turn. */
import type {FontClusterMapping,FontMappedScalar} from "../../../📇️face/🟦️.ts";
export class FontWorkingMapping{
 private slots:(FontMappedScalar|undefined)[];private gapStart=0;private gapCount:number;
 constructor(readonly face:number,readonly maximum:number){if(!Number.isSafeInteger(maximum)||maximum<0)throw Error("Invalid glyph gap capacity");this.slots=new Array(maximum);this.gapCount=maximum;}
 get length():number{return this.slots.length-this.gapCount;}
 get(index:number):FontMappedScalar{if(!Number.isSafeInteger(index)||index<0||index>=this.length)throw Error("Glyph gap read exceeds original authority");return this.slots[index+(index>=this.gapStart?this.gapCount:0)]!;}
 insert(value:FontMappedScalar):void{if(this.gapCount===0)throw Error("Glyph gap capacity exceeded");this.slots[this.gapStart++]=value;this.gapCount--;}
 delete(count:number):void{if(!Number.isSafeInteger(count)||count<0||count>this.length-this.gapStart)throw Error("Glyph gap deletion exceeds original authority");this.gapCount+=count;}
 moveStep(to:number):boolean{if(!Number.isSafeInteger(to)||to<0||to>this.length)throw Error("Glyph gap position exceeds authority");if(this.gapStart<to){this.slots[this.gapStart]=this.get(this.gapStart);this.gapStart++;}else if(this.gapStart>to){const value=this.get(this.gapStart-1);this.gapStart--;this.slots[this.gapStart+this.gapCount]=value;}return this.gapStart===to;}
 compactStep():boolean{if(!this.moveStep(this.length))return false;if(this.gapCount){this.slots.pop();this.gapCount--;return false;}return true;}
 take():FontClusterMapping{if(this.gapCount!==0||this.gapStart!==this.slots.length)throw Error("Original glyph gap is not compacted");const glyphs=this.slots as FontMappedScalar[];this.slots=[];this.gapStart=0;return{face:this.face,glyphs};}
 closeStep():boolean{if(this.slots.length){this.slots.pop();return false;}this.gapStart=0;this.gapCount=0;return true;}
}
