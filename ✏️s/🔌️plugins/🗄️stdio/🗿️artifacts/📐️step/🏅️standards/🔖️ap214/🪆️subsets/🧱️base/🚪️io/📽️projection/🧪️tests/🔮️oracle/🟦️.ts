/** 🔮️ Thirdparty geometry oracle exposes only owned numeric records. */
import {EllipseCurve,Matrix4,Vector3} from "three";
export function circlePoints(center:number[],axis:number[],normal:number[],radius:number,start:number[],forward:boolean,segments:number):number[][]{
    const z=new Vector3(...normal as[number,number,number]).normalize();const x=new Vector3(...axis as[number,number,number]).projectOnPlane(z).normalize();const y=new Vector3().crossVectors(z,x);const relative=new Vector3(...start as[number,number,number]).sub(new Vector3(...center as[number,number,number]));const first=Math.atan2(relative.dot(y),relative.dot(x));const path=new EllipseCurve(0,0,radius,radius,first,first+(forward?2:-2)*Math.PI,!forward,0);const transform=new Matrix4().makeBasis(x,y,z).setPosition(...center as[number,number,number]);return path.getPoints(segments).map(point=>new Vector3(point.x,point.y,0).applyMatrix4(transform).toArray());
}
