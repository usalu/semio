/** 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes (TextBox/Picture/Table/
 * Placeholder) + per-slide notes. `DocBlock` is document's own type (imported, not redefined). */
import type { DocBlock } from "../../../📑️document/🧬️schema/📸️snapshot/🟦️";

export interface SemioPoint2 { x: number; y: number; }

export interface SlideFrame { origin: SemioPoint2; width: number; height: number; }

export interface SlidePictureImage { assetId: string; mime: string; bytes: number[]; }

export type PlaceholderKind =
  | { kind: "title" } | { kind: "subtitle" } | { kind: "body" } | { kind: "footer" }
  | { kind: "slideNumber" } | { kind: "dateTime" } | { kind: "other"; value: string };

export interface SlideTableCell { blocks: DocBlock[]; }
export interface SlideTableRow { cells: SlideTableCell[]; }

export type SlideShape =
  | { shapeKind: "textBox"; frame: SlideFrame; blocks: DocBlock[] }
  | { shapeKind: "picture"; frame: SlideFrame; image: SlidePictureImage }
  | { shapeKind: "table"; frame: SlideFrame; rows: SlideTableRow[] }
  | { shapeKind: "placeholder"; frame: SlideFrame; kind: PlaceholderKind };

export interface SlideMaster { id: string; shapes: SlideShape[]; }
export interface SlideLayout { id: string; masterId: string; shapes: SlideShape[]; }
export interface Slide { id: string; layoutId?: string | null; shapes: SlideShape[]; notes: DocBlock[]; }

export interface SemioPresentationSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ masters: SlideMaster[];
  /** @state artifact */ layouts: SlideLayout[];
  /** @state artifact */ slides: Slide[];
}
