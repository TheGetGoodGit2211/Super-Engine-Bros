use std::collections::HashMap;
use std::path::Path;

use image::GenericImageView;
use wgpu::Label;

use crate::deserialize;
use crate::sprite;
use crate::sprite::Sprite;

struct Atlas
{
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub sprites: HashMap<String, sprite::Sprite>,
    pub size: [u16; 2]
}

impl Atlas
{
    pub fn from_atlas_data(atlas_data: &deserialize::AtlasData,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        ) -> Self
    {
        let (texture, view) = texture_from_path(device, queue, None).expect("Failed to load a texture from the given path");

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor
        {
            label: None,
            layout,
            entries: &[
                wgpu::BindGroupEntry
                {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry
                {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler)
                }
            ]
        });

        let atlas_frames = &atlas_data.frames;
        let atlas_size = &atlas_data.meta.size;

        let mut atlas = Atlas {
            texture,
            view,
            bind_group,
            sprites: HashMap::with_capacity(atlas_frames.len()),
            size: [atlas_size.w, atlas_size.h]
        };

        atlas.sprites = atlas.compute_sprites(atlas_data);

        atlas
    }

    pub fn compute_sprites(&mut self, atlas_data: &deserialize::AtlasData) -> HashMap<String, Sprite>
    {
       let frames = &atlas_data.frames; 

       frames
           .iter()
           .map(|(name, sprite_data)| {
               let sprite = self.sprite_from_frame(&sprite_data.frame);
               (name.clone(), sprite)
           })
       .collect()
    }

    fn sprite_from_frame(&self, frame: &deserialize::Rect) -> Sprite
    {
        let [screen_width, screen_height] = self.size;

        let uv_lower: [f32; 2] = [
            f32::from(frame.x / screen_width),
            f32::from(frame.y / screen_height)
        ];

        let uv_higher = [
            f32::from((frame.x + frame.w) / screen_width),
            f32::from((frame.y + frame.h) / screen_height)
        ];

        Sprite
        {
            uv_lower,
            uv_higher,
            px_size: [frame.w, frame.h]
        }
    }
}

fn texture_from_path(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    // _path: impl AsRef<Path>,
    label: Option<&str>)
    -> Result<(wgpu::Texture, wgpu::TextureView), image::ImageError>
{
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test_tiles/output/atlas.png");
    let img = image::open(path)?;

    let (width, height) = img.dimensions();

    let rgba_img = img.to_rgba8();

    let size = wgpu::Extent3d
    {
        width,
        height,
        depth_or_array_layers: 1
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor
    {
        label,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[]
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo
        {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All
        },
        &rgba_img,
        wgpu::TexelCopyBufferLayout
        {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height)
        },
        size
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    Ok((texture, view))
}
