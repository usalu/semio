import {parseDrawingFontFamily,type DrawingFontFamily} from "../../../../../✳️any/🧬️schema/📝️text/🔤️family/🟦️.ts";
/** 📝️ Replaces a text layer's authored content and size while preserving its identity and appearance. */
export interface UpdateText { readonly layerId: string; readonly content: string; readonly size: number; readonly fontFamily:DrawingFontFamily }
interface TextLayer { readonly id: string; readonly kind: string; readonly children?: readonly TextLayer[] }
interface TextDocument { readonly layers: readonly TextLayer[] }
export function applyTextEdit<T extends TextDocument>(snapshot: T, mutation: UpdateText): T {
  if (!mutation.layerId || typeof mutation.content !== "string" || typeof mutation.size !== "number" || !Number.isFinite(mutation.size) || mutation.size <= 0) throw new Error("Invalid text edit");
  parseDrawingFontFamily(mutation.fontFamily);
  let found = false;
  const visit = (layer: TextLayer): TextLayer => {
    if (layer.id === mutation.layerId) {
      if (layer.kind !== "text") throw new Error("Text target has another kind");
      found = true;
      return {...layer, ...{content: mutation.content, size: mutation.size,fontFamily:mutation.fontFamily}};
    }
    return layer.kind === "group" && layer.children ? {...layer, children: layer.children.map(visit)} : layer;
  };
  const layers = snapshot.layers.map(visit);
  if (!found) throw new Error("Text target is missing");
  return {...snapshot, layers};
}

/** 🩹️ A semantic text edit emits only changed authored facets. */
export function textEditPatch(before:{content:string;size:number;fontFamily:DrawingFontFamily},mutation:UpdateText):{textContent?:string;textSize?:number;fontFamily?:DrawingFontFamily}{parseDrawingFontFamily(mutation.fontFamily);if(typeof mutation.content!=="string"||!Number.isFinite(mutation.size)||mutation.size<=0)throw Error("Invalid text edit");return{...(before.content!==mutation.content?{textContent:mutation.content}:{}),...(before.size!==mutation.size?{textSize:mutation.size}:{}),...(before.fontFamily!==mutation.fontFamily?{fontFamily:mutation.fontFamily}:{})};}
