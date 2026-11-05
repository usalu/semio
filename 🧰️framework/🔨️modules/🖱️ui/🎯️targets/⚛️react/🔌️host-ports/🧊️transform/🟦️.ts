import type {IconRenderTransform} from "@semio-tech/ui-styling";

/** 🧱️ The host transform operations required by an icon's outer model frame. */
export interface IconRenderTransformTarget {
  readonly position:{set(x:number,y:number,z:number):unknown};
  readonly quaternion:{set(x:number,y:number,z:number,w:number):unknown};
  readonly scale:{set(x:number,y:number,z:number):unknown};
  updateMatrixWorld(force:boolean):unknown;
}

/** 🧊️ Applies the scene instance transform before camera fitting and image rendering. */
export function applyIconRenderTransform(model:IconRenderTransformTarget,transform:IconRenderTransform|undefined):void {
  if(!transform)return;
  model.position.set(...transform.position);
  model.quaternion.set(...transform.rotation);
  model.scale.set(...transform.scale);
  model.updateMatrixWorld(true);
}
