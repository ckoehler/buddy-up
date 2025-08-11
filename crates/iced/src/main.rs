use arboard::Clipboard;
use buddy_up_lib::{History, People, pair, print_table, save_history};
use iced::widget::{button, column, container, text};
use iced::{Element, Task};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    InputFileChanged(String),
    OutputDirChanged(String),
    LoadFile,
    PickInputFile,
    PickOutputDir,
    GeneratePairs,
    FileSelected(Option<PathBuf>),
    DirectorySelected(Option<PathBuf>),
    CopyToClipboard,
}

pub struct App {
    input_file: String,
    output_dir: String,
    people: People,
    history: History,
    people_status: String,
    history_status: String,
    pairs_output: String,
    clipboard_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_dir: String::new(),
            people: People::default(),
            history: History::default(),
            people_status: "No people loaded".to_string(),
            history_status: "No history loaded".to_string(),
            pairs_output: String::new(),
            clipboard_message: String::new(),
        }
    }
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    // fn title(&self) -> String {
    //     "Buddy Up".to_string()
    // }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputFileChanged(value) => {
                self.input_file = value;
            }
            Message::OutputDirChanged(value) => {
                self.output_dir = value;
            }
            Message::PickInputFile => {
                return Task::perform(
                    async move {
                        FileDialog::new()
                            .add_filter("CSV files", &["csv"])
                            .set_directory(".")
                            .pick_file()
                    },
                    Message::FileSelected,
                );
            }
            Message::PickOutputDir => {
                return Task::perform(
                    async move { FileDialog::new().set_directory(".").pick_folder() },
                    Message::DirectorySelected,
                );
            }
            Message::FileSelected(path) => {
                if let Some(file_path) = path {
                    self.input_file = file_path.to_string_lossy().to_string();
                    // Automatically load the file
                    match std::fs::File::open(&file_path) {
                        Ok(file) => match People::from_csv(file) {
                            Ok(people) => {
                                self.people = people;
                                self.people_status =
                                    format!("✓ Loaded {} people", self.people.len());
                            }
                            Err(e) => {
                                self.people_status = format!("✗ Error loading file: {e}");
                            }
                        },
                        Err(e) => {
                            self.people_status = format!("✗ Error opening file: {e}");
                        }
                    }
                }
            }
            Message::DirectorySelected(path) => {
                if let Some(dir_path) = path {
                    self.output_dir = dir_path.to_string_lossy().to_string();
                    // Automatically load the history
                    match History::from_dir(&self.output_dir) {
                        Ok(history) => {
                            self.history = history;
                            self.history_status =
                                format!("✓ Found {} pairs in history", self.history.len());
                        }
                        Err(e) => {
                            self.history_status = format!("✗ Error loading history: {e}");
                        }
                    }
                }
            }
            Message::LoadFile => {
                if !self.input_file.is_empty() {
                    match std::fs::File::open(&self.input_file) {
                        Ok(file) => match People::from_csv(file) {
                            Ok(people) => {
                                self.people = people;
                                self.people_status =
                                    format!("✓ Loaded {} people", self.people.len());
                            }
                            Err(e) => {
                                self.people_status = format!("✗ Error loading file: {e}");
                            }
                        },
                        Err(e) => {
                            self.people_status = format!("✗ Error opening file: {e}");
                        }
                    }
                }
            }
            Message::GeneratePairs => {
                if !self.people.is_empty() && !self.output_dir.is_empty() {
                    let pairs = pair(self.people.clone(), &self.history);

                    // Save the history
                    match save_history(&pairs, &self.output_dir) {
                        Ok(_) => {
                            // Reload history after saving
                            match History::from_dir(&self.output_dir) {
                                Ok(history) => {
                                    self.history = history;
                                    self.history_status =
                                        format!("✓ Found {} pairs in history", self.history.len());
                                }
                                Err(e) => {
                                    self.history_status = format!("✗ Error reloading history: {e}");
                                }
                            }

                            // Generate output table
                            self.pairs_output = print_table(pairs);
                        }
                        Err(_e) => {
                            // error saving pairs, do nothing
                        }
                    }
                }
            }
            Message::CopyToClipboard => {
                if !self.pairs_output.is_empty() {
                    match Clipboard::new() {
                        Ok(mut clipboard) => match clipboard.set_text(&self.pairs_output) {
                            Ok(_) => {
                                self.clipboard_message = "✓ Copied to clipboard!".to_string();
                            }
                            Err(_) => {
                                self.clipboard_message =
                                    "✗ Failed to copy to clipboard".to_string();
                            }
                        },
                        Err(_) => {
                            self.clipboard_message = "✗ Could not access clipboard".to_string();
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input_section = column![
            text("People:").size(20),
            button("Pick CSV File").on_press(Message::PickInputFile),
        ]
        .spacing(5);

        let output_section = column![
            text("History:").size(20),
            button("Pick Directory").on_press(Message::PickOutputDir),
        ]
        .spacing(5);

        let controls = column![if !self.people.is_empty() && !self.output_dir.is_empty() {
            column![
                text("Results:").size(20),
                button("Generate Pairs").on_press(Message::GeneratePairs)
            ]
        } else {
            column![text("Results:").size(20), button("Generate Pairs")]
        },]
        .spacing(10);

        let results_section = if !self.pairs_output.is_empty() {
            column![
                button("Copy to Clipboard").on_press(Message::CopyToClipboard),
                if !self.clipboard_message.is_empty() {
                    text(&self.clipboard_message).size(12).color(
                        if self.clipboard_message.starts_with("✓") {
                            iced::Color::from_rgb(0.0, 0.7, 0.0)
                        } else {
                            iced::Color::from_rgb(0.8, 0.0, 0.0)
                        },
                    )
                } else {
                    text("")
                },
                container(iced::widget::scrollable(
                    container(
                        text(&self.pairs_output)
                            .font(iced::Font::MONOSPACE)
                            .size(12)
                    )
                    .width(iced::Length::Fill)
                    .padding(10)
                ))
                .height(300),
            ]
            .spacing(10)
        } else {
            column![]
        };

        let left_column = column![input_section, output_section, controls, results_section,]
            .spacing(20)
            .width(iced::Length::FillPortion(2));

        let right_column = column![
            text("Status").size(20),
            container(
                column![
                    if !self.people.is_empty() {
                        text(&self.people_status)
                            .size(12)
                            .color(iced::Color::from_rgb(0.0, 0.7, 0.0))
                    } else {
                        text(&self.people_status).size(12)
                    },
                    if !self.output_dir.is_empty() {
                        text(&self.history_status)
                            .size(12)
                            .color(iced::Color::from_rgb(0.0, 0.7, 0.0))
                    } else {
                        text(&self.history_status).size(12)
                    },
                    if !self.people.is_empty() && !self.output_dir.is_empty() {
                        text("✓ Ready")
                            .size(12)
                            .color(iced::Color::from_rgb(0.0, 0.7, 0.0))
                    } else {
                        text("Ready").size(12)
                    },
                ]
                .spacing(5)
            )
            .padding(10)
        ]
        .spacing(10)
        .width(iced::Length::FillPortion(1));

        let content = iced::widget::row![left_column, right_column,]
            .spacing(20)
            .padding(20);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("Buddy Up", App::update, App::view)
        .window(iced::window::Settings {
            size: iced::Size::new(450.0, 700.0),
            ..Default::default()
        })
        .run_with(App::new)
}
