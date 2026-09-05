/** 🌊 GltfRoughnessIndicators. */
import type { GltfStatisticsMeasure, GltfScalarMeasure } from '../../../🔨️modules/🧾️measurement-contracts/🟦️.ts';
export interface GltfRoughnessIndicators { deviationFromIdeal: GltfStatisticsMeasure; deviationFromSmoothedGeometry: GltfStatisticsMeasure; normalVariation: GltfStatisticsMeasure; surfaceWaviness: GltfStatisticsMeasure; irregularity: GltfScalarMeasure }
