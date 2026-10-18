

export type IconRenderFormat = "svg" | "png";

export type IconRenderShape = "rectangle" | "ellipse";

export type IconRenderProjection = "perspective" | "orthographic";

export interface IconRenderCamera {
  readonly position: readonly [number, number, number];
  readonly target: readonly [number, number, number];
  readonly zoom: number;
  readonly fov?: number;
  readonly up?: readonly [number, number, number];
  readonly projection?: IconRenderProjection;
}

export interface IconRenderLights {
  readonly ambientIntensity: number;
  readonly ambientColor: string;
  readonly sunAzimuth: number;
  readonly sunElevation: number;
  readonly sunIntensity: number;
  readonly sunColor: string;
}

export interface IconRenderMaterial {
  readonly color?: string;
  readonly metalness?: number;
  readonly roughness?: number;
  readonly emissive?: string;
  readonly emissiveIntensity?: number;
  /** ✏️ GLB mesh edge outline color for SVG/PNG icon shots (`none`/`transparent` disables outlines). */
  readonly stroke?: string;
}

/** 🎯️ Frames the loaded asset before rendering: the camera keeps its viewing direction and zoom,
 * re-targets the asset's bounding-sphere centre and backs off so the sphere fits the frame with `padding`
 * (the icon twin of the world-3d fit lane, so an icon shows what a centred scene shows). */
export interface IconRenderFit {
  readonly enabled: boolean;
  readonly padding?: number;
}

export interface IconRenderRequest {
  readonly assetUrl: string;
  readonly camera: IconRenderCamera;
  readonly fit?: IconRenderFit;
  readonly lights: IconRenderLights;
  readonly width: number;
  readonly height: number;
  readonly format: IconRenderFormat;
  readonly shape?: IconRenderShape;
  readonly background?: string;
  readonly shadowEnabled?: boolean;
  readonly material?: IconRenderMaterial;
}

export interface IconRenderResult {
  readonly dataUrl: string;
  readonly svgMarkup?: string;
}

export interface IconRenderPort {
  render(request: IconRenderRequest): Promise<IconRenderResult>;
}
