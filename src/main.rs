use sdl2::{
    event::Event,
    keyboard::Keycode,
    joystick::HatState,
    controller::GameController,
    GameControllerSubsystem,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
};

use gilrs::{Gilrs, Event as GilrsEvent};
use std::time::{Duration, Instant};
use std::collections::HashMap;

const BALL_SIZE: u32 = 16;

struct Ball {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Ball {
    fn update(&mut self, screen_width: u32, screen_height: u32) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0 || self.x + BALL_SIZE as i32 > screen_width as i32 {
            self.vx = -self.vx;
        }

        if self.y < 0 || self.y + BALL_SIZE as i32 > screen_height as i32 {
            self.vy = -self.vy;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(self.x, self.y, BALL_SIZE, BALL_SIZE);
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let _ = canvas.fill_rect(rect);
    }
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    ttf_context: &Sdl2TtfContext,
    text: &str,
    x: i32,
    y: i32,
) {
    let font = ttf_context.load_font("assets/UbuntuMono-Regular.ttf", 20).unwrap();
    let surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let target = Rect::new(x, y, surface.width(), surface.height());
    let _ = canvas.copy(&texture, None, Some(target));
}

fn list_gamepads(gc_subsystem: &GameControllerSubsystem) -> HashMap<u32, GameController> {
    let mut controllers = HashMap::new();

    for id in 0..gc_subsystem.num_joysticks().unwrap() {
        if gc_subsystem.is_game_controller(id) {
            match gc_subsystem.open(id) {
                Ok(c) => {
                    println!(
                        "Opened Gamepad {}: {}",
                        id,
                        c.name()
                    );
                    controllers.insert(id as u32, c);
                }
                Err(e) => {
                    println!("Failed to open gamepad {}: {}", id, e);
                }
            }
        }
    }

    controllers
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Rust SDL2 Gamepad Bouncer", 320, 240)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let gc_subsystem = sdl_context.game_controller()?;
    let mut controllers = list_gamepads(&gc_subsystem);

    let mut event_pump = sdl_context.event_pump()?;
    let mut gilrs = Gilrs::new().unwrap();

    let mut ball = Ball {
        x: 100,
        y: 100,
        vx: 3,
        vy: 2,
    };

    let mut frame_count = 0;
    let mut last_fps_check = Instant::now();
    let mut fps = 0;
    let mut frame_delay = 8u64; // starts at 8ms (≈125fps)

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if frame_delay > 1 {
                        frame_delay -= 1;
                        println!("Frame delay: {}ms ({} FPS)", frame_delay, 1000 / frame_delay);
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if frame_delay < 20 {
                        frame_delay += 1;
                        println!("Frame delay: {}ms ({} FPS)", frame_delay, 1000 / frame_delay);
                    }
                }
                Event::JoyAxisMotion { axis_idx, value, .. } => {
                    if axis_idx == 1 {
                        if value < -10_000 && frame_delay > 1 {
                            if frame_delay > 1 {
                                frame_delay -= 1;
                                println!("(Gamepad) Decreased delay to {} ms", frame_delay);
                            }
                        } else if value > 10_000 {
                            if frame_delay < 20 {
                                frame_delay += 1;
                                println!("(Gamepad) In creased delay to {} ms", frame_delay);
                            }
                        }
                    }
                }
/*
                Event::JoyHatMotion { state, .. } => {
                    if state.contains(HatState::Up) && frame_delay > 1 {
                        frame_delay -= 1;
                    } else if state.contains(HatState::Down) {
                        frame_delay += 1;
                    }
                }
*/
                Event::JoyHatMotion { hat_idx, state, .. } => {
                    println!("Hat {} moved to {:?}", hat_idx, state);

                    if let HatState::Up = state {
                        if frame_delay > 1 {
                            frame_delay -= 1;
                            println!("(Gamepad-D) Decreased delay to {} ms", frame_delay);
                        }
                    } else if let HatState::Down = state {
                        if frame_delay < 20 {
                            frame_delay += 1;
                            println!("(Gamepad-D) Increased delay to {} ms", frame_delay);
                        }
                    }

                }

                Event::JoyButtonDown { button_idx, .. } => {
                    println!("Button {} down", button_idx);

                    match button_idx {
                        0 => {
                            if frame_delay > 1 {
                                frame_delay -= 1;
                                println!("(Gamepad) Decreased delay to {} ms", frame_delay);
                            }
                        }
                        1 => {
                            if frame_delay < 20 {
                                frame_delay += 1;
                                println!("(Gamepad) Increased delay to {} ms", frame_delay);
                            }
                        }
                        _ => {}
                    }
                }

                Event::ControllerButtonDown { which, button, .. } => {
                    println!("Gamepad {} Button Down: {:?}", which, button);
                    match button {
                        //ControllerButton::Start => {
                        sdl2::controller::Button::Start => {
                            println!("Gamepad Start button pressed – quitting");
                            break 'running;
                        }
                        //ControllerButton::DPadDown => {
                        sdl2::controller::Button::DPadDown => {
                            if frame_delay < 20 {
                                frame_delay += 1;
                                println!("(Gamepad-D) Increased delay to {} ms", frame_delay);
                            }
                        }
                        //ControllerButton::DPadUp => {
                        sdl2::controller::Button::DPadUp => {
                            if frame_delay > 1 {
                                frame_delay -= 1;
                                println!("(Gamepad-D) Decreased delay to {} ms", frame_delay);
                            }
                        }
                        _ => {}
                    }
                }

                Event::ControllerDeviceAdded { which, .. } => {
                    println!("Gamepad Added: {}", which);
                    if let Ok(c) = gc_subsystem.open(which) {
                        controllers.insert(which as u32, c);
                    }
                }

                Event::ControllerDeviceRemoved { which, .. } => {
                    println!("Gamepad Removed: {}", which);
                    controllers.remove(&(which as u32));
                }

                /*
                // Quit on gamepad Start button
                Event::ControllerButtonDown {
                    //button: ControllerButton::Start,
                    button: sdl2::controller::Button::Start,
                    ..
                } => {
                    println!("Gamepad Start button pressed – quitting");
                    break 'running;
                }
                */

                //Event::Quit { .. } |
                //Event::KeyDown { .. } => break 'running,
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        while let Some(GilrsEvent { id: _, event, .. }) = gilrs.next_event() {
            use gilrs::ev::EventType::*;
            match event {
                ButtonPressed(_, _) => break 'running,
                _ => {}
            }
        }

        ball.update(320, 240);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        ball.draw(&mut canvas);

        // FPS counter
        frame_count += 1;
        let now = Instant::now();
        if now.duration_since(last_fps_check) >= Duration::from_secs(1) {
            fps = frame_count;
            frame_count = 0;
            last_fps_check = now;
        }

        draw_text(
            &mut canvas,
            &texture_creator,
            &ttf_context,
            &format!("FPS: {}", fps),
            5,
            5,
        );
        draw_text(
            &mut canvas,
            &texture_creator,
            &ttf_context,
            &format!("Gamepads: {}", gilrs.gamepads().count()),
            5,
            30,
        );

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(frame_delay));
    }

    Ok(())
}
