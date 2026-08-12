/** 🔭 create-camera-calibration mutation payload — brings a new camera calibration into existence. */
export interface CreateCameraCalibration {
  camera: CameraCalibration;
}

export interface CameraCalibration {
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
}
