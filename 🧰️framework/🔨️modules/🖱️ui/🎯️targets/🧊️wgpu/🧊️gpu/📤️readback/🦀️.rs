//! 📤️ Bounded RGBA readback from an accepted prepared render target.

use std::sync::{Arc, Mutex};
use semio_framework_pixels::editing::{MAX_IMAGE_PIXELS, MAX_IMAGE_SIDE};

#[derive(Clone, Copy, Debug)]
pub(super) struct PreparedReadbackLayout {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) padded_bytes_per_row: u32,
    pub(super) byte_length: u64,
    rgba_length: usize,
    bgra: bool,
}

impl PreparedReadbackLayout {
    pub(super) fn new(width: u32, height: u32, bgra: bool) -> Result<Self, String> {
        if width == 0 || height == 0 || width > MAX_IMAGE_SIDE || height > MAX_IMAGE_SIDE || u64::from(width) * u64::from(height) > MAX_IMAGE_PIXELS as u64 {
            return Err("prepared readback dimensions exceed the image budget".into());
        }
        let padded_bytes_per_row = (width * 4).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        Ok(Self { width, height, padded_bytes_per_row, byte_length: u64::from(padded_bytes_per_row) * u64::from(height), rgba_length: width as usize * height as usize * 4, bgra })
    }

    fn append_rgba(&self, padded: &[u8], rgba: &mut Vec<u8>, budget: usize) -> Result<(), String> {
        if padded.len() as u64 != self.byte_length || rgba.len() > self.rgba_length || rgba.len() % 4 != 0 || budget < 4 {
            return Err("prepared readback byte range is invalid".into());
        }
        let end = self.rgba_length.min(rgba.len().saturating_add(budget / 4 * 4));
        while rgba.len() < end {
            let pixel = rgba.len() / 4;
            let offset = pixel / self.width as usize * self.padded_bytes_per_row as usize + pixel % self.width as usize * 4;
            let source = &padded[offset..offset + 4];
            if self.bgra {
                rgba.extend_from_slice(&[source[2], source[1], source[0], source[3]]);
            } else {
                rgba.extend_from_slice(source);
            }
        }
        Ok(())
    }
}

/// 📨️ Owns an asynchronous map and publishes only a complete normalized RGBA image.
pub struct PreparedGpuReadback {
    layout: PreparedReadbackLayout,
    buffer: wgpu::Buffer,
    mapping: Arc<Mutex<Option<Result<(), String>>>>,
    rgba: Vec<u8>,
    phase: u8,
}

impl PreparedGpuReadback {
    pub(super) fn begin(device: &wgpu::Device, queue: &wgpu::Queue, composite: &crate::wgpu::draw::PreparedCompositeTarget, layout: PreparedReadbackLayout) -> Result<Self, String> {
        if layout.byte_length > device.limits().max_buffer_size {
            return Err("prepared readback exceeds the device buffer budget".into());
        }
        let mut rgba = Vec::new();
        rgba.try_reserve_exact(layout.rgba_length).map_err(|_| "prepared readback allocation was refused".to_string())?;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor { label: Some("prepared_rgba_readback"), size: layout.byte_length, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_rgba_copy") });
        composite.copy_to_readback(&mut encoder, &buffer, layout.padded_bytes_per_row);
        queue.submit(Some(encoder.finish()));
        let mapping = Arc::new(Mutex::new(None));
        let completion = mapping.clone();
        buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| {
            if let Ok(mut state) = completion.lock() {
                *state = Some(result.map_err(|error| error.to_string()));
            }
        });
        Ok(Self { layout, buffer, mapping, rgba, phase: 0 })
    }

    pub(super) fn advance(&mut self, device: &wgpu::Device) -> Result<bool, String> {
        if self.phase >= 2 {
            return if self.phase == 2 { Ok(true) } else { Err("prepared readback is closed".into()) };
        }
        if self.phase == 0 {
            device.poll(wgpu::PollType::Poll).map_err(|error| error.to_string())?;
            let result = self.mapping.lock().map_err(|_| "prepared readback completion was poisoned".to_string())?.take();
            let Some(result) = result else { return Ok(false) };
            if let Err(error) = result {
                self.cancel();
                return Err(error);
            }
            self.phase = 1;
            return Ok(false);
        }
        {
            let mapped = self.buffer.slice(..).get_mapped_range();
            self.layout.append_rgba(&mapped, &mut self.rgba, 4096)?;
        }
        if self.rgba.len() == self.layout.rgba_length {
            self.buffer.unmap();
            self.buffer.destroy();
            self.phase = 2;
        }
        Ok(self.phase == 2)
    }

    /// 📊️ Unchanged mapping polls retain the same progress witness.
    pub fn progress(&self) -> (u8, usize, usize) {
        (self.phase, self.rgba.len(), self.layout.rgba_length)
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.layout.width, self.layout.height)
    }

    pub fn take_rgba(&mut self) -> Option<Vec<u8>> {
        if self.phase != 2 {
            return None;
        }
        self.phase = 4;
        Some(std::mem::take(&mut self.rgba))
    }

    pub fn cancel(&mut self) {
        if self.phase < 2 {
            self.buffer.unmap();
            self.buffer.destroy();
        }
        self.rgba = Vec::new();
        self.phase = 3;
    }
}

impl Drop for PreparedGpuReadback {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/📤️prepared-readback/🦀️.rs"]
mod tests;
