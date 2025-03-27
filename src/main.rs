use dioxus::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

fn main() {
    dioxus_desktop::launch(App);
}

#[derive(Clone)]
struct AudioState {
    sink: Arc<Mutex<Sink>>,
}

fn App(cx: Scope) -> Element {
    let audio_state = use_ref(cx, || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioState {
            sink: Arc::new(Mutex::new(sink)),
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Rust Music Player (Dioxus)" }
            button {
                onclick: move |_| play_audio(audio_state.clone()),
                "Play"
            }
            button {
                onclick: move |_| pause_audio(audio_state.clone()),
                "Pause"
            }
        }
    })
}

fn play_audio(audio_state: &UseRef<AudioState>) {
    let file = File::open("example.mp3").unwrap();
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
