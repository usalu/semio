/** 🖼️ Checked byte spans for solid RGBA8 rectangular edits. */
export interface RasterRegion { readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly color: readonly [number, number, number, number] }
export interface RasterRegionLimits { maximumRasterBytes: number; maximumPatchBytes: number; maximumPatches: number }
const rasterRegionPlanBrand = Symbol("checked-raster-region");
export interface RasterRegionPlan { readonly [rasterRegionPlanBrand]: true; readonly region: RasterRegion; readonly byteLength: number; readonly rowBytes: number; readonly regionRowBytes: number; readonly patchBytes: number; readonly rowsPerPatch: number; readonly chunksPerRow: number; readonly patchCount: number }
export interface RasterBytePatch { index: number; pixels: Uint8Array }
const rasterRegionPlans = new WeakSet<RasterRegionPlan>();

function refuse(code: string): never { throw new Error(code); }
function unsigned(value: number, maximum: number): boolean { return Number.isSafeInteger(value) && value >= 0 && value <= maximum; }

/** 📏 Validates a complete raster and plans bounded, nonoverlapping spans without allocating pixels. */
export function planRasterRegion(width: number, height: number, byteLength: number, region: RasterRegion, limits: RasterRegionLimits): RasterRegionPlan {
  if (![width, height, region.x, region.y, region.width, region.height].every(value => unsigned(value, 0xffffffff)) || region.color.length !== 4 || !region.color.every(value => unsigned(value, 255))) refuse("invalid-argument");
  if (region.width === 0 || region.height === 0) refuse("empty");
  if (!Object.values(limits).every(value => unsigned(value, Number.MAX_SAFE_INTEGER)) || limits.maximumPatchBytes < 4 || limits.maximumPatchBytes % 4 !== 0 || limits.maximumPatches === 0) refuse("invalid-budget");
  const rowBytes = width * 4;
  const expected = rowBytes * height;
  if (!Number.isSafeInteger(expected)) refuse("extent-overflow");
  if (expected !== byteLength) refuse("noncanonical-raster");
  if (expected > limits.maximumRasterBytes) refuse("raster-too-large");
  const right = region.x + region.width;
  const bottom = region.y + region.height;
  if (right > 0xffffffff || bottom > 0xffffffff) refuse("bounds-overflow");
  if (right > width || bottom > height) refuse("out-of-bounds");
  const regionRowBytes = region.width * 4;
  const rowsPerPatch = rowBytes <= limits.maximumPatchBytes ? Math.floor(limits.maximumPatchBytes / rowBytes) : 0;
  const chunksPerRow = rowsPerPatch === 0 ? Math.ceil(regionRowBytes / limits.maximumPatchBytes) : 0;
  const patchCount = rowsPerPatch > 0 ? Math.ceil(region.height / rowsPerPatch) : region.height * chunksPerRow;
  if (!Number.isSafeInteger(patchCount) || patchCount > limits.maximumPatches) refuse("too-many-patches");
  const plan = Object.freeze({ [rasterRegionPlanBrand]: true as const, region: Object.freeze({ ...region, color: Object.freeze([...region.color]) as RasterRegion["color"] }), byteLength, rowBytes, regionRowBytes, patchBytes: limits.maximumPatchBytes, rowsPerPatch, chunksPerRow, patchCount });
  rasterRegionPlans.add(plan);
  return plan;
}

/** 🩹️ Materializes at most one planned payload; unchanged spans allocate no patch. */
export function rasterRegionPatch(plan: RasterRegionPlan, pixels: Uint8Array, ordinal: number): RasterBytePatch | undefined {
  if (!rasterRegionPlans.has(plan)) refuse("foreign-plan");
  if (pixels.length !== plan.byteLength) refuse("noncanonical-raster");
  if (!unsigned(ordinal, plan.patchCount - 1)) refuse("invalid-ordinal");
  const { region, rowBytes, regionRowBytes } = plan;
  const relativeRow = plan.rowsPerPatch > 0 ? ordinal * plan.rowsPerPatch : Math.floor(ordinal / plan.chunksPerRow);
  const rows = plan.rowsPerPatch > 0 ? Math.min(plan.rowsPerPatch, region.height - relativeRow) : 1;
  const offset = plan.rowsPerPatch > 0 ? 0 : ordinal % plan.chunksPerRow * plan.patchBytes;
  const index = (region.y + relativeRow) * rowBytes + (plan.rowsPerPatch > 0 ? 0 : region.x * 4 + offset);
  const length = plan.rowsPerPatch > 0 ? rows * rowBytes : Math.min(plan.patchBytes, regionRowBytes - offset);
  const left = plan.rowsPerPatch > 0 ? region.x * 4 : 0;
  const paintedBytes = plan.rowsPerPatch > 0 ? regionRowBytes : length;
  let changed = false;
  for (let row = 0; row < rows; row++) for (let channel = 0; channel < paintedBytes; channel++) if (pixels[index + row * rowBytes + left + channel] !== region.color[channel % 4]) changed = true;
  if (!changed) return undefined;
  const patch = pixels.slice(index, index + length);
  for (let row = 0; row < rows; row++) for (let channel = 0; channel < paintedBytes; channel++) patch[row * rowBytes + left + channel] = region.color[channel % 4]!;
  return { index, pixels: patch };
}
