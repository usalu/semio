/** 🔤 Direct embed-font-file TypeScript payload. */
export interface EmbedFontFileMutation {
  mutation: 'embedFontFile';
  descriptorOrdinal: number;
  key: string;
  program: { num: number; gen: number };
}
