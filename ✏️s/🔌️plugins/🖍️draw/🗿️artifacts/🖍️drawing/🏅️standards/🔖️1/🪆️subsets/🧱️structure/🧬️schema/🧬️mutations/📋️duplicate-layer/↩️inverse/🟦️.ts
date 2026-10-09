/** ↩️ Inverse uses the same admitted subtree identities as the forward clone. */
import type {DuplicateLayer} from "../🦠️mutation/🟦️.ts";
import type {DrawingLayerNode} from "../../../../../✳️any/🧬️schema/🟦️.ts";
import {cloneDrawingLayerNode} from "../../../../../✳️any/🧬️schema/🪪️identity/🟦️.ts";
export function inverse(payload:DuplicateLayer,source:DrawingLayerNode|undefined,occupiedKeys:readonly string[]):Array<{layerId:string}>{if(!source)return [];if(payload.identities.some(identity=>occupiedKeys.includes(identity.target)))throw Error("Drawing duplicate target already exists");if(source.id!==payload.layerId)throw Error("Drawing duplicate source differs");return [{layerId:cloneDrawingLayerNode(source," copy",payload.identities).id}];}
