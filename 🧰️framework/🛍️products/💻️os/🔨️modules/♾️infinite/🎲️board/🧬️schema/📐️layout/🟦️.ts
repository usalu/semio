export type DagLayoutOrientation="leftRight"|"topBottom";
export interface ForceGraphLayoutOptions {
 iterations?: number;
 idealEdgeLength?: number;
 repulsionStrength?: number;
 springStrength?: number;
 gravity?: number;
 centerX?: number;
 centerY?: number;
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
 centerX?: number;
 centerY?: number;
 lockedNodeIds?: string[];
}
export interface RedrawLayoutOptions {
 mode: string;
 centerX?: number;
 centerY?: number;
 randomSeed?: number;
 redrawHandlesAfter?: boolean;
 lockedNodeIds?: string[];
 forceGraph?: ForceGraphLayoutOptions;
 hierarchicalTree?: HierarchicalTreeLayoutOptions;
}
export interface DagLayoutOptions {
 layerSpacing?: number;
 siblingGap?: number;
 orientation?: DagLayoutOrientation;
 centerX?: number;
 centerY?: number;
}
