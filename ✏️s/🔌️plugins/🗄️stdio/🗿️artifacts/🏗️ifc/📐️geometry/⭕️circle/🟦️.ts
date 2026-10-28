export type CircleAxis=[number,number,number];
/** ⭕️ Bound a normal finite circle radius under the represented two-column linear basis. */
export function circleBasisRadius(radius:number,u:CircleAxis,v:CircleAxis):number|undefined{
 if(!(radius>0)||!Number.isFinite(radius)||u.length!==3||v.length!==3||!u.every(Number.isFinite)||!v.every(Number.isFinite))return undefined;
 const largest=Math.max(Math.abs(u[0]),Math.abs(u[1]),Math.abs(u[2]),Math.abs(v[0]),Math.abs(v[1]),Math.abs(v[2]));if(!(largest>0))return undefined;
 let aa=0,bb=0,ab=0;for(let axis=0;axis<3;axis++){const a=u[axis]!/largest,b=v[axis]!/largest;aa+=a*a;bb+=b*b;ab+=a*b;}
 const lambda=(aa+bb+Math.hypot(aa-bb,2*ab))/2+64*Number.EPSILON,result=radius*largest*Math.sqrt(lambda)*(1+16*Number.EPSILON);return Number.isFinite(result)&&result>=2**-1022?result:undefined;
}
