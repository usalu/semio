/** 🖌️ Concrete Block3d world-window brush preview state. */
export interface Block3dBrushPreview {
  position: [number, number, number];
  direction: [number, number, number];
}

export interface Block3dWorldWindowTransient {
  brushPreview?: Block3dBrushPreview;
}

export interface SetBrushPreview {
  preview?: Block3dBrushPreview;
}
