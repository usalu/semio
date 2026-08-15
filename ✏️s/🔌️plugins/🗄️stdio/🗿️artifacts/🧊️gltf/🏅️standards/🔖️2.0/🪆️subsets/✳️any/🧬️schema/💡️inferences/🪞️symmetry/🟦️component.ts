/** 🪞 GltfSymmetryIndicators. */
import type { GltfScalarMeasure, GltfDirectionsMeasure } from '../../../🔨️modules/🧾️measurement-contracts/🟦️component.ts';
export interface GltfSymmetryIndicators { reflectionSymmetryScore: GltfScalarMeasure; rotationalSymmetryScore: GltfScalarMeasure; reflectionSymmetries: GltfDirectionsMeasure; rotationalSymmetries: GltfDirectionsMeasure; repetitionRatio: GltfScalarMeasure; modularityRatio: GltfScalarMeasure }
