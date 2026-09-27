/** 📷️ A measured, revision-addressed request to frame world-space canvas bounds. */
export interface Canvas2dFraming {
  readonly revision: number;
  readonly bounds: readonly [number,number,number,number];
  readonly padding: number;
}
export interface Canvas2dFrameCamera { readonly x: number; readonly y: number; readonly zoom: number }
/** 🖼️ Fits measured CSS dimensions, retaining finite cameras for points, lines and extreme zoom. */
export function fitCanvasFrame(request: Canvas2dFraming,width: number,height: number): Canvas2dFrameCamera | null {
  const [x0,y0,x1,y1] = request.bounds;
  if (![x0,y0,x1,y1,width,height,request.padding].every(Number.isFinite) || width <= 0 || height <= 0 || request.padding < 0 || x1 < x0 || y1 < y0 || !Number.isInteger(request.revision) || request.revision < 0 || request.revision > 4294967295) return null;
  const dx = x1-x0, dy = y1-y0;
  if (!Number.isFinite(dx) || !Number.isFinite(dy)) return null;
  const zx = dx > 0 ? Math.max(1,width-2*request.padding)/dx : Infinity;
  const zy = dy > 0 ? Math.max(1,height-2*request.padding)/dy : Infinity;
  const zoom = dx === 0 && dy === 0 ? 1 : Math.max(.05,Math.min(32,zx,zy));
  return { x: x0*.5+x1*.5,y: y0*.5+y1*.5,zoom };
}
