use nannou::color::named;
use nannou::geom::range::Range;
use nannou::prelude::*;

use nannou_sandbox::model::{Model, Velocity};

fn main() {
    nannou::app(Model::new)
        .update(update)
        .simple_window(view)
        .run();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let v = Velocity::new(0.0, -1.0);
    _model.add_circle_in_range(5.0, 20.0, v);
    _model.trim_circles(1000);
    _model.step_circles();
}

fn view(_app: &App, _model: &Model, frame: &Frame) {
    let draw = _app.draw();

    draw.background()
        .color(named::BLACK);

    _model.circles.iter().for_each(|circle| {
        draw.ellipse().color(named::WHITE)
            .w(circle.r)
            .h(circle.r)
            .x_y(circle.x, circle.y);
    });

    draw.to_frame(&_app, &frame);
}
