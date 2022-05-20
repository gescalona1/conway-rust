use std::cmp::{max, min};
use std::{thread, time};

struct Board {
    max_height: usize,
    max_width: usize,
    vect: Vec<Vec<i32>>
}

struct Iter<'a> {
    board: &'a Board,
    h_index: usize,
    w_index: usize
}

impl Board {
    fn new(h: usize, w: usize) -> Board {
        let mut t: Vec<Vec<i32>> = Vec::new();
        let mut h_0: usize = 0;
        loop {
            if h_0 > h {
                break;
            }
            let mut new_h: Vec<i32> = Vec::new();
            let mut w_0: usize = 0;
            loop {
                if w_0 > w {
                    break;
                }
                new_h.push(0); //default that cells are dead
                w_0 += 1;
            }
            t.push(new_h);
            h_0 += 1;
        }
        Board {
            max_height: h,
            max_width: w,
            vect: t
        }
    }

    fn cell(&self, x: usize, y: usize) -> i32 {
        self.vect[x][y]
    }

    fn update(&mut self, x: usize, y: usize, val: i32) {
        self.vect[x][y] = val;
    }

    fn iter(&self) -> Iter {
        Iter {
            board: &self,
            h_index: 0,
            w_index: 0
        }
    }

    fn neighbor_iter(&self, x: usize, y: usize) -> impl Iterator<Item = i32> + '_ {
        struct NeighborIter<'a> {
            vect: &'a Board,
            start_x: usize,
            start_y: usize,
            curr_x: usize,
            curr_y: usize,
            end_x: usize,
            end_y: usize,
            center_x: usize,
            center_y: usize
        }
        impl<'a> Iterator for NeighborIter<'a>  {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.curr_x == self.center_x && self.curr_y == self.center_y {
                    self.curr_x += 1;
                } else if self.curr_x > self.end_x {
                    self.curr_x = self.start_x;
                    self.curr_y += 1
                }

                if self.curr_y <= self.end_y {
                    let result = self.vect.vect[self.curr_x][self.curr_y];
                    self.curr_x += 1;
                    Some(result)
                } else {
                    None
                }
            }
        }
        let s_x: usize = max((x as i32) - 1, 0) as usize;
        let s_y: usize = max((y as i32) - 1, 0) as usize;

        let e_x: usize = min(x + 1, self.max_height - 1);
        let e_y: usize = min(y + 1, self.max_width - 1);

        NeighborIter {
            vect: &self,
            start_x: s_x,
            start_y: s_y,
            curr_x: s_x,
            curr_y: s_y,
            end_x: e_x,
            end_y: e_y,
            center_x: x as usize,
            center_y: y as usize
        }
    }

    fn print(&self) {
        let convert_value = |v: i32| match v {
            0 => '.',
            1 => 'x',
            2 => '\n',
            _ => unreachable!()
        };
        self.iter().for_each(|element| {
            print!("{}", convert_value(element));
        });
        println!();
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.w_index < self.board.max_width && self.h_index < self.board.max_height {
            let result: i32 = self.board.vect[self.h_index][self.w_index];
            self.w_index += 1;
            Some(result)
        } else if self.h_index >= self.board.max_height {
            None
        } else if self.w_index >= self.board.max_width {
            self.h_index += 1;
            self.w_index = 0;
            Some(2)
        } else {
            None
        }
    }
}
fn initial_state(board: &mut Board) {
    let x_offset = 5;
    let y_offset = 5;
    board.update(2 + x_offset, 3 + y_offset, 1);
    board.update(2 + x_offset, 4 + y_offset, 1);
    board.update(3 + x_offset, 3 + y_offset, 1);
    board.update(3 + x_offset, 4 + y_offset, 1);

    board.update(2 + x_offset, 10 + y_offset, 1);
    board.update(2 + x_offset, 11 + y_offset, 1);
    board.update(3 + x_offset, 10 + y_offset, 1);
    board.update(3 + x_offset, 11 + y_offset, 1);

    board.update(5 + x_offset, 7 + y_offset, 1);
    board.update(5 + x_offset, 8 + y_offset, 1);
    board.update(6 + x_offset, 7 + y_offset, 1);
    board.update(6 + x_offset, 8 + y_offset, 1);


    board.update(13 + x_offset, 34 + y_offset, 1);
    board.update(13 + x_offset, 35 + y_offset, 1);
    board.update(14 + x_offset, 34 + y_offset, 1);
    board.update(14 + x_offset, 35 + y_offset, 1);

    //end boxes



    board.update(11 + x_offset, 28 + y_offset, 1);
    board.update(11 + x_offset, 29 + y_offset, 1);
    board.update(12 + x_offset, 30 + y_offset, 1);
    board.update(13 + x_offset, 31 + y_offset, 1);
    board.update(14 + x_offset, 30 + y_offset, 1);
    board.update(15 + x_offset, 29 + y_offset, 1);




    board.update(11 + x_offset, 25 + y_offset, 1);
    board.update(11 + x_offset, 26 + y_offset, 1);

    board.update(12 + x_offset, 24 + y_offset, 1);
    board.update(13 + x_offset, 24 + y_offset, 1);
    board.update(14 + x_offset, 24 + y_offset, 1);
    board.update(14 + x_offset, 25 + y_offset, 1);
    board.update(14 + x_offset, 26 + y_offset, 1);

}
fn main() {
    println!("Hello, world!");
    let mut board: Board = Board::new(27, 90);
    initial_state(&mut board);
    let mut conway_tick = |boa: &mut Board| {
        let mut triples: Vec<(usize, usize, i32)>  = Vec::new();
        for x in 0_usize..(boa.max_height-1) {
            for y in 0_usize..(boa.max_width-1) {
                let neighbor_count = boa.neighbor_iter(x, y).filter(|&x| x == 1).count();
                if boa.cell(x, y) == 1 {
                    if neighbor_count < 2 {
                        triples.push((x, y, 0));
                    } else if 2 == neighbor_count || 3 == neighbor_count {
                    } else if neighbor_count > 3 {
                        triples.push((x, y, 0));
                    }
                } else if boa.cell(x, y) == 0 {
                    if neighbor_count == 3 {
                        triples.push((x, y, 1));
                    }
                }
            }
        }

        triples.iter().for_each(|element| {
            boa.update(element.0, element.1, element.2)
        });
    };

    board.print();
    conway_tick(&mut board);
    board.print();


    loop {
        let ten_millis = time::Duration::from_millis(10);
        {
            conway_tick(&mut board);
        }
        board.print();
        thread::sleep(ten_millis);
    }




}
