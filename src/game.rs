use std::collections::{VecDeque, LinkedList};

pub struct Game {
    snake: LinkedList<(i32, i32)>,
    food: (i32, i32),
    w: u32,
    h: u32,
}

impl Game {
    pub fn new(w: u32, h: u32) -> Self {
        let snake = LinkedList::from([(0, 0), (0, 1), (0, 2)]);
        let food = (w as i32 / 2, h as i32 / 2);

        Self {
            snake,
            food,
            w,
            h,
        }
    }

    /// **Returns**: (gameover, old_tail, new_head)
    pub fn tick(&mut self) -> (bool, (i32, i32), (i32, i32)) {
        let old_head = self.snake.back().unwrap();
        let new_head = (old_head.0 + 1, old_head.1);
        self.snake.push_back(new_head);

        let old_tail = self.snake.pop_front().unwrap();

        (false, old_tail, new_head)
    }

    pub fn width(&self) -> u32 { self.w }
    pub fn height(&self) -> u32 { self.h }
}