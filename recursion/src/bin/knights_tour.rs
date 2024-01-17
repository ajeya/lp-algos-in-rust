use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: [[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    if num_visited == (INUM_ROWS * INUM_COLS) {
        if !REQUIRE_CLOSED_TOUR {
            return true;
        }
        for item in offsets.iter().take(8) {
            let (new_row, new_col) = (cur_row + item[0], cur_col + item[1]);

            if !(0..INUM_ROWS).contains(&new_row) || !(0..INUM_COLS).contains(&new_col) {
                continue;
            }

            if board[new_row as usize][new_col as usize] == 0 {
                return true;
            }

            return false;
        }
    }

    for item in offsets.iter().take(8) {
        let (new_row, new_col) = (cur_row + item[0], cur_col + item[1]);

        if !(0..INUM_ROWS).contains(&new_row) || !(0..INUM_COLS).contains(&new_col) {
            continue;
        }

        if board[new_row as usize][new_col as usize] != UNVISITED {
            continue;
        }

        board[new_row as usize][new_col as usize] = num_visited;
        if find_tour(board, offsets, new_row, new_col, num_visited + 1) {
            return true;
        };
        board[new_row as usize][new_col as usize] = UNVISITED;
    }

    false
}

fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
    for i in board.iter().take(NUM_ROWS) {
        for item in i.iter().take(NUM_COLS) {
            print!("{:0>2} ", item)
        }
        println!()
    }
}

fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
        dump_board(&mut board);
    } else {
        println!("Could not find a tour.");
    }
}
