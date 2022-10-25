use graphics::types::Color;

///stores information of a plank
pub struct Plank {
    ///Stores the size of the plank
    ///size relative to the smalles plank
    size: u8,
    movable: bool,
    color: Color,
}


impl Plank {
    ///creates a new plank of default settings
    pub fn new(size: u8, color: Color) -> Plank{
        Plank
        {
            size,
            movable: false,
            color,
        }
    }
}

/// The 'GameScreen' stores the planks in each column
pub struct GameScreen {
    col1: Vec<Plank>,
    col2: Vec<Plank>,
    col3: Vec<Plank>,
}

impl GameScreen {
    pub fn new(no_planks: i32)-> GameScreen{

        let mut col1: Vec<Plank> = Vec::new();
        let mut col2:Vec<Plank> = Vec::new();
        let mut col3:Vec<Plank> = Vec::new();

        //push base plank of max length into each column
        //length of base plank is no_planks +1
        let def = [0.2,0.2,0.2,1.0];

        col1.push(Plank::new((no_planks+1) as u8, def));
        col2.push(Plank::new((no_planks+1) as u8, def));
        col3.push(Plank::new((no_planks+1) as u8, def));


        let vc = GameScreen::colors_gen(no_planks);
        //push (no_planks) Planks of descending lenths into col1
        let mut i = no_planks ;
        for el in vc {
            col1.push(Plank::new(i as u8, el));
            i -= 1;
        }

        GameScreen {
            col1,
            col2,
            col3
        }
    
    }

    fn colors_gen(n: i32) -> Vec<Color> {
        let mut colors = Vec::new();
        let l = (n+1) as f32;
        for i in 1..(n+1) {
            if i as f32 <=  l/3.0 {
                let a = 3.0 as f32 * ((i as f32)/l);
                colors.push([0.0,1.0-a,a,1.0]);
            }
            else if i as f32 > l/3.0 && i as f32  <= (2.0/3.0)*l {
                let a = 3.0*(i as f32 - l/3.0)/l;
                colors.push([a,0.0,1.0-a,1.0]); 
            }
            else if i as f32 > ((2.0/3.0))*l {
                let a = 3.0*(i as f32 -2.0*l/3.0)/l;
                colors.push([1.0-a,a,0.0,1.0]);
            }
        }
        colors
    }

    //temp functions
    pub fn read(&self) {
        for i in &self.col1 {
            print!("size: {}, color: {:?}",(*i).size,(*i).color);
        }
    }

}