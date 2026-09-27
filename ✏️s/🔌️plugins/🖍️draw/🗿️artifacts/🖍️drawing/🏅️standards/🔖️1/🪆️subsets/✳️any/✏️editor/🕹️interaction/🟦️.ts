/** 🎯️ Document membership and structural order for selection, independent of rendered panels. */
import type { DomainTopology, TopologyNode } from "../../../../../../../../../../../🧰️framework/🔨️modules/🕹️interaction/🟦️.ts";

export interface DrawingInteractionLayer {
  readonly kind: string;
  readonly base: { readonly id: string };
  readonly children?: readonly DrawingInteractionLayer[] | readonly string[];
}

export function drawingInteractionTopology(layers: readonly DrawingInteractionLayer[]): DomainTopology {
  const ordered: TopologyNode[] = [];
  const stack = [{ layers, index: 0, parent: undefined as string | undefined }];
  while (stack.length > 0) {
    const frame = stack[stack.length - 1]!;
    const layer = frame.layers[frame.index++];
    if (!layer) { stack.pop(); continue; }
    const id = layer.base.id;
    ordered.push({ id, granularity: "stroke", ...(frame.parent === undefined ? {} : { parent: frame.parent }) });
    if (layer.kind === "group") stack.push({ layers: layer.children as readonly DrawingInteractionLayer[] ?? [], index: 0, parent: id });
  }
  return { ordered };
}
