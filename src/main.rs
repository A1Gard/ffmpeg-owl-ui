mod engine;
mod font_installer;
mod remixicon;

use crate::engine::mute;
use crate::remixicon::remix_icon;
use iced::widget::{button, column, container, horizontal_space, progress_bar, row, text, Column, Container};
use iced::Alignment::End;
use iced::{Center, Element, Fill, Task, Theme, Color, Border};
use remixicon::remix_init;
use std::io;
use std::collections::HashMap;

fn theme(state: &Controller) -> Theme {
    print!("{}", state.value.to_string());
    Theme::CatppuccinMocha
}

#[tokio::main]
pub async fn main() -> iced::Result {
    // install icon font
    let font_path = "node_modules/remixicon/fonts/remixicon.ttf"; // Path to the font file
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
    can_image: bool,
    image_input: String,
    progress: f32,
    action: String,
    toasts: Vec<HashMap<String, String>>,
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
    Merge,
    AddCover,
    TextWatermark,
    Landscape,
    DoIt,
    SelectInputVideo,
    InputVideoOpened(Result<String, String>),
    SelectOutputVideo,
    OutputVideoOpened(Result<String, String>),
    SelectImage,
    ImageOpened(Result<String, String>),

}

impl Controller {
    fn new() -> (Self, Task<Message>) {
        let start_message = { Message::Start };
        (Self::default(), Task::done(start_message))
    }

    fn can_image(&self) -> bool {
        self.can_image
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => {
                self.source = "-".to_string();
                self.dest = "-".to_string();
                self.image_input = "-".to_string();
                self.can_image = false;
                self.action = "mute".to_string();
                self.toasts = vec![];
                Task::none()
            }
            Message::Mute => {
                self.action = "mute".to_string();
                self.can_image = false;
                Task::none()
            }
            Message::Rotate => Task::none(),
            Message::ReplaceSound => Task::none(),
            Message::Crop => Task::none(),
            Message::Compress => Task::none(),
            Message::Resize => Task::none(),
            Message::Subtitle => Task::none(),
            Message::Landscape => Task::none(),
            Message::Merge => Task::none(),
            Message::AddCover => Task::none(),
            Message::TextWatermark => Task::none(),
            Message::Watermark => {
                self.can_image = true;
                Task::none()
            }
            Message::DoIt => {
                self.toasts = vec![];
                let mut _is_err = false;
                if self.action == "mute" {
                    if self.source == "-" {
                        // self.log = text_editor::Content::with_text(&format!(
                        //     "{}\nInvalid input",
                        //     self.log.text().trim()
                        // ));
                        self.toasts.push(HashMap::from([
                            ("message".to_string(), "Invalid input".to_string()),
                            ("type".to_string(), "error".to_string()),
                        ]));
                        _is_err = true;
                    }
                    if self.dest == "-" {
                        self.toasts.push(HashMap::from([
                            ("message".to_string(), "Invalid output".to_string()),
                            ("type".to_string(), "error".to_string()),
                        ]));
                        _is_err = true;
                    }
                    if _is_err {
                        return  Task::none();
                    }
                    match mute(&self.source, &self.dest) {
                        Ok(_) => {
                            self.progress = 100.0;
                            // self.log = text_editor::Content::with_text(&format!(
                            //     "{}\n",
                            //     self.log.text().trim()
                            // ));
                            self.toasts.push(HashMap::from([
                                ("message".to_string(), "Successfully muted the audio.".to_string()),
                                ("type".to_string(), "success".to_string()),
                            ]));
                        }
                        Err(e) => {
                            self.toasts.push(HashMap::from([
                                (e.to_string(), "Invalid output".to_string()),
                                ("type".to_string(), "error".to_string()),
                            ]));
                            self.progress = 0.0;
                        }
                    }
                }
                Task::none()
            }

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
            Message::SelectImage => {
                println!("Select image");

                Task::perform(open_file(&["png", "jpg"]), Message::ImageOpened)
            }
            Message::ImageOpened(result) => {
                match result {
                    Ok(file_path) => {
                        self.image_input = file_path;
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
            },
        }
    }

    fn view(&self) -> Container<Message> {
        // &self.update(Message::Start);

        let controls = column![]
            .spacing(7)
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
            .push_maybe(self.can_image().then(|| {
                row![
                    text("Image input: ").width(200),
                    button("Choose image").on_press(Message::SelectImage),
                    container(text(self.image_input.clone()))
                        .align_x(End)
                        .width(Fill)
                        .padding(7),
                ]
                .align_y(Center)
            }));


        let toasts  = Column::with_children(
            self.toasts.iter()
                .map(|message| {
                    let mut _bg_color = Color::from_rgb(58.0 / 255.0, 132.0 / 255.0, 0.0); // Default color #3a8400

                    // Check if the "type" key exists and is a string
                    if let Some(value) = message.get("type") {
                        if value == "error" {
                            _bg_color = Color::from_rgb(173.0 / 255.0, 0.0, 0.0); // Error color #ad0000
                        }
                    }



                    let msg_style = iced::widget::container::Style::default()
                        .background(_bg_color).color(Color::from_rgb(1.0, 1.0, 1.0))
                        .border(Border::default().rounded(4.0));

                    let message_text = container(text(message.get("message").unwrap()))
                        .style(move|_theme: &Theme| msg_style.clone())
                        .padding(7).width(Fill); // Use a closure that returns the style

                    Column::new().push(message_text).push(horizontal_space().height(7))
                }).map(Element::from),
        );

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
                                text("Shrink video").width(Fill).align_x(Center),
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
                container("").height(15),
                container(
                    row![
                        button(
                            container(column![
                                remix_icon("ri-split-cells-horizontal")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Landscape to portrait").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Landscape),
                        button(
                            container(column![
                                remix_icon("ri-git-merge-line")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Merge two video").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::Merge),
                        button(
                            container(column![
                                remix_icon("ri-image-circle-fill")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Add cover").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::AddCover),
                        button(
                            container(column![
                                remix_icon("ri-text-snippet")
                                    .size(35)
                                    .width(Fill)
                                    .align_x(Center),
                                text("Text watermark").width(Fill).align_x(Center),
                            ])
                            .width(Fill)
                            .align_x(Center)
                        )
                        .width(Fill)
                        .on_press(Message::TextWatermark),
                    ]
                    .spacing(15)
                ),
            ])
            .padding(15),
            container(
                column![
                    horizontal_space(),
                    container(row![text("Current action:"), text(self.action.clone())].spacing(15)),
                    horizontal_space(),
                    container(controls),
                    horizontal_space(),
                ]
                .align_x(Center)
                .spacing(7)
            )
            .padding(15)
            .height(Fill),
            container(
                column![
                    progress_bar(0.0..=100.0, self.progress.clone()),
                    button(container(text("Do it!")).width(Fill).align_x(Center))
                        .on_press(Message::DoIt),
                    toasts,
                ]
                .spacing(15)
            )
            .padding(15)
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
