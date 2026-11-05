//! 🎯️ Intrinsic selection intent geometry; the retained config owner computes coverage.
use super::*;

pub struct RasterSelectionIntent {
    pub layer_id:String,
    pub target:&'static str,
    pub image_key:Option<String>,
    pub width:u32,
    pub height:u32,
    pub points:Vec<[f64;2]>,
    world:[f64;6],
}

fn image_key(layers:&[LayerNode],id:&str,target:PaintTarget)->Option<String>{
    for layer in layers {match layer {
        LayerNode::Pixel {id:owner,image_key,mask,..} if owner==id=>return if target==PaintTarget::Pixels{image_key.clone()}else{mask.as_ref().and_then(|mask|mask.descriptor.image_key.clone())},
        LayerNode::Group {id:owner,mask,..} if owner==id=>return mask.as_ref().and_then(|mask|mask.descriptor.image_key.clone()),
        LayerNode::Group {children,..}=>if let Some(key)=image_key(children,id,target){return Some(key);},
        _=>{}
    }}None
}

impl RasterHost {
    pub fn pixel_selection_intent(&self,points:&[[f64;2]])->Option<RasterSelectionIntent>{
        if points.len()>4096{return None;}
        let id=self.selected_ids.first()?;let (world,(width,height))=gesture::target(self,&self.document.layers,id,Affine::IDENTITY,true)?;
        let [a,b,c,d,e,f]=world.as_coeffs();let determinant=a*d-b*c;if !determinant.is_finite()||determinant.abs()<1e-12{return None;}
        let inverse=Affine::new([d/determinant,-b/determinant,-c/determinant,a/determinant,(c*f-d*e)/determinant,(b*e-a*f)/determinant]);
        let points=points.iter().map(|point|{let local=inverse*self.screen_to_world(point[0],point[1]);(local.x.is_finite()&&local.y.is_finite()).then_some([local.x,local.y])}).collect::<Option<Vec<_>>>()?;
        Some(RasterSelectionIntent {layer_id:id.clone(),target:if self.paint_target==PaintTarget::Pixels{"pixels"}else{"mask"},image_key:image_key(&self.document.layers,id,self.paint_target),width,height,points,world:world.as_coeffs()})
    }
}

impl RasterSelectionIntent {
    pub fn matches(&self,host:&RasterHost)->bool{
        host.selected_ids.first()==Some(&self.layer_id)&&(if host.paint_target==PaintTarget::Pixels{"pixels"}else{"mask"})==self.target&&image_key(&host.document.layers,&self.layer_id,host.paint_target)==self.image_key&&gesture::target(host,&host.document.layers,&self.layer_id,Affine::IDENTITY,true).is_some_and(|(world,extent)|world.as_coeffs()==self.world&&extent==(self.width,self.height))
    }
}
