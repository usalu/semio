/** 🧬️ Readonly numeric SVG node fields are shared by the serializer and typed fixture admission. */
import type {PathSegment,FillStyle,StrokeStyle} from "@semio-tech/framework/2d";
export interface DrawingSvgNode {
  readonly id: string;
  readonly groups?: readonly {readonly id:string;readonly opacity:number;readonly blendMode:string}[];
  readonly transform: readonly number[];
  readonly segments: readonly PathSegment[];
  readonly fill?: FillStyle;
  readonly stroke?: StrokeStyle;
  readonly opacity: number;
  readonly blendMode: string;
  readonly visible: boolean;
  readonly fillRule?: string;
  readonly text?: { readonly content: string; readonly size: number };
  readonly image?: { readonly assetId: string; readonly width: number; readonly height: number };
}
