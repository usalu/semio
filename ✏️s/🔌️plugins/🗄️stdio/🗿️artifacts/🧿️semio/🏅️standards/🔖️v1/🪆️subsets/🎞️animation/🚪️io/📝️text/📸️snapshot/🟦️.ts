/** 📝️ Text representation for `stdio.semio.animation.snapshot`. */
export interface AnimTimeline { name?: string; channels: AnimChannel[] }
export interface AnimChannel { target: AnimTarget; interpolation: 'linear' | 'step' | 'cubicSpline'; keyframes: AnimKeyframe[] }
export interface AnimTarget { node: string; property: { kind: 'translation' | 'rotation' | 'scale' | 'weights' } | { kind: 'custom'; name: string } }
export interface AnimKeyframe { t: number; value: AnimValue }
export type AnimValue = { kind: 'scalar'; value: number } | { kind: 'vec3'; value: { x: number; y: number; z: number } } | { kind: 'quat'; value: { x: number; y: number; z: number; w: number } } | { kind: 'weights'; values: number[] }
