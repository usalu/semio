/** 🛠 update-camera-calibration mutation payload — full-record replace of an existing calibration. */
export interface UpdateCameraCalibration {
  camera: {
    id: string;
    label: string;
    model: string;
    fx: number;
    fy: number;
    cx: number;
    cy: number;
    skew: number;
    distortion: [number, number, number, number, number];
    rmsReprojectionPx?: number;
    locked: boolean;
  };
}
