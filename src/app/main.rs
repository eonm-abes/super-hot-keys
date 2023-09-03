use crate::config::SOCKET_NAME;
use crate::hotkeys::{ShortCut, ShortCutManager};
use interprocess::local_socket::LocalSocketListener;
use std::env;

use winit::{
    event::{ElementState, Event, ModifiersState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::x11::{WindowBuilderExtX11, XWindowType},
    window::{Icon, Window, WindowBuilder},
};

use std::{
    process::exit,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct MainApp {
    window: Window,
    state: State,
    event_loop: EventLoop<()>,
    socket: LocalSocketListener,
}

impl MainApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();

        Ok(MainApp {
            window: {
                WindowBuilder::new()
                    .with_decorations(false)
                    .with_active(false)
                    .with_transparent(true)
                    .with_window_icon(Some(load_icon()))
                    .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
                    .with_inner_size(winit::dpi::LogicalSize::new(2., 2.))
                    .with_x11_window_type(vec![XWindowType::Normal])
                    .build(&event_loop)
                    .unwrap()
            },
            state: State::default(),
            event_loop,
            socket: LocalSocketListener::bind(env::temp_dir().join(SOCKET_NAME))?,
        })
    }

    pub fn run(mut self) {
        let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        ctrlc::set_handler(move || MainApp::clean()).expect("Error setting Ctrl-C handler");

        std::thread::spawn(move || {
            for _event in self.socket.incoming() {
                let _ = tx.send(());
            }
        });

        self.event_loop.run(move |event, _, control_flow| {
            if rx.try_recv().is_ok() {
                self.window.set_visible(false);
                self.window.set_visible(true);
            }

            match event {
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                winit::event::KeyboardInput {
                                    virtual_keycode,
                                    state: ElementState::Released,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    println!("{:?}", event);
                    if virtual_keycode == Some(VirtualKeyCode::Q) {
                        self.state.shortcut_manager.reset();
                        self.window.set_visible(false)
                    }

                    println!("{:?}", virtual_keycode);
                    let shortcut = ShortCut {
                        modifiers: self.state.modifiers,
                        key: virtual_keycode,
                    };

                    if self.state.shortcut_manager.trigger(shortcut) {
                        self.window.set_visible(false);
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::Focused(false),
                    ..
                } => {
                    self.state.shortcut_manager.reset();
                    self.state.modifiers = ModifiersState::default();
                }
                Event::WindowEvent {
                    event: WindowEvent::ModifiersChanged(state),
                    ..
                } => self.state.modifiers = state,
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => {
                    *control_flow = ControlFlow::Exit;
                    MainApp::clean()
                }
                _ => (),
            }
        });
    }

    fn clean() {
        let _ = std::fs::remove_file(env::temp_dir().join(SOCKET_NAME));
        exit(0);
    }
}

pub struct State {
    modifiers: ModifiersState,
    shortcut_manager: ShortCutManager,
}

impl Default for State {
    fn default() -> Self {
        State {
            modifiers: ModifiersState::default(),
            shortcut_manager: ShortCutManager::default(),
        }
    }
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../../assets/icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
