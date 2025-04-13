use iced::{
    executor, Application, Command, Element, Settings, Subscription,
    widget::{button, column, text},
    theme, Length,
};
use rfd::FileDialog;
use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    thread,
};
use rodio::{Decoder, OutputStream, Sink};

pub fn main() -> iced::Result {
    MusicPlayer::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    PlayPause,
    LoadFile,
    FileLoaded(Option<String>),
}

struct MusicPlayer {
    track_name: Option<String>,
    is_playing: bool,
    sink: Option<Arc<Sink>>,
    _stream: Option<OutputStream>,
}

impl Application for MusicPlayer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            MusicPlayer {
                track_name: None,
                is_playing: false,
                sink: None,
                _stream: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SPlayer")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::PlayPause => {
                if let Some(sink) = &self.sink {
                    if self.is_playing {
                        sink.pause();
                    } else {
                        sink.play();
                    }
                    self.is_playing = !self.is_playing;
                }
            }
            Message::LoadFile => {
                return Command::perform(async {
                    FileDialog::new().add_filter("Audio", &["mp3", "wav"]).pick_file()
                        .map(|p| p.display().to_string())
                }, Message::FileLoaded);
            }
            Message::FileLoaded(Some(path)) => {
                if let Ok((stream, handle)) = OutputStream::try_default() {
                    let sink = Sink::try_new(&handle).unwrap();
                    let file = File::open(&path).unwrap();
                    let source = Decoder::new(BufReader::new(file)).unwrap();
                    sink.append(source);
                    sink.pause();

                    self._stream = Some(stream);
                    self.sink = Some(Arc::new(sink));
                    self.track_name = Some(path);
                    self.is_playing = false;
                }
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let load_button = button("Load Audio").on_press(Message::LoadFile);
        let play_button = if self.sink.is_some() {
            button(if self.is_playing { "Pause" } else { "Play" })
                .on_press(Message::PlayPause)
        } else {
            button("Play").style(theme::Button::Secondary)
        };

        let track = text(self.track_name.clone().unwrap_or_else(|| "No file loaded".into()));

        column![track, load_button, play_button]
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}
