/** 📦 GltfSizeIndicators. */
import type { GltfScalarMeasure, GltfBoundsMeasure, GltfVectorMeasure, GltfStatisticsMeasure } from '../../../🔨️modules/🧾️measurement-contracts/🟦️component.ts';
export interface GltfSizeIndicators { overallSize: GltfScalarMeasure; axisAlignedBounds: GltfBoundsMeasure; orientedBounds: GltfBoundsMeasure; boundingBoxDimensions: GltfVectorMeasure; characteristicLength: GltfScalarMeasure; footprintArea: GltfScalarMeasure; projectedArea: GltfStatisticsMeasure }
