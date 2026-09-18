/** 📐️ Direct set-page-media-box TypeScript payload. */
export interface SetPageMediaBoxMutation {
  mutation: 'setPageMediaBox';
  index: number;
  mediaBox: [number, number, number, number];
}
