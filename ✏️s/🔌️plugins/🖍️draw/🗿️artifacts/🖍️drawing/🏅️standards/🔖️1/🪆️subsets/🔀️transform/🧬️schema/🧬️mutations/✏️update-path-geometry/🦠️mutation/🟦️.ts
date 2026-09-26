/** ✏️ Pure semantic path geometry mutation twin. */
import { parsePathSegment, type DrawingArtifact, type DrawingLayerNode, type PathSegment } from "../../../../../✳️any/🧬️schema/🟦️.ts";
export interface UpdatePathGeometry { layerId: string; segments: PathSegment[] }
export function applyPathGeometry(snapshot: DrawingArtifact, mutation: UpdatePathGeometry): DrawingArtifact {
  let found = false;
  const segments = mutation.segments.map(segment => parsePathSegment(segment));
  const visit = (layer: DrawingLayerNode): DrawingLayerNode => {
    if (layer.id === mutation.layerId) {
      if (layer.kind !== "path") throw new Error("Geometry target is not a path");
      found = true;
      return { ...layer, segments: structuredClone(segments) };
    }
    return layer.kind === "group" ? { ...layer, children: (layer.children as DrawingLayerNode[]).map(visit) } : layer;
  };
  const layers = snapshot.layers.map(visit);
  if (!found) throw new Error("Path target is missing");
  return { ...snapshot, layers };
}
