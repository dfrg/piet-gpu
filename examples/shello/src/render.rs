use piet_gpu::{test_scenes, PietGpuRenderContext, Renderer};
use piet_gpu_hal::{
    CmdBuf, Error, ImageLayout, Instance, QueryPool, Semaphore, Session, SubmittedCmdBuf, Surface,
    Swapchain,
};
use piet_scene::resource::ResourceContext;
use piet_scene::scene::Scene;

pub const NUM_FRAMES: usize = 2;

pub struct PgpuState {
    instance: Instance,
    surface: Option<Surface>,
    swapchain: Swapchain,
    session: Session,
    present_semaphores: Vec<Semaphore>,
    query_pools: Vec<QueryPool>,
    cmd_bufs: [Option<CmdBuf>; NUM_FRAMES],
    submitted: [Option<SubmittedCmdBuf>; NUM_FRAMES],
    renderer: Renderer,
    current_frame: usize,
}

impl PgpuState {
    pub fn new(
        window: &dyn raw_window_handle::HasRawWindowHandle,
        width: usize,
        height: usize,
    ) -> Result<Self, Error> {
        println!("size: {}, {}", width, height);
        let (instance, surface) = Instance::new(Some(window), Default::default())?;
        unsafe {
            let device = instance.device(surface.as_ref())?;
            let swapchain =
                instance.swapchain(width, height, &device, surface.as_ref().unwrap())?;
            let session = Session::new(device);
            let present_semaphores = (0..NUM_FRAMES)
                .map(|_| session.create_semaphore())
                .collect::<Result<Vec<_>, Error>>()?;
            let query_pools = (0..NUM_FRAMES)
                .map(|_| session.create_query_pool(Renderer::QUERY_POOL_SIZE))
                .collect::<Result<Vec<_>, Error>>()?;
            let cmd_bufs: [Option<CmdBuf>; NUM_FRAMES] = Default::default();
            let submitted: [Option<SubmittedCmdBuf>; NUM_FRAMES] = Default::default();
            let renderer = Renderer::new(&session, width, height, NUM_FRAMES)?;
            let current_frame = 0;
            Ok(Self {
                instance,
                surface,
                swapchain,
                session,
                present_semaphores,
                query_pools,
                cmd_bufs,
                submitted,
                renderer,
                current_frame,
            })
        }
    }

    pub fn frame_index(&self) -> usize {
        self.current_frame % NUM_FRAMES
    }

    pub fn pre_render(&mut self) -> Option<Vec<f64>> {
        let frame_idx = self.current_frame % NUM_FRAMES;
        if let Some(submitted) = self.submitted[frame_idx].take() {
            self.cmd_bufs[frame_idx] = submitted.wait().unwrap();
            Some(unsafe {
                self.session
                    .fetch_query_pool(&self.query_pools[frame_idx])
                    .unwrap()
            })
        } else {
            None
        }
    }

    pub fn render(&mut self, scene: &Scene, rcx: &ResourceContext) {
        let frame_idx = self.current_frame % NUM_FRAMES;
        let data = scene.data();
        let scene_ref = piet_gpu::EncodedSceneRef {
            transform_stream: &data.transform_stream,
            tag_stream: &data.tag_stream,
            pathseg_stream: &data.pathseg_stream,
            linewidth_stream: &data.linewidth_stream,
            drawtag_stream: &data.drawtag_stream,
            drawdata_stream: &data.drawdata_stream,
            n_path: data.n_path,
            n_pathseg: data.n_pathseg,
            n_clip: data.n_clip,
            ramp_data: rcx.ramp_data(),
        };
        self.renderer.upload_scene(&scene_ref, frame_idx).unwrap();
        unsafe {
            let (image_idx, acquisition_semaphore) = self.swapchain.next().unwrap();
            let swap_image = self.swapchain.image(image_idx);
            let query_pool = &self.query_pools[frame_idx];
            let mut cmd_buf = self.cmd_bufs[frame_idx]
                .take()
                .unwrap_or_else(|| self.session.cmd_buf().unwrap());
            cmd_buf.begin();
            self.renderer.record(&mut cmd_buf, &query_pool, frame_idx);

            // Image -> Swapchain
            cmd_buf.image_barrier(&swap_image, ImageLayout::Undefined, ImageLayout::BlitDst);
            cmd_buf.blit_image(&self.renderer.image_dev, &swap_image);
            cmd_buf.image_barrier(&swap_image, ImageLayout::BlitDst, ImageLayout::Present);
            cmd_buf.finish();

            self.submitted[frame_idx] = Some(
                self.session
                    .run_cmd_buf(
                        cmd_buf,
                        &[&acquisition_semaphore],
                        &[&self.present_semaphores[frame_idx]],
                    )
                    .unwrap(),
            );

            self.swapchain
                .present(image_idx, &[&self.present_semaphores[frame_idx]])
                .unwrap();
        }

        self.current_frame += 1;
    }
}
