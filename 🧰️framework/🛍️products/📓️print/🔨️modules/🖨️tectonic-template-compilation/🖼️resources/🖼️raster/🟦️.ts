/** 📐 Names one bounded SVG-to-PNG request using owned DATA only. */
export interface PrintRasterSvgRequestV1 {
  readonly svg: Uint8Array;
  readonly width: number;
  readonly maximumPixels: number;
}

/** 🧊 Names one frosted-glass panel request over actual page PNG bytes: crop, blur, saturate, tint with raw RGBA. */
export interface PrintRasterFrostRequestV1 {
  readonly png: Uint8Array;
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  readonly blurSigma: number;
  readonly saturation: number;
  readonly tintRgba: Uint8Array;
  readonly maximumPixels: number;
}

/** 🖼️ Describes the tool-independent Print raster interface; no implementation library type appears here. */
export interface PrintRasterEngineV1 {
  svgToPng(request: PrintRasterSvgRequestV1, signal: AbortSignal): Promise<Uint8Array>;
  frostPanel(request: PrintRasterFrostRequestV1): Promise<Uint8Array>;
}
