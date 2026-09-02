/** ✂️ Direct set-page-crop-box TypeScript payload. */
export interface SetPageCropBoxMutation {
  mutation: 'setPageCropBox';
  index: number;
  cropBox: [number, number, number, number] | null;
}
