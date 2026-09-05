/** 🌀 GltfCurvatureIndicators. */
import type { GltfStatisticsMeasure, GltfScalarMeasure } from '../../../🔨️modules/🧾️measurement-contracts/🟦️.ts';
export interface GltfCurvatureIndicators { meanCurvature: GltfStatisticsMeasure; gaussianCurvature: GltfStatisticsMeasure; curvatureHistogram: GltfStatisticsMeasure; sharpFeatureProportion: GltfScalarMeasure }
