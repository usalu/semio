/** 🎹️ BinaryComposer meta. */
export const meta = {
  writes: { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" },
  reads: [{ artifactKind: "s.stdio.binary", standard: "raw", subset: "*" }],
} as const;
