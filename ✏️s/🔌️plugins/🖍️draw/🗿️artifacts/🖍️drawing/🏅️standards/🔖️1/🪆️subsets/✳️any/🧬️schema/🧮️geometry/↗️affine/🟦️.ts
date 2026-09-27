/** ↗️ Full affine composition and QR decomposition, including shear and reflections. */
import type {DrawingTransform} from "../../🧬️mutations/🟦️.ts";

export function drawingTransformToMatrix(transform: DrawingTransform): [number,number,number,number,number,number] {
  const c=Math.cos(transform.rotation), s=Math.sin(transform.rotation);
  return [transform.scaleX*c,transform.scaleX*s,transform.shear*c-transform.scaleY*s,transform.shear*s+transform.scaleY*c,transform.x,transform.y];
}

export function drawingMatrixToTransform(matrix: readonly [number,number,number,number,number,number]): DrawingTransform {
  const [a,b,c,d,x,y]=matrix, scaleX=Math.hypot(a,b);
  if (scaleX===0) return {x,y,scaleX:0,scaleY:Math.hypot(c,d),rotation:c===0&&d===0?0:Math.atan2(-c,d),shear:0};
  const ux=a/scaleX,uy=b/scaleX;
  return {x,y,scaleX,scaleY:ux*d-uy*c,rotation:Math.atan2(b,a),shear:ux*c+uy*d};
}
