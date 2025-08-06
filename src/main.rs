use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;
use std::time::Duration;

fn draw_ball(canvas: &mut Canvas<Window>, x: i32, y: i32, size: i32) {
    let _ = canvas.set_draw_color(Color::RGB(255, 0, 0));
    let _ = canvas.fill_rect(Rect::new(x, y, size as u32, size as u32));
}

fn handle_input(event_pump: &mut EventPump, controller_subsystem: &GameControllerSubsystem) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return true,
            Event::KeyDown { .. } => return true, // Any key press exits
            Event::ControllerButtonDown { .. } => return true, // Any gamepad button exits
            _ => {}
        }
    }

    // Open all controllers if not already open
    if controller_subsystem.num_joysticks().unwrap_or(0) > 0 {
        for id in 0..controller_subsystem.num_joysticks().unwrap() {
            if controller_subsystem.is_game_controller(id) {
                let _ = controller_subsystem.open(id); // Ignore already-open error
            }
        }
    }

    false
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let controller_subsystem = sdl_context.game_controller()?;

    let window = video_subsystem
        .window("Bouncing Ball", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let (mut x, mut y) = (100, 100);
    let (mut dx, mut dy) = (4, 3);
    let ball_size = 20;

    'running: loop {
        if handle_input(&mut event_pump, &controller_subsystem) {
            break 'running;
        }

        x += dx;
        y += dy;

        if x <= 0 || x + ball_size >= 640 {
            dx = -dx;
        }

        if y <= 0 || y + ball_size >= 480 {
            dy = -dy;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_ball(&mut canvas, x, y, ball_size);
        canvas.present();

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
