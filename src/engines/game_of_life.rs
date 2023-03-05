
pub const DEFAULT_X_SIZE: usize = 201; // -100 -> +100
pub const DEFAULT_Y_SIZE: usize = 201; // -100 -> +100


pub struct GOL {
    x_range: usize,
    y_range: usize,
    state: Vec<Vec<bool>>,
}

impl Default for GOL {
    fn default() -> GOL {
        GOL {
            x_range: DEFAULT_X_SIZE,
            y_range: DEFAULT_Y_SIZE,
            state: vec![vec![false; DEFAULT_Y_SIZE]; DEFAULT_X_SIZE],
        }
    }
}

impl GOL {
    /// convert coordinates (-inf -> +inf) to indices (0 -> +inf)
    fn coords_2_index(&self, x: i32, y:i32) -> Result<(usize, usize), String>{
        if x.abs() > (self.x_range / 2) as i32 {
            return Err(format!("X is out of Bounds {}", x));
        }
        else if y.abs() > (self.y_range / 2) as i32 {
            return Err(format!("Y is out of bounds {}", y));
        }

        let x_indx: usize;
        let y_indx: usize;
        if x >= 0 {
            x_indx = (self.x_range / 2) + ( x as usize);
        }
        else{
            x_indx = (self.x_range / 2) - (-x as usize);
        }
        if y >= 0 {
            y_indx = (self.y_range / 2) + ( y as usize);
        }
        else{
            y_indx = (self.y_range / 2) - (-y as usize);
        }
        Ok((x_indx, y_indx))
    }

    fn index_2_coords(&self, x_ind: usize, y_ind: usize) -> Result<(i32, i32), String> {
        if x_ind >= self.x_range {
            return Err(format!("X index is out of Bounds {}", x_ind));
        }
        if y_ind >= self.y_range {
            return Err(format!("Y index is out of bounds {}", y_ind));
        }

        let x_coord: i32;
        let y_coord: i32;


        if x_ind > (self.x_range / 2) {
            x_coord = (x_ind - (self.x_range / 2)) as i32;
        }
        else{
            x_coord = ((self.x_range / 2) - x_ind) as i32 * -1;
        }
        if y_ind > (self.y_range / 2) {
            y_coord = (y_ind - (self.y_range / 2)) as i32;
        }
        else{
            y_coord = ((self.y_range / 2) - y_ind) as i32 * -1;
        }
        

        Ok((x_coord, y_coord))
    }

    #[allow(dead_code)]
    fn get_value(&mut self, x: i32, y: i32) -> Result<bool, String> {
        let (x_indx, y_indx): (usize, usize) = self.coords_2_index(x, y)?;
        Ok(self.state[x_indx][y_indx])
    }

    fn set_value(&mut self, x: i32, y: i32, value: bool)  -> Result<(), String> {
        let (x_indx, y_indx): (usize, usize) = self.coords_2_index(x, y)?;
        self.state[x_indx][y_indx] = value;

        Ok(())
    }

    fn set_true(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.set_value(x, y, true)?;
        Ok(())
    }

    fn neighbour_count(&mut self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                // if out of bounds, ignore count
                let value: bool;
                match self.get_value(x+i, y+j) {
                    Ok(i) => {value = i;},
                    Err(_) => continue
                };
                count = count + (value as i32);
            }
        }
        count
    }

    pub fn alive_count(&mut self) -> i32 {
        let mut count = 0;
        let max_x = (self.x_range/2) as i32;
        let max_y = (self.y_range/2) as i32;
        for x in -max_x..=max_x {
            for y in -max_y..=max_y {
                let value: bool = self.get_value(x, y).unwrap();
                count = count + (value as i32);
            }
        }
        count
    }

    /// clone of current state of game
    pub fn current_state(&mut self) -> Vec<Vec<bool>> {
        self.state.clone()
    }

    /// Single iteration of game of life
    pub fn iterate(&mut self) {
        // create new state object, this will replace old state object
        let mut next_state = vec![vec![false; self.y_range]; self.x_range];

        for x_indx in 0..self.x_range {
            for y_indx in 0..self.y_range {
                let x: i32;
                let y: i32;
                (x, y) = self.index_2_coords(x_indx, y_indx).unwrap();
                let n_count: i32 = self.neighbour_count(x as i32, y as i32);

                if self.state[x_indx][y_indx] {
                    // currently alive
                    if n_count == 2 || n_count == 3 {
                        next_state[x_indx][y_indx] = true;
                    }
                }
                else {
                    // currently dead
                    if n_count == 3 {
                        next_state[x_indx][y_indx] = true;
                    }
                }
            }
        }

        self.state = next_state;
        
    }
}


/// instansiate R-Pentomino game of life
pub fn r_pentomino() -> GOL {
    // Create game of life with all values defaulting to false
    let mut game_of_life: GOL = GOL {..Default::default()};

    // R-Pentomino
    game_of_life.set_true(30, 30).unwrap();
    game_of_life.set_true(30, 31).unwrap();
    game_of_life.set_true(30, 32).unwrap();

    game_of_life.set_true(29, 31).unwrap();
    game_of_life.set_true(31, 32).unwrap();

    game_of_life
}


#[cfg(test)]
mod tests {
    use super::*;

    // index_2_coords
    #[test]
    fn index_to_coords_1() {
        let game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.index_2_coords(0, 0).unwrap(), (-100, -100));
    }

    #[test]
    fn index_to_coords_2() {
        let game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.index_2_coords(100, 100).unwrap(), (0, 0));
    }

    #[test]
    fn index_to_coords_3() {
        let game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.index_2_coords(200, 200).unwrap(), (100, 100));
    }

    // coords_2_index
    #[test]
    fn coords_to_index_1() {
        let game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.coords_2_index(-100, -100).unwrap(), (0, 0));
    }

    #[test]
    fn coords_to_index_2() {
        let game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.coords_2_index(0, 0).unwrap(), (100, 100));
    }

    #[test]
    fn coords_to_index_3() {
        let game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.coords_2_index(100, 100).unwrap(), (200, 200));
    }

    // neighbour_count
    #[test]
    fn neighbour_count_1() {
        let mut game_of_life = r_pentomino();
        assert_eq!(game_of_life.neighbour_count(30, 30), 2);
    }

    #[test]
    fn neighbour_count_2() {
        let mut game_of_life = r_pentomino();
        assert_eq!(game_of_life.neighbour_count(30, 31), 4);
    }

    #[test]
    fn neighbour_count_3() {
        let mut game_of_life = r_pentomino();
        assert_eq!(game_of_life.neighbour_count(29, 31), 3);
    }

    #[test]
    fn neighbour_count_4() {
        let mut game_of_life = r_pentomino();
        assert_eq!(game_of_life.neighbour_count(20, 20), 0);
    }

    #[test]
    fn neighbour_count_5() {
        let mut game_of_life = r_pentomino();
        assert_eq!(game_of_life.neighbour_count(31, 31), 4);
    }

    // Others

    #[test]
    fn gol_00() {
        let mut game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.get_value(0, 0).unwrap(), false);
    }

    #[test]
    fn gol_extreme_positive() {
        let mut game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.get_value(100, 100).unwrap(), false);
    }

    #[test]
    fn gol_extreme_negative() {
        let mut game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.get_value(-100, -100).unwrap(), false);
    }

    #[test]
    fn gol_out_of_bounds() {
        let mut game_of_life: GOL = GOL {..Default::default()};
        assert_eq!(game_of_life.get_value(110, 80), Err(String::from("X is out of Bounds 110")));
    }

    #[test]
    fn gol_values() {
        let mut game_of_life: GOL = GOL {..Default::default()};
        game_of_life.set_value(-20, 30, true).unwrap();
        assert_eq!(game_of_life.get_value(-20, 30).unwrap(), true);
    }

    #[test]
    fn gol_iteration() {
        let mut game_of_life = r_pentomino();

        game_of_life.iterate();
        assert_eq!(game_of_life.get_value(29, 30).unwrap(), true);
        assert_eq!(game_of_life.get_value(30, 30).unwrap(), true);
        assert_eq!(game_of_life.get_value(31, 30).unwrap(), false);

        assert_eq!(game_of_life.get_value(29, 31).unwrap(), true);
        assert_eq!(game_of_life.get_value(30, 31).unwrap(), false);
        assert_eq!(game_of_life.get_value(31, 31).unwrap(), false);

        assert_eq!(game_of_life.get_value(29, 32).unwrap(), true);
        assert_eq!(game_of_life.get_value(30, 32).unwrap(), true);
        assert_eq!(game_of_life.get_value(31, 32).unwrap(), true);

        assert_eq!(game_of_life.get_value(29, 33).unwrap(), false);
        assert_eq!(game_of_life.get_value(30, 33).unwrap(), false);
        assert_eq!(game_of_life.get_value(31, 33).unwrap(), false);
        
    }
}