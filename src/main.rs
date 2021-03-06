#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;


use rust_tracer::scene::scene::{create_test_scene, Scene};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Rust Raytracer", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;
    let mut paused = false;
    let mut scene = Scene::create(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

    create_test_scene(&mut scene);

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            // life.draw(pixels.get_frame());
            scene.render(pixels.get_frame());
            if pixels
                .render()
                .map_err(|_| error!("pixels.render() failed"))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                // Space is frame-step, so ensure we're paused
                paused = true;
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                scene.camera.go_left();
            }
            if input.key_pressed(VirtualKeyCode::Right) {
                scene.camera.go_right();
            }
            if input.key_pressed(VirtualKeyCode::Up) {
                scene.camera.go_forward();
            }
            if input.key_pressed(VirtualKeyCode::Down) {
                scene.camera.go_backward();
            }
            if input.key_pressed(VirtualKeyCode::M) {
                scene.camera.change_zoom(1.1);
            }
            if input.key_pressed(VirtualKeyCode::N) {
                scene.camera.change_zoom(1.0 / 1.1);
            }
            // Handle mouse.
            let (mouse_cell, mouse_prev_cell) = input
                .mouse()
                .map(|(mx, my)| {
                    let (dx, dy) = input.mouse_diff();
                    let prev_x = mx - dx;
                    let prev_y = my - dy;

                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    let (px_i, py_i) = pixels
                        .window_pos_to_pixel((prev_x, prev_y))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    (
                        (mx_i as isize, my_i as isize),
                        (px_i as isize, py_i as isize),
                    )
                })
                .unwrap_or_default();

            if input.mouse_held(0) {
                let dx = mouse_cell.0 - mouse_prev_cell.0;
                let dy = mouse_cell.1 - mouse_prev_cell.1;
                scene.camera.turn(dx, dy);
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                let mut w = size.width;
                let mut h = size.height;
                if w % 2 != 0 {
                    w -= 1;
                }
                if h % 2 != 0 {
                    h -= 1;
                }
                pixels.resize_surface(w, h);
                pixels.resize_buffer(w, h);
                println!("resize_surface width: {}, height: {}", size.width, size.height);
                scene.width = w as i32;
                scene.height = h as i32;
            }
            if !paused || input.key_pressed(VirtualKeyCode::Space) {
                scene.update();
            }
            window.request_redraw();
        }
    });
}

/// Create a window for the ray racer.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = 1.0;//(monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}


