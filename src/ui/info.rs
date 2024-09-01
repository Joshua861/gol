use crate::prelude::*;
use crate::timing::get_timers;

pub fn draw_info(draw: &Draw, model: &Model) {
    let cache = &model.cache;

    let mut text = format!("{:.0} fps", model.fps.avg());

    if model.paused {
        text = format!("{} (paused)", text);
    }

    if model.show_info {
        text = format!(
        "{}\ngrid: ({} x {})\nwindow: ({} x {})\nrulestring: {}\ncamera offset: ({:.1} x {:.1})\nzoom: {:.2}",
        text,
        model.cache.board_width,
        model.cache.board_height,
        model.cache.window_size.0,
        model.cache.window_size.1,
        model.rulestring,
        model.cache.camera_offset.0,
        model.cache.camera_offset.1,
        model.cache.scale_factor
    );
    }

    if model.symmetry {
        text = format!("{}\nSymmetry on", text);
    }

    if model.grid_lines {
        text = format!("{}\nGrid on", text);
    }

    #[cfg(debug_assertions)]
    {
        if model.show_info {
            text += "\n";
            for timer in get_timers() {
                text = format!("{}\n{}", text, timer);
            }
        }
    }

    if model.show_info {
        text = format!("{}\n\nv{}", text, VERSION);
    }

    draw.text(&text)
        .color(CONFIG.cell_color.to_srgb())
        .x_y(
            -cache.window_size.0 / 2. + 515.,
            cache.window_size.1 / 2. - 60.,
        )
        .font_size(CONFIG.font_size)
        .font(model.font.clone())
        .left_justify()
        .align_text_top()
        .line_spacing(CONFIG.font_size as f32 * 0.25)
        .w_h(1000., 100.);
}
