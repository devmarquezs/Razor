extern crate ggez;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Image, /*Rect,*/ Text, TextFragment, Font, PxScale};

pub struct Renderer2D {
    image: Image,
    welcome_text: Text,
}

impl Renderer2D {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = Image::new(ctx, "/images/welcome.png")?;
        let font = Font::new(ctx, "/fonts/TheyPerished.ttf")?; 
        let mut welcome_text =  Text::new(TextFragment {
            text: "Projeto Razor! \nDivirtam-se Cavaleiros!".to_string(),
            color: Some(Color::from_rgb(138, 43, 226)),
            font: Some(font),
            scale: Some(PxScale::from(48.0)),
        });
        welcome_text.set_bounds([600.0, 300.0], graphics::Align::Center);

        Ok(Renderer2D { image, welcome_text })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        
        //funções de desenho

        // Obtenha as dimensões da janela
        let screen_coordinates = graphics::screen_coordinates(ctx);
        let screen_width = screen_coordinates.w;
        let screen_height = screen_coordinates.h;

        // Obtenha as dimensões da imagem
        let image_width = self.image.width() as f32;
        let image_height = self.image.height() as f32;

        // Calcule a escala para ajustar a imagem à janela
        let scale_x = screen_width / image_width;
        let scale_y = screen_height / image_height;
        let scale = [scale_x, scale_y];
        
        //desenhando uma imagem
        graphics::draw(ctx, &self.image, DrawParam::default().dest([0.0, 0.0]).scale(scale))?;

        //desenhando formas geometricas
        /*let rect = Rect::new(100.0, 100.0, 200.0, 100.0);
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            rect, 
            Color::RED,
        )?;
        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;
        */

        //desenhando um texto na cor preta
        let text_position = [screen_width / 2.0, screen_height / 2.0];
        graphics::draw(ctx, &self.welcome_text, DrawParam::default().dest(text_position).offset([0.5, 0.5]))?;
        
        
        graphics::present(ctx)?;
        Ok(())
    }
}