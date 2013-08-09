extern mod sdl;
extern mod extra;
pub struct Color{
    r: u8,
    g: u8,
    b: u8
}
pub struct GameActor<'self>{
    rect: sdl::Rect,
    speed: i16,
    color: Color,
    lvl: &'self Level
}
pub struct Level{
    width: int,
    height: int,
    fps: int,
    screen: sdl::video::Surface
}

pub struct GameManager<'self>{
    lvl: Level,
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
impl <'self> GameManager <'self>{
    pub fn new(width: int, height: int,fps:int) -> GameManager<'self>{
        GameManager {lvl: Level::new(width,height,fps), game_actors: ~[], dt: DeltaTime::new()  }
    }
    pub fn spawn_actor<'r>(&'r self, x: i16,y: i16, width: u16, height: u16,speed:i16, r:u8,g:u8,b:u8) -> GameActor<'r>{
        
        GameActor::new(&'r self.lvl,x,y,width,height,speed,r,g,b)
                   
        
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
    let fps = 1.0f/10.0f;
    loop{
        match sdl::event::poll_event() {
                sdl::event::QuitEvent => break,
                _ => {}
        }
        if dt_sum > fps {
        lvl.screen.fill(sdl::video::RGBA(0,0,0,255));    
        f();    
        lvl.screen.flip();
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
        self.rect.x += x;
        self.rect.y += y; 
        self
    }
    pub fn redraw(&self) {
       // self.lvl.screen.fill(sdl::video::RGBA(0,0,0,255));
        self.lvl.screen.fill_rect(Some(self.rect), sdl::video::RGBA(self.color.r,self.color.g,self.color.b,255));
    
    }
    pub fn new(lvl: &'self Level,x: i16,y: i16, width: u16, height: u16,speed:i16, r:u8,g:u8,b:u8) -> GameActor<'self>{
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






