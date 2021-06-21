
#[derive(Clone)]
pub struct Board {
    pub board_size:(usize, usize),
    pub current_state: Vec<Vec<char>>,
    pub new_state: Vec<Vec<char>>,
    pub ch: char,
}

impl Board {

    pub fn new(ch: char, size: (usize, usize)) -> Self {
        Self {
            board_size: size,
            current_state: vec![vec![ch; size.1]; size.0],
            new_state: vec![vec![ch; size.1]; size.0],
            ch: ch,
        }
    }

    pub fn update(&mut self) {

        self.current_state = self.new_state.clone();

    }

    pub fn find_neighbors(&self, spot:(usize, usize)) -> u8 {

        let neighbors = Neighbors {
            //top: Some(self.current_state[spot.0 + 1][spot.1]),
            top: if spot.0 + 1 < self.board_size.0 {

                    Some(self.current_state[spot.0 + 1][spot.1])

                } else {
                    None
                },
            bottom: if spot.0 > 0 {

                    Some(self.current_state[spot.0 - 1][spot.1])

                } else {
                    None
                },
            left: if spot.1 > 0 {

                    Some(self.current_state[spot.0][spot.1 - 1])

                } else {
                    None
                },
            right: if spot.1 + 1 < self.board_size.1 {

                    Some(self.current_state[spot.0][spot.1 + 1])

                } else {
                    None
                },
            top_left: if spot.0 + 1 < self.board_size.0 && spot.1 > 0 {

                    Some(self.current_state[spot.0 + 1][spot.1 - 1])

                }else {
                    None
                },
            top_right: if spot.0 + 1 < self.board_size.0 && spot.1 + 1 < self.board_size.1 {

                    Some(self.current_state[spot.0 + 1][spot.1 + 1])

                }else {
                    None
                },
            bottom_left: if spot.0 > 0 && spot.1 > 0 {

                    Some(self.current_state[spot.0 - 1][spot.1 - 1])

                }else {
                    None
                },
            bottom_right: if spot.0 > 0 && spot.1 + 1 < self.board_size.1 {

                Some(self.current_state[spot.0 - 1][spot.1 + 1])
                }else {
                    None
                },
        };

        let number_of_neighbors = neighbors.get_living_neighbors();

        number_of_neighbors
    }

    //changes to 0 or x by mouse click
    pub fn manual_change(&mut self, spot:(usize, usize)) {

        if spot.0 < self.board_size.0 && spot.1 < self.board_size.1 {

            if self.new_state[spot.0][spot.1] == '0' {

                self.new_state[spot.0][spot.1] = 'X';

            } else {

                self.new_state[spot.0][spot.1] = '0';
            }

            self.current_state = self.new_state.clone();

        } else {
            println!("out of bounds change")
        }
        
    }
    
    pub fn clear(&mut self) {

        self.new_state = vec![vec![self.ch; self.board_size.1]; self.board_size.0]
    }

    pub fn spawn_blinker(&mut self, spot:(usize, usize)) {

        self.new_state[spot.0][spot.1] = 'X';
        self.new_state[spot.0][spot.1 - 1] = 'X';
        self.new_state[spot.0][spot.1 - 2] = 'X';
    }

    pub fn spawn_tub(&mut self, spot:(usize, usize)) {

        self.new_state[spot.0][spot.1] = 'X';
        self.new_state[spot.0 - 1][spot.1 - 1] = 'X';
        self.new_state[spot.0][spot.1 - 2] = 'X';
        self.new_state[spot.0 + 1][spot.1 - 1] = 'X';
        
    }


}

struct Neighbors {
    top:Option<char>,
    bottom:Option<char>,
    left:Option<char>,
    right:Option<char>,
    top_left:Option<char>,
    top_right:Option<char>,
    bottom_left:Option<char>,
    bottom_right:Option<char>,
}

impl Neighbors {

    //needed to iterate over the struct
    fn get_array(self) -> Vec<Option<char>>{

        let mut array = Vec::new();

        array.push(self.top);
        array.push(self.bottom);
        array.push(self.left);
        array.push(self.right);
        array.push(self.top_left);
        array.push(self.top_right);
        array.push(self.bottom_left);
        array.push(self.bottom_right);

        array
    }

    fn get_living_neighbors(self) -> u8 {

        let mut living:u8 = 0;
        //needed to iterate over the struct
        let index = self.get_array();

        for n in index {

            match n {

                Some(p) => if p == 'X' {
                    living += 1
                }
                None => (),
            }
        }

        living
    }
}