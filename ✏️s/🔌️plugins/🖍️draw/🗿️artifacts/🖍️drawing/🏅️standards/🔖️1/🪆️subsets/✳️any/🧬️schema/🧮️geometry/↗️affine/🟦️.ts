/** ↗️ Full affine composition and QR decomposition, including shear and reflections. */
import {binary64,binary64Value} from "@semio-tech/framework/value/ieee754";
import type {DrawingTransform as DrawingTransformWords} from "../../🟦️.ts";
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

/** 📥️ Capture a numeric transform as the persisted binary64 words, the single place a computed transform enters the model. */
export function drawingTransformToWords(transform: DrawingTransform): DrawingTransformWords {
  return {x:binary64(transform.x),y:binary64(transform.y),scaleX:binary64(transform.scaleX),scaleY:binary64(transform.scaleY),shear:binary64(transform.shear),rotation:binary64(transform.rotation)};
}

/** 📤️ Derive the numeric transform of a persisted one, the single computational read of its words. */
export function drawingTransformFromWords(words: DrawingTransformWords): DrawingTransform {
  return {x:binary64Value(words.x),y:binary64Value(words.y),scaleX:binary64Value(words.scaleX),scaleY:binary64Value(words.scaleY),shear:binary64Value(words.shear),rotation:binary64Value(words.rotation)};
}
