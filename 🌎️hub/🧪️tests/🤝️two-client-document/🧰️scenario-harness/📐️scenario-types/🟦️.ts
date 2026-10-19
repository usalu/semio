/** 📐️ The frame, socket holder and minted grant records the two-client document e2e passes between its phases. */
export type Frame = Record<string, any>;
export type Holder = { label: string; actor: string; socket: WebSocket; frames: Frame[]; waiters: Array<(f: Frame) => void>; closed: Promise<{ code: number; reason: string }>; welcome?: any; resumeToken?: string };
export type Minted = { label: string; token: string; actor: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; wsUrl: string; protocols: string[] };
