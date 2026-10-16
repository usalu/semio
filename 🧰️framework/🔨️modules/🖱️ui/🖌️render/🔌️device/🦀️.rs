//! 🔌️ Finite render descriptors carry work data; only a backend issues actual resource owners.
use std::num::NonZeroU64;
pub use semio_framework_ui_render_shader_contract::{VertexFormat,VertexStepMode,CompareFunction,PrimitiveTopology};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ResourceFault { ForeignSession, InvalidDescriptor(&'static str), Backend(&'static str) }
#[derive(Debug)]
pub enum OwnedFailure<T> { Returned { fault:ResourceFault, resource:T }, Retained { fault:ResourceFault } }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ResourceKind { Buffer, Texture }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ResourceSpec { pub kind:ResourceKind,pub bytes:u64,pub width:u32,pub height:u32,pub layers:u32,pub mips:u32,pub samples:u32 }
#[derive(Clone,Copy,Debug)]
pub struct ResourceLimits { pub bytes:u64,pub dimension:u32 }
impl ResourceSpec {
    pub fn validate(self,limits:ResourceLimits)->Result<(),ResourceFault>{
        if self.bytes==0||self.bytes>limits.bytes||self.layers!=1||!matches!(self.samples,1|4)||self.width==0||self.height==0||self.mips==0{return Err(ResourceFault::InvalidDescriptor("finite resource dimensions"));}
        if self.kind==ResourceKind::Buffer{return if self.width==1&&self.height==1&&self.mips==1&&self.samples==1{Ok(())}else{Err(ResourceFault::InvalidDescriptor("buffer shape"))};}
        if self.width>limits.dimension||self.height>limits.dimension||self.mips>u32::BITS-self.width.max(self.height).leading_zeros(){return Err(ResourceFault::InvalidDescriptor("texture extent"));}
        let mut bytes=0u64;
        for level in 0..self.mips{bytes=bytes.checked_add(u64::from((self.width>>level).max(1)).checked_mul(u64::from((self.height>>level).max(1))).and_then(|value|value.checked_mul(4)).and_then(|value|value.checked_mul(u64::from(self.samples))).ok_or(ResourceFault::InvalidDescriptor("texture byte overflow"))?).ok_or(ResourceFault::InvalidDescriptor("texture byte overflow"))?;}
        if bytes==self.bytes{Ok(())}else{Err(ResourceFault::InvalidDescriptor("texture byte extent"))}
    }
}
pub trait RenderDevice {
    type Buffer;
    type Texture;
    fn buffer(&self,descriptor:&BufferDescriptor<'_>)->Result<Self::Buffer,ResourceFault>;
    fn texture(&self,descriptor:&TextureDescriptor<'_>)->Result<Self::Texture,ResourceFault>;
}
pub type BufferAddress=u64;
pub const COPY_BYTES_PER_ROW_ALIGNMENT:u32=256;

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum TextureFormat { #[default] Rgba8Unorm, Rgba8UnormSrgb, Bgra8Unorm, Bgra8UnormSrgb, R8Unorm, Depth24PlusStencil8, Depth32Float }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum TextureDimension { #[default] D2 }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum TextureViewDimension { #[default] D2 }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum TextureAspect { #[default] All }


#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum StencilOperation { #[default] Keep, Invert, Replace }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum Face { #[default] Back }


#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum FilterMode { #[default] Nearest, Linear }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum MipmapFilterMode { #[default] Nearest, Linear }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum AddressMode { #[default] ClampToEdge, MirrorRepeat, Repeat }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum SamplerBindingType { #[default] Filtering, Comparison }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum BufferBindingType { #[default] Uniform }



#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum IndexFormat { #[default] Uint32 }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum StoreOp { #[default] Store }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum MapMode { #[default] Read }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum PollType { #[default] Poll }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum PresentMode { #[default] AutoVsync }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum CompositeAlphaMode { #[default] PreMultiplied, Auto, Opaque, PostMultiplied, Inherit }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct BufferUsages(pub u32);
impl BufferUsages { pub const fn empty()->Self{Self(0)} pub const COPY_DST:Self=Self(1);pub const INDEX:Self=Self(2);pub const MAP_READ:Self=Self(4);pub const STORAGE:Self=Self(8);pub const UNIFORM:Self=Self(16);pub const VERTEX:Self=Self(32); }
impl std::ops::BitOr for BufferUsages { type Output=Self;fn bitor(self,right:Self)->Self{Self(self.0|right.0)} }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct TextureUsages(pub u32);
impl TextureUsages { pub const fn empty()->Self{Self(0)} pub const COPY_DST:Self=Self(1);pub const COPY_SRC:Self=Self(2);pub const RENDER_ATTACHMENT:Self=Self(4);pub const STORAGE_BINDING:Self=Self(8);pub const TEXTURE_BINDING:Self=Self(16); }
impl std::ops::BitOr for TextureUsages { type Output=Self;fn bitor(self,right:Self)->Self{Self(self.0|right.0)} }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ShaderStages(pub u32);
impl ShaderStages { pub const fn empty()->Self{Self(0)} pub const VERTEX:Self=Self(1);pub const FRAGMENT:Self=Self(2); }
impl std::ops::BitOr for ShaderStages { type Output=Self;fn bitor(self,right:Self)->Self{Self(self.0|right.0)} }

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ColorWrites(pub u32);
impl ColorWrites { pub const fn empty()->Self{Self(0)} pub const ALL:Self=Self(1); }
impl std::ops::BitOr for ColorWrites { type Output=Self;fn bitor(self,right:Self)->Self{Self(self.0|right.0)} }

impl TextureFormat {
    pub fn is_srgb(self)->bool{matches!(self,Self::Rgba8UnormSrgb|Self::Bgra8UnormSrgb)}
    pub fn add_srgb_suffix(self)->Self{match self{Self::Rgba8Unorm=>Self::Rgba8UnormSrgb,Self::Bgra8Unorm=>Self::Bgra8UnormSrgb,other=>other}}
    pub fn remove_srgb_suffix(self)->Self{match self{Self::Rgba8UnormSrgb=>Self::Rgba8Unorm,Self::Bgra8UnormSrgb=>Self::Bgra8Unorm,other=>other}}
}
#[derive(Clone,Copy,Debug,Default)]
pub struct Extent3d {pub width:u32,pub height:u32,pub depth_or_array_layers:u32}
#[derive(Clone,Copy,Debug,Default)]
pub struct Origin3d {pub x:u32,pub y:u32,pub z:u32}
impl Origin3d {pub const ZERO:Self=Self{x:0,y:0,z:0};}
#[derive(Clone,Copy,Debug,Default)]
pub struct Color {pub r:f64,pub g:f64,pub b:f64,pub a:f64}
impl Color {pub const TRANSPARENT:Self=Self{r:0.0,g:0.0,b:0.0,a:0.0};}
#[derive(Clone,Copy,Debug)]
pub enum LoadOp<T> {Load,Clear(T)}
#[derive(Clone,Copy,Debug)]
pub struct Operations<T> {pub load:LoadOp<T>,pub store:StoreOp}
#[derive(Clone,Copy,Debug,Default)]
pub struct BufferDescriptor<'a>{pub label:Option<&'a str>,pub size:u64,pub usage:BufferUsages,pub mapped_at_creation:bool}
#[derive(Clone,Copy,Debug)]
pub struct TextureDescriptor<'a>{pub label:Option<&'a str>,pub size:Extent3d,pub mip_level_count:u32,pub sample_count:u32,pub dimension:TextureDimension,pub format:TextureFormat,pub usage:TextureUsages,pub view_formats:&'a[TextureFormat]}
#[derive(Clone,Copy,Debug,Default)]
pub struct TextureViewDescriptor<'a>{pub label:Option<&'a str>,pub format:Option<TextureFormat>,pub dimension:Option<TextureViewDimension>,pub base_mip_level:u32,pub mip_level_count:Option<u32>}
#[derive(Clone,Copy,Debug)]
pub struct SamplerDescriptor<'a>{pub label:Option<&'a str>,pub address_mode_u:AddressMode,pub address_mode_v:AddressMode,pub address_mode_w:AddressMode,pub mag_filter:FilterMode,pub min_filter:FilterMode,pub mipmap_filter:MipmapFilterMode,pub lod_min_clamp:f32,pub lod_max_clamp:f32,pub compare:Option<CompareFunction>,pub anisotropy_clamp:u16}
impl Default for SamplerDescriptor<'_>{fn default()->Self{Self{label:None,address_mode_u:AddressMode::ClampToEdge,address_mode_v:AddressMode::ClampToEdge,address_mode_w:AddressMode::ClampToEdge,mag_filter:FilterMode::Nearest,min_filter:FilterMode::Nearest,mipmap_filter:MipmapFilterMode::Nearest,lod_min_clamp:0.0,lod_max_clamp:32.0,compare:None,anisotropy_clamp:1}}}
#[derive(Clone,Copy,Debug)]
pub enum TextureSampleType {Float{filterable:bool},Depth}
#[derive(Clone,Copy,Debug)]
pub enum BindingType {Buffer{ty:BufferBindingType,has_dynamic_offset:bool,min_binding_size:Option<NonZeroU64>},Sampler(SamplerBindingType),Texture{sample_type:TextureSampleType,view_dimension:TextureViewDimension,multisampled:bool}}
#[derive(Clone,Copy,Debug)]
pub struct BindGroupLayoutEntry{pub binding:u32,pub visibility:ShaderStages,pub ty:BindingType,pub count:Option<std::num::NonZeroU32>}
#[derive(Clone,Copy,Debug)]
pub struct BindGroupLayoutDescriptor<'a>{pub label:Option<&'a str>,pub entries:&'a[BindGroupLayoutEntry]}
#[derive(Clone,Copy,Debug,Default)]
pub struct CommandEncoderDescriptor<'a>{pub label:Option<&'a str>}
#[derive(Clone,Copy,Debug,Default)]
pub struct PipelineCompilationOptions;
#[derive(Clone,Copy,Debug)]
pub struct VertexAttribute{pub format:VertexFormat,pub offset:u64,pub shader_location:u32}
#[derive(Clone,Copy,Debug)]
pub struct VertexBufferLayout<'a>{pub array_stride:u64,pub step_mode:VertexStepMode,pub attributes:&'a[VertexAttribute]}
#[derive(Clone,Copy,Debug)]
pub struct PrimitiveState{pub topology:PrimitiveTopology,pub cull_mode:Option<Face>}
#[derive(Clone,Copy,Debug,Default)]
pub struct DepthBiasState{pub constant:i32,pub slope_scale:f32,pub clamp:f32}
#[derive(Clone,Copy,Debug)]
pub struct StencilFaceState{pub compare:CompareFunction,pub fail_op:StencilOperation,pub depth_fail_op:StencilOperation,pub pass_op:StencilOperation}
#[derive(Clone,Copy,Debug)]
pub struct StencilState{pub front:StencilFaceState,pub back:StencilFaceState,pub read_mask:u32,pub write_mask:u32}
#[derive(Clone,Copy,Debug)]
pub struct DepthStencilState{pub format:TextureFormat,pub depth_write_enabled:Option<bool>,pub depth_compare:Option<CompareFunction>,pub stencil:StencilState,pub bias:DepthBiasState}
#[derive(Clone,Copy,Debug)]
pub struct MultisampleState{pub count:u32,pub mask:u64,pub alpha_to_coverage_enabled:bool}
impl Default for MultisampleState{fn default()->Self{Self{count:1,mask:u64::MAX,alpha_to_coverage_enabled:false}}}
#[derive(Clone,Copy,Debug)]
pub enum BlendState{Alpha,Replace}
impl BlendState{pub const ALPHA_BLENDING:Self=Self::Alpha;pub const REPLACE:Self=Self::Replace;}
#[derive(Clone,Copy,Debug)]
pub struct ColorTargetState{pub format:TextureFormat,pub blend:Option<BlendState>,pub write_mask:ColorWrites}
#[derive(Clone,Copy,Debug,Default)]
pub struct TexelCopyBufferLayout{pub offset:u64,pub bytes_per_row:Option<u32>,pub rows_per_image:Option<u32>}
#[derive(Clone,Debug)]
pub struct SurfaceConfiguration{pub usage:TextureUsages,pub format:TextureFormat,pub width:u32,pub height:u32,pub present_mode:PresentMode,pub alpha_mode:CompositeAlphaMode,pub view_formats:Vec<TextureFormat>,pub desired_maximum_frame_latency:u32}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

impl Default for StencilState{fn default()->Self{Self{front:Default::default(),back:Default::default(),read_mask:u32::MAX,write_mask:u32::MAX}}}
impl std::fmt::Display for ResourceFault{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{self:?}")}}
impl std::error::Error for ResourceFault{}

impl Default for PrimitiveState{fn default()->Self{Self{topology:PrimitiveTopology::TriangleList,cull_mode:None}}}
impl Default for StencilFaceState{fn default()->Self{Self{compare:CompareFunction::Always,fail_op:StencilOperation::Keep,depth_fail_op:StencilOperation::Keep,pass_op:StencilOperation::Keep}}}

pub enum ShaderSource<'a>{Wgsl(std::borrow::Cow<'a,str>)}
pub struct ShaderModuleDescriptor<'a>{pub label:Option<&'a str>,pub source:ShaderSource<'a>}
#[derive(Clone,Copy)]
pub struct BufferBinding<'a,B>{pub buffer:&'a B,pub offset:u64,pub size:Option<NonZeroU64>}
#[derive(Clone,Copy)]
pub enum BindingResource<'a,B,V,S>{Buffer(BufferBinding<'a,B>),TextureView(&'a V),Sampler(&'a S)}
pub struct BindGroupEntry<'a,B,V,S>{pub binding:u32,pub resource:BindingResource<'a,B,V,S>}
pub struct BindGroupDescriptor<'a,L,B,V,S>{pub label:Option<&'a str>,pub layout:&'a L,pub entries:&'a[BindGroupEntry<'a,B,V,S>]}
pub struct PipelineLayoutDescriptor<'a,L>{pub label:Option<&'a str>,pub bind_group_layouts:&'a[Option<&'a L>],pub immediate_size:u32}
pub struct VertexState<'a,S>{pub module:&'a S,pub entry_point:Option<&'a str>,pub compilation_options:PipelineCompilationOptions,pub buffers:&'a[VertexBufferLayout<'a>]}
pub struct FragmentState<'a,S>{pub module:&'a S,pub entry_point:Option<&'a str>,pub compilation_options:PipelineCompilationOptions,pub targets:&'a[Option<ColorTargetState>]}
pub struct RenderPipelineDescriptor<'a,L,S>{pub label:Option<&'a str>,pub layout:Option<&'a L>,pub vertex:VertexState<'a,S>,pub primitive:PrimitiveState,pub depth_stencil:Option<DepthStencilState>,pub multisample:MultisampleState,pub fragment:Option<FragmentState<'a,S>>,pub multiview_mask:Option<std::num::NonZeroU32>,pub cache:Option<()>}
pub struct RenderPassColorAttachment<'a,V>{pub view:&'a V,pub depth_slice:Option<u32>,pub resolve_target:Option<&'a V>,pub ops:Operations<Color>}
pub struct RenderPassDepthStencilAttachment<'a,V>{pub view:&'a V,pub depth_ops:Option<Operations<f32>>,pub stencil_ops:Option<Operations<u32>>}
pub struct RenderPassDescriptor<'a,V>{pub label:Option<&'a str>,pub color_attachments:&'a[Option<RenderPassColorAttachment<'a,V>>],pub depth_stencil_attachment:Option<RenderPassDepthStencilAttachment<'a,V>>,pub timestamp_writes:Option<()>,pub occlusion_query_set:Option<()>,pub multiview_mask:Option<std::num::NonZeroU32>}
#[derive(Clone,Copy)]
pub struct TexelCopyTextureInfo<'a,T>{pub texture:&'a T,pub mip_level:u32,pub origin:Origin3d,pub aspect:TextureAspect}
#[derive(Clone,Copy)]
pub struct TexelCopyBufferInfo<'a,B>{pub buffer:&'a B,pub layout:TexelCopyBufferLayout}
