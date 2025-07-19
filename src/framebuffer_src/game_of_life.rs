use raylib::prelude::*;
use crate::framebuffer_src::framebuffer::Framebuffer;

pub struct GameOfLife {
    width: u32,
    height: u32,
    current_generation: Vec<Vec<bool>>,
    next_generation: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let current_generation = vec![vec![false; height as usize]; width as usize];
        let next_generation = vec![vec![false; height as usize]; width as usize];
        
        GameOfLife {
            width,
            height,
            current_generation,
            next_generation,
        }
    }

    pub fn init_with_pattern(&mut self, pattern: &[(i32, i32)], offset_x: i32, offset_y: i32) {
        for &(x, y) in pattern {
            let px = offset_x + x;
            let py = offset_y + y;
            
            if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                self.current_generation[px as usize][py as usize] = true;
            }
        }
    }

    pub fn update(&mut self) {
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let alive = self.current_generation[x as usize][y as usize];
                let neighbors = self.count_live_neighbors(x, y);
                
                self.next_generation[x as usize][y as usize] = match (alive, neighbors) {
                    (true, n) if n < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, n) if n > 3 => false,
                    (false, 3) => true,
                    _ => alive,
                };
            }
        }
        
        std::mem::swap(&mut self.current_generation, &mut self.next_generation);
    }

    fn count_live_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = (x + dx + self.width as i32) % self.width as i32;
                let ny = (y + dy + self.height as i32) % self.height as i32;
                
                if self.current_generation[nx as usize][ny as usize] {
                    count += 1;
                }
            }
        }
        
        count
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let alive = self.current_generation[x as usize][y as usize];
                framebuffer.set_cell(window, raylib_thread, x, y, alive);
            }
        }
    }
}

pub mod patterns {
    // Still lifes
    pub fn block() -> Vec<(i32, i32)> {
        vec![(0, 0), (1, 0), (0, 1), (1, 1)]
    }
    
    pub fn beehive() -> Vec<(i32, i32)> {
        vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)]
    }
    
    pub fn loaf() -> Vec<(i32, i32)> {
        vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)]
    }
    
    pub fn tub() -> Vec<(i32, i32)> {
        vec![(1, 0), (0, 1), (2, 1), (1, 2)]
    }
    
    // Oscillators
    pub fn blinker() -> Vec<(i32, i32)> {
        vec![(0, 0), (0, 1), (0, 2)]
    }
    
    pub fn toad() -> Vec<(i32, i32)> {
        vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]
    }
    
    pub fn beacon() -> Vec<(i32, i32)> {
        vec![(0, 0), (1, 0), (0, 1), 
             (3, 2), (2, 3), (3, 3)]
    }
    
    pub fn pulsar() -> Vec<(i32, i32)> {
        vec![
            (2, 0), (3, 0), (4, 0),
            (0, 2), (5, 2),
            (0, 3), (5, 3),
            (0, 4), (5, 4),
            (2, 5), (3, 5), (4, 5),
            (2, 7), (3, 7), (4, 7),
            (0, 8), (5, 8),
            (0, 9), (5, 9),
            (0, 10), (5, 10),
            (2, 12), (3, 12), (4, 12)
        ]
    }
    
    pub fn pentadecathlon() -> Vec<(i32, i32)> {
        vec![
            (1, 0), (2, 0), (0, 1), (3, 1),
            (1, 2), (2, 2),
            (1, 3), (2, 3),
            (1, 4), (2, 4),
            (1, 5), (2, 5),
            (0, 6), (3, 6),
            (1, 7), (2, 7)
        ]
    }
    
    // Spaceships
    pub fn glider() -> Vec<(i32, i32)> {
        vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)]
    }
    
    pub fn lwss() -> Vec<(i32, i32)> { // Lightweight spaceship
        vec![(0, 0), (3, 0), (4, 1), (0, 2), (4, 2), 
            (1, 3), (2, 3), (3, 3), (4, 3)]
    }
    
    pub fn mwss() -> Vec<(i32, i32)> { // Middleweight spaceship
        vec![
            (1, 0), (4, 0),
            (0, 1), (5, 1),
            (0, 2), (5, 2),
            (1, 3), (2, 3), (3, 3), (4, 3), (5, 3)
        ]
    }
    
    pub fn hwss() -> Vec<(i32, i32)> { // Heavyweight spaceship
        vec![
            (1, 0), (5, 0),
            (0, 1), (6, 1),
            (0, 2), (6, 2),
            (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3)
        ]
    }
}
