export * from "./types.js";
export * from "./draw-layers.js";
export * from "./flow-dag.js";
export * from "./note-blocks.js";
export * from "./concrete-forest.js";
export * from "./puzzle2d-default.js";
export * from "./puzzle3d-default.js";
export * from "./puzzle5d-default.js";
export * from "./wires.js";
export * from "./s-resources.js";
export * from "./nakagin.js";
export * from "./rewrite-lhs.js";
export * from "./rewrite-rhs.js";
export * from "./writer-languages.js";

import { DRAWLAYERS_MANIFEST_DOCUMENT } from "./draw-layers.js";
import { FLOWDAG_MANIFEST_DOCUMENT } from "./flow-dag.js";
import { NOTEBLOCKS_MANIFEST_DOCUMENT } from "./note-blocks.js";
import { CONCRETEFOREST_MANIFEST_DOCUMENT } from "./concrete-forest.js";
import { PUZZLE2DDEFAULT_MANIFEST_DOCUMENT } from "./puzzle2d-default.js";
import { PUZZLE3DDEFAULT_MANIFEST_DOCUMENT } from "./puzzle3d-default.js";
import { PUZZLE5DDEFAULT_MANIFEST_DOCUMENT } from "./puzzle5d-default.js";
import { WIRES_MANIFEST_DOCUMENT } from "./wires.js";
import { SRESOURCES_MANIFEST_DOCUMENT } from "./s-resources.js";
import { NAKAGIN_MANIFEST_DOCUMENT } from "./nakagin.js";
import { REWRITELHS_MANIFEST_DOCUMENT } from "./rewrite-lhs.js";
import { REWRITERHS_MANIFEST_DOCUMENT } from "./rewrite-rhs.js";
import { WRITERLANGUAGES_MANIFEST_DOCUMENT } from "./writer-languages.js";
import type { GraphManifestDocument } from "./types.js";

export function manifestById(id: string): GraphManifestDocument | undefined {
  switch (id) {
    case "draw-layers": return DRAWLAYERS_MANIFEST_DOCUMENT;
    case "flow-dag": return FLOWDAG_MANIFEST_DOCUMENT;
    case "note-blocks": return NOTEBLOCKS_MANIFEST_DOCUMENT;
    case "concrete-forest": return CONCRETEFOREST_MANIFEST_DOCUMENT;
    case "puzzle2d-default": return PUZZLE2DDEFAULT_MANIFEST_DOCUMENT;
    case "puzzle3d-default": return PUZZLE3DDEFAULT_MANIFEST_DOCUMENT;
    case "puzzle5d-default": return PUZZLE5DDEFAULT_MANIFEST_DOCUMENT;
    case "wires": return WIRES_MANIFEST_DOCUMENT;
    case "s-resources": return SRESOURCES_MANIFEST_DOCUMENT;
    case "nakagin": return NAKAGIN_MANIFEST_DOCUMENT;
    case "rewrite-lhs": return REWRITELHS_MANIFEST_DOCUMENT;
    case "rewrite-rhs": return REWRITERHS_MANIFEST_DOCUMENT;
    case "writer-languages": return WRITERLANGUAGES_MANIFEST_DOCUMENT;
    default: return undefined;
  }
}
