
pub struct Board{
    state: [[i8; 3]; 3],
    player_sign: i8,
    winner: i8,
} 

impl Board {
    pub fn new(player_sign: i8) -> Self{
        let state:[[i8; 3]; 3]  = [[0; 3]; 3];
        Self { state: state, player_sign: player_sign, winner: 0 }
    }

    pub fn print_board(&self){
        for i in self.state{
            print!("-------------\n");
            for j in i {
                if j == 0{
                    print!("|   ");
                }
                else if j == 1 {
                    print!("| O ");
                }
                else {
                    print!("| X ");
                }
            }
            print!("|\n");
        }
        print!("-------------\n");

    }

    pub fn set_symbol(&mut self, x:usize, y:usize, symbol: i8 ){
        self.state[x][y] = symbol; 
    }

    pub fn check_row_winner(&self, row: &[i8]) {
        let first =true;
        for i in row{

        }
    }

    pub fn check_coll_winner(&self) {
        
    }

    pub fn check_for_winner(&self){

    }

}