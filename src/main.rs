use macroquad::prelude::*;
const MIN_SIZE: i32 = 1;
const MAX_SIZE: i32 = 100;
const GRID_SIZE: usize = 32;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut grid: [[i32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    let mut k = 0;
    let mut l = 0;
    // prefil grid
    for i in grid {
        l = 0;
        for j in i {
            println!("l: {l:?}, k: {k:?}");
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
            if cell_size > MAX_SIZE {
                cell_size = MAX_SIZE;
            }
        }
        if is_key_down(KeyCode::Down) {
            cell_size -= 1;
            if cell_size < MIN_SIZE {
                cell_size = MIN_SIZE;
            }
        }

        // for each element (array) in grid - y / k
        // for each elemennt in that array - x / l??????????????
        // grain of salt cuz i just fiddled till it works
        let mut k = 0;
        let mut l = 0;
        for i in grid {
            l = 0;
            for j in i {
                // println!("k: {k:?}, j: {j:?}");
                if j == 1 {
                    draw_rectangle(
                        ((k * cell_size) - x) as f32,
                        ((l * cell_size) - y) as f32,
                        (cell_size) as f32,
                        (cell_size) as f32,
                        BLACK,
                    );
                }
                l = l + 1;
            }
            k = k + 1;
        }

        // get mouse position
        let mouse_position = mouse_position();
        let mousex = mouse_position.0;
        let mousey = mouse_position.1;
        let mouse_tile_x = (((mousex + x as f32).floor() / (cell_size) as f32).floor() as usize)
            .clamp(0, GRID_SIZE - 1);
        let mouse_tile_y = (((mousey + y as f32).floor() / (cell_size) as f32).floor() as usize)
            .clamp(0, GRID_SIZE - 1);

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

        if is_mouse_button_pressed(MouseButton::Left) {
            grid[mouse_tile_x][mouse_tile_y] = if grid[mouse_tile_x][mouse_tile_y] == 1 {
                0
            } else {
                1
            };
        }

        // draw info text
        let fps = get_fps();
        let ft = macroquad::time::get_frame_time();
        let frametime_ms = 1000.0 * ft;
        let str = format!("Pos: {x}, {y}");
        let str2 = format!("fps: {fps}");
        let str3 = format!("frametime: {frametime_ms:.2}ms");
        let str4 = format!("mouse pos: {mouse_position:?}");
        let str5 = format!("mouse tile: {mouse_tile_x}, {mouse_tile_y}");
        draw_text(&str, 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&str2, 20.0, 40.0, 30.0, DARKGRAY);
        draw_text(&str3, 20.0, 60.0, 30.0, DARKGRAY);
        draw_text(&str4, 20.0, 80.0, 30.0, DARKGRAY);
        draw_text(&str5, 20.0, 100.0, 30.0, DARKGRAY);

        // handle frame rate

        let minimum_frame_time = 1. / 60.; // 60 FPS
        let mut time_to_sleep = 0.;
        if ft < minimum_frame_time {
            time_to_sleep = (minimum_frame_time - ft) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
        }

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
}
