//! @emoji 🌆️ The offscreen scene color target: a `SCENE_MIP_LEVELS`-level mip chain (content at mip 0,
//! progressively box-blurred into higher mips for the glass backdrop) plus a same-shaped `blur_scratch`
//! texture the downsample pass reads from (can't sample and render into the same mip simultaneously).
//! Ported from `🎯️targets/🧊️wgpu/🦀️draw.rs`'s `SceneColorTarget`.

//#region 🔖️SceneTarget

/// 🌫️ Matches the reference implementation's `SCENE_MIP_LEVELS` — five box-downsample steps.
pub(crate) const SCENE_MIP_LEVELS: u32 = 5;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn mip_extent(width: u32, height: u32, level: u32) -> (u32, u32) {
    ((width >> level).max(1), (height >> level).max(1))
}

pub(crate) struct SceneColorTarget {
    texture: wgpu::Texture,
    blur_scratch: wgpu::Texture,
    blur_scratch_mip_views: Vec<wgpu::TextureView>,
    sample_view: wgpu::TextureView,
    mip_views: Vec<wgpu::TextureView>,
    sampler: wgpu::Sampler,
    width: u32,
    height: u32,
}

impl SceneColorTarget {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn new(device: &wgpu::Device, width: u32, height: u32, format: wgpu::TextureFormat) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_color"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[format],
        });
        let blur_scratch = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_blur_scratch"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[format],
        });
        let blur_scratch_mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                blur_scratch.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("scene_blur_scratch_mip_{level}")),
                    format: Some(format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sample_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("scene_color_sample"),
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_mip_level: 0,
            mip_level_count: Some(SCENE_MIP_LEVELS),
            ..Default::default()
        });
        let mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                texture.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("scene_color_mip_{level}")),
                    format: Some(format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sampler =
            device.create_sampler(&wgpu::SamplerDescriptor { label: Some("scene_color_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default() });
        Self { texture, blur_scratch, blur_scratch_mip_views, sample_view, mip_views, sampler, width, height }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn matches(&self, width: u32, height: u32) -> bool {
        self.width == width.max(1) && self.height == height.max(1)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn mip_view(&self, level: u32) -> &wgpu::TextureView {
        &self.mip_views[level as usize]
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn sample_view(&self) -> &wgpu::TextureView {
        &self.sample_view
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn blur_scratch_mip_view(&self, level: u32) -> &wgpu::TextureView {
        &self.blur_scratch_mip_views[level as usize]
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn copy_mip_to_blur_scratch(&self, encoder: &mut wgpu::CommandEncoder, src_mip: u32) {
        let (width, height) = mip_extent(self.width, self.height, src_mip);
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.texture, mip_level: src_mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyTextureInfo { texture: &self.blur_scratch, mip_level: src_mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }
}

//#endregion 🔖️SceneTarget

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mip_extent_halves_each_level_and_floors_at_one() {
        assert_eq!(mip_extent(800, 600, 0), (800, 600));
        assert_eq!(mip_extent(800, 600, 1), (400, 300));
        assert_eq!(mip_extent(800, 600, 2), (200, 150));
        assert_eq!(mip_extent(3, 3, 3), (1, 1));
        assert_eq!(mip_extent(3, 3, 10), (1, 1));
    }

    #[test]
    fn mip_extent_never_reaches_zero() {
        for level in 0..SCENE_MIP_LEVELS {
            let (width, height) = mip_extent(1, 1, level);
            assert!(width >= 1 && height >= 1);
        }
    }
}

//#endregion Tests
