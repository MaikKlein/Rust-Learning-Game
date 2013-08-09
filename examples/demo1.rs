extern mod rlg;
use rlg::game::*;
pub fn main() {
        

        do GameManager::new_game(~"my game") {
            let gm = GameManager::new(800,600,10);
                       
            let actor1 = gm.spawn_actor(0,0,50,50,1,
                                        200,100,20);
            

            let actor2 = gm.spawn_actor(0,100,50,50,1,
                                        200,100,20);
            //gm.game_actors.push(actor1);


            let mut v = ~[actor1,actor2];

            do GameManager::default_game_loop(&gm.lvl){
                for x in v.mut_iter(){
                    
                        x.move_right();
                    
                }
            }    

               
             
        }
        
    }