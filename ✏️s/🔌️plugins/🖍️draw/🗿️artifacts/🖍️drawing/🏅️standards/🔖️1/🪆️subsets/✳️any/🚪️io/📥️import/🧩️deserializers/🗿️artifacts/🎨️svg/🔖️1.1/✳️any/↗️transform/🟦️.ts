/** ↗️ Normalize SVG transform lists without losing shear, reflection, or collapsed axes. */
import {drawingMatrixToTransform} from "../../../../../../../../🧬️schema/🧮️geometry/↗️affine/🟦️.ts";
import {multiply,type Matrix} from "../../../../../../../../🧬️schema/🧮️geometry/🟦️.ts";
export function parseEditableSvgTransform(source:string) {
  let cursor=0,matrix:Matrix=[1,0,0,1,0,0];
  const skip=()=>{while(cursor<source.length&&/[ \t\r\n]/.test(source[cursor]!))cursor++;};
  const number=/[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?/y;
  while(true){
    skip();if(cursor===source.length)break;
    const call=/([A-Za-z]+)[ \t\r\n]*\(/y;call.lastIndex=cursor;
    const match=call.exec(source);if(!match)throw new Error("Invalid SVG transform function");cursor=call.lastIndex;
    const values:number[]=[];
    skip();
    while(true){
      number.lastIndex=cursor;const value=number.exec(source);
      if(!value||!Number.isFinite(+value[0]))throw new Error("Invalid SVG transform number");
      values.push(+value[0]);cursor=number.lastIndex;
      if(values.length>6)throw new Error("Too many SVG transform operands");
      const end=cursor;skip();
      if(source[cursor]===")"){cursor++;break;}
      if(source[cursor]===","){cursor++;skip();}
      else if(cursor===end)throw new Error("SVG transform operands need a separator");
    }
    const end=cursor;skip();
    if(cursor<source.length){
      if(source[cursor]===","){while(source[cursor]===","){cursor++;skip();}if(cursor===source.length)throw new Error("Trailing SVG transform separator");}
      else if(cursor===end)throw new Error("SVG transforms need a separator");
    }
    const [a=0,b=0,c=0,d=0,e=0,f=0]=values,name=match[1];let next:Matrix;
    if(name==="matrix"&&values.length===6)next=[a,b,c,d,e,f];
    else if(name==="translate"&&(values.length===1||values.length===2))next=[1,0,0,1,a,b];
    else if(name==="scale"&&(values.length===1||values.length===2))next=[a,0,0,values.length===1?a:b,0,0];
    else if(name==="rotate"&&(values.length===1||values.length===3)){
      const angle=(a%360)*Math.PI/180,s=Math.sin(angle),co=Math.cos(angle);
      next=[co,s,-s,co,b-co*b+s*c,c-s*b-co*c];
    } else if((name==="skewX"||name==="skewY")&&values.length===1){
      if(Math.abs(a%180)===90)throw new Error("Undefined SVG skew angle");
      const shear=Math.tan((a%180)*Math.PI/180);next=name==="skewX"?[1,0,shear,1,0,0]:[1,shear,0,1,0,0];
    } else throw new Error("Unknown SVG transform or invalid operands");
    matrix=multiply(matrix,next);if(!matrix.every(Number.isFinite))throw new Error("SVG transform exceeds finite coordinates");
  }
  const result=drawingMatrixToTransform(matrix);
  if(!Object.values(result).every(Number.isFinite))throw new Error("SVG transform cannot be represented");
  return result;
}
