/** 📋️ The time-travel band readout shape shared by the React and wgpu readers. */
export type BandControl = { control: string; disabled: boolean; title: string | null; keys: string | null; text: string; native?: boolean; id?: string };

export type Band = { stage: string; role: string | null; live: string | null; generation: string | null; text: string; review: string | null; outcome: string | null; fault: string | null; target: string | null; progress: string | null; controls: BandControl[] };
