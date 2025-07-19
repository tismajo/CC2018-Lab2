mod framebuffer_src;

use raylib::prelude::*;
use crate::framebuffer_src::{
    framebuffer::Framebuffer, 
    game_of_life::{GameOfLife, patterns}
};

fn main() {
    let window_width = 800;
    let window_height = 600;
    let game_width = 100;
    let game_height = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Game of Life")
        .vsync()  // Para una animación más suave
        .build();

    let mut framebuffer = Framebuffer::new(
        &mut window,
        &raylib_thread,
        game_width,
        game_height,
        Color::BLACK,
    );

    let mut game = GameOfLife::new(game_width, game_height);
    
    // Inicializar patrones
    game.init_with_pattern(&patterns::block(), 10, 10);
    game.init_with_pattern(&patterns::beehive(), 30, 10);
    game.init_with_pattern(&patterns::blinker(), 50, 10);
    game.init_with_pattern(&patterns::glider(), 70, 10);

    game.init_with_pattern(&patterns::block(), 10, 30);
    game.init_with_pattern(&patterns::toad(), 30, 30);
    game.init_with_pattern(&patterns::beacon(), 50, 30);
    game.init_with_pattern(&patterns::lwss(), 70, 30);

    game.init_with_pattern(&patterns::pulsar(), 10, 50);
    game.init_with_pattern(&patterns::pentadecathlon(), 30, 50);

    game.init_with_pattern(&patterns::lwss(), 10, 70);
    game.init_with_pattern(&patterns::mwss(), 40, 70);
    game.init_with_pattern(&patterns::hwss(), 70, 70);

    game.init_with_pattern(&patterns::block(), 10, 90);
    game.init_with_pattern(&patterns::beehive(), 30, 90);
    game.init_with_pattern(&patterns::blinker(), 50, 90);
    game.init_with_pattern(&patterns::glider(), 70, 90);

    game.init_with_pattern(&patterns::block(), 5, 5);
    game.init_with_pattern(&patterns::beehive(), 25, 5);
    game.init_with_pattern(&patterns::blinker(), 45, 5);
    game.init_with_pattern(&patterns::glider(), 65, 5);
    game.init_with_pattern(&patterns::loaf(), 85, 5);

    game.init_with_pattern(&patterns::toad(), 5, 25);
    game.init_with_pattern(&patterns::beacon(), 25, 25);
    game.init_with_pattern(&patterns::pulsar(), 45, 25);
    game.init_with_pattern(&patterns::lwss(), 75, 25);

    game.init_with_pattern(&patterns::pentadecathlon(), 5, 45);
    game.init_with_pattern(&patterns::mwss(), 35, 45);
    game.init_with_pattern(&patterns::hwss(), 65, 45);

    game.init_with_pattern(&patterns::glider(), 5, 65);
    game.init_with_pattern(&patterns::blinker(), 25, 65);
    game.init_with_pattern(&patterns::block(), 45, 65);
    game.init_with_pattern(&patterns::beehive(), 65, 65);
    game.init_with_pattern(&patterns::tub(), 85, 65);

    game.init_with_pattern(&patterns::lwss(), 5, 85);
    game.init_with_pattern(&patterns::mwss(), 35, 85);
    game.init_with_pattern(&patterns::hwss(), 65, 85);

    
    while !window.window_should_close() {
        // Actualización del juego (siempre activa, sin pausa)
        game.update();

        // Fase de renderizado
        {
            // Limpiar framebuffer
            framebuffer.clear(&mut window, &raylib_thread);
            
            // Renderizar el juego
            game.render(&mut framebuffer, &mut window, &raylib_thread);
            
            // Intercambiar buffers
            framebuffer.swap_buffers(&mut window, &raylib_thread);
        }

        // Pequeña pausa para controlar la velocidad
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
