/** 🧬️ SemioPresentationMutation — named-variant mutation vocabulary, discriminated by `mutation`. */
import type { SemioPresentationSnapshot, Slide, SlideShape, SlideFrame, SlideMaster, SlideLayout } from "../📸️snapshot/🟦️";
import type { DocBlock } from "../../../📑️document/🧬️schema/📸️snapshot/🟦️";

export type SemioPresentationMutation =
  | { mutation: "setSnapshot"; snapshot: SemioPresentationSnapshot }
  | { mutation: "insertSlide"; index: number; slide: Slide }
  | { mutation: "removeSlide"; index: number }
  | { mutation: "setSlideLayout"; index: number; layoutId?: string | null }
  | { mutation: "setSlideNotes"; index: number; notes: DocBlock[] }
  | { mutation: "insertShape"; slideIndex: number; shapeIndex: number; shape: SlideShape }
  | { mutation: "removeShape"; slideIndex: number; shapeIndex: number }
  | { mutation: "setShapeFrame"; slideIndex: number; shapeIndex: number; frame: SlideFrame }
  | { mutation: "setTextBoxBlocks"; slideIndex: number; shapeIndex: number; blocks: DocBlock[] }
  | { mutation: "insertMaster"; master: SlideMaster }
  | { mutation: "removeMaster"; id: string }
  | { mutation: "insertLayout"; layout: SlideLayout }
  | { mutation: "removeLayout"; id: string }
  | { mutation: "setLayoutMaster"; id: string; masterId: string };
