
pub struct Renderer<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a winit::window::Window,
}

impl Renderer<'_> {
    pub async fn new<'a>(window: &'a winit::window::Window) -> Renderer<'a> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let surface =  instance.create_surface(window).expect("Failed to create surface");
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: None,
            memory_hints: wgpu::MemoryHints::Performance,
        }, None).await.unwrap();
        Renderer {
            size,
            device,
            queue,
            surface,
            window,
        }
    }

    pub fn render(&self, elements: &[crate::element::Element]) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
