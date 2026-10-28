import type {IfcMatrix} from "./geometry.ts";
import {IfcStepError} from './control.ts';

type Vector=[number,number,number];
const binary64=(value:number,view:DataView):[bigint,number]=>{view.setFloat64(0,value,false);const bits=view.getBigUint64(0,false),exponent=Number((bits>>52n)&2047n),mantissa=(bits&0xfffffffffffffn)|(exponent?0x10000000000000n:0n);return[(bits>>63n)?-mantissa:mantissa,exponent?exponent-1075:-1074];};

/** 🧭 Resolve the determinant sign of represented finite binary64 coefficients exactly. */
export function ifcMatrixOrientation(matrix:IfcMatrix):-1|0|1 {
  if(!matrix.every(Number.isFinite))return 0;
  const view=new DataView(new ArrayBuffer(8));
  const terms=[[0,1,2,1],[1,2,0,1],[2,0,1,1],[2,1,0,-1],[1,0,2,-1],[0,2,1,-1]] as const;
  let determinant=0n;
  for(const [a,b,c,sign] of terms) {
    const [x,xe]=binary64(matrix[a*4]!,view),[y,ye]=binary64(matrix[b*4+1]!,view),[z,ze]=binary64(matrix[c*4+2]!,view);
    determinant+=BigInt(sign)*x*y*z<<BigInt(xe+ye+ze+3222);
  }
  return determinant<0n?-1:determinant>0n?1:0;
}

/** 🧮️ Prepare exact fixed-size cofactors once, then transform finite directions without magnitude overflow. */
export class IfcNormalTransform{
 private constructor(private readonly cofactors:bigint[][],private readonly sign:-1|1){}
 /** 📐️ Capture a finite nonsingular affine frame independently of subsequent caller mutations. */
 static capture(matrix:IfcMatrix):IfcNormalTransform{
  const sign=ifcMatrixOrientation(matrix);if(!sign||matrix[3]!==0||matrix[7]!==0||matrix[11]!==0||matrix[15]!==1)throw new IfcStepError('geometry.normal','Finite nonsingular affine normal frame required');const view=new DataView(new ArrayBuffer(8)),rows=Array.from({length:3},(_,row)=>Array.from({length:3},(_,column)=>binary64(matrix[column*4+row]!,view))),exponent=Math.min(...rows.flat().filter(([mantissa])=>mantissa!==0n).map(([,exponent])=>exponent))*2,product=(a:[bigint,number],b:[bigint,number])=>a[0]===0n||b[0]===0n?0n:a[0]*b[0]<<BigInt(a[1]+b[1]-exponent),cofactors=Array.from({length:3},(_,row)=>Array.from({length:3},(_,column)=>product(rows[(row+1)%3]![(column+1)%3]!,rows[(row+2)%3]![(column+2)%3]!)-product(rows[(row+1)%3]![(column+2)%3]!,rows[(row+2)%3]![(column+1)%3]!)));return new IfcNormalTransform(cofactors,sign);
 }
 /** 🧭️ Preserve common-scale component ratios, tiny representable directions and exact zero sources. */
 direction(normal:Vector):Vector{
  if(normal.length!==3||!normal.every(Number.isFinite))throw new IfcStepError('geometry.normal','Finite normal triple required');if(normal.every(n=>n===0))return[0,0,0];const view=new DataView(new ArrayBuffer(8)),parts=normal.map(n=>binary64(n,view)),exponent=Math.min(...parts.filter(([mantissa])=>mantissa!==0n).map(([,exponent])=>exponent)),values=this.cofactors.map(row=>row.reduce((sum,coefficient,column)=>{const[mantissa,power]=parts[column]!;return mantissa===0n?sum:sum+(coefficient*mantissa<<BigInt(power-exponent));},0n)),lengths=values.map(value=>(value<0n?-value:value).toString(2).length),maximum=Math.max(...lengths),scaled=values.map((value,at)=>{if(value===0n)return 0;const bits=lengths[at]!,retained=Math.min(bits,53),significand=Number((value<0n?-value:value)>>BigInt(bits-retained))/2**(retained-1),power=bits-maximum;return(value<0n?-1:1)*this.sign*significand*2**Math.max(power,-1022)*2**Math.min(power+1022,0);}) as Vector,length=Math.hypot(...scaled);if(!(length>0)||!Number.isFinite(length))throw new IfcStepError('geometry.normal','Nonzero transformed normal direction required');return scaled.map(n=>n/length) as Vector;
 }
}
