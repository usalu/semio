/** 🖌️ block2d update-presentation/🦠️mutation — the whole rim-presentation facet atomically (shape/radius/width/height/color/iconKind are edited together in the shape inspector — see report). */
export interface UpdatePresentation {
  newShape?: string;
  newRadius?: number;
  newWidth?: number;
  newHeight?: number;
  newColor?: string;
  newIconKind?: string;
}
