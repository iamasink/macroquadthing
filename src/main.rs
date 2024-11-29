use macroquad::prelude::*;
use std::time::Duration;
const MIN_SIZE: i32 = 1;
const MAX_SIZE: i32 = 100;

#[macroquad::main("Window")]
async fn main() {
    // setup spin_sleep
    let max_fps = 240;
    let mut interval = spin_sleep_util::interval(Duration::from_secs(1) / max_fps);

    // vars
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let grid_width = 256;
    let grid_height = 256;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; grid_height]; grid_width];

    let mut mousepos = (0, 0);
    let mut oldmousepos = (0, 0);

    let mut click = false;

    let mut cell_size: i32 = 64;

    // prefil grid
    for col in 0..grid_width {
        for row in 0..grid_height {
            if col == 0 || col == grid_width - 1 || row == 0 || row == grid_height - 1 {
                grid[col][row] = 1;
            } else {
                grid[col][row] = 0;
            }
        }
    }
    grid[10][5] = 1;

    loop {
        let mut next_grid = grid.clone();

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
            (((mousex + x) as f32 / (cell_size) as f32).floor() as usize).clamp(0, grid_width - 1);
        let mouse_tile_y =
            (((mousey + y) as f32 / (cell_size) as f32).floor() as usize).clamp(0, grid_height - 1);

        if is_key_down(KeyCode::E) {
            next_grid[mouse_tile_x][mouse_tile_y] = 3;
            click = true;
        }
        // if click & !is_key_down(KeyCode::E) {
        //     click = false;
        // }
        if is_mouse_button_down(MouseButton::Right) {
            let dx = oldmousepos.0 - mousepos.0;
            let dy = oldmousepos.1 - mousepos.1;
            x += dx;
            y += dy;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let maximum = 3;
            next_grid[mouse_tile_x][mouse_tile_y] = if grid[mouse_tile_x][mouse_tile_y] >= maximum {
                0
            } else {
                grid[mouse_tile_x][mouse_tile_y] + 1
            };
        }

        //

        // run sand simulation for cell type 3
        for col in (0..grid_width) {
            for row in 0..grid_height - 1 {
                if grid[col][row] == 3 && grid[col][row + 1] == 0 {
                    next_grid[col][row] = 0;
                    next_grid[col][row + 1] = 3;
                }
            }
        }

        grid = next_grid; // Update the grid

        // draw grid
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell > 0 {
                    let color = match cell {
                        1 => BLACK,
                        2 => GREEN,
                        3 => BEIGE,
                        _ => panic!("Invalid color!"),
                    };
                    draw_rectangle(
                        ((row_idx as i32 * cell_size) - x) as f32,
                        ((col_idx as i32 * cell_size) - y) as f32,
                        cell_size as f32,
                        cell_size as f32,
                        color,
                    );
                }
            }
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
