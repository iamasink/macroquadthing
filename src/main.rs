use macroquad::prelude::*;
use std::time::Duration;
const MIN_SIZE: i32 = 1;
const MAX_SIZE: i32 = 100;
const GRID_SIZE: usize = 32;

#[macroquad::main("Window")]
async fn main() {
    // setup spin_sleep
    let max_fps = 120;
    let mut interval = spin_sleep_util::interval(Duration::from_secs(1) / max_fps);

    // vars
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut grid: [[i32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut mousepos = (0, 0);
    let mut oldmousepos = (0, 0);

    let mut k = 0;
    let mut l = 0;

    // prefil grid
    for i in grid {
        l = 0;
        for j in i {
            // println!("l: {l:?}, k: {k:?}");
            // println!("test")
            grid[l][k] = ((l) % 2) as i32;
            l = l + 1;
        }
        k = k + 1;
    }
    grid[10][2];

    let mut click = false;

    let mut cell_size: i32 = 64;

    loop {
        clear_background(RED);

        // handle inputs

        if is_key_down(KeyCode::D) {
            x += 1;
        }
        if is_key_down(KeyCode::A) {
            x -= 1;
        }
        if is_key_down(KeyCode::W) {
            y -= 1;
        }
        if is_key_down(KeyCode::S) {
            y += 1;
        }
        if is_key_down(KeyCode::Up) {
            cell_size += 1;
        }
        if is_key_down(KeyCode::Down) {
            cell_size -= 1;
        }
        if mouse_wheel().1 != 0. {
            cell_size += (mouse_wheel().1 / 120.).floor() as i32;
        }
        // if mouse_wheel().1 < 0. {
        //     cell_size -= (mouse_wheel().1 / 120.).floor() as i32;
        // }

        if cell_size < MIN_SIZE {
            cell_size = MIN_SIZE;
        }
        if cell_size > MAX_SIZE {
            cell_size = MAX_SIZE;
        }
        // get mouse position

        oldmousepos = mousepos;
        mousepos = (mouse_position().0 as i32, mouse_position().1 as i32);

        let mousex = mousepos.0;
        let mousey = mousepos.1;
        let mouse_tile_x =
            (((mousex + x) as f32 / (cell_size) as f32).floor() as usize).clamp(0, GRID_SIZE - 1);
        let mouse_tile_y =
            (((mousey + y) as f32 / (cell_size) as f32).floor() as usize).clamp(0, GRID_SIZE - 1);

        if !click & is_key_down(KeyCode::E) {
            grid[mouse_tile_x][mouse_tile_y] = if grid[mouse_tile_x][mouse_tile_y] == 1 {
                0
            } else {
                1
            };
            click = true;
        }
        if click & !is_key_down(KeyCode::E) {
            click = false;
        }
        if is_mouse_button_down(MouseButton::Right) {
            let dx = oldmousepos.0 - mousepos.0;
            let dy = oldmousepos.1 - mousepos.1;
            x += dx;
            y += dy;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let maximum = 2;
            grid[mouse_tile_x][mouse_tile_y] = if grid[mouse_tile_x][mouse_tile_y] >= maximum {
                0
            } else {
                grid[mouse_tile_x][mouse_tile_y] + 1
            };
        }

        // draw grid

        // for each element (array) in grid - y / k
        // for each elemennt in that array - x / l??????????????
        // grain of salt cuz i just fiddled till it works
        let mut k = 0;
        let mut l = 0;
        for i in grid {
            l = 0;
            for j in i {
                // println!("k: {k:?}, j: {j:?}");
                if j > 0 {
                    let colour = match j {
                        1 => BLACK,
                        2 => GREEN,
                        _ => panic!("invalid colour!"),
                    };
                    draw_rectangle(
                        ((k * cell_size) - x) as f32,
                        ((l * cell_size) - y) as f32,
                        (cell_size) as f32,
                        (cell_size) as f32,
                        colour,
                    );
                }
                l = l + 1;
            }
            k = k + 1;
        }

        // draw info text
        let fps = get_fps();
        let ft = get_frame_time();
        let frametime_ms = 1000.0 * ft;
        let wheel = mouse_wheel();
        let str = format!("Pos: {x}, {y}");
        let str2 = format!("fps: {fps}");
        let str3 = format!("frametime: {frametime_ms:.2}ms");
        let str4 = format!("mouse pos: {mousex} {mousey}");
        let str5 = format!("mouse tile: {mouse_tile_x}, {mouse_tile_y}");
        let str6 = format!("mouse wheel: {wheel:?}");
        draw_text(&str, 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&str2, 20.0, 40.0, 30.0, DARKGRAY);
        draw_text(&str3, 20.0, 60.0, 30.0, DARKGRAY);
        draw_text(&str4, 20.0, 80.0, 30.0, DARKGRAY);
        draw_text(&str5, 20.0, 100.0, 30.0, DARKGRAY);
        draw_text(&str6, 20.0, 120.0, 30.0, DARKGRAY);

        // next_frame().await;
        // spin_sleep::sleep(Duration::from_millis(time_to_sleep as u64));
        // tick the spin_sleep
        next_frame().await;
        interval.tick();
    }
}
