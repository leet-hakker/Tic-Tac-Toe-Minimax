#[macro_use] extern crate text_io;
use array2d::Array2D;

#[derive(Default, Debug, Clone, Copy)]
enum CellState {
    Player,
    Bot,
    #[default] Empty
}


#[derive(Debug)]
struct BoardState {
    data: Array2D<CellState>,
    next_move: CellState,
}

impl BoardState {
    fn print_board(&self) {
        for col in self.data.columns_iter() {
            print!("|");
            for item in col {
                match item {
                    CellState::Player => { print!("X|") }
                    CellState::Bot => { print!("O|") }
                    CellState::Empty => { print!(" |") }
                }
            }
            println!();
        }
    }

    fn check_space_available(&self, row: &usize, column: &usize) -> bool {
        matches!(self.data.get(*column, *row).unwrap(), &CellState::Empty)
    }

    fn make_move(&mut self, row: &usize, column: &usize) -> bool {
        if !self.check_space_available(row, column) {
            false
        } else {
            self.data.set(*column, *row, self.next_move).ok();
            match self.next_move {
                CellState::Player => {self.next_move = CellState::Bot}
                CellState::Bot => {self.next_move = CellState::Player}
                _ => {}
            }
            true
        }
    } 
}

fn main() {
    let board_size: usize = 3;

    let mut board = BoardState{ data: Array2D::filled_with(CellState::Empty, board_size, board_size),
                            next_move: CellState::Player,
    };
    board.print_board();
    println!();

    for _i in 0..9 {
        let row: usize = read!("{}\n").parse::<usize>().unwrap(); 
        let column: usize = read!("{}\n").parse::<usize>().unwrap(); 

        board.make_move(&row, &column);
        
        board.print_board();
        println!();

    }
}
