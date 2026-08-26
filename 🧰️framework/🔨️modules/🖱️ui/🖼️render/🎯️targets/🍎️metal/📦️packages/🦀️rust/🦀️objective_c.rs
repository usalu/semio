//! @emoji 🧭 Private Objective-C ownership and message ABI used by the Metal backend.

#![allow(dead_code, non_snake_case, unused_unsafe)]

use std::ffi::{c_char, c_void, CStr};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::OnceLock;

//#region 🧱Runtime

#[link(name = "objc")]
unsafe extern "C-unwind" {
    fn objc_lookUpClass(name: *const c_char) -> *mut c_void;
    fn sel_registerName(name: *const c_char) -> *mut c_void;
    fn objc_msgSend();
    fn objc_retain(object: *mut c_void) -> *mut c_void;
    fn objc_retainAutoreleasedReturnValue(object: *mut c_void) -> *mut c_void;
    fn objc_release(object: *mut c_void);
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(token: *mut c_void);
}

unsafe fn class(name: &CStr) -> *mut c_void {
    let value = unsafe { objc_lookUpClass(name.as_ptr()) };
    assert!(!value.is_null(), "Objective-C class is unavailable: {}", name.to_string_lossy());
    value
}

macro_rules! cached_selector {
    ($selector:literal) => {{
        static SELECTOR: OnceLock<usize> = OnceLock::new();
        *SELECTOR.get_or_init(|| unsafe { sel_registerName(concat!($selector, "\0").as_ptr().cast()) as usize }) as *mut c_void
    }};
}

macro_rules! message {
    ($receiver:expr, $selector:literal => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector)) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty, $e:expr => $E:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D, $E) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d, $e) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty, $e:expr => $E:ty, $f:expr => $F:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D, $E, $F) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d, $e, $f) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty, $e:expr => $E:ty, $f:expr => $F:ty, $g:expr => $G:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D, $E, $F, $G) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d, $e, $f, $g) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty, $e:expr => $E:ty, $f:expr => $F:ty, $g:expr => $G:ty, $h:expr => $H:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D, $E, $F, $G, $H) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d, $e, $f, $g, $h) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty, $e:expr => $E:ty, $f:expr => $F:ty, $g:expr => $G:ty, $h:expr => $H:ty, $i:expr => $I:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D, $E, $F, $G, $H, $I) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d, $e, $f, $g, $h, $i) }
    }};
    ($receiver:expr, $selector:literal, $a:expr => $A:ty, $b:expr => $B:ty, $d:expr => $D:ty, $e:expr => $E:ty, $f:expr => $F:ty, $g:expr => $G:ty, $h:expr => $H:ty, $i:expr => $I:ty, $j:expr => $J:ty => $return:ty) => {{
        let function: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, $A, $B, $D, $E, $F, $G, $H, $I, $J) -> $return = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
        unsafe { function($receiver, cached_selector!($selector), $a, $b, $d, $e, $f, $g, $h, $i, $j) }
    }};
}

#[repr(transparent)]
pub struct Owned<T> {
    pointer: NonNull<T>,
    marker: PhantomData<T>,
}

impl<T> Owned<T> {
    unsafe fn from_new(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(|pointer| Self { pointer, marker: PhantomData })
    }

    unsafe fn from_autoreleased(pointer: *mut T) -> Option<Self> {
        let pointer = unsafe { objc_retainAutoreleasedReturnValue(pointer.cast()).cast::<T>() };
        unsafe { Self::from_new(pointer) }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.pointer.as_ptr().cast()
    }

    pub fn as_ref(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }
}

impl<T> Clone for Owned<T> {
    fn clone(&self) -> Self {
        unsafe { objc_retain(self.as_ptr()) };
        Self { pointer: self.pointer, marker: PhantomData }
    }
}

impl<T> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> AsRef<T> for Owned<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }
}

impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe { objc_release(self.as_ptr()) };
    }
}

impl<T> fmt::Debug for Owned<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("OwnedObjectiveCObject").field(&self.as_ptr()).finish()
    }
}

fn pointer<T>(value: &T) -> *mut c_void {
    (value as *const T).cast_mut().cast()
}

fn optional<T>(value: Option<&T>) -> *mut c_void {
    value.map(pointer).unwrap_or(std::ptr::null_mut())
}

fn required_new<T>(pointer: *mut c_void, selector: &'static str) -> Owned<T> {
    unsafe { Owned::from_new(pointer.cast()) }.unwrap_or_else(|| panic!("Objective-C selector returned nil: {selector}"))
}

fn required_autoreleased<T>(pointer: *mut c_void, selector: &'static str) -> Owned<T> {
    unsafe { Owned::from_autoreleased(pointer.cast()) }.unwrap_or_else(|| panic!("Objective-C selector returned nil: {selector}"))
}

pub struct AutoreleasePool(*mut c_void);

impl AutoreleasePool {
    pub fn new() -> Self {
        Self(unsafe { objc_autoreleasePoolPush() })
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.0) };
    }
}

pub fn autorelease_pool<R>(run: impl FnOnce() -> R) -> R {
    let pool = AutoreleasePool::new();
    let result = run();
    drop(pool);
    result
}

macro_rules! object_type {
    ($($name:ident),+ $(,)?) => {$(
        #[repr(C)]
        pub struct $name {
            opaque: [u8; 0],
        }
    )+};
}

object_type!(
    AnyObject,
    NSString,
    NSError,
    MTLDevice,
    MTLCommandQueue,
    MTLCommandBuffer,
    MTLRenderCommandEncoder,
    MTLBlitCommandEncoder,
    MTLBuffer,
    MTLTexture,
    MTLLibrary,
    MTLFunction,
    MTLRenderPipelineState,
    MTLDepthStencilState,
    MTLSamplerState,
    MTLTextureDescriptor,
    MTLRenderPassDescriptor,
    MTLRenderPassColorAttachmentDescriptor,
    MTLRenderPassColorAttachmentDescriptorArray,
    MTLRenderPassDepthAttachmentDescriptor,
    MTLRenderPassStencilAttachmentDescriptor,
    MTLRenderPipelineDescriptor,
    MTLRenderPipelineColorAttachmentDescriptor,
    MTLRenderPipelineColorAttachmentDescriptorArray,
    MTLDepthStencilDescriptor,
    MTLStencilDescriptor,
    MTLSamplerDescriptor,
    MTLVertexDescriptor,
    MTLVertexBufferLayoutDescriptorArray,
    MTLVertexBufferLayoutDescriptor,
    MTLVertexAttributeDescriptorArray,
    MTLVertexAttributeDescriptor,
    CAMetalLayer,
    CAMetalDrawable,
);

//#endregion 🧱Runtime

//#region 📝Foundation

impl NSString {
    pub fn from_str(value: &str) -> Owned<Self> {
        let allocated = message!(unsafe { class(c"NSString") }, "alloc" => *mut c_void);
        let initialized = message!(allocated, "initWithBytes:length:encoding:", value.as_ptr() => *const u8, value.len() => usize, 4usize => usize => *mut c_void);
        required_new(initialized, "-[NSString initWithBytes:length:encoding:]")
    }

    fn utf8(&self) -> &str {
        let bytes = message!(pointer(self), "UTF8String" => *const c_char);
        if bytes.is_null() {
            return "";
        }
        unsafe { CStr::from_ptr(bytes) }.to_str().unwrap_or("<invalid UTF-8 NSString>")
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.utf8())
    }
}

//#endregion 📝Foundation

//#region 🧰Descriptors

macro_rules! new_descriptor {
    ($type:ty, $class:literal) => {
        impl $type {
            pub fn new() -> Owned<Self> {
                let object = message!(unsafe { class(unsafe { CStr::from_bytes_with_nul_unchecked(concat!($class, "\0").as_bytes()) }) }, "new" => *mut c_void);
                required_new(object, concat!("+[", $class, " new]"))
            }
        }
    };
}

new_descriptor!(MTLTextureDescriptor, "MTLTextureDescriptor");
new_descriptor!(MTLRenderPassDescriptor, "MTLRenderPassDescriptor");
new_descriptor!(MTLRenderPipelineDescriptor, "MTLRenderPipelineDescriptor");
new_descriptor!(MTLDepthStencilDescriptor, "MTLDepthStencilDescriptor");
new_descriptor!(MTLStencilDescriptor, "MTLStencilDescriptor");
new_descriptor!(MTLSamplerDescriptor, "MTLSamplerDescriptor");
new_descriptor!(MTLVertexDescriptor, "MTLVertexDescriptor");

macro_rules! scalar_setter {
    ($type:ty, $method:ident, $selector:literal, $value:ty) => {
        impl $type {
            pub fn $method(&self, value: $value) {
                message!(pointer(self), $selector, value => $value => ());
            }
        }
    };
}

scalar_setter!(MTLTextureDescriptor, setPixelFormat, "setPixelFormat:", objc2_metal::MTLPixelFormat);
scalar_setter!(MTLTextureDescriptor, setWidth, "setWidth:", usize);
scalar_setter!(MTLTextureDescriptor, setHeight, "setHeight:", usize);
scalar_setter!(MTLTextureDescriptor, setMipmapLevelCount, "setMipmapLevelCount:", usize);
scalar_setter!(MTLTextureDescriptor, setUsage, "setUsage:", objc2_metal::MTLTextureUsage);
scalar_setter!(MTLTextureDescriptor, setResourceOptions, "setResourceOptions:", objc2_metal::MTLResourceOptions);

impl MTLRenderPassDescriptor {
    pub fn renderPassDescriptor() -> Owned<Self> {
        required_autoreleased(message!(unsafe { class(c"MTLRenderPassDescriptor") }, "renderPassDescriptor" => *mut c_void), "+[MTLRenderPassDescriptor renderPassDescriptor]")
    }
    pub fn colorAttachments(&self) -> Owned<MTLRenderPassColorAttachmentDescriptorArray> {
        required_autoreleased(message!(pointer(self), "colorAttachments" => *mut c_void), "-[MTLRenderPassDescriptor colorAttachments]")
    }
    pub fn colorAttachment(&self, index: usize) -> Owned<MTLRenderPassColorAttachmentDescriptor> {
        let attachments = message!(pointer(self), "colorAttachments" => *mut c_void);
        let attachment = message!(attachments, "objectAtIndexedSubscript:", index => usize => *mut c_void);
        required_autoreleased(attachment, "-[MTLRenderPassColorAttachmentDescriptorArray objectAtIndexedSubscript:]")
    }

    pub fn depthAttachment(&self) -> Owned<MTLRenderPassDepthAttachmentDescriptor> {
        required_autoreleased(message!(pointer(self), "depthAttachment" => *mut c_void), "-[MTLRenderPassDescriptor depthAttachment]")
    }

    pub fn stencilAttachment(&self) -> Owned<MTLRenderPassStencilAttachmentDescriptor> {
        required_autoreleased(message!(pointer(self), "stencilAttachment" => *mut c_void), "-[MTLRenderPassDescriptor stencilAttachment]")
    }
}

impl MTLRenderPassColorAttachmentDescriptorArray {
    pub unsafe fn objectAtIndexedSubscript(&self, index: usize) -> Owned<MTLRenderPassColorAttachmentDescriptor> {
        required_autoreleased(message!(pointer(self), "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLRenderPassColorAttachmentDescriptorArray objectAtIndexedSubscript:]")
    }
}

impl MTLRenderPassColorAttachmentDescriptor {
    pub fn setTexture(&self, texture: Option<&MTLTexture>) { message!(pointer(self), "setTexture:", optional(texture) => *mut c_void => ()); }
    pub fn setLevel(&self, level: usize) { message!(pointer(self), "setLevel:", level => usize => ()); }
    pub fn setLoadAction(&self, action: objc2_metal::MTLLoadAction) { message!(pointer(self), "setLoadAction:", action => objc2_metal::MTLLoadAction => ()); }
    pub fn setStoreAction(&self, action: objc2_metal::MTLStoreAction) { message!(pointer(self), "setStoreAction:", action => objc2_metal::MTLStoreAction => ()); }
    pub fn setClearColor(&self, color: objc2_metal::MTLClearColor) { message!(pointer(self), "setClearColor:", color => objc2_metal::MTLClearColor => ()); }
}

impl MTLRenderPassDepthAttachmentDescriptor {
    pub fn setTexture(&self, texture: Option<&MTLTexture>) { message!(pointer(self), "setTexture:", optional(texture) => *mut c_void => ()); }
    pub fn setLoadAction(&self, action: objc2_metal::MTLLoadAction) { message!(pointer(self), "setLoadAction:", action => objc2_metal::MTLLoadAction => ()); }
    pub fn setStoreAction(&self, action: objc2_metal::MTLStoreAction) { message!(pointer(self), "setStoreAction:", action => objc2_metal::MTLStoreAction => ()); }
    pub fn setClearDepth(&self, depth: f64) { message!(pointer(self), "setClearDepth:", depth => f64 => ()); }
}

impl MTLRenderPassStencilAttachmentDescriptor {
    pub fn setTexture(&self, texture: Option<&MTLTexture>) { message!(pointer(self), "setTexture:", optional(texture) => *mut c_void => ()); }
    pub fn setLoadAction(&self, action: objc2_metal::MTLLoadAction) { message!(pointer(self), "setLoadAction:", action => objc2_metal::MTLLoadAction => ()); }
    pub fn setStoreAction(&self, action: objc2_metal::MTLStoreAction) { message!(pointer(self), "setStoreAction:", action => objc2_metal::MTLStoreAction => ()); }
    pub fn setClearStencil(&self, stencil: u32) { message!(pointer(self), "setClearStencil:", stencil => u32 => ()); }
}

impl MTLRenderPipelineDescriptor {
    pub fn setLabel(&self, label: Option<&NSString>) { message!(pointer(self), "setLabel:", optional(label) => *mut c_void => ()); }
    pub fn setVertexFunction(&self, function: Option<&MTLFunction>) { message!(pointer(self), "setVertexFunction:", optional(function) => *mut c_void => ()); }
    pub fn setFragmentFunction(&self, function: Option<&MTLFunction>) { message!(pointer(self), "setFragmentFunction:", optional(function) => *mut c_void => ()); }
    pub fn setVertexDescriptor(&self, descriptor: Option<&MTLVertexDescriptor>) { message!(pointer(self), "setVertexDescriptor:", optional(descriptor) => *mut c_void => ()); }
    pub fn colorAttachment(&self, index: usize) -> Owned<MTLRenderPipelineColorAttachmentDescriptor> {
        let attachments = message!(pointer(self), "colorAttachments" => *mut c_void);
        required_autoreleased(message!(attachments, "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLRenderPipelineColorAttachmentDescriptorArray objectAtIndexedSubscript:]")
    }
    pub fn colorAttachments(&self) -> Owned<MTLRenderPipelineColorAttachmentDescriptorArray> {
        required_autoreleased(message!(pointer(self), "colorAttachments" => *mut c_void), "-[MTLRenderPipelineDescriptor colorAttachments]")
    }
    pub fn setDepthAttachmentPixelFormat(&self, format: objc2_metal::MTLPixelFormat) { message!(pointer(self), "setDepthAttachmentPixelFormat:", format => objc2_metal::MTLPixelFormat => ()); }
    pub fn setStencilAttachmentPixelFormat(&self, format: objc2_metal::MTLPixelFormat) { message!(pointer(self), "setStencilAttachmentPixelFormat:", format => objc2_metal::MTLPixelFormat => ()); }
}

impl MTLRenderPipelineColorAttachmentDescriptorArray {
    pub unsafe fn objectAtIndexedSubscript(&self, index: usize) -> Owned<MTLRenderPipelineColorAttachmentDescriptor> {
        required_autoreleased(message!(pointer(self), "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLRenderPipelineColorAttachmentDescriptorArray objectAtIndexedSubscript:]")
    }
}

impl MTLRenderPipelineColorAttachmentDescriptor {
    pub fn setPixelFormat(&self, value: objc2_metal::MTLPixelFormat) { message!(pointer(self), "setPixelFormat:", value => objc2_metal::MTLPixelFormat => ()); }
    pub fn setWriteMask(&self, value: objc2_metal::MTLColorWriteMask) { message!(pointer(self), "setWriteMask:", value => objc2_metal::MTLColorWriteMask => ()); }
    pub fn setBlendingEnabled(&self, value: bool) { message!(pointer(self), "setBlendingEnabled:", value => bool => ()); }
    pub fn setSourceRGBBlendFactor(&self, value: objc2_metal::MTLBlendFactor) { message!(pointer(self), "setSourceRGBBlendFactor:", value => objc2_metal::MTLBlendFactor => ()); }
    pub fn setDestinationRGBBlendFactor(&self, value: objc2_metal::MTLBlendFactor) { message!(pointer(self), "setDestinationRGBBlendFactor:", value => objc2_metal::MTLBlendFactor => ()); }
    pub fn setRgbBlendOperation(&self, value: objc2_metal::MTLBlendOperation) { message!(pointer(self), "setRgbBlendOperation:", value => objc2_metal::MTLBlendOperation => ()); }
    pub fn setSourceAlphaBlendFactor(&self, value: objc2_metal::MTLBlendFactor) { message!(pointer(self), "setSourceAlphaBlendFactor:", value => objc2_metal::MTLBlendFactor => ()); }
    pub fn setDestinationAlphaBlendFactor(&self, value: objc2_metal::MTLBlendFactor) { message!(pointer(self), "setDestinationAlphaBlendFactor:", value => objc2_metal::MTLBlendFactor => ()); }
    pub fn setAlphaBlendOperation(&self, value: objc2_metal::MTLBlendOperation) { message!(pointer(self), "setAlphaBlendOperation:", value => objc2_metal::MTLBlendOperation => ()); }
}

impl MTLDepthStencilDescriptor {
    pub fn setDepthCompareFunction(&self, value: objc2_metal::MTLCompareFunction) { message!(pointer(self), "setDepthCompareFunction:", value => objc2_metal::MTLCompareFunction => ()); }
    pub fn setDepthWriteEnabled(&self, value: bool) { message!(pointer(self), "setDepthWriteEnabled:", value => bool => ()); }
    pub fn setFrontFaceStencil(&self, value: Option<&MTLStencilDescriptor>) { message!(pointer(self), "setFrontFaceStencil:", optional(value) => *mut c_void => ()); }
    pub fn setBackFaceStencil(&self, value: Option<&MTLStencilDescriptor>) { message!(pointer(self), "setBackFaceStencil:", optional(value) => *mut c_void => ()); }
}

impl MTLStencilDescriptor {
    pub fn setStencilCompareFunction(&self, value: objc2_metal::MTLCompareFunction) { message!(pointer(self), "setStencilCompareFunction:", value => objc2_metal::MTLCompareFunction => ()); }
    pub fn setStencilFailureOperation(&self, value: objc2_metal::MTLStencilOperation) { message!(pointer(self), "setStencilFailureOperation:", value => objc2_metal::MTLStencilOperation => ()); }
    pub fn setDepthFailureOperation(&self, value: objc2_metal::MTLStencilOperation) { message!(pointer(self), "setDepthFailureOperation:", value => objc2_metal::MTLStencilOperation => ()); }
    pub fn setDepthStencilPassOperation(&self, value: objc2_metal::MTLStencilOperation) { message!(pointer(self), "setDepthStencilPassOperation:", value => objc2_metal::MTLStencilOperation => ()); }
    pub fn setReadMask(&self, value: u32) { message!(pointer(self), "setReadMask:", value => u32 => ()); }
    pub fn setWriteMask(&self, value: u32) { message!(pointer(self), "setWriteMask:", value => u32 => ()); }
}

impl MTLSamplerDescriptor {
    pub fn setMinFilter(&self, value: objc2_metal::MTLSamplerMinMagFilter) { message!(pointer(self), "setMinFilter:", value => objc2_metal::MTLSamplerMinMagFilter => ()); }
    pub fn setMagFilter(&self, value: objc2_metal::MTLSamplerMinMagFilter) { message!(pointer(self), "setMagFilter:", value => objc2_metal::MTLSamplerMinMagFilter => ()); }
    pub fn setMipFilter(&self, value: objc2_metal::MTLSamplerMipFilter) { message!(pointer(self), "setMipFilter:", value => objc2_metal::MTLSamplerMipFilter => ()); }
    pub fn setSAddressMode(&self, value: objc2_metal::MTLSamplerAddressMode) { message!(pointer(self), "setSAddressMode:", value => objc2_metal::MTLSamplerAddressMode => ()); }
    pub fn setTAddressMode(&self, value: objc2_metal::MTLSamplerAddressMode) { message!(pointer(self), "setTAddressMode:", value => objc2_metal::MTLSamplerAddressMode => ()); }
}

impl MTLVertexDescriptor {
    pub fn layouts(&self) -> Owned<MTLVertexBufferLayoutDescriptorArray> {
        required_autoreleased(message!(pointer(self), "layouts" => *mut c_void), "-[MTLVertexDescriptor layouts]")
    }
    pub fn attributes(&self) -> Owned<MTLVertexAttributeDescriptorArray> {
        required_autoreleased(message!(pointer(self), "attributes" => *mut c_void), "-[MTLVertexDescriptor attributes]")
    }
    pub fn layout(&self, index: usize) -> Owned<MTLVertexBufferLayoutDescriptor> {
        let layouts = message!(pointer(self), "layouts" => *mut c_void);
        required_autoreleased(message!(layouts, "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLVertexBufferLayoutDescriptorArray objectAtIndexedSubscript:]")
    }
    pub fn attribute(&self, index: usize) -> Owned<MTLVertexAttributeDescriptor> {
        let attributes = message!(pointer(self), "attributes" => *mut c_void);
        required_autoreleased(message!(attributes, "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLVertexAttributeDescriptorArray objectAtIndexedSubscript:]")
    }
}

impl MTLVertexBufferLayoutDescriptorArray {
    pub unsafe fn objectAtIndexedSubscript(&self, index: usize) -> Owned<MTLVertexBufferLayoutDescriptor> {
        required_autoreleased(message!(pointer(self), "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLVertexBufferLayoutDescriptorArray objectAtIndexedSubscript:]")
    }
}

impl MTLVertexAttributeDescriptorArray {
    pub unsafe fn objectAtIndexedSubscript(&self, index: usize) -> Owned<MTLVertexAttributeDescriptor> {
        required_autoreleased(message!(pointer(self), "objectAtIndexedSubscript:", index => usize => *mut c_void), "-[MTLVertexAttributeDescriptorArray objectAtIndexedSubscript:]")
    }
}

impl MTLVertexBufferLayoutDescriptor {
    pub fn setStride(&self, value: usize) { message!(pointer(self), "setStride:", value => usize => ()); }
    pub fn setStepFunction(&self, value: objc2_metal::MTLVertexStepFunction) { message!(pointer(self), "setStepFunction:", value => objc2_metal::MTLVertexStepFunction => ()); }
}

impl MTLVertexAttributeDescriptor {
    pub fn setFormat(&self, value: objc2_metal::MTLVertexFormat) { message!(pointer(self), "setFormat:", value => objc2_metal::MTLVertexFormat => ()); }
    pub fn setOffset(&self, value: usize) { message!(pointer(self), "setOffset:", value => usize => ()); }
    pub fn setBufferIndex(&self, value: usize) { message!(pointer(self), "setBufferIndex:", value => usize => ()); }
}

//#endregion 🧰Descriptors

//#region 🧊Metal Objects

pub fn system_default_device() -> Option<Owned<MTLDevice>> {
    #[link(name = "Metal", kind = "framework")]
    unsafe extern "C-unwind" {
        fn MTLCreateSystemDefaultDevice() -> *mut MTLDevice;
    }
    unsafe { Owned::from_new(MTLCreateSystemDefaultDevice()) }
}

impl MTLDevice {
    pub fn newCommandQueue(&self) -> Option<Owned<MTLCommandQueue>> { unsafe { Owned::from_new(message!(pointer(self), "newCommandQueue" => *mut MTLCommandQueue)) } }
    pub fn newBufferWithLength_options(&self, length: usize, options: objc2_metal::MTLResourceOptions) -> Option<Owned<MTLBuffer>> { unsafe { Owned::from_new(message!(pointer(self), "newBufferWithLength:options:", length => usize, options => objc2_metal::MTLResourceOptions => *mut MTLBuffer)) } }
    pub unsafe fn newBufferWithBytes_length_options(&self, bytes: NonNull<c_void>, length: usize, options: objc2_metal::MTLResourceOptions) -> Option<Owned<MTLBuffer>> { unsafe { Owned::from_new(message!(pointer(self), "newBufferWithBytes:length:options:", bytes.as_ptr() => *mut c_void, length => usize, options => objc2_metal::MTLResourceOptions => *mut MTLBuffer)) } }
    pub fn newTextureWithDescriptor(&self, descriptor: &MTLTextureDescriptor) -> Option<Owned<MTLTexture>> { unsafe { Owned::from_new(message!(pointer(self), "newTextureWithDescriptor:", pointer(descriptor) => *mut c_void => *mut MTLTexture)) } }
    pub fn newDepthStencilStateWithDescriptor(&self, descriptor: &MTLDepthStencilDescriptor) -> Option<Owned<MTLDepthStencilState>> { unsafe { Owned::from_new(message!(pointer(self), "newDepthStencilStateWithDescriptor:", pointer(descriptor) => *mut c_void => *mut MTLDepthStencilState)) } }
    pub fn newSamplerStateWithDescriptor(&self, descriptor: &MTLSamplerDescriptor) -> Option<Owned<MTLSamplerState>> { unsafe { Owned::from_new(message!(pointer(self), "newSamplerStateWithDescriptor:", pointer(descriptor) => *mut c_void => *mut MTLSamplerState)) } }
    pub fn newLibraryWithSource_options_error(&self, source: &NSString, _options: Option<&AnyObject>) -> Result<Owned<MTLLibrary>, Owned<NSError>> {
        let mut error = std::ptr::null_mut();
        let library = message!(pointer(self), "newLibraryWithSource:options:error:", pointer(source) => *mut c_void, std::ptr::null_mut() => *mut c_void, &mut error => *mut *mut c_void => *mut MTLLibrary);
        unsafe { Owned::from_new(library) }.ok_or_else(|| unsafe { Owned::from_autoreleased(error.cast()) }.expect("Metal library compilation failed without NSError"))
    }
    pub fn newRenderPipelineStateWithDescriptor_error(&self, descriptor: &MTLRenderPipelineDescriptor) -> Result<Owned<MTLRenderPipelineState>, Owned<NSError>> {
        let mut error = std::ptr::null_mut();
        let state = message!(pointer(self), "newRenderPipelineStateWithDescriptor:error:", pointer(descriptor) => *mut c_void, &mut error => *mut *mut c_void => *mut MTLRenderPipelineState);
        unsafe { Owned::from_new(state) }.ok_or_else(|| unsafe { Owned::from_autoreleased(error.cast()) }.expect("Metal pipeline creation failed without NSError"))
    }
    pub fn hasUnifiedMemory(&self) -> bool { message!(pointer(self), "hasUnifiedMemory" => bool) }
    pub fn isLowPower(&self) -> bool { message!(pointer(self), "isLowPower" => bool) }
}

impl MTLLibrary {
    pub fn newFunctionWithName(&self, name: &NSString) -> Option<Owned<MTLFunction>> { unsafe { Owned::from_new(message!(pointer(self), "newFunctionWithName:", pointer(name) => *mut c_void => *mut MTLFunction)) } }
}

impl MTLBuffer {
    pub fn contents(&self) -> NonNull<c_void> { NonNull::new(message!(pointer(self), "contents" => *mut c_void)).expect("Metal buffer contents is nil") }
    pub fn length(&self) -> usize { message!(pointer(self), "length" => usize) }
}

impl MTLTexture {
    pub unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(&self, region: objc2_metal::MTLRegion, level: usize, bytes: NonNull<c_void>, bytes_per_row: usize) { message!(pointer(self), "replaceRegion:mipmapLevel:withBytes:bytesPerRow:", region => objc2_metal::MTLRegion, level => usize, bytes.as_ptr() => *mut c_void, bytes_per_row => usize => ()); }
    pub unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(&self, bytes: NonNull<c_void>, bytes_per_row: usize, region: objc2_metal::MTLRegion, level: usize) { message!(pointer(self), "getBytes:bytesPerRow:fromRegion:mipmapLevel:", bytes.as_ptr() => *mut c_void, bytes_per_row => usize, region => objc2_metal::MTLRegion, level => usize => ()); }
    pub fn get_width(&self) -> usize { message!(pointer(self), "width" => usize) }
    pub fn get_height(&self) -> usize { message!(pointer(self), "height" => usize) }
}

impl MTLCommandQueue {
    pub fn commandBuffer(&self) -> Option<Owned<MTLCommandBuffer>> { unsafe { Owned::from_autoreleased(message!(pointer(self), "commandBuffer" => *mut MTLCommandBuffer)) } }
}

impl MTLCommandBuffer {
    pub fn renderCommandEncoderWithDescriptor(&self, descriptor: &MTLRenderPassDescriptor) -> Option<Owned<MTLRenderCommandEncoder>> { unsafe { Owned::from_autoreleased(message!(pointer(self), "renderCommandEncoderWithDescriptor:", pointer(descriptor) => *mut c_void => *mut MTLRenderCommandEncoder)) } }
    pub fn blitCommandEncoder(&self) -> Option<Owned<MTLBlitCommandEncoder>> { unsafe { Owned::from_autoreleased(message!(pointer(self), "blitCommandEncoder" => *mut MTLBlitCommandEncoder)) } }
    pub fn presentDrawable(&self, drawable: &CAMetalDrawable) { message!(pointer(self), "presentDrawable:", pointer(drawable) => *mut c_void => ()); }
    pub fn commit(&self) { message!(pointer(self), "commit" => ()); }
}

impl MTLRenderCommandEncoder {
    pub fn setRenderPipelineState(&self, state: &MTLRenderPipelineState) { message!(pointer(self), "setRenderPipelineState:", pointer(state) => *mut c_void => ()); }
    pub fn setDepthStencilState(&self, state: Option<&MTLDepthStencilState>) { message!(pointer(self), "setDepthStencilState:", optional(state) => *mut c_void => ()); }
    pub unsafe fn setVertexBuffer_offset_atIndex(&self, buffer: Option<&MTLBuffer>, offset: usize, index: usize) { message!(pointer(self), "setVertexBuffer:offset:atIndex:", optional(buffer) => *mut c_void, offset => usize, index => usize => ()); }
    pub unsafe fn setFragmentBuffer_offset_atIndex(&self, buffer: Option<&MTLBuffer>, offset: usize, index: usize) { message!(pointer(self), "setFragmentBuffer:offset:atIndex:", optional(buffer) => *mut c_void, offset => usize, index => usize => ()); }
    pub unsafe fn setFragmentBytes_length_atIndex(&self, bytes: NonNull<c_void>, length: usize, index: usize) { message!(pointer(self), "setFragmentBytes:length:atIndex:", bytes.as_ptr() => *mut c_void, length => usize, index => usize => ()); }
    pub unsafe fn setFragmentTexture_atIndex(&self, texture: Option<&MTLTexture>, index: usize) { message!(pointer(self), "setFragmentTexture:atIndex:", optional(texture) => *mut c_void, index => usize => ()); }
    pub unsafe fn setFragmentSamplerState_atIndex(&self, sampler: Option<&MTLSamplerState>, index: usize) { message!(pointer(self), "setFragmentSamplerState:atIndex:", optional(sampler) => *mut c_void, index => usize => ()); }
    pub fn setScissorRect(&self, value: objc2_metal::MTLScissorRect) { message!(pointer(self), "setScissorRect:", value => objc2_metal::MTLScissorRect => ()); }
    pub fn setViewport(&self, value: objc2_metal::MTLViewport) { message!(pointer(self), "setViewport:", value => objc2_metal::MTLViewport => ()); }
    pub fn setStencilReferenceValue(&self, value: u32) { message!(pointer(self), "setStencilReferenceValue:", value => u32 => ()); }
    pub fn setCullMode(&self, value: objc2_metal::MTLCullMode) { message!(pointer(self), "setCullMode:", value => objc2_metal::MTLCullMode => ()); }
    pub fn setDepthBias_slopeScale_clamp(&self, bias: f32, scale: f32, clamp: f32) { message!(pointer(self), "setDepthBias:slopeScale:clamp:", bias => f32, scale => f32, clamp => f32 => ()); }
    pub unsafe fn drawPrimitives_vertexStart_vertexCount(&self, primitive: objc2_metal::MTLPrimitiveType, start: usize, count: usize) { message!(pointer(self), "drawPrimitives:vertexStart:vertexCount:", primitive => objc2_metal::MTLPrimitiveType, start => usize, count => usize => ()); }
    pub unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount(&self, primitive: objc2_metal::MTLPrimitiveType, start: usize, count: usize, instances: usize) { message!(pointer(self), "drawPrimitives:vertexStart:vertexCount:instanceCount:", primitive => objc2_metal::MTLPrimitiveType, start => usize, count => usize, instances => usize => ()); }
    pub unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(&self, primitive: objc2_metal::MTLPrimitiveType, count: usize, index_type: objc2_metal::MTLIndexType, buffer: &MTLBuffer, offset: usize, instances: usize) { message!(pointer(self), "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:", primitive => objc2_metal::MTLPrimitiveType, count => usize, index_type => objc2_metal::MTLIndexType, pointer(buffer) => *mut c_void, offset => usize, instances => usize => ()); }
    pub fn endEncoding(&self) { message!(pointer(self), "endEncoding" => ()); }
}

impl MTLBlitCommandEncoder {
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(&self, source: &MTLTexture, source_slice: usize, source_level: usize, source_origin: objc2_metal::MTLOrigin, source_size: objc2_metal::MTLSize, destination: &MTLTexture, destination_slice: usize, destination_level: usize, destination_origin: objc2_metal::MTLOrigin) {
        message!(pointer(self), "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:", pointer(source) => *mut c_void, source_slice => usize, source_level => usize, source_origin => objc2_metal::MTLOrigin, source_size => objc2_metal::MTLSize, pointer(destination) => *mut c_void, destination_slice => usize, destination_level => usize, destination_origin => objc2_metal::MTLOrigin => ());
    }
    pub fn endEncoding(&self) { message!(pointer(self), "endEncoding" => ()); }
}

//#endregion 🧊Metal Objects

//#region 🪟AppKit And QuartzCore

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoreGraphicsSize {
    pub width: f64,
    pub height: f64,
}

impl CoreGraphicsSize {
    pub const OBJECTIVE_C_ENCODING: &'static str = "{CGSize=dd}";
}

impl AnyObject {
    pub unsafe fn setWantsLayer(&self, value: bool) { message!(pointer(self), "setWantsLayer:", value => bool => ()); }
    pub unsafe fn setLayer(&self, layer: &CAMetalLayer) { message!(pointer(self), "setLayer:", pointer(layer) => *mut c_void => ()); }
}

impl CAMetalLayer {
    pub fn new() -> Owned<Self> { required_new(message!(unsafe { class(c"CAMetalLayer") }, "new" => *mut c_void), "+[CAMetalLayer new]") }
    pub fn setDevice(&self, device: Option<&MTLDevice>) { message!(pointer(self), "setDevice:", optional(device) => *mut c_void => ()); }
    pub fn setPixelFormat(&self, format: objc2_metal::MTLPixelFormat) { message!(pointer(self), "setPixelFormat:", format => objc2_metal::MTLPixelFormat => ()); }
    pub fn setFramebufferOnly(&self, value: bool) { message!(pointer(self), "setFramebufferOnly:", value => bool => ()); }
    pub fn drawableSize(&self) -> CoreGraphicsSize { message!(pointer(self), "drawableSize" => CoreGraphicsSize) }
    pub fn setDrawableSize(&self, size: CoreGraphicsSize) { message!(pointer(self), "setDrawableSize:", size => CoreGraphicsSize => ()); }
    pub fn nextDrawable(&self) -> Option<Owned<CAMetalDrawable>> { unsafe { Owned::from_autoreleased(message!(pointer(self), "nextDrawable" => *mut CAMetalDrawable)) } }
    pub fn set_device(&self, device: Option<&MTLDevice>) { self.setDevice(device); }
    pub fn set_pixel_format(&self, format: objc2_metal::MTLPixelFormat) { self.setPixelFormat(format); }
    pub fn set_framebuffer_only(&self, value: bool) { self.setFramebufferOnly(value); }
    pub fn drawable_size(&self) -> CoreGraphicsSize { self.drawableSize() }
    pub fn set_drawable_size(&self, size: CoreGraphicsSize) { self.setDrawableSize(size); }
    pub fn next_drawable(&self) -> Option<Owned<CAMetalDrawable>> { self.nextDrawable() }
}

impl CAMetalDrawable {
    pub fn texture(&self) -> Owned<MTLTexture> { required_autoreleased(message!(pointer(self), "texture" => *mut c_void), "-[CAMetalDrawable texture]") }
}

pub unsafe fn with_appkit_view<R>(pointer: *mut c_void, run: impl for<'a> FnOnce(&'a AnyObject) -> R) -> R {
    assert!(!pointer.is_null(), "AppKit window handle contains a null NSView");
    run(unsafe { &*pointer.cast::<AnyObject>() })
}

pub fn retain_count<T>(object: &T) -> usize {
    message!(pointer(object), "retainCount" => usize)
}

//#endregion 🪟AppKit And QuartzCore

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    const MAX_RETAIN_CYCLES: usize = 4096;

    fn bounded_retain_cycles(value: usize) -> Result<usize, &'static str> {
        (value <= MAX_RETAIN_CYCLES).then_some(value).ok_or("retain-cycle fixture exceeds its bounded test budget")
    }

    #[test]
    fn owned_runtime_preserves_empty_single_max_max_plus_one_and_hostile_contract() {
        autorelease_pool(|| {
            for requested in [0usize, 1, MAX_RETAIN_CYCLES] {
                let cycles = bounded_retain_cycles(requested).expect("fixture case must be accepted");
                let value = CAMetalLayer::new();
                let before = retain_count(&*value);
                for _ in 0..cycles {
                    let clone = value.clone();
                    assert_eq!(retain_count(&*value), before + 1);
                    drop(clone);
                }
                assert_eq!(retain_count(&*value), before);
            }
            assert_eq!(bounded_retain_cycles(MAX_RETAIN_CYCLES + 1), Err("retain-cycle fixture exceeds its bounded test budget"));
            assert_eq!(bounded_retain_cycles(usize::MAX), Err("retain-cycle fixture exceeds its bounded test budget"));
            assert!(unsafe { Owned::<NSString>::from_new(std::ptr::null_mut()) }.is_none());
            let fixture = format!(
                "{{\n  \"$schema\": \"../🧬️schema/🔣️objc2-runtime-abi.schema.json\",\n  \"schemaVersion\": 1,\n  \"contract\": \"owned-objective-c-runtime\",\n  \"oracle\": {{ \"package\": \"objc2\", \"version\": \"0.6.4\" }},\n  \"layout\": {{ \"ownedBytes\": {}, \"ownedAlign\": {}, \"optionalOwnedBytes\": {} }},\n  \"ownership\": {{ \"cloneRetainDelta\": 1, \"dropRestores\": true, \"nullOwnedAccepted\": false, \"autoreleasePoolDrained\": true }},\n  \"boundaries\": {{ \"empty\": \"accepted\", \"single\": \"accepted\", \"maximum\": 4096, \"maximumPlusOne\": \"rejected\", \"hostileNull\": \"rejected\" }}\n}}\n",
                size_of::<Owned<NSString>>(),
                align_of::<Owned<NSString>>(),
                size_of::<Option<Owned<NSString>>>()
            );
            assert_eq!(fixture, include_str!("🧫️fixtures/🔣️objc2-runtime-abi.json"));
            println!("empty=ok single=ok max=4096 maxPlusOne=rejected hostileNull=rejected retainDelta=1 restored=true pool=drained");
        });
    }
}

//#endregion Tests
