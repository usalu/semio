/** ⚖️ GltfMassIndicators. */
import type { GltfVectorMeasure, GltfPrincipalFrameMeasure, GltfDirectionsMeasure, GltfMeasure } from '../🧾️measure/🟦️component.ts';
export interface GltfMassIndicators { centroid: GltfVectorMeasure; principalFrame: GltfPrincipalFrameMeasure; principalAxes: GltfDirectionsMeasure; momentsOfInertia: GltfVectorMeasure; inertiaTensor: GltfMeasure<number[]> }

