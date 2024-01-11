use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, SystemTime};
mod model;
use crate::model::*;

pub const SCREEN_WIDTH: i32 = 600;
pub const SCREEN_HEIGHT: i32 = 640;
pub const CARD_W: i32 = 124;
pub const CARD_H: i32 = 176;

struct Image<'a> {
    texture: Texture<'a>,
    #[allow(dead_code)]
    w: u32,
    h: u32,
}

impl<'a> Image<'a> {
    fn new(texture: Texture<'a>) -> Self {
        let q = texture.query();
        let image = Image {
            texture,
            w: q.width,
            h: q.height,
        };
        image
    }
}

struct Resources<'a> {
    images: HashMap<String, Image<'a>>,
    fonts: HashMap<String, sdl2::ttf::Font<'a, 'a>>,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-blackjack", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    sdl_context.mouse().show_cursor(false);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);

    let texture_creator = canvas.texture_creator();
    let mut resources = load_resources(&texture_creator, &mut canvas, &ttf_context);

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();

    println!("Keys:");
    println!("  Left  : Hit");
    println!("  Right : Stand");
    println!("  Space : Restart when game over");

    'running: loop {
        let started = SystemTime::now();
        let mut is_keydown = false;

        let mut command = Command::None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    match code {
                        Keycode::Left => command = Command::Hit,
                        Keycode::Right => command = Command::Stand,
                        Keycode::Escape => {
                            break 'running;
                        }
                        Keycode::Space => {
                            if game.is_over {
                                game = Game::new();
                            }
                        }
                        _ => {}
                    };
                    is_keydown = true;
                }
                _ => {}
            }
        }
        if !game.is_debug || is_keydown {
            game.update(command);
        }
        render(&mut canvas, &game, &mut resources)?;

        // play_sounds(&mut game, &resources);

        let finished = SystemTime::now();
        let elapsed = finished.duration_since(started).unwrap();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / model::FPS as u32);
        if elapsed < frame_duration {
            ::std::thread::sleep(frame_duration - elapsed)
        }
    }

    Ok(())
}

fn load_resources<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    #[allow(unused_variables)] canvas: &mut Canvas<Window>,
    ttf_context: &'a Sdl2TtfContext,
) -> Resources<'a> {
    let mut resources = Resources {
        images: HashMap::new(),
        fonts: HashMap::new(),
    };

    let entries = fs::read_dir("resources/image").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".bmp") {
            let temp_surface = sdl2::surface::Surface::load_bmp(&path).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&temp_surface)
                .expect(&format!("cannot load image: {}", path_str));

            let basename = path.file_name().unwrap().to_str().unwrap();
            let image = Image::new(texture);
            resources.images.insert(basename.to_string(), image);
        }
    }

    let entries = fs::read_dir("./resources/font").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".ttf") {
            let font = ttf_context
                .load_font(path_str, 32) // FIXME: サイズ固定になっちゃってる
                .expect(&format!("cannot load font: {}", path_str));
            let basename = path.file_name().unwrap().to_str().unwrap();
            resources.fonts.insert(basename.to_string(), font);
        }
    }

    resources
}

fn render(
    canvas: &mut Canvas<Window>,
    game: &Game,
    resources: &mut Resources,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let cards = vec![
        "card01.bmp",
        "card02.bmp",
        // "card03.bmp",
        // "card04.bmp",
        // "card05.bmp",
        // "card06.bmp",
        // "card07.bmp",
        // "card08.bmp",
        // "card09.bmp",
        // "card10.bmp",
        // "card11.bmp",
        // "card12.bmp",
    ];

    // render dealer cards
    let mut x: i32;
    let mut y: i32;
    for (i, card) in cards.iter().enumerate() {
        let image = resources.images.get(*card).unwrap();
        x = 50 + CARD_W * (i % 4) as i32;
        y = 50 + 35 * (i as i32 / 4);
        canvas
            .copy(
                &image.texture,
                Rect::new(0, 0, image.w, image.h),
                Rect::new(x, y, CARD_W as u32, CARD_H as u32),
            )
            .unwrap();
    }

    // render player cards
    let mut x: i32;
    let mut y: i32;
    for (i, card) in cards.iter().enumerate() {
        let image = resources.images.get(*card).unwrap();
        x = 50 + CARD_W * (i % 4) as i32;
        y = 320 + 35 * (i as i32 / 4);
        canvas
            .copy(
                &image.texture,
                Rect::new(0, 0, image.w, image.h),
                Rect::new(x, y, CARD_W as u32, CARD_H as u32),
            )
            .unwrap();
    }

    let font = resources.fonts.get_mut("boxfont2.ttf").unwrap();
    render_font(
        canvas,
        font,
        "Hit or Stand ?".to_string(),
        200,
        600,
        Color::RGBA(255, 255, 255, 255),
    );

    canvas.present();

    Ok(())
}

fn render_font(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    text: String,
    x: i32,
    y: i32,
    color: Color,
) {
    let texture_creator = canvas.texture_creator();

    let surface = font.render(&text).blended(color).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(x, y, texture.query().width, texture.query().height),
        )
        .unwrap();
}

// fn play_sounds(game: &mut Game, resources: &Resources) {
//     for sound_key in &game.requested_sounds {
//         let chunk = resources
//             .chunks
//             .get(&sound_key.to_string())
//             .expect("cannot get sound");
//         sdl2::mixer::Channel::all()
//             .play(&chunk, 0)
//             .expect("cannot play sound");
//     }
//     game.requested_sounds = Vec::new();
// }
