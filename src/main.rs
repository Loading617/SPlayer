use dioxus::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use rfd::FileDialog;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread;
use dioxus_desktop_hooks::use_window_event;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use wgpu::{Device, Queue, Surface, SwapChain, SwapChainDescriptor};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    dioxus_desktop::launch(App);
}

#[derive(Clone)]
struct AudioState {
    sink: Arc<Mutex<Sink>>,
    playlist: Arc<Mutex<Vec<String>>>,
    current_index: Arc<Mutex<Option<usize>>>,
    shuffle: Arc<Mutex<bool>>,
    repeat: Arc<Mutex<bool>>,
}


fn App(cx: Scope) -> Element {
    let audio_state = use_ref(cx, || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioState {
            sink: Arc::new(Mutex::new(sink)),
            current_track: Arc::new(Mutex::new(None)),
            div {
                canvas {
                    id: "Grut",
                    width: "600",
                    height: "200",
                }
            }
            use_window_event(cx, move |event: dioxus_desktop_hooks::FileDropEvent| {
                if let dioxus_desktop_hooks::FileDropEvent::Dropped(files) = event {
                    let mut playlist = audio_state.write().playlist.lock().unwrap();
                    for file in files {
                        let path = file.to_string_lossy().to_string();
                        if path.ends_with(".mp3") || path.ends_with(".wav") || path.ends_with(".flac") {
                            playlist.push(path);
                        }
                    }
                }
            });
        }
    });


    cx.render(rsx! {
        div {
            h1 { "SPlayer" }
            p { "Drag and drop files here!" }
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
            button {
                onclick: move |_| next_track(audio_state.clone()),
                "Next"
            }
            button {
                onclick: move |_| previous_track(audio_state.clone()),
                "Previous"
            }
            button {
                onclick: move |_| toggle_shuffle(audio_state.clone()),
                "Shuffle"
            }
            button {
                onclick: move |_| toggle_repeat(audio_state.clone()),
                "Repeat"
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

fn next_track(audio_state: &UseRef<AudioState>) {
    let mut index = audio_state.write().current_index.lock().unwrap();
    let playlist = audio_state.read().playlist.lock().unwrap();
    let repeat = *audio_state.read().repeat.lock().unwrap();

    if playlist.is_empty() {
        return;
    }

    if let Some(i) = *index {
        if i + 1 < playlist.len() {
            *index = Some(i + 1);
        } else if repeat {
            *index = Some(0);
        } else {
            return;
        }
            play_audio(audio_state);
        }
    }

fn previous_track(audio_state: &UseRef<AudioState>) {
    let mut index = audio_state.write().current_index.lock().unwrap();

    if let Some(i) = *index {
        if i > 0 {
            *index = Some(i - 1);
            play_audio(audio_state);
        }
    }
}

fn toggle_shuffle(audio_state: &UseRef<AudioState>) {
    let mut shuffle = audio_state.write().shuffle.lock().unwrap();
    *shuffle = !*shuffle;

    if *shuffle {
        let mut playlist = audio_state.write().playlist.lock().unwrap();
        playlist.shuffle(&mut rand::thread_rng());
    }
}

fn toggle_repeat(audio_state: &UseRef<AudioState>) {
    let mut repeat = audio_state.write().repeat.lock().unwrap();
    *repeat = !*repeat;
}

fn start_visualizer() {
    thread::spawn(|| {
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let config = device.default_input_config().unwrap();
        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                draw_waveform(data);
            },
            move |err| eprintln!("Stream error: {}", err),
        ).unwrap();
        stream.play().unwrap();
    });
}

fn draw_waveform(samples: &[f32]) {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create("waveform.txt").unwrap();
    for &sample in samples {
        writeln!(file, "{}", sample).unwrap();
    }
}

fn init_visualizer(window: &Window) -> (Device, Queue, Surface, SwapChain) {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();
    
    let swap_chain_descriptor = SwapChainDescriptor {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: 600,
        height: 200,
        present_mode: wgpu::PresentMode::Fifo,
    };

    let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
    (device, queue, surface, swap_chain)
}
