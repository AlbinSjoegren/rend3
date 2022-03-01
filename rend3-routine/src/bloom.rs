use rend3::util::mipmap::MipmapGenerator;

pub fn bloom(
        renderer: &Arc<rend3::Renderer>,
        width: u32, 
        height: u32, 
        surface_texture: wgpu::SurfaceTexture, 
        steps: u32,
) -> wgpu::Texture {
        let device = &renderer.device;
        let queue = &renderer.queue;

        let texture = surface_texture.texture;

        let generator = MipmapGenerator::new(device, rend3::types::TextureFormat::Rg32Float);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("texture_buffer_copy_encoder"),
        });

        let texture_size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
        };

        generator.generate_mipmaps(device, &encoder, &texture, &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 9, 
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rg32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("bloom_texture"),
        });


        let downscaliong_vec = vec![];

        let steps = 8;

        for i in 1..steps {
                let lable = format!("{}", i);
                let texture_downscale = rend3::types::TextureFromTexture {
                        lable: Some(lable),
                        src: texture,
                        start_mip: i,
                        mip_count: Some(9 - i)
                };
                downscaliong_vec.push(texture_downscale)
        }
        

        return texture;
}
