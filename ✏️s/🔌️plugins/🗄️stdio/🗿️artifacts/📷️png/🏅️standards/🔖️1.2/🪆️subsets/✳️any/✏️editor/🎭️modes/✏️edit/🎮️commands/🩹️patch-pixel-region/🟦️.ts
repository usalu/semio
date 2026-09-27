/** 🩹️ Typed PNG RGBA8 region operation shared by the neutral witness. */
export type PngPixelRegion = Readonly<{
  x: number;
  y: number;
  width: number;
  height: number;
  red: number;
  green: number;
  blue: number;
  alpha: number;
}>;

const integer = (value: number, name: string, minimum: number, maximum: number): number => {
  if (!Number.isSafeInteger(value) || value < minimum || value > maximum) throw new RangeError(`${name} is outside ${minimum}..${maximum}`);
  return value;
};

/** 🎨️ Applies one checked solid RGBA8 rectangle without changing bytes outside it. */
export const applyPngPixelRegionRgba8 = (pixels: Uint8Array, imageWidth: number, imageHeight: number, region: PngPixelRegion): Uint8Array => {
  const width = integer(imageWidth, "imageWidth", 0, 0xffff_ffff);
  const height = integer(imageHeight, "imageHeight", 0, 0xffff_ffff);
  if (pixels.length !== width * height * 4) throw new RangeError("pixels must be canonical width * height * 4 RGBA8 bytes");
  const x = integer(region.x, "x", 0, 0xffff_ffff);
  const y = integer(region.y, "y", 0, 0xffff_ffff);
  const regionWidth = integer(region.width, "width", 1, 0xffff_ffff);
  const regionHeight = integer(region.height, "height", 1, 0xffff_ffff);
  if (x + regionWidth > width || y + regionHeight > height) throw new RangeError("pixel region exceeds the image bounds");
  const color = [
    integer(region.red, "red", 0, 255),
    integer(region.green, "green", 0, 255),
    integer(region.blue, "blue", 0, 255),
    integer(region.alpha, "alpha", 0, 255),
  ];
  const next = new Uint8Array(pixels);
  for (let row = y; row < y + regionHeight; row += 1) {
    for (let column = x; column < x + regionWidth; column += 1) next.set(color, (row * width + column) * 4);
  }
  return next;
};
