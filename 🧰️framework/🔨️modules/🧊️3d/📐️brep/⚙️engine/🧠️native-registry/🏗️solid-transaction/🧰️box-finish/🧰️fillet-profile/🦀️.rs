//! 🧰️ Exact axial boundary of a box with selected parallel edges rounded.

use semio_framework_3d_brep_construction::{circular_prism::{CircularPrism,CircleSupports},primitives::{BoxGeometry,SolidPrimitive}};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3};
use std::f64::consts::FRAC_PI_2;

/// 📐️ Certifies distinct parallel corner sections with strictly positive straight spans.
pub(super) fn fillet_box(stock:SolidPrimitive,edges:&[usize],radius:f64)->Option<CircularPrism>{
    let SolidPrimitive::Box{origin,width,depth,height}=stock else{return None;};
    stock.validate().ok()?;
    fillet_placed(origin,[Vec3::X,Vec3::Y,Vec3::Z],[width,depth,height],edges,radius)
}

/// 🧭️ Rebases the exact circular profile on the authored orthonormal stock axes.
pub(super) fn fillet_placed(origin:Pnt3,axes:[Vec3;3],size:[f64;3],edges:&[usize],radius:f64)->Option<CircularPrism>{
    let tol=Tol::DEFAULT.value();
    if !(1..=4).contains(&edges.len())||!radius.is_finite()||radius<=tol{return None;}
    let [width,depth,height]=size;
    let corners=BoxGeometry::corners(width,depth,height);
    let endpoints=|index:usize| BoxGeometry::EDGES.get(index).map(|pair|pair.map(|id|corners[id].to_array()));
    let [a,b]=endpoints(edges[0])?;
    let axis=(0..3).find(|&i|a[i]!=b[i])?;
    let (u,v)=((axis+1)%3,(axis+2)%3);
    let mut ring=[[0.,0.],[size[u],0.],[size[u],size[v]],[0.,size[v]]];
    if axes[u].cross(axes[v]).dot(axes[axis])<0. {ring.swap(1,3);}
    let mut selected=[false;4];
    for &edge in edges{
        let [a,b]=endpoints(edge)?;
        if a[axis]==b[axis]{return None;}
        let corner=ring.iter().position(|p|*p==[a[u],a[v]])?;
        if std::mem::replace(&mut selected[corner],true){return None;}
    }
    let world=|[a,b]:[f64;2]|origin+(axes[u]*a+axes[v]*b);
    let points=ring.map(world);
    let mut corners=[origin;8];
    let mut arcs=[0.;8];
    let mut count=0;
    for i in 0..4 {
        let next=(i+1)%4;
        let straight=(points[next]-points[i]).norm()-radius*f64::from(u8::from(selected[i])+u8::from(selected[next]));
        if !straight.is_finite()||straight<=tol{return None;}
        let corner=points[i];
        if selected[i] {
            let previous=(i+3)%4;
            let start=corner+(points[previous]-corner).normalized()?*radius;
            let end=corner+(points[next]-corner).normalized()?*radius;
            if start==corner||end==corner||start==points[previous]||end==points[next]{return None;}
            corners[count]=start;
            arcs[count]=FRAC_PI_2;
            count+=1;
            corners[count]=end;
        }else{corners[count]=corner;}
        count+=1;
    }
    let length=size[axis];
    let volume=width*depth*height-length*radius*radius*(1.-FRAC_PI_2/2.)*edges.len() as f64;
    if !volume.is_finite()||volume<=0.{return None;}
    let direction=axes[axis];
    let center=world([size[u]*0.5,size[v]*0.5]);
    CircularPrism::from_profile(corners,arcs,count,center,direction,CircleSupports::Shared{radius,center:None},length,volume)
}
