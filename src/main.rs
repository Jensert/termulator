mod app;
mod camera;
mod types;

use app::{App, RenderMode};
use types::MyShapes;

use color_eyre::Result;
use crossterm::ExecutableCommand;

use ratatui::{
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Paragraph,
    },
    DefaultTerminal,
};

fn clip_line_to_viewport(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
) -> Option<(f64, f64, f64, f64)> {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let mut t0 = 0.0;
    let mut t1 = 1.0;

    let clip = |p: f64, q: f64, t0: &mut f64, t1: &mut f64| -> bool {
        if p == 0.0 {
            return q >= 0.0;
        }
        let r = q / p;
        if p < 0.0 {
            if r > *t1 {
                return false;
            }
            if r > *t0 {
                *t0 = r;
            }
        } else {
            if r < *t0 {
                return false;
            }
            if r < *t1 {
                *t1 = r;
            }
        }
        true
    };

    if clip(-dx, x1 - xmin, &mut t0, &mut t1)
        && clip(dx, xmax - x1, &mut t0, &mut t1)
        && clip(-dy, y1 - ymin, &mut t0, &mut t1)
        && clip(dy, ymax - y1, &mut t0, &mut t1)
    {
        let nx1 = x1 + t0 * dx;
        let ny1 = y1 + t0 * dy;
        let nx2 = x1 + t1 * dx;
        let ny2 = y1 + t1 * dy;
        Some((nx1, ny1, nx2, ny2))
    } else {
        None
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    std::io::stdout()
        .execute(crossterm::event::EnableMouseCapture)
        .unwrap();
    let terminal = ratatui::init();
    let app = App::default();

    let result = run(app, terminal);

    ratatui::restore();

    result
}

fn run(mut app: App, mut terminal: DefaultTerminal) -> Result<()> {
    let shapes = MyShapes::create_shapes();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let canvas = Canvas::default()
                .x_bounds([-1.0, 1.0])
                .y_bounds([-1.0, 1.0])
                .marker(app.draw_mode)
                .background_color(Color::Blue)
                .paint(|ctx| {

                    match app.render_mode {
                        RenderMode::Vertex => {
                             for &(start_idx, end_idx) in &shapes._cube_edges {
                                let start_vertex = shapes._cube_vertices[start_idx];
                                let end_vertex = shapes._cube_vertices[end_idx];

                                let start = start_vertex - app.camera.pos;
                                let end = end_vertex - app.camera.pos;

                                let (x1, y1) = app.camera.project_vertex(&start);
                                let (x2, y2) = app.camera.project_vertex(&end);

                                if let Some((x1, y1, x2, y2)) =
                                    clip_line_to_viewport(x1, y1, x2, y2, -1.0, 1.0, -1.0, 1.0)
                                {
                                    let color = Color::Red;
                                    ctx.draw(&Line {
                                        x1,
                                        y1,
                                        x2,
                                        y2,
                                        color,
                                    });
                                }
                            }       
                        },
                        RenderMode::Raycast => {
                            let rows = app.terminal_size.y as i32;
                            let cols = app.terminal_size.x as i32;
                            for row in 0..rows {
                                for col in 0..cols {
                                    let u = (col as f64 / app.terminal_size.x) * 2.0 - 1.0;
                                    let v = 1.0 - (row as f64 / app.terminal_size.y) * 2.0;

                                    let ray_dir = app.camera.cast_ray(u, v);

                                    // IMPLEMENT RENDERING LOGIC HERE
                                }
                            }
                        }
                    }
                    
                });

            let debug_info = Paragraph::new(format!(
                "terminal size: {:?}\naspect ratio {:?}\ndrawmode: {:?}\nrendermode: {:?}\n\ncamera pos: {:?} \nyaw: {:.1}, \npitch: {:.1}",
                app.terminal_size, app.camera.aspect_ratio, app.draw_mode, app.render_mode, app.camera.pos, app.camera.yaw, app.camera.pitch
            ));

            frame.render_widget(canvas, area);
            frame.render_widget(debug_info, area);
        })?;

        let event = app.get_event()?;
        let action = app.process_event(event)?;
        app.process_action(action);

        if app.should_quit {
            return Ok(());
        }
    }
}
