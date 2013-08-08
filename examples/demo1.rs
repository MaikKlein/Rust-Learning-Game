extern mod rlg;
use rlg::game::*;
pub fn main() {
        
        do new_game(~"my game") {

            let lvl = Level::new(800,600);
            
           
            let actor1 = GameActor::new(0,0,50,50,
                                        200,100,20);
    
            let mut actors = ~[actor1];

            do default_game_loop(&lvl){

                for a in actors.mut_iter() {
                    if a.rect.x as int > lvl.width {
                        a.update_pos(-a.rect.x,0).redraw(&lvl);
                    }
                    else {
                        a.move_right().redraw(&lvl);
                    }
                }  
            } 
        }
    }