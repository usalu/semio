export interface NoteCamera { x: number; y: number; zoom: number; }
export interface NoteCompositeWindowConfig { camera: NoteCamera; }
export interface NoteInkToolEntry { key: string; mutation: { mutation: string } & Record<string, unknown>; }
export interface NoteInkToolState { states: string[]; verb: string; authoringSeed: string; baseRevision: string; transaction: { id: string; tool: string }; entries: NoteInkToolEntry[]; }
export interface NoteCompositeWindowTransient { engagementInput: string; inkTool?: NoteInkToolState | null; }
