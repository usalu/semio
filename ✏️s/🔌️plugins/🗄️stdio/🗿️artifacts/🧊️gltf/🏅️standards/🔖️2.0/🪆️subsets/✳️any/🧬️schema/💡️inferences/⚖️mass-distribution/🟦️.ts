/** ⚖️ GltfMassIndicators. */
import type { GltfVectorMeasure, GltfPrincipalFrameMeasure, GltfDirectionsMeasure, GltfMeasure } from '../../../🔨️modules/🧾️measurement-contracts/🟦️.ts';
export interface GltfMassIndicators { centroid: GltfVectorMeasure; principalFrame: GltfPrincipalFrameMeasure; principalAxes: GltfDirectionsMeasure; momentsOfInertia: GltfVectorMeasure; inertiaTensor: GltfMeasure<number[]> }
