/** 🧭️ A semantic selection address shared by block-list renderers. */
export type BlockListSelectionTarget = {readonly granularity:string;readonly id:string};
/** 🧱️ The renderer projection of one domain block. */
export type BlockListBlock = {readonly id:string;readonly label:string;readonly kind:string;readonly description?:string;readonly target?:BlockListSelectionTarget};
/** 🪜️ One ordered section with its directly owned block records. */
export type BlockListStep = {readonly id:string;readonly title:string;readonly description?:string;readonly blocks:readonly BlockListBlock[];readonly target?:BlockListSelectionTarget};
/** 🎨️ One insertable block kind with its presentation identity. */
export type BlockListPaletteEntry = {readonly blockKind:string;readonly label:string;readonly iconId:string};
/** 🎬️ A typed block-list presentation without embedded serialized documents. */
export type BlockListScene = {readonly steps:readonly BlockListStep[];readonly palette:readonly BlockListPaletteEntry[];readonly domainId?:string;readonly selectedId?:string;readonly draggingId?:string};
/** 🎯️ Selects the current section, the selected block's parent, or the first section. */
export function blockListPaletteTargetStepIdV1(steps:readonly BlockListStep[],selectedId?:string):string|undefined {
 if(selectedId){const section=steps.find(step=>step.id===selectedId);if(section)return section.id;const parent=steps.find(step=>step.blocks.some(block=>block.id===selectedId));if(parent)return parent.id;}
 return steps[0]?.id;
}
