export type DagLayoutOrientation="leftRight"|"topBottom";
export interface ForceGraphLayoutOptions {
 iterations?: number;
 idealEdgeLength?: number;
 repulsionStrength?: number;
 springStrength?: number;
 gravity?: number;
 centerX?: number | null;
 centerY?: number | null;
 timeStep?: number;
 velocityDamping?: number;
 maxSpeed?: number;
 randomSeed?: number;
 barnesHutTheta?: number;
 pairwiseRepulsionMaxBodies?: number;
 lockedNodeIds?: string[];
}
export interface HierarchicalTreeLayoutOptions {
 layerSpacing?: number;
 siblingGap?: number;
 direction?: string;
 centerX?: number | null;
 centerY?: number | null;
 lockedNodeIds?: string[];
}
export interface RedrawLayoutOptions {
 mode: string;
 centerX?: number | null;
 centerY?: number | null;
 randomSeed?: number | null;
 redrawHandlesAfter?: boolean;
 lockedNodeIds?: string[];
 forceGraph?: ForceGraphLayoutOptions | null;
 hierarchicalTree?: HierarchicalTreeLayoutOptions | null;
}
export interface DagLayoutOptions {
 layerSpacing?: number;
 siblingGap?: number;
 orientation?: DagLayoutOrientation;
 centerX?: number | null;
 centerY?: number | null;
}
