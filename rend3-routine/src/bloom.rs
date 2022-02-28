use wgpu::SamplerDescriptor;

pub fn bloom(renderer: &Arc<rend3::Renderer>, width: u32, height: u32, surface_texture: wgpu::SurfaceTexture, texture_view_descriptor: wgpu::TextureViewDescriptor threshold: f32, size: f32) -> wgpu::SurfaceTexture {
        let device = &renderer.device;
        let queue = &renderer.queue;
 
        let format = wgpu::TextureFormat::Rgba32Float;

        let texture = surface_texture.texture;

        let texture_2 = create_texture(renderer, format, Some("2"), ((width as f32) / 2.), ((height as f32) / 2.));
        queue.write_texture(texture);

        let texture_3 = create_texture(renderer, format, Some("4"), ((width as f32) / 4.), ((height as f32) / 4.));
        queue.write_texture(texture_2);

        let texture_4 = create_texture(renderer, format, Some("8"), ((width as f32) / 8.), ((height as f32) / 8.));
        queue.write_texture(texture_3);

        let texture_5 = create_texture(renderer, format, Some("16"), ((width as f32) / 16.), ((height as f32) / 16.));
        queue.write_texture(texture_4;

        let texture_6 = create_texture(renderer, format, Some("32"), ((width as f32) / 32.), ((height as f32) / 32.));
        queue.write_texture(texture_5);

        let texture_7 = create_texture(renderer, format, Some("64"), ((width as f32) / 64.), ((height as f32) / 64.));
        queue.write_texture(texture_6);

        let texture_8 = create_texture(renderer, format, Some("128"), ((width as f32) / 128.), ((height as f32) / 128.));
        queue.write_texture(texture_7);

        let texture_9 = create_texture(renderer, format, Some("256"), ((width as f32) / 256.), ((height as f32) / 256.));
        queue.write_texture(texture_8);


}

fn create_texture(renderer: &Arc<rend3::Renderer>, format: wgpu::TextureFormat, label: Option<&str>, width: f32, height: f32) -> wgpu::Texture {
        
        let texture_size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
        };
    
        let image_texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label,
        });
}
