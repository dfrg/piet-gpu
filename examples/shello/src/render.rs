use piet_gpu::{test_scenes, PietGpuRenderContext, Renderer};
use piet_gpu_hal::{
    CmdBuf, Error, ImageLayout, Instance, QueryPool, Semaphore, Session, SubmittedCmdBuf, Surface,
    Swapchain,
};

pub const NUM_FRAMES: usize = 2;

struct PgpuState {
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
            let mut cmd_bufs: [Option<CmdBuf>; NUM_FRAMES] = Default::default();
            let mut submitted: [Option<SubmittedCmdBuf>; NUM_FRAMES] = Default::default();
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
}
