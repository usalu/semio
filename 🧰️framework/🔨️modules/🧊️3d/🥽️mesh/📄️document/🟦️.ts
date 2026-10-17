/** 🥽️ Document: module exporting `MeshTriangleTopologyV1`, `meshTriangleTopologyV1`. */
export type MeshTriangleTopologyV1=Readonly<{vertices:number;faces:number;halfedges:number;edges:number;normal:readonly[number,number,number]}>;
/** 📄️ Projects the triangle document boundary without a retained modeling owner. */
export function meshTriangleTopologyV1(positions:readonly(readonly number[])[],indices:readonly number[]):MeshTriangleTopologyV1 {
 if(indices.length===0||indices.length%3!==0||positions.some(position=>position.length!==3||position.some(value=>!Number.isFinite(value)))||indices.some(index=>!Number.isSafeInteger(index)||index<0||index>=positions.length))throw Error("Valid triangle document required");
 const directed=new Map<string,number>(),twins: (number|undefined)[]=[];
 for(let offset=0;offset<indices.length;offset+=3)for(let corner=0;corner<3;corner++){const a=indices[offset+corner],b=indices[offset+(corner+1)%3];if(a===undefined||b===undefined)throw Error("Complete triangle required");const id=offset+corner,twin=directed.get(b+":"+a);twins[id]=twin;if(twin!==undefined)twins[twin]=id;directed.set(a+":"+b,id);}
 const points=indices.slice(0,3).map(index=>positions[index]);const[a,b,c]=points;if(!a||!b||!c)throw Error("First triangle required");
 const coordinate=(point:readonly number[],axis:number)=>{const value=point[axis];if(value===undefined)throw Error("Three coordinates required");return Math.fround(value);};
 const ab=[0,1,2].map(axis=>coordinate(b,axis)-coordinate(a,axis)),ac=[0,1,2].map(axis=>coordinate(c,axis)-coordinate(a,axis));
 const component=(vector:readonly number[],axis:number)=>{const value=vector[axis];if(value===undefined)throw Error("Three components required");return value;};
 const x=component(ab,1)*component(ac,2)-component(ab,2)*component(ac,1),y=component(ab,2)*component(ac,0)-component(ab,0)*component(ac,2),z=component(ab,0)*component(ac,1)-component(ab,1)*component(ac,0),length=Math.hypot(x,y,z);
 const normal:readonly[number,number,number]=length===0?[0,0,0]:[Math.fround(x/length),Math.fround(y/length),Math.fround(z/length)];
 return Object.freeze({vertices:positions.length,faces:indices.length/3,halfedges:indices.length,edges:twins.reduce<number>((count,twin,index)=>count+Number(twin===undefined||index<twin),0),normal:Object.freeze(normal)});
}
