extern mod rlg;
extern mod sdl;
use rlg::game::*;

struct MyActor<'self>{
    actor: GameActor<'self>,
    b_dir: bool
}
impl <'self> MyActor<'self> {
    fn new(actor: GameActor<'self>)-> MyActor<'self>{
        MyActor {actor: actor,
                 b_dir: false}
    }
    fn travel(&mut self) {
        
    if self.b_dir {
        self.actor.move_left();
        if(self.actor.rect.x == 0){
            self.b_dir = false;
        }
    }

                
        else {
                
            self.actor.move_right();
            if(self.actor.rect.x == self.actor.lvl.width as i16 - self.actor.rect.w  as i16){
                self.b_dir = true;
            }
                
    
    }
}
}
pub fn main() {
        

        do GameManager::new_game(~"my game") {
            let gm = GameManager::new(850,250,100);
            
            let actor1 = gm.spawn_actor(600,100,50,50,1,
                                        255,0,0);
            
            let actor2 = gm.spawn_actor(400,50,50,50,2,
                                        0,255,0);
            
            let mut myactor1 = MyActor::new(actor1);
            let mut myactor2 = MyActor::new(actor2);
          
            do GameManager::default_game_loop(gm.get_lvl()){
                gm.get_lvl().screen.fill(sdl::video::RGBA(0,0,0,255));
                myactor1.travel();
                myactor2.travel();
                myactor1.actor.redraw();
                myactor2.actor.redraw();
                gm.get_lvl().screen.flip();
            }    
        }
        
    }