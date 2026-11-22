/** 📏️ Sizes one canvas' backing store to `logicalW`×`logicalH` at `dpr` and returns whether the store
 * actually changed. Deliberately synchronous and free of any animation frame: a hidden/background tab
 * never fires `requestAnimationFrame`, so a canvas whose size is only reached from a rAF tick stays at
 * the HTML default 300×150 forever (`📓️runtime-verification-2026-09-09.md` boot #3). */
export function resizeCanvasBackingStore(canvas: HTMLCanvasElement | null | undefined, logicalW: number, logicalH: number, dpr: number): boolean {
  if (!canvas) return false;
  const pixelW = Math.max(1, Math.round(logicalW * dpr));
  const pixelH = Math.max(1, Math.round(logicalH * dpr));
  const changed = canvas.width !== pixelW || canvas.height !== pixelH;
  if (changed) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  canvas.style.width = `${Math.max(1, Math.round(logicalW))}px`;
  canvas.style.height = `${Math.max(1, Math.round(logicalH))}px`;
  return changed;
}
