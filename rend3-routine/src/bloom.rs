pub fn bloom(
        renderer: &Arc<rend3::Renderer>, 
        width: u32, 
        height: u32, 
        surface_texture: wgpu::SurfaceTexture, 
) -> wgpu::Texture {
        let device = &renderer.device;
        let queue = &renderer.queue;

        let texture_size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
        };

        let texture = surface_texture.texture;

        queue.write_texture(wgpu::ImageCopyTextureBase {
                        texture: &texture,
                        mip_level: 9,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                }, 
                texture_size,        
        );

        return texture;
}
