//! 🧿️ Samples exact inner, outer, annular and meridional Revolve faces.

use super::{AnnularRevolution,Vertex};

fn disk(shape:AnnularRevolution,face:usize)->Option<(usize,bool)>{
    if shape.has_two_steps(){return match face{0=>Some((0,true)),6=>Some((5,false)),_=>None};}
    match face{
        0 if shape.has_floor()||shape.has_step()=>Some((1,true)),
        2 if shape.has_void_shell()=>Some((2,false)),
        4 if shape.has_floor()=>Some((0,false)),
        4 if shape.has_step()=>Some((3,false)),
        5 if shape.has_void_shell()=>Some((3,true)),
        _=>None,
    }
}

pub(super) fn slots(shape:AnnularRevolution,face:usize,segments:usize)->usize{
    if disk(shape,face).is_some(){segments}
    else if !shape.full()&&face<2{2}
    else{segments*2}
}

pub(super) fn triangle(shape:AnnularRevolution,face:usize,slot:usize,segments:usize)->[Vertex;3]{
    if let Some((corner,reverse))=disk(shape,face){
        let normal=if reverse{-shape.frame.z}else{shape.frame.z};
        let angle=|column|shape.sweep*((column%segments) as f64/segments as f64);
        let center=Vertex{point:shape.center(corner),normal};
        let a=Vertex{point:shape.point(corner,angle(slot)),normal};
        let b=Vertex{point:shape.point(corner,angle(slot+1)),normal};
        return if reverse{[center,b,a]}else{[center,a,b]};
    }
    if !shape.full()&&face<2{
        let angle=if face==0{0.}else{shape.sweep};
        let indices=if slot==0{[0,1,2]}else{[0,2,3]};
        let normal=shape.cut_plane(face).z;
        return indices.map(|corner|Vertex{point:shape.point(corner,angle),normal});
    }
    let side=if shape.full(){face}else{face-2};
    let column=slot/2;
    let angle=|column|{
        if shape.full()&&column==segments{0.}else{shape.sweep*(column as f64/segments as f64)}
    };
    let sample=|corner,column|{
        let angle=angle(column);
        let normal=if shape.has_two_steps(){
            match face{2=>-shape.frame.z,4=>shape.frame.z,_=>shape.radial(angle)}
        }else{match side{0=>-shape.frame.z,1=>shape.radial(angle),2=>shape.frame.z,_ if shape.has_step()=>shape.radial(angle),_=>-shape.radial(angle)}};
        Vertex{point:shape.point(corner,angle),normal}
    };
    let [first,next]=if shape.has_two_steps(){match face{1=>[0,1],2=>[2,1],3=>[2,3],4=>[3,4],_=>[4,5]}}
        else{[side,if side==2&&shape.has_step(){0}else{(side+1)%4}]};
    let a=sample(first,column);
    let c=sample(next,column+1);
    if slot%2==0{[a,sample(first,column+1),c]}else{[a,c,sample(next,column)]}
}
