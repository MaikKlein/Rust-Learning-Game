extern mod sdl;
extern mod extra;
pub struct Color{
    r: u8,
    g: u8,
    b: u8
}
struct ActorController{
    lvl_ctr: LevelController
}
struct LevelController{
    lvl: Level
}
pub struct GameActor<'self>{
    rect: sdl::Rect,
    speed: i16,
    color: Color,
    lvl: &'self Level
}
struct Level{
    width: int,
    height: int,
    fps: int,
    screen: sdl::video::Surface
}

pub struct GameManager<'self>{
    priv actor_ctr: ActorController ,
    game_actors: ~[GameActor<'self>],
    dt: DeltaTime
}
pub struct DeltaTime{
    current_time: float,
    old_time: float
}
trait Move{
    fn move_up<'r>(&'r mut self)-> &'r mut Self;
    fn move_down<'r>(&'r mut self)-> &'r mut Self;
    fn move_left<'r>(&'r mut self)-> &'r mut Self;
    fn move_right<'r>(&'r mut self)-> &'r mut Self;
}
impl ActorController {
    pub fn spawn_actor<'r>(&'r self,x: i16,y: i16, width: u16, height: u16,speed:i16, r:u8,g:u8,b:u8)-> GameActor<'r>{
        GameActor::new(&'r self.lvl_ctr.lvl,x,y,width,height,speed,r,g,b)
    }
}
impl <'self> GameManager <'self>{
    pub fn get_lvl<'r>(&'r self) -> &'r Level{
        &'r self.actor_ctr.lvl_ctr.lvl
    }
    pub fn new(width: int, height: int,fps:int) -> GameManager<'self>{
        GameManager { actor_ctr: ActorController {lvl_ctr: LevelController{lvl: Level::new(width,height,fps)}},game_actors: ~[], dt: DeltaTime::new()  }
    }
    pub fn spawn_actor<'r>(&'r self,x: i16,y: i16, width: u16, height: u16,speed:i16, r:u8,g:u8,b:u8)-> GameActor<'r>{
        self.actor_ctr.spawn_actor(x,y,width,height,speed,r,g,b)
    }
    
    pub fn draw_actor(lvl: &Level,a: &GameActor){
        lvl.screen.fill_rect(Some(a.rect), sdl::video::RGBA(a.color.r,a.color.g,a.color.b,255));
    }
    pub fn redraw_actors(&self,v: &mut ~[GameActor]){
        self.actor_ctr.lvl_ctr.lvl.screen.fill(sdl::video::RGBA(0,0,0,255));
        for a in v.mut_iter() {
            GameManager::draw_actor(&self.actor_ctr.lvl_ctr.lvl,a);
        }
        self.actor_ctr.lvl_ctr.lvl.screen.flip();
    }
    pub fn new_game(name: ~str,f: ~fn()) {
    do sdl::start {
        sdl::init([sdl::InitVideo]);
        sdl::wm::set_caption(name,name);
        f();
        sdl::quit();
    }
    
}
    pub fn default_game_loop(lvl: &Level, f: &fn() ){
    lvl.screen.fill(sdl::video::RGBA(0,0,0,255));
    let mut dt = DeltaTime::new();
    let mut dt_sum = 0.0f;
    let fps = 1.0f/lvl.fps as float;
    loop{
        match sdl::event::poll_event() {
                sdl::event::QuitEvent => break,
                _ => {}
        }
        if dt_sum > fps {  
        f();    
        dt_sum = 0.0f 
        }
        else {
            dt_sum += dt.get_dt();
        }
    }
}

}
impl DeltaTime {
    pub fn new() -> DeltaTime {
        DeltaTime {current_time: 0.0f, old_time: 0.0f}   
    }
    pub fn get_dt(&mut self) -> float{
            self.old_time = self.current_time;
            self.current_time = extra::time::precise_time_s();
            (self.current_time - self.old_time)
    }
    
}

impl Level {
    pub fn new(width:int,height: int,fps: int)-> Level {
        let screen = match sdl::video::set_video_mode(width, height, 32, [sdl::video::HWSurface],
                                                                    [sdl::video::DoubleBuf]) {
            Ok(screen) => screen,
            Err(err) => fail!(fmt!("failed to set video mode: %s", err))
        };
        Level{width: width, height: height,screen: *screen,fps:fps}
        
    }
}

impl<'self> GameActor<'self> {
    pub fn update_pos <'a>(&'a mut self,x: i16,y:i16) -> &'a mut GameActor<'self> {
        let update_x = self.rect.x + x;
        let update_y = self.rect.y + y;

        let b_x_bt_zero = update_x > 0;
        let b_x_lt_width = self.lvl.width as i16 >= (update_x + self.rect.w as i16);

        let b_y_bt_zero = update_y > 0;
        let b_y_lt_height = self.lvl.height as i16 >= (update_y + self.rect.h as i16);

        if  b_x_bt_zero && b_x_lt_width{
            self.rect.x = update_x;
        }
        else if !b_x_bt_zero{
            self.rect.x = 0;
        }
        else{
            self.rect.x = self.lvl.width as i16 - self.rect.w as i16;
        }


        if b_y_bt_zero && b_y_lt_height{    
            self.rect.y = update_y; 
        }
        else if !b_y_bt_zero{
            self.rect.y = 0;
        }
        else{
            self.rect.y = self.lvl.height as i16 - self.rect.h as i16;   
        }
        self
    }
    pub fn redraw(&self) {
       // self.lvl.screen.fill(sdl::video::RGBA(0,0,0,255));
        
       //self.lvl.screen.fill_rect(Some(self.rect), sdl::video::RGBA(self.color.r,self.color.g,self.color.b,255));
    
    }
    pub fn new<'r>(lvl: &'r Level,x: i16,y: i16, width: u16, height: u16,speed:i16, r:u8,g:u8,b:u8) -> GameActor<'r>{
        GameActor { lvl: lvl,
                    rect: sdl::Rect {
                    x: x,
                    y: y,
                    w: width,
                    h: height},
                   speed: speed,
                   color: Color{r:r,g:g,b:b}
                   
        }
    }
}
impl <'self> Move for GameActor<'self> {
    fn move_up<'r>(&'r mut self)-> &'r mut GameActor<'self> {
        self.update_pos(0,self.speed).redraw();
        self
    }
    fn move_down<'r>(&'r mut self)-> &'r mut GameActor<'self> {
        self.update_pos(0,-self.speed).redraw();
        self
    }
    fn move_left<'r>(&'r mut self)-> &'r mut GameActor<'self> {
        self.update_pos(-self.speed,0).redraw();
        self
    }
    fn move_right<'r>(&'r mut self)-> &'r mut GameActor<'self> {
        self.update_pos(self.speed,0).redraw();
        self
    }
    
}






