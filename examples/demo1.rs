extern mod rlg;
use rlg::game::*;
pub fn main() {
        

        do GameManager::new_game(~"my game") {
            let gm = GameManager::new(800,600,50);
                       
            let actor1 = gm.spawn_actor(0,0,50,50,2,
                                        200,100,20);
            
            let actor2 = gm.spawn_actor(400,100,50,50,1,
                                        200,100,20);
        
            let mut v = ~[actor1,actor2];

            do GameManager::default_game_loop(gm.get_lvl()){
                for x in v.mut_iter(){
                    x.move_right();
                }
                gm.redraw_actors(&mut v);
            }    

               
             
        }
        
    }