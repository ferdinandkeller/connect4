use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

const GRID_WIDTH: u8 = 7;
const GRID_HEIGHT: u8 = 6;
const MAX_RECURSION_DEPTH: u8 = 12;

// [● 0 computer] | [○ 1 ennemy]

// entry of the program
fn main() {
  let mut grid: u128 = 0;
  let mut known_moves: HashMap<u128, (u8, i32)> = HashMap::new();

  display_grid(grid);
  while is_winning_grid(grid) == 0 {
    let (computer_move, _) = solver(grid, MAX_RECURSION_DEPTH, false, &mut known_moves);
    grid = play(grid, computer_move, false);
    display_grid(grid);
  
    let (computer_move, _) = solver(grid, MAX_RECURSION_DEPTH, false, &mut known_moves);
    grid = play(grid, computer_move, true);
    display_grid(grid);
  }
}

// ask move to player
fn get_player_move() -> u8 {
  let mut buf = String::new();
  let mut m: u8;
  'mainloop: loop {
    print!("> ");
    match stdout().flush() {
      Ok(_) => {},
      Err(_) => {}
    }
    match stdin().read_line(&mut buf) {
      Ok(_) => {
        match buf.trim().parse() {
          Ok(val) => {
            m = val;
            if m < GRID_WIDTH {
              break 'mainloop;
            }
          },
          Err(_) => {}
        }
      },
      Err(_) => {}
    }
  }
  return m;
}

// solving function
fn solver(grid: u128, recursion_index: u8, player: bool, known_moves: &mut HashMap<u128, (u8, i32)>) -> (u8, i32) {
  if recursion_index == 0 {
    return (3, is_winning_grid(grid));
  } else {
    let mut defined = false;
    let mut best_move: u8 = 3;
    let mut best_score: i32 = 0;
    if known_moves.contains_key(&grid) {
      match known_moves.get(&grid) {
        Some(val) => return *val,
        None => {} 
      }
    }
    for m in get_valid_moves(grid).iter() {
      if *m == -1 {
        break;
      } else {
        let new_grid = play(grid, *m as u8, player);
        let res = is_winning_grid(grid);
        if res != 0 {
          known_moves.insert(grid, (*m as u8, res));
          return (*m as u8, res);
        }
        let (_, new_score) = solver(new_grid, recursion_index - 1, !player, known_moves);
        if !defined || (player && new_score < best_score) || (!player && new_score > best_score) {
          defined = true;
          best_score = new_score;
          best_move = *m as u8;
        }
      }
    }
    known_moves.insert(grid, (best_move, best_score));
    return (best_move, best_score);
  }
}

// test if the grid is a winning one
fn is_winning_grid(grid: u128) -> i32 {
  let computer: u64 = ((grid & !(grid>>GRID_WIDTH*GRID_HEIGHT)) % (1<<GRID_WIDTH*GRID_HEIGHT)) as u64;
  let ennemy: u64 = ((grid & (grid>>GRID_WIDTH*GRID_HEIGHT)) % (1<<GRID_WIDTH*GRID_HEIGHT)) as u64;

  // check vertical
  if computer &
    ((computer&0b000000011111111111111111111111111111111111)<<7) &
    ((computer&0b000000000000001111111111111111111111111111)<<14) &
    ((computer&0b000000000000000000000111111111111111111111)<<21) != 0 {
    return GRID_WIDTH as i32 * GRID_HEIGHT as i32;
  }
  if ennemy &
    ((ennemy&0b000000011111111111111111111111111111111111)<<7) &
    ((ennemy&0b000000000000001111111111111111111111111111)<<14) &
    ((ennemy&0b000000000000000000000111111111111111111111)<<21) != 0 {
    return - (GRID_WIDTH as i32 * GRID_HEIGHT as i32);
  }

  // check horizontal
  if computer &
    ((computer&0b011111101111110111111011111101111110111111)<<1) & 
    ((computer&0b001111100111110011111001111100111110011111)<<2) & 
    ((computer&0b000111100011110001111000111100011110001111)<<3) != 0 {
    return GRID_WIDTH as i32 * GRID_HEIGHT as i32;
  }
  if ennemy &
    ((ennemy&0b011111101111110111111011111101111110111111)<<1) & 
    ((ennemy&0b001111100111110011111001111100111110011111)<<2) & 
    ((ennemy&0b000111100011110001111000111100011110001111)<<3) != 0 {
    return - (GRID_WIDTH as i32 * GRID_HEIGHT as i32);
  }

  // check vertical droite
  if computer &
    ((computer&0b000000001111110111111011111101111110111111)<<8) &
    ((computer&0b000000000000000011111001111100111110011111)<<16) &
    ((computer&0b000000000000000000000000111100011110001111)<<24) != 0 {
    return GRID_WIDTH as i32 * GRID_HEIGHT as i32;
  }
  if ennemy &
    ((ennemy&0b000000001111110111111011111101111110111111)<<8) &
    ((ennemy&0b000000000000000011111001111100111110011111)<<16) &
    ((ennemy&0b000000000000000000000000111100011110001111)<<24) != 0 {
    return - (GRID_WIDTH as i32 * GRID_HEIGHT as i32);
  }

  // check vertical gauche
  if computer &
    ((computer&0b000000011111101111110111111011111101111110)<<6) &
    ((computer&0b000000000000001111100111110011111001111100)<<12) &
    ((computer&0b000000000000000000000111100011110001111000)<<18) != 0 {
    return GRID_WIDTH as i32 * GRID_HEIGHT as i32;
  }
  if ennemy &
    ((ennemy&0b000000011111101111110111111011111101111110)<<6) &
    ((ennemy&0b000000000000001111100111110011111001111100)<<12) &
    ((ennemy&0b000000000000000000000111100011110001111000)<<18) != 0 {
    return - (GRID_WIDTH as i32 * GRID_HEIGHT as i32);
  }

  return 0;
}

// get a list of all valid moves
fn get_valid_moves(grid: u128) -> [i8; GRID_WIDTH as usize] {
  let mut valid_moves = [-1; GRID_WIDTH as usize];
  let mut index = 0;
  for m in [3, 2, 4, 1, 5, 0, 6].iter() {
    if grid & (1<<*m) == 0 {
      valid_moves[index] = *m;
      index += 1;
    }
  }
  valid_moves
}

// play a given move
fn play(grid: u128, pos: u8, player: bool) -> u128 {
  let mut index: u128 = 1 << pos + GRID_WIDTH*(GRID_HEIGHT-1);
  while index & grid != 0 {
    index >>= 7;
  }
  grid | index | if player { index << GRID_WIDTH*GRID_HEIGHT } else { 0 }
}

// display a grid to console
fn display_grid(grid: u128) {
  let mut grid_str = String::new();
  let mut grid = grid;
  for _ in 0..GRID_HEIGHT {
    for _ in 0..GRID_WIDTH {
      match grid&1 {
        0 => grid_str.push_str("· "),
        _ => {
          match grid & (1 << GRID_WIDTH*GRID_HEIGHT){
            0 => grid_str.push_str("● "),
            _ => grid_str.push_str("○ ")
          }
        }
      }
      grid >>= 1;
    }
    grid_str.push_str("\n");
  }
  println!("{}", grid_str);
}