mod render;
mod test_scenes;

use druid_shell::kurbo::{Rect, Size};
use druid_shell::{
    Application, Cursor, FileDialogToken, FileInfo, KeyEvent, MouseEvent, Region, TimerToken,
    WinHandler, WindowHandle, Scalable,
};
use piet_scene::scene::Scene;
use piet_scene::resource::ResourceContext;
use std::any::Any;

const WIDTH: usize = 2048;
const HEIGHT: usize = 1536;

fn main() {
    let app = Application::new().unwrap();
    let mut window_builder = druid_shell::WindowBuilder::new(app.clone());
    window_builder.resizable(false);
    window_builder.set_size((WIDTH as f64 / 2., HEIGHT as f64 / 2.).into());
    window_builder.set_handler(Box::new(WindowState::default()));
    let window_handle = window_builder.build().unwrap();
    window_handle.show();
    app.run(None);
}

#[derive(Default)]
struct WindowState {
    handle: WindowHandle,
    pgpu_state: Option<render::PgpuState>,
    scene: Scene,
    resource_context: ResourceContext,
}

impl WinHandler for WindowState {
    fn connect(&mut self, handle: &WindowHandle) {
        let scale = handle.get_scale().unwrap();
        let insets = handle.content_insets().to_px(scale);
        let mut size = handle.get_size().to_px(scale);
        size.width -= insets.x_value();
        size.height -= insets.y_value();
        println!("size: {:?}", size);
        self.pgpu_state = Some(render::PgpuState::new(handle, size.width as usize, size.height as usize).unwrap());
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, _: &Region) {
        // let rect = self.size.to_rect();
        // piet.fill(rect, &BG_COLOR);
        // piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
        if let Some(pgpu_state) = self.pgpu_state.as_mut() {
            if let Some(_timestamps) = pgpu_state.pre_render() {

            }
            self.resource_context.advance();
            let arg = pgpu_state.frame_index();
            test_scenes::fill(&mut self.scene, &mut self.resource_context, 0, arg);
            pgpu_state.render(&self.scene, &self.resource_context);
        }
        self.handle.invalidate();
    }

    fn command(&mut self, id: u32) {}

    fn open_file(&mut self, _token: FileDialogToken, file_info: Option<FileInfo>) {
        println!("open file result: {:?}", file_info);
    }

    fn save_as(&mut self, _token: FileDialogToken, file: Option<FileInfo>) {
        println!("save file result: {:?}", file);
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        println!("mouse_wheel {:?}", event);
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        //println!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        println!("mouse_down {:?}", event);
        self.handle.invalidate();
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        println!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken) {
        println!("timer fired: {:?}", id);
    }

    fn size(&mut self, size: Size) {
        //self.size = size;
    }

    fn got_focus(&mut self) {
        println!("Got focus");
    }

    fn lost_focus(&mut self) {
        println!("Lost focus");
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

