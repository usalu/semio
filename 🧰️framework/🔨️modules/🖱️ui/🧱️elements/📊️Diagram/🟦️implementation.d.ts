//#region 📐️GraphImplementations
/** @emoji 🔒️ Untyped runtime implementations are narrowed only by the owned Diagram ports. */
declare module "dagre" {
  export const graphlib: unknown;
  export const layout: unknown;
}

declare module "d3-force" {
  export const forceCollide: unknown;
  export const forceLink: unknown;
  export const forceManyBody: unknown;
  export const forceSimulation: unknown;
  export const forceX: unknown;
  export const forceY: unknown;
}
//#endregion 📐️GraphImplementations
