#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate image;

use piston_window::{Texture, EventLoop, PistonWindow, UpdateEvent, WindowSettings, Window};
use image::ImageBuffer;

widget_ids!(
    struct Ids {
        canvas,
        art
    }
);

fn main() {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;

    let mut window: PistonWindow = {
        let title = "character_creator_game";
        let resolution = [WIDTH, HEIGHT];
        let opengl = piston_window::OpenGL::V3_2;
        let mut window_result = WindowSettings::new(title, resolution)
            .exit_on_esc(true)
            .srgb(true)
            .opengl(opengl)
            .build();
        if window_result.is_err() {
            window_result = WindowSettings::new(title, resolution)
                .exit_on_esc(true)
                .srgb(false)
                .opengl(opengl)
                .build();
        }
        PistonWindow::new(opengl, 0, window_result
            .unwrap_or_else(|e| {
                panic!("Failed to build PistonWindow: {}", e)
            })
        )
    };

    window.set_ups(20);

    let mut ui = conrod::UiBuilder::new().build();

    let ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    let mut image_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::new(1920 / 1, 1080 / 1);

    let art_image = {
        let factory = &mut window.factory;
        let settings = piston_window::TextureSettings::new();
        Texture::from_image(factory, &image_buffer, &settings).unwrap()
    };

    let mut image_map = image_map! {
        (ids.art, art_image)
    };

    let mut frame = 0;

    while let Some(event) = window.next() {
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            for (x, y, mut pixel) in image_buffer.enumerate_pixels_mut() {
                pixel[0] = (((x) % 255).wrapping_add((y + frame) % 255)) as u8;
                pixel[1] = (((x) % 255).wrapping_add((y - frame) % 255)) as u8;
                pixel[2] = ((x) % 255) as u8;
                pixel[3] = 255;
            }

            image_map.get_mut(ids.art).unwrap().update(&mut window.encoder, &image_buffer).unwrap();

            let size = window.size();
            let (image_width, image_height) = (size.width as f64, size.height as f64);

            {
                let mut ui = &mut ui.set_widgets();
                let ids = &ids;
                use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget}; //, Borderable

                widget::Canvas::new().color(color::DARK_CHARCOAL).set(ids.canvas, ui);

                widget::Image::new().w_h(image_width, image_height).middle().set(ids.art, ui);
            }

            ui.needs_redraw();

            frame += 1;
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(image: &T) -> &T { image };
                conrod::backend::piston_window::draw(
                    c, g, primitives, &mut text_texture_cache, &image_map, texture_from_image
                );
            }
        });
    }
}
