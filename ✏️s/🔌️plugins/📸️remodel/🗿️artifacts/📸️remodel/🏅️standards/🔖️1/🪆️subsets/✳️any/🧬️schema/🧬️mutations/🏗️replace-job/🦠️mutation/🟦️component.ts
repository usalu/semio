/** 🏗 replace-job mutation payload — whole-value swap of the live reconstruction run state. */
export interface ReplaceJob {
  job: {
    id: string;
    stage: string;
    progress01: number;
    cancelRequested: boolean;
    stageCursor: number;
    startedAtMs?: number;
    error?: string;
    cameraPosesPreview: { cameraId: string; rotationWxyz: [number, number, number, number]; translation: [number, number, number] }[];
    sparsePointCloudPreview: string;
  };
}
