use nannou::{
    prelude::*,
    rand::{thread_rng, Rng},
};
use rand_distr::StandardNormal;

fn main() {
    nannou::app(model).update(update).run();
}

struct Fields {
    field: f64,
}

struct Model {
    position: Vec2,
    fields: Fields,
}

const SIZE: usize = 100;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}/images/{app_name}.png",
            &app.exe_name().unwrap(),
            app_name = &app.exe_name().unwrap()
        )),
        Key::Up => model.fields.field += 0.001,
        Key::Down => {
            if model.fields.field > 0.0 {
                model.fields.field -= 0.001;
            }
        }
        Key::Right => model.fields.field += 1.0,
        Key::Left => {
            if model.fields.field > 0.0 {
                model.fields.field -= 0.1;
            }
        }
        _other_key => {}
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        position: vec2(0.0, 0.0),
        fields: Fields { field: 120.0 },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let rand = random::<f32>();
    let add = if rand < 0.25 {
        vec2(1.0, 0.0)
    } else if rand < 0.5 {
        vec2(-1.0, 0.0)
    } else if rand < 0.75 {
        vec2(0.0, 1.0)
    } else {
        vec2(0.0, -1.0)
    };
    model.position += thread_rng().sample::<f32, _>(StandardNormal) * add;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }
    draw.ellipse().xy(model.position).radius(1.0).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
