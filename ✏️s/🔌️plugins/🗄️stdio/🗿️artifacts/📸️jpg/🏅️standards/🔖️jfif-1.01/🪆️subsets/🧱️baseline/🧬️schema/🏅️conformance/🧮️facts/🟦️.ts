/** 🧮️ First-party observations for pure baseline conformance rules. */
export interface JpgSamplingFact { id:number;horizontal:number;vertical:number }
export interface JpgBaselineFacts { hasFrame:boolean;baselineSequential:boolean;samplePrecision:number;arithmeticConditioning:boolean;dcTableCount:number;acTableCount:number;components:JpgSamplingFact[] }
