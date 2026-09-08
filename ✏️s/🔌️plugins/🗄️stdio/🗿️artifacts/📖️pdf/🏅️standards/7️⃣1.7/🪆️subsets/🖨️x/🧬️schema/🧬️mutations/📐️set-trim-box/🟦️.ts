/** 📐️ Direct set-trim-box TypeScript payload. */
export interface SetTrimBoxMutation {
  mutation: 'setTrimBox';
  pageIndex: number;
  trimBox: [number, number, number, number];
}
