use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
    EventPump,
};
use gilrs::{Gilrs, Event as GilrsEvent};
use std::time::{Duration, Instant};

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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { .. } => break 'running,
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
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
