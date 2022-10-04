use std::ops::Add;
use rand::Rng;

// Constants:
const MIN_BACTER_SIZE: f32 = 0.2;
const MAX_BACTER_SIZE: f32 = 1.0;


// POINTS in 2D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float2D{
    pub x : f64,
    pub y : f64,
}
impl Float2D
{
    // Another associated function, taking two arguments:
    fn new(x: f64, y: f64) -> Float2D {
        Float2D { x: x, y: y }
    }    

    fn multiply (&mut self, factor : f64) -> Float2D{
        Float2D::new(self.x * factor,self.y * factor)
    }

    fn distance_square (&self, other : Float2D) -> f64{
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }

    fn distance (&self, other : Float2D) -> f64{
        Float2D::distance_square(self, other).sqrt()
    }

    fn versor (&mut self, other : Float2D) -> Float2D{

        // TODO optimize this!
        Float2D{
            x: (self.x - other.x) / self.distance(other),
            y: (self.y - other.y) / self.distance(other),
        }
    }

    fn abs (&self) -> f64{
        self.distance(Float2D{x:0., y:0.})
    }

}
impl Add for Float2D{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// VECTORS in 2D
#[derive(Clone, Copy)]
pub struct Vector2D{
	pub pos : Float2D,
    pub vel : Float2D,
}
impl Add for Vector2D{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            pos: self.pos + other.pos,
            vel: self.vel + other.vel,
        }
    }
}


// Cell has all the properties. Since algae don't need to move, only the ones for static cells
// are mandatory.
pub trait Cell{
    // Without Default Implementations
    fn get_vector(&self) -> Vector2D;
    fn set_pos(&mut self, pos: Float2D);
    fn get_index(&self) -> i64;
    fn get_size(&self) -> f32;
}

// ALGA
#[derive(Clone, Copy)]
pub struct Alga{
    alga_vector : Vector2D, // TODO maybe a Float2D would work instead. -> half memory usage!
    food_value : f32,
    index : i64, // Maybe useless? TODO find if there's an implicit way to handle deletions.
    dead : bool,
}

impl Cell for Alga{
    fn get_vector(&self) -> Vector2D{
        self.alga_vector // TODO maybe a Float2D would work instead. -> half memory usage!
    }
    fn set_pos(&mut self, pos: Float2D){
        self.alga_vector.pos = pos;
    }
    fn get_index(&self) -> i64{
        self.index
    }
    fn get_size(&self) -> f32{
        0.1 //TODO MAGIC NUMBER
    }
}

impl Alga {
    // TODO is the index necessary?
    pub fn new_random(area_size : Float2D, index: i64) -> Alga{
        let mut rng = rand::thread_rng();
        Alga { 
            alga_vector: Vector2D{
                pos: Float2D{
                    x: (rng.gen::<f64>() - 0.5) * area_size.x, 
                    y: (rng.gen::<f64>() - 0.5) * area_size.y}, 
                vel: Float2D{
                    x: 0., 
                    y: 0.}},
             food_value: 25., // TODO make it parametric (no MAGIC NUMBER)
             index: index,
             dead: false,
        }
    }

    pub fn is_alive(&self) -> bool{
        !self.dead
    }

    pub fn kill(&mut self) {
        self.dead = true;
    }
}

// BACTER
#[derive(Clone, Copy)]
pub struct Bacter{
    bacter_vector : Vector2D,
    index : i64,

    // Defining the parameters, from 0 to 1:
    size : f32, // the bigger the stronger, but more food is needed
    aggro: f32, // the higher the more chances to attack, and better digestion of bacters (and worse of plants).
    
    // Food in the stomach is 100 * size.
    food_value : f32,

    // Flag to see if the cell is dead and must be removed.
    dead: bool,
}

impl Bacter {
    pub fn new_random(area_size : Float2D, index: i64) -> Bacter{
        let mut rng = rand::thread_rng();
        let temp_size = rng.gen::<f32>().clamp(MIN_BACTER_SIZE, MAX_BACTER_SIZE);
        let temp_aggro = rng.gen::<f32>();
        Bacter{
            bacter_vector : Vector2D{
                pos: Float2D{
                    x: (rng.gen::<f64>() - 0.5) * area_size.x, 
                    y: (rng.gen::<f64>() - 0.5) * area_size.y}, 
                vel: Float2D{
                    x: (rng.gen::<f64>() - 0.5), 
                    y: (rng.gen::<f64>() - 0.5)}},
                size: temp_size,
                aggro: temp_aggro,
                food_value: 50. * temp_size, // half-full belly.
                index: index,
                dead: false,}
    }

    pub fn new(pos : Float2D, vel : Float2D, index: i64, size: f32, aggro: f32) -> Bacter{
    Bacter{
            bacter_vector : Vector2D{
            pos: pos, vel: vel},
            size: size,
            aggro: aggro,
            food_value: 50. * size, // half-full belly.
            index: index,
            dead:false,}
    }

    pub fn _set_vel(&mut self, vel: Float2D){      
        self.bacter_vector.vel = vel;
    }
    pub fn apply_movement(&mut self, time: f64){
        //move in a straight line
        self.bacter_vector.pos = self.bacter_vector.pos + self.bacter_vector.vel.multiply(time);
    }

    pub fn bounce_with_box(&mut self, box_size: Float2D){
        let x_lim : f64= box_size.x / 2.;
        let y_lim : f64= box_size.y / 2.;

        // TODO do this in pipe? Now it's tedious!
        if self.bacter_vector.pos.x < -x_lim{
            self.bacter_vector.pos.x = -x_lim;
            if self.bacter_vector.vel.x < 0. {
                self.bacter_vector.vel.x *= -1.;
            }
        }
        if self.bacter_vector.pos.x > x_lim{
            self.bacter_vector.pos.x = x_lim;
            if self.bacter_vector.vel.x > 0. {
                self.bacter_vector.vel.x *= -1.;
            }
        }
        if self.bacter_vector.pos.y < -y_lim{
            self.bacter_vector.pos.y = -y_lim;
            if self.bacter_vector.vel.y < 0. {
                self.bacter_vector.vel.y *= -1.;
            }
        }
        if self.bacter_vector.pos.y > y_lim{
            self.bacter_vector.pos.y = y_lim;
            if self.bacter_vector.vel.y > 0. {
                self.bacter_vector.vel.y *= -1.;
            }
        }
    }

    pub fn bounce_with_cells<T>(&mut self, other_cells: &[T]) -> Option<usize>
    where T: Cell{

        let mut last_interaction_index: Option<usize> = None;

        // Cycle on the vector of cells. for each, checking if the distance is below a certain point:
        // However, if the two are almost overlapping skipping them
        // TODO: Find a smarter way to avoid checking one cell with itself.
        for i in 0..other_cells.len(){
            if other_cells[i].get_index() != self.index{
                let cells_distance: f64 = self.bacter_vector.pos.distance_square(other_cells[i].get_vector().pos);
                let cells_impact_distance = 10. * (self.get_size() + other_cells[i].get_size()) as f64;
                if  cells_distance > 0.1 && cells_distance < cells_impact_distance * cells_impact_distance {

                    // Reversing the speed:
                    // V = |V| * -ver(A-B) 
                    self.bacter_vector.vel =
                    self.bacter_vector.pos.versor(other_cells[i].get_vector().pos).multiply(self.bacter_vector.vel.abs());

                    // updating the interacting index:
                    last_interaction_index = Some(i);
                }
            }
        }

        last_interaction_index
    } 

    pub fn try_reproducing(&mut self) -> Option<Bacter>{

        if self.dead{
            return None;
        }
        // If the bacter's belly is full, creating an offspring.
        // For now putting it in the same position.
        if self.food_value > self.size * 100.{
            
            // Calculating the versor:
            let versor = self.bacter_vector.vel.multiply(1. / self.bacter_vector.vel.abs());
            let mut rng = rand::thread_rng();
            let offspring = Bacter::new(
                self.bacter_vector.pos.add(
                    Float2D {
                        x: self.get_size()  as f64*2. * -10.1 * versor.x,
                        y: self.get_size()  as f64*2. * -10.1 * versor.y,}),
                self.bacter_vector.vel.multiply(-1.),
                0, 
                (self.size + (rng.gen::<f32>() - 0.5) * 0.05).clamp(MIN_BACTER_SIZE, MAX_BACTER_SIZE), // MAGIC NUMBER
                (self.aggro + (rng.gen::<f32>() - 0.5) * 0.05).clamp(0.0, 1.0)); // MAGIC NUMBER

            self.food_value = self.food_value / 2.;
            return Some(offspring);
            
        }
        None
    }

    // Get Parameters: size
    pub fn get_size(&self) -> f32{
        self.size
    }

    // Get Parameters: size
    pub fn get_aggro(&self) -> f32{
        self.aggro
    }

    pub fn is_alive(&self) -> bool{
        !self.dead
    }

    pub fn kill(&mut self){
        self.dead = true;
    }

    pub fn set_index(&mut self, index: i64){
        self.index = index;
    }

    pub fn consume_food(&mut self, time: f64){
        // Reduce the amount of food in the bacter's belly.
        self.food_value -= self.size * self.size * time as f32;
        if self.food_value < 0.{
            self.dead = true
        }
    }

    pub fn try_kill_bacter(&mut self, other : Bacter) -> bool {
        
        if self.dead || other.dead{
            //println!("you can't kill what is already dead");
            return false;
        }
        // Checking the aggro probability: both the rng and the aggro range between 0 and 1.
        // as a start, if rng > aggro, one tries to eat the other.
        // Note that the "victim" cannot fight back.
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.aggro{
            // Adding a +- 0.5 chance to the size of the two.
            if self.get_size() > other.get_size() + rng.gen::<f32>() - 0.5{

                // the victim is killed, and the food transfered to capacity to the one eating.
                self.food_value += other.food_value * 0.5; // TODO add a dampening factor?
                return true;
            }
        }
        false
    }

    pub fn try_eat_alga(&mut self, other : Alga) -> bool {
        if self.dead || other.dead {
            //println!("you can't kill what is already dead");
            return false;
        }

        // Complementary to eating cells, the less the aggro the higher the chances to eat.
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() > self.aggro{
            self.food_value += other.food_value;
            return true;
        }

        false
    }


}

impl Cell for Bacter{
    fn get_vector(&self) -> Vector2D
    {
        self.bacter_vector
    }
    fn set_pos(&mut self, pos: Float2D)
    {
        self.bacter_vector.pos = pos;
    }
    fn get_index(&self) -> i64{
        self.index
    }
    fn get_size(&self) -> f32{
        self.size
    }
}