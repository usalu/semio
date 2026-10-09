export type AffineQuad = {rect: readonly [number,number,number,number]; axes: readonly [number,number]};

/** ↗️ Packs a transformed rectangle as its origin and two screen-space axes. */
export function affineQuad(rect:readonly [number,number,number,number],matrix:readonly [number,number,number,number,number,number]):AffineQuad {
  const [x,y,width,height]=rect,[a,b,c,d,e,f]=matrix;
  return {rect:[a*x+c*y+e,b*x+d*y+f,a*width,b*width],axes:[c*height,d*height]};
}

/** 📍️ Resolves one unit-square vertex using the renderer's packed affine contract. */
export function affineQuadPoint(quad:AffineQuad,corner:readonly [number,number]):readonly [number,number] {
  return [quad.rect[0]+corner[0]*quad.rect[2]+corner[1]*quad.axes[0],quad.rect[1]+corner[0]*quad.rect[3]+corner[1]*quad.axes[1]];
}
