/** 📐️ Lossless placement changes between invertible affine parent frames. */
import {inverse,multiply,type CompositeAffine} from "../🟦️.ts";

export type AffineControls={x:number;y:number;scaleX:number;scaleY:number;rotation:number;shearX:number};

/** 🎛️ Composes translation, degree rotation, horizontal shear and signed scale. */
export function compose(value:AffineControls):CompositeAffine {
  if(!value||![value.x,value.y,value.scaleX,value.scaleY,value.rotation,value.shearX].every(Number.isFinite))throw new RangeError("Affine controls require finite coordinates");
  const angle=value.rotation*(Math.PI/180),c=Math.cos(angle),s=Math.sin(angle);
  const result:CompositeAffine=[c*value.scaleX,s*value.scaleX,(c*value.shearX-s)*value.scaleY,(s*value.shearX+c)*value.scaleY,value.x,value.y];
  inverse(result);return result;
}

/** 🪞️ Canonical controls retain shear and place reflection in the signed vertical scale. */
export function decompose(matrix:CompositeAffine):AffineControls {
  inverse(matrix);
  const [a,b,c,d,x,y]=matrix,scaleX=Math.hypot(a,b),scaleY=(a*d-b*c)/scaleX;
  const result:AffineControls={x,y,scaleX,scaleY,rotation:Math.atan2(b,a)*(180/Math.PI),shearX:((a/scaleX)*c+(b/scaleX)*d)/scaleY};
  compose(result);return result;
}

/** 🧭️ Preserves world placement when changing an object's parent coordinate frame. */
export function reframe(transform:CompositeAffine,source:CompositeAffine,target:CompositeAffine):CompositeAffine {
  inverse(transform);inverse(source);
  const result=multiply(inverse(target),multiply(source,transform));
  inverse(result);return result;
}
