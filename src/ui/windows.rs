use crate::config::Color;
use crate::prelude::*;
use bon::builder;

#[builder(builder_type = WindowBuilder, start_fn = new, on(String, into))]
pub struct Window {
    width: Option<f32>,
    padding: Option<f32>,
    lines: Option<usize>,
    bg_color: Option<Color>,
    text: String,
    open: bool,
    line_height: Option<f32>,
}

impl Window {
    pub fn render(&self, draw: &Draw, cache: &Cache, model: &Model) {
        let Window {
            width,
            padding,
            bg_color,
            text,
            open,
            lines,
            line_height,
        } = self;

        if *open {
            let width = width.unwrap_or((cache.window_size.0 / 2.).min(800.));
            let padding = padding.unwrap_or(40.);
            let lines = lines.unwrap_or(text.lines().count());
            let bg_color = bg_color.unwrap_or(CONFIG.window_color);
            let line_height = line_height.unwrap_or(1.25);
            let height = lines as f32 * CONFIG.font_size as f32 * line_height;

            let w = width + padding * 2.;
            let h = height + padding * 2.;

            draw.rect().width(w).height(h).color(bg_color.to_srgb());

            draw.text(text)
                .left_justify()
                .font(model.font.clone())
                .font_size(CONFIG.font_size)
                .align_text_top()
                .color(CONFIG.text_color.to_srgb())
                .width(width)
                .line_spacing(CONFIG.font_size as f32 * (line_height - 1.))
                .height(height);
        }
    }
}
