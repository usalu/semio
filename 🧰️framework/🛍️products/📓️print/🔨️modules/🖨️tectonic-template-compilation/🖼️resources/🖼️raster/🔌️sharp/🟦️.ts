import sharp from "sharp";
import type { PrintRasterEngineV1, PrintRasterFrostRequestV1, PrintRasterSvgRequestV1 } from "../🟦️.ts";

/** 🔌 Implements the Print raster interface with sharp; this is the only Print source importing it. */
export const sharpPrintRasterEngineV1: PrintRasterEngineV1 = {
  async svgToPng(request: PrintRasterSvgRequestV1, signal: AbortSignal): Promise<Uint8Array> {
    const { width, maximumPixels } = request;
    const image = sharp(Buffer.from(request.svg), { density: 72, limitInputPixels: maximumPixels });
    const abort = () => image.destroy(signal.reason instanceof Error ? signal.reason : Error("SVG rendering cancelled"));
    signal.addEventListener("abort", abort, { once: true });
    try {
      signal.throwIfAborted();
      const metadata = await image.metadata();
      if (!metadata.width || !metadata.height || Math.ceil(metadata.height * width / metadata.width) * width > maximumPixels) throw Error("Bounded SVG resource dimensions required");
      signal.throwIfAborted();
      const bytes = await image.resize({ width, withoutEnlargement: false }).png().toBuffer();
      signal.throwIfAborted();
      return new Uint8Array(bytes);
    } finally { signal.removeEventListener("abort", abort); image.destroy(); }
  },
  async frostPanel(request: PrintRasterFrostRequestV1): Promise<Uint8Array> {
    const { width, height } = request;
    const output = await sharp(Buffer.from(request.png), { limitInputPixels: request.maximumPixels })
      .extract({ left: request.left, top: request.top, width, height })
      .removeAlpha()
      .blur(request.blurSigma)
      .modulate({ saturation: request.saturation })
      .composite([{ input: Buffer.from(request.tintRgba), raw: { width, height, channels: 4 }, blend: "over" }])
      .png({ compressionLevel: 9 })
      .toBuffer();
    return new Uint8Array(output);
  }
};
