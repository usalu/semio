/** 📦 GltfSizeIndicators. */
import type { GltfScalarMeasure, GltfBoundsMeasure, GltfVectorMeasure, GltfStatisticsMeasure } from '../🧾️measure/🟦️component.ts';
export interface GltfSizeIndicators { overallSize: GltfScalarMeasure; axisAlignedBounds: GltfBoundsMeasure; orientedBounds: GltfBoundsMeasure; boundingBoxDimensions: GltfVectorMeasure; characteristicLength: GltfScalarMeasure; footprintArea: GltfScalarMeasure; projectedArea: GltfStatisticsMeasure }

