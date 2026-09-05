/** 🧬️ JpgArtifact schema — reduced UI-editable view: identity + the raster the user is directly
 * manipulating. `pixels` is canonical 8-bit-per-channel RGBA, `width * height * 4` bytes. */
export interface JpgArtifact {
  schema: string;
  width: number;
  height: number;
  pixels: number[];
}
