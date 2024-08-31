use std::{iter::Cycle, ops::{Deref, DerefMut}};


#[derive(Clone,Default)]
pub(crate) struct Context {
    pub focus: FocusCtx,
    //pub state: UserState,
    pub search: String,

    pub depth: usize,
    pub count: Counter,


    pub open_popup: Trigger,
    pub close_popup: Trigger,
    pub inc_table: Trigger,
    pub dec_table: Trigger,
}

#[derive(Clone, Copy, Default)]
pub struct Counter(usize);

impl Counter { 
    pub fn get(&mut self) -> usize { self.0 += 1; self.0 - 1 }
    pub fn reset(&mut self) { self.0 = 0; }
    pub fn new() -> Self { Self(0) }
    pub fn new_with(usize: usize) -> Self { Self(usize)} 
}

impl Context {
    fn clear(&mut self) {
        // when you change your focus ... 
        // note that we should get the taps
    }
}

#[derive(Clone)]
pub struct FocusCtx {
    pub turn_next: Trigger,
    pub edit_next: Trigger,
    pub lost_focus_this_frame: Trigger,
    pub edit_once: Trigger,
    pub on_button: Trigger,

    pub tab_target_id: std::iter::Cycle<std::vec::IntoIter<usize>>,
}

impl Default for FocusCtx {
    fn default() -> Self {
        Self {
            tab_target_id: vec![].into_iter().cycle(),
            turn_next: Default::default(),
            edit_next: Default::default(),
            lost_focus_this_frame: Default::default(),
            edit_once: Default::default(),
            on_button: Default::default(),
            
        }
    }
}

#[derive(Clone, Copy,Default)]
pub struct Trigger {
    set: bool,
}

impl Deref for Trigger {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl DerefMut for Trigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set
    }
}

impl Trigger {
    pub fn on(&mut self) {self.set = true;}
    pub fn off(&mut self) {self.set = false;}
    pub fn run(&mut self,f: impl FnOnce()->()) -> bool {
        if self.set {
            f(); 
            self.set = false;
            true
        }
        else {
            false
        }
    }
}

#[derive(Clone, Copy)]
enum UserState {
    Observing,
    Texting,
    Holding,
    Setting,
}