#![windows_subsystem = "windows"]

use native_windows_derive::NwgUi;
use native_windows_gui as nwg;
use nwg::NativeUi;
use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control(title: "Complossy", flags: "WINDOW|VISIBLE", size: (500, 150), center: true)]
    #[nwg_events(OnWindowClose: [App::exit])]
    window: nwg::Window,

    #[nwg_control(text: "No file...")]
    input_path: nwg::Label,

    #[nwg_control(text: "Select File")]
    #[nwg_events(OnButtonClick: [App::select_file])]
    browse_button: nwg::Button,

    #[nwg_control(text: "Compress")]
    #[nwg_events(OnButtonClick: [App::convert])]
    convert_button: nwg::Button,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,
}

impl App {

    fn select_file(&self) {
        let mut dialog = nwg::FileDialog::default();

        nwg::FileDialog::builder()
            .action(nwg::FileDialogAction::Open)
            .title("Select an input file")
            .build(&mut dialog)
            .unwrap();

        if dialog.run(Some(&self.window)) {
            if let Ok(path) = dialog.get_selected_item() {
                self.input_path
                    .set_text(path.to_string_lossy().as_ref());
            }
        }

    }

    fn convert(&self) {

        println!("Converting");
        // "ffmpeg -i input.png -c:v mjpeg -q:v 1 output.jpg"
        let file = self.input_path.text();

        let path = std::path::PathBuf::from(&file);

        let output = format!("{}\\{}-complossy-compressed.jpg", path.parent().unwrap().to_str().unwrap(), path.file_prefix().unwrap().to_str().unwrap());

        let binding : &mut Command = &mut Command::new("ffmpeg");

        let runner = binding.arg("-i")
            .arg(file)
            .arg("-c:v")
            .arg("mjpeg")
            .arg("-q:v")
            .arg("1")
            .arg(output)
            .creation_flags(CREATE_NO_WINDOW);

        let running = runner.spawn();

        let _status = running.expect("this isn't really running :(").wait();

        self.input_path.set_text("Compression successfull!");
    }

    fn build_layout(&self) {
        nwg::GridLayout::builder()
            .parent(&self.window)
            .spacing(8)
            .margin([10, 10, 10, 10])
            .child(0, 0, &self.browse_button)
            .child(1, 0, &self.input_path)
            .child(0, 1, &self.convert_button)
            .build(&self.layout)
            .unwrap();
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().unwrap();

    nwg::Font::set_global_family("Segoe UI").unwrap();

    let app = App::build_ui(Default::default()).unwrap();
    app.build_layout();

    nwg::dispatch_thread_events();
}
