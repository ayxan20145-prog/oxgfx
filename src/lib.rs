use sdl3::{
    EventPump, Sdl, event::Event, init, keyboard::Keycode, pixels::Color, rect::Rect,
    render::Canvas, ttf, video,
};
use std::ops::{Add, Div, Mul, Sub};

pub struct Input {
    pub keys: Vec<Keycode>,
}

impl Input {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }
    pub fn is_key_down(&self, key: Keycode) -> bool {
        self.keys.contains(&key)
    }
    pub fn key_down(&mut self, key: Keycode) {
        if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }
    pub fn key_up(&mut self, key: Keycode) {
        self.keys.retain(|k| *k != key);
    }
}

pub struct Window {
    pub sdl: Sdl,
    pub canvas: Canvas<video::Window>,
    pub events: EventPump,

    pub quit: bool,
}

impl Window {
    pub fn create_window(title: &str, width: u32, height: u32) -> Self {
        let sdl = init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window(title, width, height).build().unwrap();

        let events = sdl.event_pump().unwrap();
        let canvas = window.into_canvas();

        Window {
            sdl,
            canvas,
            events,
            quit: false,
        }
    }
    pub fn handle_events(&mut self, input: &mut Input) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => self.quit = true,
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    input.key_down(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    input.key_up(key);
                }
                _ => {}
            }
        }
    }
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.canvas.set_draw_color(color);

        let rect = Rect::new(x, y, width, height);

        self.canvas.fill_rect(rect).unwrap();
    }
    pub fn draw_text(&mut self, font: &ttf::Font, text: &str, x: i32, y: i32, color: Color) {
        let surface = font.render(text).blended(color).unwrap();

        let idk = self.canvas.texture_creator();
        let texture = idk.create_texture_from_surface(&surface).unwrap();

        let rect = Rect::new(x, y, surface.width(), surface.height());

        self.canvas.copy(&texture, None, rect).unwrap();
    }
    pub fn draw_line(&mut self, x1: i32, x2: i32, y1: i32, y2: i32, color: Color) {
        self.canvas.set_draw_color(color);

        self.canvas.draw_line((x1, y1), (x2, y2)).unwrap();
    }
    pub fn draw_point(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);

        self.canvas.draw_point((x, y)).unwrap();
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn distance(&self, other: Self) -> f32 {
        (self.clone() - other).length()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
