use nannou::{
    color::{PLUM, STEELBLUE},
    event::Update,
    prelude::PI,
    rand::{prelude::StdRng, random_range, Rng, SeedableRng},
    App, Frame, LoopMode,
};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    random_seed: u64,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    let random_seed = random_range(0, 1000000);
    Model { random_seed }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(PLUM);

    let mut rng = StdRng::seed_from_u64(model.random_seed);

    for y in 0..ROWS {
        for x in 0..COLS {
            let cdraw = gdraw.x_y(x as f32, y as f32);
            let factor = y as f32 / ROWS as f32;
            let x_offset = factor * rng.gen_range(-0.5..0.5);
            let y_offset = factor * rng.gen_range(-0.5..0.5);
            let rotation = factor * rng.gen_range(-PI / 4.0..PI / 4.0);
            cdraw
                .rect()
                .no_fill()
                .stroke(STEELBLUE)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x_offset, y_offset)
                .rotate(rotation);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
