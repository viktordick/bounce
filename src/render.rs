use std::collections::HashMap;

use sdl2::video::WindowContext;
use sdl2::render::{Texture,TextureCreator};
use sdl2::surface::Surface;
use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::gfx::primitives::DrawRenderer;

pub struct Marbles<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    marbles: HashMap<(u32, [u8;3]), Texture<'a>>,
}

impl<'a> Marbles<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Marbles<'a> {
        Marbles {
            texture_creator: texture_creator,
            marbles: HashMap::new(),
        }
    }

    pub fn get(&mut self, radius: u32, color: [u8;3]) -> Result<&Texture, String> {
        if self.marbles.contains_key(&(radius, color)) {
            return Ok(&self.marbles[&(radius, color)])
        }
        let entry = self.marbles.entry((radius, color));

        let mut color = Color::RGB(color[0], color[1], color[2]);
        let r = radius as i16;
        let len = 2*r + 1;
        let canvas = Surface::new(len as u32, len as u32, PixelFormatEnum::RGBA8888)
            ?.into_canvas()?;

        for i in 0..len {
            color.a = (256 - (((len-i) * 180)/(len+1)) as u16) as u8;
            let halflength = ((r*r-(i-r)*(i-r)) as f64).sqrt() as i16;
            canvas.hline(r-halflength, r+halflength, i, Color::RGB(200, 200, 200))?;
            canvas.hline(r-halflength, r+halflength, i, color)?;
        }
        let result = self.texture_creator
            .create_texture_from_surface(canvas.into_surface())
            .map_err(|e| e.to_string())?;
        Ok(entry.or_insert(result))
    }
}
