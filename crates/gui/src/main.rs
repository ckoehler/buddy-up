use buddy_up_lib::{History, People, pair, print_table, save_history};
use dioxus::prelude::*;
use rfd::FileDialog;
use std::fs::File;
use std::path::PathBuf;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let people = use_signal(People::default);
    let history = use_signal(History::default);
    let out_dir = use_signal(String::new);
    let input_file = use_signal(PathBuf::new);
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        main {
            Input {
                people,
                history,
                input_file,
                out_dir,
            }
            Results { people, history, out_dir }
        }
    }
}

fn input_status(file: PathBuf) -> People {
    let f = File::open(file).unwrap();
    People::from_csv(f).unwrap()
}

#[component]
pub fn Input(
    mut people: Signal<People>,
    mut history: Signal<History>,
    mut input_file: Signal<PathBuf>,
    mut out_dir: Signal<String>,
) -> Element {
    rsx! {
        section { id: "input",
            h1 { "People" }
            p { "Pick your people CSV file." }
            div {
                button {
                    onclick: move |_| {
                        let path = FileDialog::new().set_directory(".").pick_file();
                        if let Some(file) = path {
                            input_file.set(file.clone());
                            people.set(input_status(file));
                            println!("setting context")
                        }
                    },
                    "Pick file"
                }
            }
            if !people.read().is_empty() {
                p { "Input File: { input_file.read().to_string_lossy() }" }
                p { "Found { people.read().len() } people in the input." }
            }
            h1 { "History" }
            p { "Pick your history folder, either a new one or an existing one." }
            div {
                button {
                    onclick: move |_| {
                        let path = FileDialog::new().set_directory(".").pick_folder();
                        if let Some(dir) = path {
                            out_dir.set(dir.to_str().unwrap().to_string());
                            println!("picked folder {dir:?}");
                            history.set(History::from_dir(dir.to_str().unwrap()).unwrap());
                        }
                    },
                    "Pick History Folder"
                }
            if !out_dir.read().is_empty() {
                p { "Chosen History Directory: { out_dir.read() }" }
                p { "Found { history.read().len() } pairs in the history." }
            }
            }
        }
    }
}

#[component]
pub fn Results(
    mut people: Signal<People>,
    mut history: Signal<History>,
    mut out_dir: Signal<String>,
) -> Element {
    let people = people.read().clone();
    let mut output = use_signal(String::new);
    rsx! {
        section { id: "results",
            h1 { "Buddy Up!" }
            p { "Generate the pairs!" }
            div {
                button {
                    id: "generate",
                    onclick: move |_ev| {
                        if people.is_empty() || out_dir.read().is_empty() {
                            return;
                        }
                        let out = pair(people.clone(), &history.read());
                        let _ = save_history(&out, &out_dir.read());
                        // create a new history now that we have added more files
                        history.set(History::from_dir(&out_dir.read()).unwrap());
                        output.set(print_table(out));
                    },
                    "Generate"
                }
            }
            h1 { "Results" }
            div {
                textarea { rows: 20, cols: 30, "{ output }" }
            }
        }
    }
}
