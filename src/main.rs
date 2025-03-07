use core::time;
use std::thread::sleep;

const WIDTH: usize = 25;
const HEIGHT: usize = 25;
const FPS: u64 = 5;

struct State {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl State {
    fn new(width: usize, height: usize) -> State {
        State {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    fn update(&mut self) {
        let mut new_state = self.cells.clone();

        for i in 0..self.height {
            for j in 0..self.width {
                let num_neighbours = self._num_of_alive_neighbours(i, j);

                if self.cells[i][j] {
                    if num_neighbours < 2 || num_neighbours > 3 {
                        new_state[i][j] = false;
                    }
                } else {
                    if num_neighbours == 3 {
                        new_state[i][j] = true;
                    }
                }
            }
        }

        self.cells = new_state;
    }

    fn set_cells(&mut self, input: Vec<(usize, usize)>) {
        for (i, j) in input {
            self.cells[i][j] = true;
        }
    }

    fn print_cells(&self) {
        for row in self.cells.clone() {
            for cell in row {
                if cell {
                    print!("■ ");
                } else {
                    print!("□ ");
                }
            }
            print!("\n");
        }
        print!("{esc}c", esc = 27 as char);
    }

    fn _num_of_alive_neighbours(&self, i: usize, j: usize) -> u32 {
        let mut count: u32 = 0;
        let offsets: Vec<(i32, i32)> = vec![(1, 1), (-1, -1), (1, -1), (-1 , 1), (1, 0), (-1, 0), (0, 1), (0, -1)];

        for (off_i, off_j) in offsets {
            if let (Some(idx_i), Some(idx_j)) = (i.checked_add_signed(off_i as isize), j.checked_add_signed(off_j as isize)) {
                if (idx_i < self.height - 1) && (idx_i > 0) && (idx_j < self.width - 1) && (idx_j > 0) {
                    if self.cells[idx_i][idx_j] { count += 1; }
                }
            }
        }

        return count;
    }
}

fn main() {
    let mut state: State = State::new(WIDTH, HEIGHT);

    state.set_cells(vec![(5,5), (6,6), (6,7), (5,7), (4, 7)]);

    loop {
        sleep(time::Duration::from_millis(1000 / FPS));
        state.update();
        state.print_cells();
    }
}
