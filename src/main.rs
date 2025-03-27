use dioxus::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use rfd::FileDialog;

fn main() {
    dioxus_desktop::launch(App);
}

#[derive(Clone)]
struct AudioState {
    sink: Arc<Mutex<Sink>>,
    playlist: Arc<Mutex<Vec<String>>>,
    current_index: Arc<Mutex<Option<usize>>>,
}


fn App(cx: Scope) -> Element {
    let audio_state = use_ref(cx, || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioState {
            sink: Arc::new(Mutex::new(sink)),
            current_track: Arc::new(Mutex::new(None)),
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Rust Music Player (Dioxus)" }
            button {
                onclick: move |_| pick_file(audio_state.clone()),
                "Pick Music File"
            }
            button {
                onclick: move |_| play_audio(audio_state.clone()),
                "Play"
            }
            button {
                onclick: move |_| pause_audio(audio_state.clone()),
                "Pause"
            }
            p { "Now Playing: ", audio_state.read().current_track.lock().unwrap().as_deref().unwrap_or("None") }
        }
    })
}

fn pick_file(audio_state: &UseRef<AudioState>) {
    if let Some(paths) = FileDialog::new().add_filter("Audio", &["mp3", "wav", "flac"]).pick_files() {
        let mut playlist = audio_state.write().playlist.lock().unwrap();
        for path in paths {
            playlist.push(path.to_string_lossy().to_string());
        }
    }
}


fn play_audio(audio_state: &UseRef<AudioState>) {
    let playlist = audio_state.read().playlist.lock().unwrap();
    let mut index = audio_state.write().current_index.lock().unwrap();

    if playlist.is_empty() {
        return;
    }

    if index.is_none() {
        *index = Some(0);
    }

    let current_track = &playlist[index.unwrap()];
    let file = File::open(current_track).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let sink = &audio_state.read().sink;
    let mut sink = sink.lock().unwrap();

    if sink.empty() {
        sink.append(source);
    }
    sink.play();
}


fn pause_audio(audio_state: &UseRef<AudioState>) {
    let sink = &audio_state.read().sink;
    let mut sink = sink.lock().unwrap();
    sink.pause();
}
