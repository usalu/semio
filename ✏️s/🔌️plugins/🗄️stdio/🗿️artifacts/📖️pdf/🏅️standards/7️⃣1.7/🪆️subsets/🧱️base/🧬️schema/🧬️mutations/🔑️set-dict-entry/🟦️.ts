/** 🔑️ Direct set-dict-entry TypeScript payload. */
export interface SetDictEntryMutation {
  mutation: 'setDictEntry';
  id: unknown;
  path: unknown[];
  key: string;
  value: unknown;
}
