use std::collections::LinkedList;

pub struct GameKeyPoints {
    pub old_tail: Option<(i32, i32)>, // None on growth
    pub old_head: (i32, i32),
    pub new_head: (i32, i32),
    pub food: (i32, i32)
}

pub struct Game {
    snake: LinkedList<(i32, i32)>,
    food: (i32, i32),
    w: u32,
    h: u32,
    direction: (i32, i32)
}

impl Game {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            snake: LinkedList::from([(0, 0), (0, 1), (0, 2)]),
            food: (w as i32 / 2, h as i32 / 2),
            w,
            h,
            direction: (1, 0),
        }
    }

    /// **Returns**: (gameover, keypoints)
    pub fn tick(&mut self) -> (bool, GameKeyPoints) {
        let old_head = self.snake.back().unwrap().clone();
        let mut new_head = (old_head.0 + self.direction.0, old_head.1 + self.direction.1);

        if new_head.0 < 0 {
            new_head.0 = self.w as i32 - 1;
        } else if new_head.0 >= self.w as i32 {
            new_head.0 = 0;
        }

        if new_head.1 < 0 {
            new_head.1 = self.h as i32 - 1;
        } else if new_head.1 >= self.h as i32 {
            new_head.1 = 0;
        }

        let new_head = new_head; // make new_head immutable

        let gameover = self.snake.contains(&new_head);

        self.snake.push_back(new_head);

        // growth
        let old_tail = if new_head == self.food {
            None
        } else {
            Some(self.snake.pop_front().unwrap())
        };

        (
            gameover,
            GameKeyPoints {old_tail, old_head, new_head, food: self.food}
        )
    }

    pub fn width(&self) -> u32 { self.w }
    pub fn height(&self) -> u32 { self.h }
}
