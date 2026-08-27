/** ➕️ Direct append-page-content TypeScript payload. */
export interface AppendPageContentMutation {
  mutation: 'appendPageContent';
  index: number;
  text: string;
}
