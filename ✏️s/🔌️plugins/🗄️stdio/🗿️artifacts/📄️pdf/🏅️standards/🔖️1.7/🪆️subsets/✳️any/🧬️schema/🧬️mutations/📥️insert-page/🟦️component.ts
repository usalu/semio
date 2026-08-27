/** 📥️ Direct insert-page TypeScript payload. */
export interface InsertPageMutation {
  mutation: 'insertPage';
  index: number;
  page: unknown;
}
