use macroquad::prelude::*;

struct Block {
    x: f32,
    y: f32,
    z: f32,
    color: Color,
}

#[macroquad::main("Rust Minecraft Clone")]
async fn main() {
    let mut blocks = Vec::new();

    // Generiamo un piccolo terreno di blocchi
    for x in -5..5 {
        for z in -5..5 {
            let height = (x as f32 * 0.5).sin() + (z as f32 * 0.5).cos();
            blocks.push(Block {
                x: x as f32 * 2.0,
                y: height * 2.0,
                z: z as f32 * 2.0,
                color: GREEN,
            });
        }
    }

    let mut camera_pos = vec3(0.0, 5.0, -15.0);
    let mut rotation: f32 = 0.0;

    loop {
        clear_background(SKYBLUE);

        // Gestione input semplice per muovere la visuale
        if is_key_down(KeyCode::Right) { rotation += 0.02; }
        if is_key_down(KeyCode::Left) { rotation -= 0.02; }
        if is_key_down(KeyCode::Up) { camera_pos[2] += 0.1; }
        if is_key_down(KeyCode::Down) { camera_pos[2] -= 0.1; }

        // Renderizzazione semplificata (proiezione 2D per brevità della demo)
        for block in &blocks {
            let screen_x = 400.0 + (block.x * rotation.cos() - block.z * rotation.sin());
            let screen_y = 300.0 - (block.y * 20.0) + (block.z * rotation.cos());

            draw_rectangle(screen_x, screen_y, 20.0, 20.0, block.color);
        }

        draw_text("Rust Minecraft Proto - Usa le frecce per muoverti", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
