//! 🗑️ Retained retirement: the retirement step of the retained draw list items and the retirement emptiness query. See [`super`].

use super::*;

// #region draw_types
impl DrawList {
    /// ♻️ Retires one retained item and reports a failed page release instead of dropping it; `Ok(true)` once nothing is retained.
    pub fn retire_step_checked(&mut self) -> Result<bool, PagedListError> {
        if let Some(token) = self.active_retained_append_token() {
            if self.cancel_retained_append_step(token).0.is_err() { return Err(PagedListError { kind: semio_framework_value_paged_list::PagedListRefusalKind::InvariantViolated, reason: "retained append retirement invariant" }); }
            return Ok(false);
        }
        if let Some(cursor) = &mut self.border_finalize {
            if cursor.retire_step() { self.border_finalize = None; }
            return Ok(false);
        }
        if let Some(pass) = self.scene_passes.last_mut() {
            if let Some(draw) = pass.textured_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.texture_key.pop().is_some() {
                        return Ok(false);
                    }
                    if instance.texture_key.capacity() > 0 {
                        instance.texture_key = String::new();
                        return Ok(false);
                    }
                    draw.instances.pop();
                    return Ok(false);
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return Ok(false);
                }
                pass.textured_draws.pop();
                return Ok(false);
            }
            if let Some(draw) = pass.material_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.id.pop().is_some() {
                        return Ok(false);
                    }
                    if instance.id.capacity() > 0 {
                        instance.id = String::new();
                        return Ok(false);
                    }
                    draw.instances.pop();
                    return Ok(false);
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return Ok(false);
                }
                if let Some(texture_key) = draw.material.texture_key_mut() {
                    if texture_key.pop().is_some() {
                        return Ok(false);
                    }
                    if texture_key.capacity() > 0 {
                        *texture_key = String::new();
                        return Ok(false);
                    }
                }
                if draw.mesh_key.pop().is_some() {
                    return Ok(false);
                }
                if draw.mesh_key.capacity() > 0 {
                    draw.mesh_key = String::new();
                    return Ok(false);
                }
                pass.material_draws.pop();
                return Ok(false);
            }
            if let Some(draw) = pass.translucent_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.id.pop().is_some() {
                        return Ok(false);
                    }
                    if instance.id.capacity() > 0 {
                        instance.id = String::new();
                        return Ok(false);
                    }
                    draw.instances.pop();
                    return Ok(false);
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return Ok(false);
                }
                if draw.mesh_key.pop().is_some() {
                    return Ok(false);
                }
                if draw.mesh_key.capacity() > 0 {
                    draw.mesh_key = String::new();
                    return Ok(false);
                }
                pass.translucent_draws.pop();
                return Ok(false);
            }
            if let Some(draw) = pass.line_draws.last_mut() {
                if draw.vertices.pop().is_some() {
                    return Ok(false);
                }
                if draw.vertices.capacity() > 0 {
                    draw.vertices = Vec::new();
                    return Ok(false);
                }
                pass.line_draws.pop();
                return Ok(false);
            }
            if let Some(draw) = pass.draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.id.pop().is_some() {
                        return Ok(false);
                    }
                    if instance.id.capacity() > 0 {
                        instance.id = String::new();
                        return Ok(false);
                    }
                    draw.instances.pop();
                    return Ok(false);
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return Ok(false);
                }
                if draw.mesh_key.pop().is_some() {
                    return Ok(false);
                }
                if draw.mesh_key.capacity() > 0 {
                    draw.mesh_key = String::new();
                    return Ok(false);
                }
                pass.draws.pop();
                return Ok(false);
            }
            if let Some(draw) = pass.shadow_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.id.pop().is_some() {
                        return Ok(false);
                    }
                    if instance.id.capacity() > 0 {
                        instance.id = String::new();
                        return Ok(false);
                    }
                    draw.instances.pop();
                    return Ok(false);
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return Ok(false);
                }
                if draw.mesh_key.pop().is_some() {
                    return Ok(false);
                }
                if draw.mesh_key.capacity() > 0 {
                    draw.mesh_key = String::new();
                    return Ok(false);
                }
                pass.shadow_draws.pop();
                return Ok(false);
            }
            if pass.textured_draws.capacity() > 0 {
                pass.textured_draws = Vec::new();
                return Ok(false);
            }
            if pass.material_draws.capacity() > 0 {
                pass.material_draws = Vec::new();
                return Ok(false);
            }
            if pass.translucent_draws.capacity() > 0 {
                pass.translucent_draws = Vec::new();
                return Ok(false);
            }
            if pass.line_draws.capacity() > 0 {
                pass.line_draws = Vec::new();
                return Ok(false);
            }
            if pass.draws.capacity() > 0 {
                pass.draws = Vec::new();
                return Ok(false);
            }
            if pass.shadow_draws.capacity() > 0 {
                pass.shadow_draws = Vec::new();
                return Ok(false);
            }
            self.scene_passes.pop();
            return Ok(false);
        }
        if self.shared_borders.pop().is_some() { return Ok(false); }
        if !self.shared_borders.terminal_is_empty() {
            let bytes = self.shared_borders.next_release_allocation_bytes()?;
            self.shared_borders.release_empty_page_leased(bytes)?;
            return Ok(false);
        }
        if let Some(layer) = self.layers.last_mut() {
            if let Some(clip) = layer.clip.as_mut() {
                if clip.scissors.pop().is_some() {
                    return Ok(false);
                }
                if !clip.scissors.terminal_is_empty() {
                    let bytes = clip.scissors.next_release_allocation_bytes()?;
                    clip.scissors.release_empty_page_leased(bytes)?;
                    return Ok(false);
                }
                layer.clip = None;
                return Ok(false);
            }
            if let Some((key, _)) = layer.raster_instances.last_mut() {
                if key.pop().is_some() {
                    return Ok(false);
                }
                if key.capacity() > 0 {
                    *key = String::new();
                    return Ok(false);
                }
                layer.raster_instances.pop();
                return Ok(false);
            }
            if let Some((key, _)) = layer.overlay_raster_instances.last_mut() {
                if key.pop().is_some() {
                    return Ok(false);
                }
                if key.capacity() > 0 {
                    *key = String::new();
                    return Ok(false);
                }
                layer.overlay_raster_instances.pop();
                return Ok(false);
            }
            if layer.overlay_vector_vertices.pop().is_some() || layer.overlay_ui_instances.pop().is_some() || layer.vector_vertices.pop().is_some() || layer.ui_instances.pop().is_some() {
                return Ok(false);
            }
            if layer.overlay_vector_vertices.capacity() > 0 {
                layer.overlay_vector_vertices = Vec::new();
                return Ok(false);
            }
            if layer.overlay_ui_instances.capacity() > 0 {
                layer.overlay_ui_instances = Vec::new();
                return Ok(false);
            }
            if layer.overlay_raster_instances.capacity() > 0 {
                layer.overlay_raster_instances = Vec::new();
                return Ok(false);
            }
            if layer.vector_vertices.capacity() > 0 {
                layer.vector_vertices = Vec::new();
                return Ok(false);
            }
            if layer.ui_instances.capacity() > 0 {
                layer.ui_instances = Vec::new();
                return Ok(false);
            }
            if layer.raster_instances.capacity() > 0 {
                layer.raster_instances = Vec::new();
                return Ok(false);
            }
            self.layers.pop();
            return Ok(false);
        }
        if self.glass_regions.pop().is_some() || self.scissor_stack.pop().is_some() || self.glass_content_stack.pop().is_some() {
            return Ok(false);
        }
        if let Some(clip) = self.clip_stack.last_mut() {
            if clip.scissors.pop().is_some() {
                return Ok(false);
            }
            if !clip.scissors.terminal_is_empty() {
                let bytes = clip.scissors.next_release_allocation_bytes()?;
                clip.scissors.release_empty_page_leased(bytes)?;
                return Ok(false);
            }
            self.clip_stack.pop();
            return Ok(false);
        }
        if !self.scene_passes.terminal_is_empty() {
            let bytes = self.scene_passes.next_release_allocation_bytes()?;
            self.scene_passes.release_empty_page_leased(bytes)?;
            return Ok(false);
        }
        if !self.layers.terminal_is_empty() {
            let bytes = self.layers.next_release_allocation_bytes()?;
            self.layers.release_empty_page_leased(bytes)?;
            return Ok(false);
        }
        if !self.glass_regions.terminal_is_empty() {
            let bytes = self.glass_regions.next_release_allocation_bytes()?;
            self.glass_regions.release_empty_page_leased(bytes)?;
            return Ok(false);
        }
        if self.scissor_stack.capacity() > 0 {
            self.scissor_stack = Vec::new();
            return Ok(false);
        }
        if self.clip_stack.capacity() > 0 {
            self.clip_stack = Vec::new();
            return Ok(false);
        }
        if self.glass_content_stack.capacity() > 0 {
            self.glass_content_stack = Vec::new();
            return Ok(false);
        }
        if self.retained_output.take().is_some() {
            return Ok(false);
        }
        if self.prepared_items != 0 {
            self.prepared_items = 0;
            return Ok(false);
        }
        if self.prepared_bytes != 0 {
            self.prepared_bytes = 0;
            return Ok(false);
        }
        if self.border_retained_bytes != 0 { self.border_retained_bytes = 0; return Ok(false); }
        Ok(true)
    }

    /// ♻️ Retires one retained item; a failed page release is a stuck list, so the unchecked form is for tests and drops only.
    pub fn retire_step(&mut self) -> bool { self.retire_step_checked().unwrap_or(false) }

    pub fn retirement_is_empty(&self) -> bool {
        self.scene_passes.is_empty()
            && self.scene_passes.terminal_is_empty()
            && self.shared_borders.terminal_is_empty()
            && self.layers.is_empty()
            && self.layers.terminal_is_empty()
            && self.glass_regions.is_empty()
            && self.glass_regions.terminal_is_empty()
            && self.scissor_stack.is_empty()
            && self.scissor_stack.capacity() == 0
            && self.clip_stack.is_empty()
            && self.clip_stack.capacity() == 0
            && self.glass_content_stack.is_empty()
            && self.glass_content_stack.capacity() == 0
            && self.retained_output.is_none()
            && self.prepared_items == 0
            && self.prepared_bytes == 0
            && self.retained_append.is_none()
            && self.border_finalize.is_none()
            && self.border_retained_bytes == 0
    }

}
// #endregion draw_types
