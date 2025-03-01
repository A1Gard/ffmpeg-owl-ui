mod font_installer;
mod remixicon;

use iced::widget::{button, column, container, row, text, Container};
use iced::window;
use iced::{Center, Fill, Font, Task, Theme};
use remixicon::{remix_init, ri_icon};

fn theme(state: &Controller) -> Theme {
    print!("{}", state.value.to_string());
    Theme::CatppuccinMocha
}

pub fn main() -> iced::Result {
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
}

#[derive(Debug, Clone, Copy)]
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
    SelectInputVideo,
}

impl Controller {

    fn new() -> (Self, Task<Message>) {
        let start_message =  { Message::Start };
        (
            Self::default(),
            Task::done(start_message),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                self.source = "-".to_string();
            }
            Message::Mute => {
                self.value += 1;
            }
            Message::Rotate => {
                self.value -= 1;
            }
            Message::ReplaceSound => {
                self.value -= 1;
            }
            Message::Crop => {
                self.value -= 1;
            }
            Message::Compress => {
                self.value -= 1;
            }
            Message::Resize => {
                self.value -= 1;
            }
            Message::Subtitle => {
                self.value -= 1;
            }
            Message::Watermark => {
                self.value -= 1;
            }

            Message::SelectInputVideo => {
                self.source = "-".to_string();
            }
        }
    }

    fn view(&self) -> Container<Message> {
        // &self.update(Message::Start);

        container(column![
            container(column![
                container(
                    row![
                        button(
                            container(column![
                                text(ri_icon("ri-volume-mute-line"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-clockwise-2-line"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-voiceprint-fill"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-crop-line"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-file-zip-line"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-expand-diagonal-line"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-text-snippet"))
                                    .font(Font::with_name("remixicon"))
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
                                text(ri_icon("ri-image-add-line"))
                                    .font(Font::with_name("remixicon"))
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
                column![row![
                    text("Input video: "),
                    button("Choose source").on_press(Message::SelectInputVideo),
                    text(self.source.clone())
                ]
                .align_y(Center)]
                .align_x(Center)
            )
            .padding(15)
            .height(Fill),
        ])
        .width(Fill)
        .center_x(Fill)
        .height(Fill)
        .center_y(Fill)
        .align_x(Center)
    }
}
