use graphics::{types::Color, Context, Rectangle,rectangle::{Shape, Border}};
use opengl_graphics::GlGraphics;

use crate::{PLANK_LENGTH,PLANK_WIDTH,COLUMN_LENGTH,PLANK_NUMBER, COLUMN_WIDTH};

#[derive(Debug)]
pub struct Plank{
    color: Color,
    pub size: f64,
    pub movable: bool,    
    pub rect: [f64;4],
}

impl Plank {
    pub fn new(size:f64,color:Color,pos:[f64;2]) -> Plank {
        Plank {
            size,
            color,
            movable: false,
            rect:   [pos[0] + (COLUMN_LENGTH - PLANK_LENGTH*size)/2.0,
                    pos[1] + COLUMN_WIDTH - PLANK_WIDTH*(PLANK_NUMBER + 2.0 - size),
                    PLANK_LENGTH*size,
                    PLANK_WIDTH],
        // PLANK_WIDTH*((size - 1) as f64)
        }
    }

    pub fn draw(&self,c: &Context, g: &mut GlGraphics) {
        let r = Rectangle{color: self.color, shape: Shape::Square,border:None};
        r.draw(self.rect, &c.draw_state, c.transform, g)
    }
}

pub struct Column {
    pub planks: Vec<Plank>,
    pub selected: bool,
    border_color: Color,
    background_color: Color,
    selected_border_color: Color,
    pub rect: [f64;4],
}

impl Column {
    pub fn new(n:f64,init:bool,rect:[f64;4]) -> Column {
        let colors = Column::colors_gen(n as i32);
        let def = [0.2,0.2,0.2,1.0];
        let mut col: Vec<Plank> = Vec::new();
        col.push(Plank::new(n+1.0,def,[rect[0],rect[1]]));
        if init {
            let mut i = n;
            for el in colors {
                col.push(Plank::new(i, el,[rect[0],rect[1]]));
                i -= 1.0;
            }
            for plank in &col{
                println!("{:?}, pos: {:?}, col_l:{},pla_l:{},pla_w:{},pla_n:{}",plank.rect,[rect[0],rect[1]],COLUMN_LENGTH,PLANK_LENGTH,PLANK_WIDTH,PLANK_NUMBER);

            }
        }
        Column {
            planks: col,
            selected: false,
            border_color: [0.0,0.0,0.0,1.0],
            background_color: [1.0;4],
            selected_border_color: [0.4;4],
            rect,
        }
    }       

    pub fn draw(&self,c:&Context,g: &mut GlGraphics, sel: bool) {
        let cl = match sel{
            true => self.selected_border_color,
            false => self.border_color,
        };
        let r = Rectangle{color:self.background_color,shape: Shape::Square,border: Some(Border{color:cl, radius: 2.0})};
        r.draw(self.rect, &c.draw_state, c.transform, g);
        for plank in &self.planks {
            plank.draw(c, g);
            //println!("{:?}",plank.size);
        }
    }

    pub fn remove_top(&mut self) -> Plank{
        let mut plk = self.planks.pop().unwrap();
        plk.rect[1] = self.rect[1];
        println!("{:?}",plk.rect);
        plk
    }

    pub fn insert_top(&mut self, mut plk: Plank){
        plk.rect[1] = self.rect[1] + COLUMN_WIDTH - PLANK_WIDTH*(self.planks.len() as f64 + 1.0);
        self.planks.push(plk);
    }

    fn colors_gen(n: i32) -> Vec<Color> {
        let mut colors = Vec::new();
        let l = (n+1) as f32;
        for i in 1..(n+1) {
            if i as f32 <=  l/3.0 {
                let a = 3.0 * ((i as f32)/l);
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
}

pub enum Selection {
    Left,
    Centre,
    Right,
    Null,
}

pub struct GameState {
    pub left_c: Column,
    pub centre_c: Column,
    pub right_c: Column,
    pub sel_c: Selection,
}

impl GameState {
    pub fn new(pos:[f64;2]) -> GameState{
        let left_c = Column::new(PLANK_NUMBER,true, [
            pos[0],pos[1],COLUMN_LENGTH,COLUMN_WIDTH
        ]);
        let centre_c = Column::new(PLANK_NUMBER,false,[
            pos[0] + COLUMN_LENGTH, pos[1], COLUMN_LENGTH,COLUMN_WIDTH
        ]);
        let right_c = Column::new(PLANK_NUMBER,false,[
            pos[0] + 2.0*COLUMN_LENGTH, pos[1], COLUMN_LENGTH,COLUMN_WIDTH
        ]);

        GameState {
            left_c,
            centre_c,
            right_c,    
            sel_c: Selection::Null
        }
    }

    pub fn draw(&self, c: &Context, g: &mut GlGraphics) {
        let (c1,c2,c3) = match self.sel_c {
            Selection::Left   => (true,false,false),
            Selection::Centre => (false,true,false), 
            Selection::Right  => (false,false,true),
            Selection::Null => (false,false,false), 
        };
        
        self.left_c.draw(c, g, c1);
        self.centre_c.draw(c, g, c2);
        self.right_c.draw(c, g, c3);

    }
}