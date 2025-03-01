mod font_installer;
mod remixicon;

use iced::widget::{
    button, column, container, horizontal_space, progress_bar, row, text, Container,
};
use iced::Alignment::End;
use iced::{Center, Fill, Task, Theme};
use remixicon::{remix_init};
use std::io;
use crate::remixicon::remix_icon;

fn theme(state: &Controller) -> Theme {
    print!("{}", state.value.to_string());
    Theme::CatppuccinMocha
}

#[tokio::main]
pub async fn main() -> iced::Result {
    // install icon font
    let font_path = "assets/fonts/remixicon.ttf"; // Path to the font file
    match font_installer::install_font(font_path) {
        Ok(_) => println!("Font installed successfully!"),
        Err(e) => eprintln!("Error installing font: {}", e),
    }

    // init icon font
    remix_init();

    iced::application("FFmpeg owl ui", Controller::update, Controller::view)
        .theme(theme)
        .run_with(Controller::new)
}

#[derive(Default)]
struct Controller {
    value: i64,
    source: String,
    dest: String,
    can_logo: bool,
    logo_input: String,
    progress: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Mute,
    Rotate,
    ReplaceSound,
    Crop,
    Compress,
    Resize,
    Subtitle,
    Watermark,
    DoIt,
    SelectInputVideo,
    InputVideoOpened(Result<String, String>),
    SelectOutputVideo,
    OutputVideoOpened(Result<String, String>),
    SelectLogo,
    LogoOpened(Result<String, String>),
}

impl Controller {
    fn new() -> (Self, Task<Message>) {
        let start_message = { Message::Start };
        (Self::default(), Task::done(start_message))
    }

    fn can_logo(&self) -> bool {
        self.can_logo
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => {
                self.source = "-".to_string();
                self.dest = "-".to_string();
                self.logo_input = "-".to_string();
                self.can_logo = false;
                Task::none()
            }
            Message::Mute => Task::none(),
            Message::Rotate => Task::none(),
            Message::ReplaceSound => Task::none(),
            Message::Crop => Task::none(),
            Message::Compress => Task::none(),
            Message::Resize => Task::none(),
            Message::Subtitle => Task::none(),
            Message::Watermark => {
                self.can_logo = true;
                Task::none()
            }
            Message::DoIt => {
                self.progress = 15.7;
                Task::none()
            },

            Message::SelectInputVideo => {
                println!("Select input video");
                // tokio::spawn(async {
                //     match open_file().await {
                //         Ok(file_path) => {
                //             print!("{}", file_path);
                //         }
                //         Err(e) => {
                //             eprintln!("Error selecting file: {}", e); // Handle the error (optional)
                //             // Do nothing if the file selection failed
                //         }
                //     }
                // });

                Task::perform(open_file(&["mp4", "mkv"]), Message::InputVideoOpened)
            }
            Message::InputVideoOpened(result) => {
                match result {
                    Ok(file_path) => {
                        self.source = file_path;
                    }
                    Err(e) => {
                        eprintln!("Error selecting file: {}", e); // Handle the error (optional)
                                                                  // Do nothing if the file selection failed
                    }
                }

                Task::none()
            }
            Message::SelectLogo => {
                println!("Select logo");

                Task::perform(open_file(&["png", "jpg"]), Message::LogoOpened)
            }
            Message::LogoOpened(result) => {
                match result {
                    Ok(file_path) => {
                        self.logo_input = file_path;
                    }
                    Err(e) => {
                        eprintln!("Error selecting file: {}", e); // Handle the error (optional)
                                                                  // Do nothing if the file selection failed
                    }
                }

                Task::none()
            }
            Message::SelectOutputVideo => {
                println!("Select out video");

                Task::perform(save_file(&["mp4", "mkv"]), Message::OutputVideoOpened)
            }
            Message::OutputVideoOpened(result) => {
                match result {
                    Ok(file_path) => {
                        // Check if the file_path ends with ".mp4"
                        if !file_path.ends_with(".mp4") {
                            // Append ".mp4" to the file_path
                            self.dest = format!("{}.mp4", file_path);
                        } else {
                            self.dest = file_path;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error selecting file: {}", e); // Handle the error (optional)
                                                                  // Do nothing if the file selection failed
                    }
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Container<Message> {
        // &self.update(Message::Start);

        let controls = column![].spacing(7)
            .push(
                row![
                        text("Input video: ").width(200),
                        button("Choose source").on_press(Message::SelectInputVideo),
                        container(text(self.source.clone()))
                            .align_x(End)
                            .width(Fill)
                            .padding(7),
                    ]
                    .align_y(Center),
            )
            .push(
                row![
                        text("Output video: ").width(200),
                        button("Choose destination").on_press(Message::SelectOutputVideo),
                        container(text(self.dest.clone()))
                            .align_x(End)
                            .width(Fill)
                            .padding(7),
                    ]
                    .align_y(Center),
            )
            .push_maybe(self.can_logo().then(|| {
                row![
                    text("Logo input: ").width(200),
                    button("Choose logo").on_press(Message::SelectLogo),
                    container(text(self.logo_input.clone()))
                        .align_x(End)
                        .width(Fill)
                        .padding(7),
                ].align_y(Center)
            }));

        container(column![
            container(column![
                container(
                    row![
                        button(
                            container(column![
                                remix_icon("ri-volume-mute-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Mute video").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Mute),
                        button(
                            container(column![
                                remix_icon("ri-clockwise-2-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Rotate video").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Rotate),
                        button(
                            container(column![
                                remix_icon("ri-voiceprint-fill")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Replace sound").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::ReplaceSound),
                        button(
                            container(column![
                                remix_icon("ri-crop-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Crop video").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Crop),
                    ]
                    .spacing(15)
                ),
                container("").height(15),
                container(
                    row![
                        button(
                            container(column![
                                remix_icon("ri-file-zip-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Compress video").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Compress),
                        button(
                            container(column![
                                remix_icon("ri-expand-diagonal-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Resize video").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Resize),
                        button(
                            container(column![
                                remix_icon("ri-text-snippet")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Replace sound").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Subtitle),
                        button(
                            container(column![
                                remix_icon("ri-image-add-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Watermark").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Watermark),
                    ]
                    .spacing(15)
                ),
            ])
            .padding(15),
            container(
                column![
                    horizontal_space(),
                    container(controls),
                    horizontal_space(),
                ]
                .align_x(Center)
                .spacing(7)
            )
            .padding(15)
            .height(Fill),
            container(column![
                progress_bar(0.0..=100.0, self.progress.clone()),
                button(container(
                    text("Do it!")
                ).width(Fill).align_x(Center)).on_press(Message::DoIt)
            ].spacing(15)).padding(15)
        ])
        .width(Fill)
        .center_x(Fill)
        .height(Fill)
        .center_y(Fill)
        .align_x(Center)
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn open_file(support_ext: &[impl ToString]) -> Result<String, String> {
    println!("Opening file...");
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open file...")
        .add_filter("Supported", support_ext)
        .add_filter("All files", &["*"])
        .pick_file()
        .await;

    // Handle the case where the user cancels the dialog
    let picked_file = match picked_file {
        Some(file) => file,
        None => return Err("No file was selected.".to_string()),
    };

    // Handle the case where the path is not valid
    let path = match picked_file.path().to_str() {
        Some(path) => path,
        None => return Err("File path is not valid UTF-8.".to_string()),
    };

    Ok(path.to_string())
}
async fn save_file(support_ext: &[impl ToString]) -> Result<String, String> {
    println!("Opening file...");
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Save file...")
        .add_filter("Supported", support_ext)
        .add_filter("All files", &["*"])
        .save_file()
        .await;

    // Handle the case where the user cancels the dialog
    let picked_file = match picked_file {
        Some(file) => file,
        None => return Err("No file was selected.".to_string()),
    };

    // Handle the case where the path is not valid
    let path = match picked_file.path().to_str() {
        Some(path) => path,
        None => return Err("File path is not valid UTF-8.".to_string()),
    };

    Ok(path.to_string())
}


