use conrod::{widget, Positionable, Widget, UiCell};

widget_ids!{
    pub struct Ids {
        canvas,
        title
    }
}

pub fn build_ui(ref mut ui: UiCell, ids: &Ids) {
    widget::Canvas::new().set(ids.canvas, ui);

    widget::Text::new("Hello, world!")
        .font_size(60)
        .mid_top_of(ids.canvas)
        .set(ids.title, ui);
}
