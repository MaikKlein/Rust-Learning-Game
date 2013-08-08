extern mod sdl;
extern mod extra;

pub struct Actor{
    rect: sdl::Rect
}
pub struct Level{
    width: int,
    height: int,
    block_size: int,
    actors: ~[Actor],
    screen: sdl::video::Surface
}
pub struct DeltaTime{
    current_time: float,
    old_time: float
}
impl DeltaTime {
    pub fn new() -> DeltaTime {
        
        DeltaTime {current_time: 0.0f, old_time: 0.0f}   
    }
    pub fn get_dt(&mut self) -> float{
            self.old_time = self.current_time;
            self.current_time = extra::time::precise_time_s();
            (self.current_time - self.old_time) / 1000.0f
    }
    
}

impl Level {
    pub fn new(width:int,height: int)-> Level {
        let screen = match sdl::video::set_video_mode(width, height, 32, [sdl::video::HWSurface],
                                                                    [sdl::video::DoubleBuf]) {
            Ok(screen) => screen,
            Err(err) => fail!(fmt!("failed to set video mode: %s", err))
        };
        Level{width: width, height: height,block_size: 1, actors: ~[], screen: *screen}
        
    }
}

impl Actor {
    pub fn update <'a>(&'a mut self,x: i16,y:i16) -> &'a mut Actor {
        self.rect.x = x;
        self.rect.y = y; 
        self
    }
    pub fn redraw(&self, lvl: &Level) {
        lvl.screen.fill_rect(Some(self.rect), sdl::video::RGBA(0,0,255,255));
    }
    pub fn new(x: i16,y: i16, width: u16, height: u16) -> Actor{
        Actor {rect: sdl::Rect {
                    x: x,
                    y: y,
                    w: width,
                    h: height}}
    }
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
    loop{
    lvl.screen.fill(sdl::video::RGBA(0,0,0,255));
    f();
    match sdl::event::poll_event() {
                sdl::event::QuitEvent => break,
                _ => {}
    }
            
    lvl.screen.flip(); 
    }

}


