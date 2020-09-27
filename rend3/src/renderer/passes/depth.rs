use crate::renderer::util;
use crate::{
    renderer::{
        material::MAX_MATERIALS,
        shaders::{ShaderArguments, ShaderManager},
        INTERNAL_RENDERBUFFER_DEPTH_FORMAT, INTERNAL_RENDERBUFFER_FORMAT,
    },
    TLS,
};
use shaderc::ShaderKind;
use std::{cell::RefCell, future::Future, sync::Arc};
use switchyard::Switchyard;
use tracing_futures::Instrument;
use wgpu::{
    BindGroup, BindGroupLayout, BlendDescriptor, Buffer, ColorStateDescriptor, ColorWrite, CompareFunction, CullMode,
    DepthStencilStateDescriptor, Device, FrontFace, IndexFormat, PipelineLayout, PipelineLayoutDescriptor,
    PrimitiveTopology, ProgrammableStageDescriptor, RasterizationStateDescriptor, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, ShaderModule, StencilStateDescriptor, VertexStateDescriptor,
};

pub struct DepthPass {
    pipeline: RenderPipeline,
    vertex: Arc<ShaderModule>,
    fragment: Arc<ShaderModule>,
}
impl DepthPass {
    pub fn new<'a, TLD>(
        device: &'a Arc<Device>,
        yard: &Switchyard<RefCell<TLD>>,
        shader_manager: &Arc<ShaderManager>,
        input_bgl: &BindGroupLayout,
        output_noindirect_bgl: &BindGroupLayout,
        material_bgl: &BindGroupLayout,
        texture_bgl: &BindGroupLayout,
        uniform_bgl: &BindGroupLayout,
    ) -> impl Future<Output = Self> + 'a
    where
        TLD: AsMut<TLS> + 'static,
    {
        let new_span = tracing::warn_span!("Creating DepthPass");
        let new_span_guard = new_span.enter();

        let vertex = shader_manager.compile_shader(
            &yard,
            Arc::clone(device),
            ShaderArguments {
                file: String::from("rend3/shaders/depth.vert"),
                defines: vec![],
                kind: ShaderKind::Vertex,
                debug: cfg!(debug_assertions),
            },
        );

        let fragment = shader_manager.compile_shader(
            &yard,
            Arc::clone(device),
            ShaderArguments {
                file: String::from("rend3/shaders/depth.frag"),
                defines: vec![(String::from("MATERIAL_COUNT"), Some(MAX_MATERIALS.to_string()))],
                kind: ShaderKind::Fragment,
                debug: cfg!(debug_assertions),
            },
        );

        let layout = util::create_render_pipeline_layout(
            device,
            input_bgl,
            output_noindirect_bgl,
            material_bgl,
            texture_bgl,
            uniform_bgl,
            util::RenderPipelineType::Depth,
        );

        drop(new_span_guard);

        async move {
            let vertex = vertex.await.unwrap();
            let fragment = fragment.await.unwrap();

            let pipeline =
                util::create_render_pipeline(device, &layout, &vertex, &fragment, util::RenderPipelineType::Depth);

            Self {
                pipeline,
                vertex,
                fragment,
            }
        }
        .instrument(new_span)
    }

    pub fn update_pipeline(
        &mut self,
        device: &Device,
        input_bgl: &BindGroupLayout,
        output_noindirect_bgl: &BindGroupLayout,
        material_bgl: &BindGroupLayout,
        texture_bgl: &BindGroupLayout,
        uniform_bgl: &BindGroupLayout,
    ) {
        span_transfer!(_ -> update_pipeline_span, INFO, "Depth Pass Update Pipeline");
        let layout = util::create_render_pipeline_layout(
            device,
            input_bgl,
            output_noindirect_bgl,
            material_bgl,
            texture_bgl,
            uniform_bgl,
            util::RenderPipelineType::Depth,
        );
        let pipeline = util::create_render_pipeline(
            device,
            &layout,
            &self.vertex,
            &self.fragment,
            util::RenderPipelineType::Depth,
        );
        self.pipeline = pipeline;
    }

    pub fn run<'a>(
        &'a self,
        rpass: &mut RenderPass<'a>,
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        indirect_buffer: &'a Buffer,
        count_buffer: &'a Buffer,
        input_bg: &'a BindGroup,
        output_noindirect_bg: &'a BindGroup,
        material_bg: &'a BindGroup,
        texture_bg: &'a BindGroup,
        uniform_bg: &'a BindGroup,
        object_count: u32,
    ) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        rpass.set_index_buffer(index_buffer.slice(..));
        rpass.set_bind_group(0, &input_bg, &[]);
        rpass.set_bind_group(1, &output_noindirect_bg, &[]);
        rpass.set_bind_group(2, &material_bg, &[]);
        rpass.set_bind_group(3, &texture_bg, &[]);
        rpass.set_bind_group(4, &uniform_bg, &[]);
        rpass.multi_draw_indexed_indirect_count(indirect_buffer, 0, count_buffer, 0, object_count);
    }
}
