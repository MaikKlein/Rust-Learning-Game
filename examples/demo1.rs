extern mod rlg;
use rlg::game::*;
pub fn main() {
        
        do new_game(~"my game") {

            let lvl = Level::new(800,600);
            let mut count = 0i16;
            let mut dt = DeltaTime::new();
            let mut deltasum = 0.0f;
            let mut actor = Actor::new(0,0,50,50);

            do default_game_loop(&lvl){
                if deltasum > 0.0001f {
                    deltasum = 0.0f;
                    count +=1;
            }

            deltasum += dt.get_dt();
            actor.update(count,count).redraw(&lvl);
            }
        }
    }