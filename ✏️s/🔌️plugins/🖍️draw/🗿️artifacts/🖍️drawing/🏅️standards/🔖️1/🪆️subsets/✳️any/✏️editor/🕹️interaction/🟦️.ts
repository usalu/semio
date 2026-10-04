/** 🎯️ Document membership and structural order for selection, independent of rendered panels. */
import type { DomainTopology, TopologyNode } from "../../../../../../../../../../../🧰️framework/🔨️modules/🕹️interaction/🟦️.ts";

import type { PathGeometrySegment } from "../../🧬️schema/🟦️.ts";
import { geometryId,pointId,pointSlots } from "./🎯️points/🟦️.ts";

export interface DrawingInteractionLayer {
  readonly kind: string;
  readonly base: { readonly id: string; readonly visible?: boolean; readonly locked?: boolean };
  readonly segments?: readonly PathGeometrySegment[];
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

/** 📍️ Only visible, unlocked path coordinates belong to the node-selection domain. */
export function drawingPointTopology(layers:readonly DrawingInteractionLayer[]):DomainTopology {
  const ordered:TopologyNode[]=[],stack=[{layers,index:0}];
  while(stack.length) {
    const frame=stack[stack.length-1]!,layer=frame.layers[frame.index++];
    if(!layer){stack.pop();continue;}
    if(layer.base.visible===false || layer.base.locked===true)continue;
    if(layer.kind==="group")stack.push({layers:layer.children as readonly DrawingInteractionLayer[] ?? [],index:0});
    if(layer.kind!=="path")continue;
    const segments=layer.segments??[],geometry=geometryId(segments);
    if(!geometry)continue;
    for(const [index,segment] of segments.entries())for(const point of pointSlots(segment)) {
      const id=pointId({layerId:layer.base.id,geometry,index,point});
      if(id)ordered.push({id,granularity:"point"});
    }
  }
  return {ordered};
}
