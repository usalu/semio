//! 🛬️ Fragment hydration moves bounded records and admits one authored leaf or text chunk per turn.
use super::*;
use semio_framework_value::{DslValue,FromValue,RetireOwned,ValueError,ValueRefusalKind,NativeDecodeControl,list::PagedList};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Invalid drawing clipboard record / Ungültiger Zeichnungsdatensatz in der Zwischenablage")}
#[derive(RetireOwned)]
enum Kind{Layer,Retired,Text(&'static str),Segments,Points,Stops,Dash,References,Children,Samples(String)}
#[derive(RetireOwned)]
struct Task{path:[usize;MAX_DEPTH],depth:usize,kind:Kind,source:DslValue,at:usize,offset:usize}
#[derive(RetireOwned)]
pub(super) struct ClipboardHydration{pub(super) packet:DrawingClipboard,tasks:PagedList<Task,MAX_TASKS>,at:usize,source:DslValue,rejected:Option<DslValue>,rejected_task:Option<Task>,pending_node:Option<DrawingLayerNode>,pending_asset:Option<(String,DrawingImageAsset)>,retired_key:Option<String>,asset_append:usize,rejected_error:Option<ValueError>,initialized:bool,nodes:usize}
fn take(entries:&mut[(String,DslValue)],key:&str)->Option<DslValue>{entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|std::mem::replace(value,DslValue::Null))}
fn bounded(value:&DslValue,budget:&mut usize)->bool{
    if *budget==0{return false;}*budget-=1;
    match value{DslValue::String(value)=>value.len()<=64,DslValue::Array(values)=>values.iter().all(|value|bounded(value,budget)),DslValue::Object(values)=>values.iter().all(|(key,value)|key.len()<=64&&bounded(value,budget)),DslValue::Number(semio_framework_value::Number::Float(value))=>value.is_finite(),DslValue::Bytes(_)=>false,_=>true}
}
fn node_mut<'a>(packet:&'a mut DrawingClipboard,path:&[usize])->Result<&'a mut DrawingLayerNode,ValueError>{
    let mut node=packet.roots.get_mut(*path.first().ok_or_else(invalid)?).ok_or_else(invalid)?;
    for index in &path[1..]{let DrawingLayerNode::Group(group)=node else{return Err(invalid());};node=group.children.get_mut(*index).ok_or_else(invalid)?;}
    Ok(node)
}
fn append<T:FromValue>(list:&mut PagedList<T,{usize::MAX}>,source:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
    while !list.has_reserved_slot(){let bytes=list.next_allocation_bytes()?;control.charge(bytes)?;list.reserve_one(bytes).map_err(|error|ValueError::from(error.refusal()))?;}
    let value=T::from_value_controlled(source,control)?;list.push_reserved(value).map_err(|_|invalid())
}
fn reserve<T>(values:&mut Vec<T>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if values.len()==values.capacity(){let capacity=(values.capacity().max(1)*2).min(MAX_NODES);if capacity<=values.len(){return Err(invalid());}control.charge(capacity*std::mem::size_of::<T>())?;values.try_reserve_exact(capacity-values.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"Drawing clipboard vector allocation failed"))?;}Ok(())}
impl ClipboardHydration{
    pub(super) fn new(source:DslValue)->Self{Self{packet:DrawingClipboard{schema:String::new(),roots:Vec::new(),selected:Vec::new(),assets:Default::default()},tasks:PagedList::new(),at:0,source,rejected:None,rejected_task:None,pending_node:None,pending_asset:None,retired_key:None,asset_append:0,rejected_error:None,initialized:false,nodes:0}}
    fn queue(&mut self,path:&[usize],kind:Kind,source:DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        let mut address=[0;MAX_DEPTH];let length=path.len().min(MAX_DEPTH);address[..length].copy_from_slice(&path[..length]);let original=Task{path:address,depth:path.len(),kind,source,at:0,offset:0};
        if path.len()>MAX_DEPTH||self.tasks.len()==MAX_TASKS{self.rejected_task=Some(original);return Err(invalid());}
        while !self.tasks.has_reserved_slot(){let bytes=match self.tasks.next_allocation_bytes(){Ok(bytes)=>bytes,Err(error)=>{self.rejected_task=Some(original);return Err(error);}};if let Err(error)=control.charge(bytes){self.rejected_task=Some(original);return Err(error);}if let Err(error)=self.tasks.reserve_one(bytes){self.rejected_task=Some(original);return Err(error.refusal().into());}}
        self.tasks.push_reserved(original).map_err(|task|{self.rejected_task=Some(task);invalid()})
    }
    fn strip(&mut self,path:&[usize],entries:&mut[(String,DslValue)],key:&'static str,kind:Kind,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name==key){let source=std::mem::replace(value,match kind{Kind::Text(_)=>DslValue::String(String::new()),_=>DslValue::Array(Vec::new())});self.queue(path,kind,source,control)?;}Ok(())}
    fn layer(&mut self,path:&[usize],mut source:DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        if path.len()>MAX_DEPTH||self.nodes==MAX_NODES{self.rejected=Some(source);return Err(invalid());}self.nodes+=1;
        let capacity=self.tasks.len()+16;if capacity>MAX_TASKS{self.rejected=Some(source);return Err(invalid());}
        while self.tasks.capacity()<capacity{let bytes=match self.tasks.next_capacity_allocation_bytes(capacity){Ok(Some(bytes))=>bytes,Ok(None)=>break,Err(error)=>{self.rejected=Some(source);return Err(error.into());}};if let Err(error)=control.charge(bytes){self.rejected=Some(source);return Err(error);}if let Err(error)=self.tasks.reserve_capacity_one(capacity,bytes){self.rejected=Some(source);return Err(error.refusal().into());}}
        let DslValue::Object(entries)=&mut source else{self.queue(path,Kind::Layer,source,control)?;return Err(invalid());};
        if entries.len()>20||entries.iter().any(|(key,_)|key.len()>64){self.queue(path,Kind::Layer,source,control)?;return Err(invalid());}
        let kind=entries.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str());
        if !kind.is_some_and(|kind|matches!(kind,"shape"|"path"|"text"|"image"|"group"|"boolean"|"trace")){self.queue(path,Kind::Layer,source,control)?;return Err(invalid());}let kind=match kind.unwrap(){"shape"=>"shape","path"=>"path","text"=>"text","image"=>"image","group"=>"group","boolean"=>"boolean","trace"=>"trace",_=>unreachable!()};
        if entries.iter().any(|(key,_)|!matches!(key.as_str(),"kind"|"id"|"name"|"visible"|"locked"|"opacity"|"blendMode"|"transform"|"attributes")&&!match kind{"shape"=>matches!(key.as_str(),"shapeKind"|"rect"|"ellipse"|"circle"|"line"|"polygon"),"path"=>key=="segments","text"=>matches!(key.as_str(),"x"|"y"|"content"|"size"),"image"=>matches!(key.as_str(),"imageKey"|"width"|"height"),"group"=>matches!(key.as_str(),"children"|"isolation"),"boolean"=>matches!(key.as_str(),"operation"|"children"),"trace"=>matches!(key.as_str(),"sourceKey"|"params"),_=>false}){self.queue(path,Kind::Layer,source,control)?;return Err(invalid());}
        for key in ["id","name","blendMode","shapeKind","content","imageKey","operation","sourceKey"]{self.strip(path,entries,key,Kind::Text(key),control)?;}
        self.strip(path,entries,"segments",Kind::Segments,control)?;
        if kind=="group"{self.strip(path,entries,"children",Kind::Children,control)?;}else if kind=="boolean"{self.strip(path,entries,"children",Kind::References,control)?;}
        if let Some((_,DslValue::Object(polygon)))=entries.iter_mut().find(|(key,_)|key=="polygon"){self.strip(path,polygon,"points",Kind::Points,control)?;}
        if let Some((_,DslValue::Object(attributes)))=entries.iter_mut().find(|(key,_)|key=="attributes"){
            if let Some((_,DslValue::Object(fill)))=attributes.iter_mut().find(|(key,_)|key=="fill"){self.strip(path,fill,"stops",Kind::Stops,control)?;}
            if let Some((_,DslValue::Object(stroke)))=attributes.iter_mut().find(|(key,_)|key=="stroke"){self.strip(path,stroke,"dash",Kind::Dash,control)?;}
        }
        if !bounded(&source,&mut 128){self.queue(path,Kind::Layer,source,control)?;return Err(invalid());}
        let node=DrawingLayerNode::from_value_controlled(&source,control);match node{Ok(node)=>self.pending_node=Some(node),Err(error)=>self.rejected_error=Some(error)}self.queue(path,Kind::Retired,source,control)?;if self.rejected_error.is_some(){return Err(invalid());}
        if path.len()==1{if path[0]!=self.packet.roots.len(){return Err(invalid());}reserve(&mut self.packet.roots,control)?;self.packet.roots.push(self.pending_node.take().ok_or_else(invalid)?);}else{
            let parent=node_mut(&mut self.packet,&path[..path.len()-1])?;let DrawingLayerNode::Group(parent)=parent else{return Err(invalid());};
            if *path.last().unwrap()!=parent.children.len(){return Err(invalid());}while !parent.children.has_reserved_slot(){let bytes=parent.children.next_allocation_bytes()?;control.charge(bytes)?;parent.children.reserve_one(bytes).map_err(|error|ValueError::from(error.refusal()))?;}parent.children.push_reserved(self.pending_node.take().ok_or_else(invalid)?).map_err(|node|{self.pending_node=Some(node);invalid()})?;
        }
        Ok(())
    }
    pub(super) fn step(&mut self,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
        control.checkpoint()?;
        if !self.initialized{
            let DslValue::Object(entries)=&mut self.source else{return Err(invalid());};
            if entries.len()!=4||entries.iter().any(|(key,_)|!matches!(key.as_str(),"schema"|"roots"|"selected"|"assets")){return Err(invalid());}
            if entries.iter().find(|(key,_)|key=="schema").and_then(|(_,value)|value.as_str())!=Some(SCHEMA){return Err(invalid());}
            let Some((_,DslValue::String(schema)))=entries.iter_mut().find(|(key,_)|key=="schema")else{return Err(invalid());};self.packet.schema=std::mem::take(schema);
            let root_count=entries.iter().find(|(key,_)|key=="roots").and_then(|(_,value)|value.as_array()).ok_or_else(invalid)?.len();let selected_count=entries.iter().find(|(key,_)|key=="selected").and_then(|(_,value)|value.as_array()).ok_or_else(invalid)?.len();if root_count==0||root_count>MAX_NODES||selected_count==0||selected_count>MAX_NODES{return Err(invalid());}
            self.packet.roots=control.allocate_vec(root_count)?;self.packet.selected=control.allocate_vec(selected_count)?;
            let roots=take(entries,"roots").ok_or_else(invalid)?;self.queue(&[],Kind::Children,roots,control)?;
            let DslValue::Object(entries)=&mut self.source else{return Err(invalid());};let selected=take(entries,"selected").ok_or_else(invalid)?;self.queue(&[],Kind::Text("selected"),selected,control)?;
            let DslValue::Object(entries)=&mut self.source else{return Err(invalid());};let assets=take(entries,"assets").ok_or_else(invalid)?;self.queue(&[],Kind::Samples(String::new()),assets,control)?;
            self.initialized=true;
            return Ok(false);
        }
        if self.pending_asset.is_some(){self.packet.assets.append_candidate_step(&mut self.pending_asset,&mut self.asset_append,control)?;return Ok(false);}
        if self.at==self.tasks.len(){return Ok(true);}
        let task=&mut self.tasks[self.at];
        match &task.kind{
            Kind::Children=>{
                let DslValue::Array(values)=&mut task.source else{return Err(invalid());};if values.len()>1024{return Err(invalid());}
                if task.at==values.len(){self.at+=1;return Ok(false);}let index=task.at;let source=std::mem::replace(&mut values[index],DslValue::Null);task.at+=1;
                if task.depth==MAX_DEPTH{self.rejected=Some(source);return Err(invalid());}let mut path=task.path;path[task.depth]=index;let depth=task.depth+1;self.layer(&path[..depth],source,control)?;
            },
            Kind::Text(field) if *field=="selected"=>{
                let DslValue::Array(values)=&mut task.source else{return Err(invalid());};if values.len()>1024{return Err(invalid());}
                if task.at==values.len(){self.at+=1;return Ok(false);}let DslValue::String(value)=&mut values[task.at] else{return Err(invalid());};if value.is_empty()||value.len()>MAX_ID_BYTES{return Err(invalid());}reserve(&mut self.packet.selected,control)?;self.packet.selected.push(std::mem::take(value));task.at+=1;
            },
            Kind::Samples(id) if id.is_empty()=>{
                let DslValue::Object(entries)=&mut task.source else{return Err(invalid());};if entries.len()>1024{return Err(invalid());}
                if task.at==entries.len(){self.at+=1;return Ok(false);}let (id,value)=&mut entries[task.at];if id.is_empty()||id.len()>4096{return Err(invalid());}
                let DslValue::Object(fields)=value else{return Err(invalid());};if fields.len()!=3||fields.iter().any(|(key,_)|!matches!(key.as_str(),"width"|"height"|"samples"))||!fields.iter().any(|(key,_)|key=="samples"){return Err(invalid());}
                let sample_id=control.copy_text(id)?;let samples=take(fields,"samples").ok_or_else(invalid)?;if let Some((_,value))=fields.iter_mut().find(|(key,_)|key=="samples"){*value=DslValue::Array(Vec::new());}
                let asset=if bounded(value,&mut 16){DrawingImageAsset::from_value_controlled(value,control)}else{Err(invalid())};
                let id=std::mem::take(id);task.at+=1;match asset{Ok(asset)=>{self.pending_asset=Some((id,asset));self.queue(&[],Kind::Samples(sample_id),samples,control)?;},Err(error)=>{self.rejected_error=Some(error);self.retired_key=Some(id);self.queue(&[],Kind::Samples(sample_id),samples,control)?;return Err(invalid());}}
            },
            Kind::Text(field)=>{
                let DslValue::String(source)=&task.source else{return Err(invalid());};if *field!="content"&&*field!="name"&&source.len()>MAX_ID_BYTES{return Err(invalid());}let mut end=(task.at+1024).min(source.len());while !source.is_char_boundary(end){end-=1;}
                let node=node_mut(&mut self.packet,&task.path[..task.depth])?;
                let target=match *field{"id"=>&mut layer_base_mut(node).id,"name"=>&mut layer_base_mut(node).name,"blendMode"=>&mut layer_base_mut(node).blend_mode,"shapeKind"=>if let DrawingLayerNode::Shape(value)=node{&mut value.shape_kind}else{return Err(invalid());},"content"=>if let DrawingLayerNode::Text(value)=node{&mut value.content}else{return Err(invalid());},"imageKey"=>if let DrawingLayerNode::Image(value)=node{&mut value.image_key}else{return Err(invalid());},"operation"=>if let DrawingLayerNode::Boolean(value)=node{&mut value.operation}else{return Err(invalid());},"sourceKey"=>if let DrawingLayerNode::Trace(value)=node{&mut value.source_key}else{return Err(invalid());},_=>return Err(invalid())};
                target.try_push_str_controlled(&source[task.at..end],control)?;task.at=end;if end==source.len(){self.at+=1;}
            },
            Kind::Retired=>self.at+=1,
            Kind::Layer=>return Err(invalid()),
            Kind::References=>{
                let DslValue::Array(values)=&task.source else{return Err(invalid());};if values.len()>MAX_NODES{return Err(invalid());}if task.at==values.len(){self.at+=1;return Ok(false);}let source=values[task.at].as_str().ok_or_else(invalid)?;if source.is_empty()||source.len()>MAX_ID_BYTES{return Err(invalid());}
                let node=node_mut(&mut self.packet,&task.path[..task.depth])?;let DrawingLayerNode::Boolean(boolean)=node else{return Err(invalid());};
                if task.offset==0{while !boolean.children.has_reserved_slot(){let bytes=boolean.children.next_allocation_bytes()?;control.charge(bytes)?;boolean.children.reserve_one(bytes).map_err(|error|ValueError::from(error.refusal()))?;}boolean.children.push_reserved(Default::default()).map_err(|_|invalid())?;}
                let mut end=(task.offset+1024).min(source.len());while !source.is_char_boundary(end){end-=1;}boolean.children.get_mut(task.at).ok_or_else(invalid)?.try_push_str_controlled(&source[task.offset..end],control)?;task.offset=end;if end==source.len(){task.offset=0;task.at+=1;}
            },
            _=>{
                let DslValue::Array(values)=&mut task.source else{return Err(invalid());};if values.len()>262144||matches!(&task.kind,Kind::References)&&values.len()>MAX_NODES{return Err(invalid());}
                if task.at==values.len(){self.at+=1;return Ok(false);}let value=&values[task.at];if matches!(&task.kind,Kind::References){if value.as_str().is_none_or(|id|id.is_empty()||id.len()>MAX_ID_BYTES){return Err(invalid());}}else if !bounded(value,&mut 32){return Err(invalid());}task.at+=1;
                if let Kind::Samples(id)=&task.kind{append(&mut self.packet.assets.get_mut(id).ok_or_else(invalid)?.samples,value,control)?;return Ok(false);}
                let node=node_mut(&mut self.packet,&task.path[..task.depth])?;
                match &task.kind{
                    Kind::Segments=>{let DrawingLayerNode::Path(path)=node else{return Err(invalid());};append(&mut path.segments,value,control)?;},
                    Kind::Points=>{let DrawingLayerNode::Shape(shape)=node else{return Err(invalid());};append(&mut shape.polygon.as_mut().ok_or_else(invalid)?.points,value,control)?;},
                    Kind::Stops=>{let Some(crate::FillStyle::LinearGradient{stops,..}|crate::FillStyle::RadialGradient{stops,..})=layer_base_mut(node).attributes.fill.as_mut()else{return Err(invalid());};append(stops,value,control)?;},
                    Kind::Dash=>{append(layer_base_mut(node).attributes.stroke.as_mut().ok_or_else(invalid)?.dash.as_mut().ok_or_else(invalid)?,value,control)?;},
                    Kind::References=>{let DrawingLayerNode::Boolean(boolean)=node else{return Err(invalid());};append(&mut boolean.children,value,control)?;},
                    _=>return Err(invalid())
                }
            }
        }
        Ok(false)
    }
}
