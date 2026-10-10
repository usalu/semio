/** 🏷️ Original GDEF classes resolve eligibility through one genuine ordered probe per turn. */
import {FontReader,type FontFace} from "../../🟦️.ts";
import {FontIndexCursor} from "../../🤝️kerning/🟦️.ts";
export class FontGlyphClassCursor{
 private reader:FontReader|null=null;private probe:FontIndexCursor|null=null;private output:number|null=null;
 constructor(private face:FontFace,private glyph:number){if(!Number.isInteger(glyph)||glyph<0||glyph>=face.glyphs)throw Error("Invalid original font glyph class");}
 step():boolean{if(this.output!==null)return true;if(!this.reader){if(!this.face.gdef){this.output=0;return true;}this.reader=new FontReader(this.face.bytes,this.face.gdef.offset,this.face.gdef.offset+this.face.gdef.length);if(this.reader.u16(this.face.gdef.offset)!==1)throw Error("Unsupported font class version");const at=this.reader.u16(this.face.gdef.offset+4);if(at===0){this.output=0;return true;}this.probe=new FontIndexCursor(this.reader,this.face.gdef.offset+at,this.glyph,true);return false;}if(this.probe!.step()){const value=this.probe!.result();if(value<0||value>4)throw Error("Font glyph class exceeds authority");this.output=value;return true;}return false;}
 result():number{if(this.output===null)throw Error("Font glyph class incomplete");return this.output;}
}
