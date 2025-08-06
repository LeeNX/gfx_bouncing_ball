use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

struct Ball {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    radius: u32,
    color: Color,
}

impl Ball {
    fn update(&mut self, width: u32, height: u32) {
        self.x += self.vx;
        self.y += self.vy;

        let max_x = width as i32 - self.radius as i32;
        let max_y = height as i32 - self.radius as i32;

        if self.x <= 0 || self.x >= max_x {
            self.vx = -self.vx;
        }

        if self.y <= 0 || self.y >= max_y {
            self.vy = -self.vy;
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let rect = Rect::new(
            self.x,
            self.y,
            self.radius,
            self.radius,
        );
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(rect);
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let width = 640;
    let height = 480;

    let window = video_subsystem
        .window("Bouncing Ball", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut ball = Ball {
        x: 100,
        y: 100,
        vx: 3,
        vy: 2,
        radius: 20,
        color: Color::RED,
    };

    let mut last_frame = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let delta = now.duration_since(last_frame);
        if delta < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - delta);
        }
        last_frame = Instant::now();

        ball.update(width, height);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        ball.draw(&mut canvas);

        canvas.present();
    }

    Ok(())
}
