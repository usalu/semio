//! 🖥️ Raster host of surface paint: the layer buffers, the raster host record and its compositing, painting and document operations. See [`super`].

use super::*;

// #region 🔖️Host
#[derive(Default)]
pub(super) struct RasterLayerBuffers {
    pub(super) paint: HashMap<String, Arc<semio_framework_pixels::RasterImage>>,
}

pub struct RasterHost {
    /// 🖱️ (c) Preview/Effect — live viewport camera during pan/zoom, never dispatched at frame rate.
    /// Mirrors `RasterConfig.camera` (`RasterConfigMutation` LOCAL_UI, plugin-owned) once a gesture settles.
    pub(super) camera: Camera,
    /// 🖥️ (d) Render-session wiring — device viewport size/DPR, not document content.
    pub(super) viewport: Viewport,
    /// 🎞️ (a) elsewhere, not here — wholesale mirror of the plugin's real `RasterSnapshot.layers`
    /// (`crate::artifacts::raster::schema::snapshot`), refreshed by [`RasterHost::sync_document_json`].
    /// The 11 real `create/delete/rename/move/resize/reorder-layers` + `change-layer-*` +
    /// `add/remove-layer-asset` triads already live on that owner; authoring a second mutation set
    /// here would duplicate authoritative state, the exact violation this ticket exists to remove.
    pub(super) document: RasterDocument,
    /// 🖌️ (d) ephemeral working representation during an active paint gesture — raw pixel scratch
    /// buffers keyed by layer: drop at any instant and nothing a user has committed is lost. The eventual persisted commit of
    /// painted pixels is an `image:in`-shaped asset import through the plugin's real `add-layer-asset`
    /// mutation (see the module docstring) — this host never calls that; it only holds the scratch.
    pub(super) buffers: RasterLayerBuffers,
    pub(super) composite: compositing::RetainedComposite,
    /// 🧰️ (c) Preview/Effect — active tool id. Mirrors `RasterConfig.active_utility_id`
    /// (`RasterConfigMutation::SetActiveUtility`, plugin-owned LOCAL_UI state).
    pub(super) active_utility: String,
    /// 🖌️ (c) Preview/Effect — brush diameter. Mirrors `RasterConfig.brush_size`
    /// (`RasterConfigMutation::SetBrushSize`, plugin-owned LOCAL_UI state).
    pub(super) brush_size: f32,
    /// 🖌️ (c) Preview/Effect — brush opacity. Mirrors `RasterConfig.brush_opacity`, plugin-owned.
    pub(super) brush_opacity: f32,
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3c: (c) Preview/Effect — hover
    /// feedback id, read from the framework's `DomainHover` via [`RasterHost::sync_interaction`], not
    /// pushed by the app anymore.
    pub(super) hovered_id: Option<String>,
    /// 🕹️ (c) Preview/Effect — selection ids, read from the framework's `DomainSelection` via
    /// [`RasterHost::sync_interaction`].
    pub(super) selected_ids: Vec<String>,
    pub(super) paint_gesture: Option<gesture::PaintGesture>,
    pub(super) paint_edit: Option<PaintEditCommand>,
    /// 🖐️ (c) Preview/Effect — pan-gesture-in-progress flag, discarded on release.
    pub(super) panning: bool,
    pub(super) brush_color: [u8; 4],
    pub(super) brush_hardness: f32,
    pub(super) paint_target: PaintTarget,
    pub(super) mask_value: u8,
    /// 🖐️ (c) Preview/Effect — pan interpolation anchor, discarded on release.
    pub(super) pan_last: Option<Point>,
    /// 👁️ (c) Preview/Effect — selection-chrome visibility toggle, UI rendering only.
    pub(super) show_selection_chrome: bool,
    /// 🎨️ (d) runtime wiring — clear color derived from the app's UI theme, recomputed on theme change.
    pub(super) theme_clear: Color,
    /// 🎨️ (d) runtime wiring — checkerboard fill cache derived from `theme_clear`.
    pub(super) checkerboard_light_cell: u8,
    /// 🎨️ (d) runtime wiring — checkerboard fill cache derived from `theme_clear`.
    pub(super) checkerboard_dark_cell: u8,
}

impl Default for RasterHost {
    fn default() -> Self {
        Self::new()
    }
}

impl RasterHost {
    pub fn advance_composite(&mut self,budget:usize)->Result<semio_framework_pixels::editing::PixelProgress,String>{self.composite.advance(budget,&self.document.stack,&self.buffers.paint)}
    pub fn composite_pixels(&self)->Option<&semio_framework_pixels::RasterImage>{self.composite.output.as_ref().map(|output|&output.image)}
    pub fn composite_pending(&self)->bool{self.composite.pending}
    pub fn cancel_composite(&mut self){self.composite.cancel();}
    pub fn composite_progress_json(&self)->String{self.composite.progress_json()}
    pub fn set_composite_view(&mut self,mode:&str,id:Option<&str>){self.composite.select(match (mode,id){("layer",Some(id))=>compositing::View::Layer(id.into()),("mask",Some(id))=>compositing::View::Mask(id.into()),_=>compositing::View::Composite});}

    pub fn new() -> Self {
        let theme_clear = theme::canvas_clear_for(ui_styling::appearance::AppearanceName::Light);
        let (checkerboard_light_cell, checkerboard_dark_cell) = theme::checkerboard_shades_for_clear(theme_clear);
        Self {
            camera: Camera { x: 0.0, y: 0.0, zoom: 1.0 },
            viewport: Viewport { width: 800, height: 600, dpr: 1.0 },
            document: RasterDocument { layers: vec![],stack:vec![] },
            buffers: RasterLayerBuffers::default(),
            composite: compositing::RetainedComposite::default(),
            active_utility: "selectMarquee".into(),
            brush_size: 24.0,
            brush_opacity: 1.0,
            hovered_id: None,
            selected_ids: vec![],
            panning: false,
            brush_color: [40, 120, 220, 255],
            brush_hardness: 1.0,
            paint_target: PaintTarget::Pixels,
            mask_value: 255,
            paint_gesture: None,
            paint_edit: None,
            pan_last: None,
            show_selection_chrome: true,
            theme_clear,
            checkerboard_light_cell,
            checkerboard_dark_cell,
        }
    }

    /// 👻️ World coordinates to surface-local pixels through the raster canvas's live camera.
    pub fn world_to_screen_point(&self, wx: f64, wy: f64) -> (f64, f64) {
        let point = camera::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy));
        (point.x, point.y)
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), FrameworkSurfacePaintError> {
        let v: serde_json::Value = serde_json::from_str(json)?;
        theme::merge_color_field(&mut self.theme_clear, &v, "rasterClear");
        let (checkerboard_light_cell, checkerboard_dark_cell) = theme::checkerboard_shades_for_clear(self.theme_clear);
        self.checkerboard_light_cell = checkerboard_light_cell;
        self.checkerboard_dark_cell = checkerboard_dark_cell;
        Ok(())
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport.width = width.max(1);
        self.viewport.height = height.max(1);
        self.viewport.dpr = dpr.max(1.0);
    }

    pub fn set_show_selection_chrome(&mut self, enabled: bool) {
        self.show_selection_chrome = enabled;
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = camera::clamp_zoom(zoom);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        camera::wheel_screen(&mut self.camera, &self.viewport, sx, sy, delta_y);
    }

    pub(super) fn screen_to_world(&self, sx: f64, sy: f64) -> Point {
        camera::screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy))
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        if button == 1 {
            self.panning = true;
            self.pan_last = Some(Point::new(sx, sy));
            return;
        }
        if button == 0 && self.paint_edit.is_none() {
            match self.active_utility.as_str() {
                "paintBrush" | "paintEraser" => self.paint_gesture = gesture::PaintGesture::begin(self, self.screen_to_world(sx, sy), self.active_utility == "paintEraser"),
                "paintBucket" => self.paint_edit = gesture::PaintGesture::click(self, self.screen_to_world(sx, sy)),
                _ => {}
            }
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        if self.panning {
            if let Some(last) = self.pan_last {
                let dx = (sx - last.x) / self.camera.zoom;
                let dy = (sy - last.y) / self.camera.zoom;
                self.camera.x -= dx;
                self.camera.y -= dy;
            }
            self.pan_last = Some(Point::new(sx, sy));
            return;
        }
        let world = self.screen_to_world(sx, sy);
        if let Some(gesture) = self.paint_gesture.as_mut() {
            gesture.push(world);
            if self.paint_edit.is_none() { self.paint_edit = gesture.stream(); }
        }
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        self.panning = false;
        self.pan_last = None;
        if let Some(mut gesture) = self.paint_gesture.take() {
            gesture.push(self.screen_to_world(sx, sy));
            self.paint_edit = match (self.paint_edit.take(), gesture.finish()) {
                (Some(pending), Some(mut release)) if pending.phase == Some("stream") && pending.gesture == release.gesture => {
                    release.points.splice(0..0, pending.points);
                    Some(release)
                }
                (pending, release) => release.or(pending),
            };
        }
    }

    pub fn pointer_cancel_screen(&mut self) {
        self.panning = false;
        self.pan_last = None;
        self.abandon_paint_gesture();
    }

    /// 🧯️ Drops the gesture in flight; one that already streamed leaves the abort of its press to publish.
    fn abandon_paint_gesture(&mut self) {
        if let Some(abort) = self.paint_gesture.take().and_then(gesture::PaintGesture::abandon) {
            self.paint_edit.get_or_insert(abort);
        }
    }

    pub fn paint_edit(&self) -> Option<&PaintEditCommand> { self.paint_edit.as_ref() }
    pub fn take_paint_edit(&mut self) -> Option<PaintEditCommand> { self.paint_edit.take() }
    pub fn set_paint_target(&mut self,target:PaintTarget){if self.paint_target!=target{self.paint_target=target;self.invalidate_paint_gesture();}}
    pub fn set_mask_value(&mut self,value:u8){self.mask_value=value;}
    pub fn set_brush_color(&mut self, color: [u8; 4]) { self.brush_color = color; }
    pub fn set_brush_hardness(&mut self, hardness: f32) { self.brush_hardness = hardness.clamp(0.0, 1.0); }

    pub(super) fn layer_pixel_buffer_key(id: &str) -> String { format!("layer:{id}") }

    pub fn sync_document_json(&mut self, json: &str) -> Result<(), FrameworkSurfacePaintError> {
        self.document = parse_document(json)?;
        self.refresh_image_extents();
        self.invalidate_paint_gesture();
        self.composite.invalidate();
        Ok(())
    }

    fn invalidate_paint_gesture(&mut self){
        if self.paint_gesture.as_ref().is_some_and(|gesture|!gesture.matches(self)){self.abandon_paint_gesture();}
    }

    fn refresh_image_extents(&mut self){
        fn visit(nodes:&mut [LayerNode],stack:&[RasterStackLayer],images:&HashMap<String,Arc<semio_framework_pixels::RasterImage>>){
            for (node,layer) in nodes.iter_mut().zip(stack){match (node,&layer.content){
                (LayerNode::Pixel {id,width,height,..},RasterStackContent::Pixel {width:display_width,height:display_height,image_key})=>{
                    let fallback=RasterHost::layer_pixel_buffer_key(id);let image=images.get(image_key.as_deref().unwrap_or(&fallback));
                    *width=display_width.unwrap_or_else(||image.map_or(512,|image|image.width));*height=display_height.unwrap_or_else(||image.map_or(512,|image|image.height));
                }
                (LayerNode::Group {children,..},RasterStackContent::Group(stack))=>visit(children,stack,images),
                _=>{}
            }}
        }
        visit(&mut self.document.layers,&self.document.stack,&self.buffers.paint);
    }

    pub fn upload_layer_image(&mut self, layer_id: &str, bytes: &[u8]) -> Result<(), FrameworkSurfacePaintError> {
        let decoded = semio_framework_pixels::decode_png(bytes)?;
        let key = Self::layer_pixel_buffer_key(layer_id);
        self.buffers.paint.insert(key.clone(),Arc::new(decoded));
        self.refresh_image_extents();
        self.invalidate_paint_gesture();
        self.composite.invalidate();
        Ok(())
    }

    pub fn upload_raster_image_key(&mut self, key: &str, bytes: &[u8]) -> Result<(), FrameworkSurfacePaintError> {
        let decoded = semio_framework_pixels::decode_png(bytes)?;
        self.buffers.paint.insert(key.to_string(),Arc::new(decoded));
        self.refresh_image_extents();
        self.invalidate_paint_gesture();
        self.composite.invalidate();
        Ok(())
    }

    pub fn set_active_utility(&mut self, utility: &str) {
        if self.active_utility!=utility {self.abandon_paint_gesture();}
        self.active_utility = utility.to_string();
    }

    pub fn set_brush_size(&mut self, size: f32) {
        self.brush_size = size;
    }

    pub fn set_brush_opacity(&mut self, opacity: f32) {
        self.brush_opacity = opacity.clamp(0.0, 1.0);
    }

    /// 🕹️ Replaces the deleted `set_hovered_id`/`set_selection_ids_json` push-setters — reads the
    /// framework's current `DomainSelection.ids`/`DomainHover.ids.first()` for this domain, called at
    /// render time instead of pushed arbitrarily by app code.
    pub fn sync_interaction(&mut self, selected_ids: &[String], hovered_id: Option<&str>) {
        self.selected_ids = selected_ids.to_vec();
        self.invalidate_paint_gesture();
        self.hovered_id = hovered_id.map(str::to_string);
    }

    pub fn camera_json(&self) -> String {
        serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }).to_string()
    }

    fn append_selection_chrome(&self,scene:&mut Scene,parent:Affine,layers:&[LayerNode]) {
        for layer in layers {
            match layer {
                LayerNode::Pixel {id,visible,transform,width,height,..} if *visible=>{
                    if self.show_selection_chrome&&(self.hovered_id.as_deref()==Some(id.as_str())||self.selected_ids.iter().any(|selected|selected==id)) {
                        let world=parent*(*transform)*Affine::IDENTITY.translate(Vec2::new(-f64::from(*width)/2.0,-f64::from(*height)/2.0));
                        scene.stroke(&Stroke::new(2.0/self.camera.zoom.max(0.1)),world,Color::from_rgba8(80,160,255,220),None,&Rect::new(0.0,0.0,f64::from(*width),f64::from(*height)));
                    }
                }
                LayerNode::Group {visible,transform,children,..} if *visible=>self.append_selection_chrome(scene,parent*(*transform),children),
                _=>{}
            }
        }
    }

    pub fn build_vector_scene(&mut self) -> Scene {
        self.build_scene_for_layer(None)
    }

    pub fn build_layer_scene(&mut self, layer_id: &str) -> Scene {
        self.build_scene_for_layer(Some(layer_id))
    }

    pub fn build_mask_scene(&mut self,layer_id:&str)->Scene {
        self.set_composite_view("mask",Some(layer_id));self.build_current_scene()
    }

    fn build_scene_for_layer(&mut self,isolated:Option<&str>)->Scene {
        self.set_composite_view(if isolated.is_some(){"layer"}else{"composite"},isolated);self.build_current_scene()
    }

    pub fn build_current_scene(&mut self)->Scene {
        let _=self.advance_composite(65536);
        let mut scene=Scene::new();
        let cam=camera::camera_content_affine(&self.camera,&self.viewport);
        if let (Some(output),Some(image))=(&self.composite.output,&self.composite.image) {
            let world=cam*Affine::IDENTITY.translate(Vec2::new(output.origin[0],output.origin[1]));
            if matches!(self.composite.view,compositing::View::Mask(_)) {
                scene.fill(FillRule::NonZero,world,Color::from_rgba8(0,0,0,255),None,&Rect::new(0.0,0.0,f64::from(output.image.width),f64::from(output.image.height)));
            }
            raster::draw_image_arc(&mut scene,image,world);
        }
        if !matches!(self.composite.view,compositing::View::Mask(_)){self.append_selection_chrome(&mut scene,cam,&self.document.layers);}
        let isolated=match &self.composite.view{compositing::View::Layer(id)|compositing::View::Mask(id)=>Some(id.as_str()),compositing::View::Composite=>None};
        if let Some(gesture) = &self.paint_gesture {
            if isolated.is_none_or(|id| id == gesture.command.layer_id) {
                let world = cam * gesture.world;
                let alpha = (self.brush_opacity * f32::from(self.brush_color[3])).round() as u8;
                let color = if self.paint_target==PaintTarget::Mask {let value=if self.active_utility=="paintEraser"{0}else{self.mask_value};Color::from_rgba8(value,value,value,alpha)} else if self.active_utility == "paintEraser" { Color::from_rgba8(255, 255, 255, alpha) } else { Color::from_rgba8(self.brush_color[0], self.brush_color[1], self.brush_color[2], alpha) };
                let mut path = BezPath::new();
                if let Some(point) = gesture.points.first() {
                    path.move_to((point[0], point[1]));
                    for point in &gesture.points[1..] { path.line_to((point[0], point[1])); }
                    scene.stroke(&Stroke::new(f64::from(self.brush_size)), world, color, None, &path);
                    scene.fill(FillRule::NonZero, world, color, None, &Circle::new(Point::new(point[0], point[1]), f64::from(self.brush_size) * 0.5));
                }
            }
        }
        scene
    }

    pub fn build_render_scene(&mut self) -> Scene {
        let inner = self.build_vector_scene();
        render::scale_scene_for_device_pixel_ratio(inner, self.viewport.dpr)
    }
}

impl canvas_content::CanvasContent for RasterHost {
    fn build_scene(&self) -> Scene {
        Scene::new()
    }

    fn clear_color(&self) -> Color {
        self.theme_clear
    }
}
// #endregion 🔖️Host
