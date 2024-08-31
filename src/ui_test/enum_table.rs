use egui::{ahash::RandomState, Color32, Id, Key, PopupCloseBehavior, Pos2, RichText};
use simsearch::SimSearch;

use crate::template::enumset::Enum;


// Wait a minute ...

pub struct EnumSpawn {
    vec: Vec<RichText>, 
    selected: RichText, // just to select the one. when updating the vec, update the selected.
    // Shall we have a map to get the id of a .. ?
    pub position: Pos2,
    text_cache: String,
    popup_toggle: Toggle,
    tried_gain_focus: bool,
    searcher: SimSearch<usize>,
    tap_target: std::iter::Cycle<std::vec::IntoIter<usize>>,
    pub id: usize,
    pub focus_button: bool,
    pub lost_focus: bool,
} // Every enum element should have a string for searching.

/*impl From<(Enum,Id,&Field)> for EnumSpawn {
    fn from((value,id,field): (Enum,Id,&Field)) -> Self {
        let vector = match value.cond {
            crate::template::EnumCond::Elem(elem) => elem,
            crate::template::EnumCond::Tags(_, cache) => cache.vec,
        };
        let count = vector.len();
        let vector = vector.into_iter().fold(
            (Vec::with_capacity(count),
            Vec::with_capacity(count)), 
            |(mut e,mut s),(ex,sx)| {
                e.push((ex,field).into());
                s.push(sx);
                (e,s)
            });
        let mut searcher = SimSearch::new();
        vector.1
            .iter()
            .enumerate()
            .for_each(|(id,content)|{
                searcher.insert(id, content);
            });


        Self { 
            vec: {
                vector.0
            }, selected: {
                id
            }, position: {
                Pos2 { x: 0., y: 0. }
            }, text_cache: String::new(), 
            popup_toggle: Toggle::None, 
            tried_gain_focus: false, 
            searcher,
            tap_target: vec![].into_iter().cycle(), 
            id: 0, 
            focus_button: false, 
            lost_focus: false 
        }
    }
}*/

impl Default for EnumSpawn {
    fn default() -> Self {
        let vec: Vec<RichText> = vec![
            RichText::new("Noel").color(Color32::from_rgb(70,70,0)), 
            RichText::new("Primula").color(Color32::from_rgb(75,0,0)), 
            RichText::new("Alma").color(Color32::from_rgb(0,75,0)),
            RichText::new("Ixia").color(Color32::from_rgb(65,65,40))
        ];
        let mut searcher = SimSearch::new();
        vec.iter().enumerate().for_each(|(id,content)|{searcher.insert(id, content.text());});
        EnumSpawn { 
            vec,
            selected: "unselected".into(), 
            position: Pos2 { x: 0., y: 0. },
            text_cache: String::new(),
            popup_toggle: Toggle::None,
            tried_gain_focus: false,
            searcher,
            focus_button: false,
            tap_target: vec![].into_iter().cycle(),
            lost_focus: false,
            id: 0,
        }
    }
}

enum Toggle {
    On,
    Off,
    // Not,
    None,
}

impl EnumSpawn {
    #[inline]
    pub fn with_id(&mut self,id: usize) -> &mut Self {
        self.id = id;
        self
    }
}

impl EnumSpawn {
    // 1. Button.
    // 2. KeyBoard.
    // 3. Popups.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.lost_focus = false;
        let popup_id = egui::Id::new("popup_id").with(self.id);
        
        if ui.memory(|mem| mem.is_popup_open(popup_id)) {
            let mut tab_choice = ui.input(|i| i.key_pressed(egui::Key::Tab));
            let resp = ui.text_edit_singleline(&mut self.text_cache);
            // fetching focus
            if (self.text_cache.is_empty() && !self.tried_gain_focus) 
                || ui.input(|i| i.key_pressed(egui::Key::Backspace)){ 
                    resp.request_focus();
                    self.tried_gain_focus = true;
                }
            // turn off
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) && 
                self.text_cache.is_empty() {
                    self.popup_toggle = Toggle::Off;
                }
            // start search
            if resp.lost_focus() && !self.text_cache.is_empty() {
                tab_choice = true;
                self.tap_target = self.searcher.search(&self.text_cache).into_iter().cycle();
            }
            egui::popup_below_widget(
                ui, 
                popup_id, 
                &resp, 
                PopupCloseBehavior::CloseOnClick, 
                |ui| {
    
                    /*let text_height = egui::TextStyle::Body
                        .resolve(ui.style())
                        .size
                        .max(ui.spacing().interact_size.y);*/
    
                    use egui_extras::{TableBuilder,Column};
                    TableBuilder::new(ui)
                        .max_scroll_height(18.0*8.0)
                        .column(Column::auto())
                        .body(|body| {
                            let target = if tab_choice { self.tap_target.next() } else { None };
                            body.rows(18.0, self.vec.len() , |mut row| {
                                let index = row.index();
                                row.col(|ui|
                                    {
                                        let rt = &self.vec[index];
                                        let resp = ui.add(
                                            egui::Button::new(rt.clone())
                                            .wrap_mode(egui::TextWrapMode::Extend)
                                        );
                                        /*if  ! resp.has_focus() &&
                                            ! parent_on_focus &&
                                            ! self.text_cache.is_empty() && 
                                            rt.text().contains(&self.text_cache) {
                                                resp.request_focus();
                                        }*/
                                        if let Some(u) = target {
                                            if u == index {
                                                if self.text_cache == rt.text() && ui.input(|i| i.key_pressed(Key::Enter)) {
                                                    self.lost_focus = true;
                                                    self.selected = rt.clone();
                                                    self.popup_toggle = Toggle::Off;
                                                }
                                                resp.request_focus();
                                            }
                                        }
                                        if resp.clicked() {
                                            self.lost_focus = true;
                                            self.selected = rt.clone();
                                            self.popup_toggle = Toggle::Off;
                                        }
                                    }
                                );
                            })
                        });
                    /*ui.vertical(|ui| {
                        for rt in &self.vec {
                            let resp = ui.add(
                                Button::new(rt.clone())
                                .wrap_mode(egui::TextWrapMode::Extend)
                            );
                            if  ! resp.has_focus() &&
                                ! parent_on_focus &&
                                ! self.text_cache.is_empty() && 
                                rt.text().contains(&self.text_cache) {
                                    resp.request_focus();
                            }
                            if resp.clicked() {
                                self.selected = rt.clone();
                                self.popup_toggle = Toggle::Off;
                            }
                        }
                    })*/
                },
            );
        } else {
            let resp = ui.button(self.selected.clone());
            if resp.clicked() || self.focus_button {
                self.focus_button = false;
                self.popup_toggle = Toggle::On;
                self.tried_gain_focus = false;
                self.text_cache.clear();
            }
        }
        

        match self.popup_toggle {
            Toggle::On => {ui.memory_mut(|mem| mem.open_popup(popup_id));},
            Toggle::Off => {ui.memory_mut(|mem| mem.close_popup());},
            // Toggle::Not => {ui.memory_mut(|mem| mem.toggle_popup(popup_id));},
            Toggle::None => (),
        }

        self.popup_toggle = Toggle::None; 
    }
}