/** 🖼️ One caller-owned RGBA image holds the actual typed bytes through explicit funded writes and close. */
import {validateExtent,type PixelImage} from "../🟦️.ts";
import {RetainedWorkBytes,RetainedWorkRetirement,assertRetainedWorkTurn,type RetainedWorkLedger,type RetainedWorkTurn,type RetainedWorkRetirementCursor} from "../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
const byte=(value:number)=>Number.isInteger(value)&&value>=0&&value<=255;
export class OwnedPixelImage{
 #body=new RetainedWorkBytes();#view:PixelImage|null=null;#closed=false;readonly count:number;
 constructor(readonly width:number,readonly height:number){this.count=validateExtent(width,height);}
 admit(work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{assertRetainedWorkTurn(turn);if(this.#closed)throw Error("Pixel image ownership transferred");if(this.#view!==null)return true;if(this.#body.terminalIsEmpty()){this.#body.admit(this.count*4,work,turn);return false;}if(!work.canStep())return false;this.#view={width:this.width,height:this.height,pixels:this.#body.bytes};turn.commitActualWork();return true;}
 writeRGBA(index:number,r:number,g:number,b:number,a:number,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{assertRetainedWorkTurn(turn);if(this.#closed||this.#view===null)throw Error("Pixel image backing not live");if(!Number.isSafeInteger(index)||index<0||index>=this.count||!byte(r)||!byte(g)||!byte(b)||!byte(a))throw RangeError("Invalid owned RGBA cell");if(!work.copyBytes(4))return false;const values=this.#view.pixels,at=index*4;values[at]=r;values[at+1]=g;values[at+2]=b;values[at+3]=a;turn.commitActualWork();work.notePhysicalEffect();return true;}
 borrow():PixelImage{if(this.#closed||this.#view===null)throw Error("Pixel image ownership transferred or not admitted");return this.#view;}
 intoRetirement():RetainedWorkRetirementCursor{if(this.#closed)throw Error("Pixel image ownership transferred");this.#closed=true;this.#view=null;let body:RetainedWorkBytes|null=this.#body;return new RetainedWorkRetirement((work,turn)=>{if(body===null)return true;if(!body.retire(work,turn)||turn.actualWork()>0)return false;if(!work.canStep())return false;body=null;turn.commitActualWork();return false;});}
}
