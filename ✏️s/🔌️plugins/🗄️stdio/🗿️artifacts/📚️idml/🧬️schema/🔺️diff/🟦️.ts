/** 🔺️ IDML package events carry ordered sparse part changes. */
export interface IdmlPartPatch { path: string; bytes: number[] | null }
export interface IdmlPatch { domVersion?: string; partOrder?: string[]; parts: IdmlPartPatch[] }
export interface IdmlDiff { steps: IdmlPatch[] }
