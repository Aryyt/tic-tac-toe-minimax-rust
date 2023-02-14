use core::num;
use colored::Colorize;
use std::collections::HashMap;
use std::vec::Vec;



pub struct Board{
    pub state: [[i8; 3]; 3],
    player_sign: i8,
    winner: i8,
    squares: HashMap<usize,(usize,usize)>,
} 

impl Board {
    pub fn new(player_sign: i8) -> Self{
        let state:[[i8; 3]; 3]  = [[0; 3]; 3];
        let mut squares: HashMap<usize,(usize,usize)> = HashMap::new();
        let mut key = 1;
        for i in 0..state.len(){
            for j in 0..state.len(){
                squares.insert(key, (i,j));
                key += 1;
            }
        }
        Self { state: state, player_sign: player_sign, winner: 0, squares: squares}
    }

    pub fn print_board(&self){
        let mut number = 1;
        for i in self.state{
            print!("-------------\n");
            for j in i {
                if j == 0{
                    print!("| {} ", number);
                    number += 1;
                }
                else if j == 1 {
                    print!("| {} ", "o".blue());
                    number += 1;
                }
                else {
                    print!("| {} ", "x".red());
                    number += 1;
                }
            }
            print!("|\n");
        }
        print!("-------------\n");

    }

    pub fn set_symbol(&mut self, suqare: usize, symbol: i8 ){
        match self.squares.get(&suqare) {
            None => println!("{} is not a valid square", suqare),
            Some(cords) => {
                self.state[cords.0][cords.1] = symbol; 
            },
        }
    }

    pub fn check_arr_for_winner(&self, arr: &[i8]) -> i8 {
        let curr = arr[0];
        if curr == 0{
            return curr;
        } 
        else {
            for i in 1..arr.len(){
                if curr != arr[i] {
                    return 0;
                }
            }
            return curr;
        }
    }

    pub fn get_colls(&self) -> [[i8; 3]; 3]{
        let mut new_mat: [[i8; 3]; 3] = [[0; 3]; 3];
        let mut nx = 0;
        for y in 0..self.state.len() {
            let mut ny = 0;
            for x in 0..self.state.len(){
                new_mat[nx][ny] = self.state[x][y] as i8;
                ny += 1;
            }
            nx += 1;
        }
        return new_mat;
    }

    pub fn get_diag(&self) {
        
    }

}