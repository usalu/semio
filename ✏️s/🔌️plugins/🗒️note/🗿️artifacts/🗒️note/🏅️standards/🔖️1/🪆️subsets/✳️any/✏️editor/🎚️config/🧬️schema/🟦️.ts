/** 🧬️ NoteConfig */
export interface NoteConfig {}
export const parseNoteConfig = (value: unknown): NoteConfig => {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== 0) throw new Error("NoteConfig must be an empty object");
  return {};
};
