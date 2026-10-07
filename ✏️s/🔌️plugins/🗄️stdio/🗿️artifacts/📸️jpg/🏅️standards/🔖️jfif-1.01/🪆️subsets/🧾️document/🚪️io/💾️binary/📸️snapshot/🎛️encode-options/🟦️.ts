/** 🎛️ Physical baseline JPEG export policy. */
export interface JpgEncodeComponent { id: number; hSampling: number; vSampling: number; }
/** 🎚️ Native quantization and sampling choices for one export request. */
export interface JpgEncodeOptions { quality: number; components: JpgEncodeComponent[]; }
