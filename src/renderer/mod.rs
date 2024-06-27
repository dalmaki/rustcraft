use std::sync::Arc;

use winit::window::Window;

struct GraphicsContext<'w> {
    surface: wgpu::Surface<'w>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    win_size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
}

impl<'w> GraphicsContext<'w> {
    async fn new(window: Arc<Window>) -> GraphicsContext<'w> {
        let win_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: win_size.width,
            height: win_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window: window.clone(),
            surface,
            device,
            queue,
            config,
            win_size,
        }
    }
}

pub struct Renderer<'w> {
    context: GraphicsContext<'w>,
}

impl<'w> Renderer<'w> {
    pub async fn new(window: Arc<Window>) -> Renderer<'w> {
        let context = GraphicsContext::new(window.clone()).await;
        Self { context }
    }
}
