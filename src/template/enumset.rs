use egui::{Button, Id, IdMap, Key, Label, PopupCloseBehavior, Response, Widget};
use egui_extras::TableRow;

use super::{Component, Driver, Element, Field, Spawner};
use crate::{context::Context, template::{Spawn, TagCache}};


#[derive(Clone)]
pub struct Enum {
    pub cond: Cond,   
    pub default: Id,
}

impl Enum {
    pub fn enum_default(field: &mut Field) -> Id {
        let id = Id::new("enum").with("unselected");
        if field.map.get(&id).is_none() {
            field.map.insert(id, super::Chunk {
                elem: Element::Static(super::Static::LabelRT("unselected".into())),
                drv: Driver::None,
                tags: vec![],
            });
        };
        id
    }
    pub fn into_elem(self) -> Element {
        Element::Driven(super::Driven::Enum(self))
    }
    pub fn from_strings(vec: Vec<String>, ctx: &mut Context, field: &mut Field) -> Self {
        Self {
            cond: Cond::Elem(vec.into_iter().map(|s| {
                let id = Id::new("enum").with(ctx.count.get());
                field.map.insert( id, super::Chunk { 
                    elem: Element::Static(super::Static::LabelRT(s.clone().into())),
                    drv: Driver::None,
                    tags: vec![s]
                });
                id
            }).collect()),
            default: {
                let id = Id::new("enum").with("unselected");
                if field.map.get(&id).is_none() {
                    field.map.insert(id, super::Chunk { 
                        elem: Element::Static(super::Static::LabelRT("unselected".into())),
                        drv: Driver::None,
                        tags: vec![]
                    });
                };
                id
            }
        }
    }
}


#[derive(Clone)]
pub struct Cache {
    pub searcher: simsearch::SimSearch<usize>,
    pub selected: Box<(Spawn,Element,Driver)>,
    pub selectable: Vec<Id>,
}

#[derive(Clone)]
pub enum Cond {
    Elem(Vec<Id>),
    Tags(Vec<String>),
}

pub struct EnumSpawn<'a> {
    pub spw: &'a mut Vec<Spawn>,
    pub elem: &'a Cond,
    pub drv: &'a mut Spawner, // why? drv should be mutable

    pub cache: &'a mut Cache,

    pub id: egui::Id, // to assign popup_id, etc
    pub field: &'a Field,
    pub ctx: &'a mut Context,
}

impl<'a> Widget for EnumSpawn<'a> {
    // 1. Button.
    // 2. KeyBoard.
    // 3. Popups.
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let Self { mut drv, cache, id, field, ctx, spw, elem: cond } = self;
        let resp;
        ctx.focus.lost_focus_this_frame.off();
        let popup_id = egui::Id::new("popup_id").with(self.id);
        
        if ui.memory(|mem| mem.is_popup_open(popup_id)) {
            let mut tab_choice = ui.input(|i| i.key_pressed(egui::Key::Tab));
            resp = ui.text_edit_singleline(&mut ctx.search);
            // fetching focus
            if *ctx.focus.edit_once || ui.input(|i| i.key_pressed(egui::Key::Backspace)){ 
                    resp.request_focus();
                    *ctx.focus.edit_once = true;
                }
            // turn off
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) && 
                ctx.search.is_empty() {
                    ctx.close_popup.on();
                }
            // start search
            if resp.lost_focus() && !ctx.search.is_empty() {
                tab_choice = true;
                ctx.focus.tab_target_id = cache.searcher.search(&ctx.search).into_iter().cycle();
            }

            let target = if tab_choice { ctx.focus.tab_target_id.next() } else { None };

            let unit = |mut row: TableRow| {
                let index = row.index();
                row.col(|ui|
                    {
                        let elem = &match cond {
                            Cond::Elem(vec) => vec[index],
                            Cond::Tags(_) => cache.selectable[index]
                        };
                        let chunk = &field.map[elem];
                        let resp = ui.button("|");

                        Component {
                            spw: &mut field.spw[elem].clone(),
                            elem: &chunk.elem,
                            drv: &mut chunk.drv.clone(),
                            id,
                            field,
                            ctx,
                        }.ui(ui);
                        // to show something ...
                        // surely ...
                        let mut click = || {
                            ctx.focus.lost_focus_this_frame.on();
                            *drv = field.get_spawner(*elem).unwrap();
                            ctx.close_popup.on()
                        };
                        if let Some(u) = target {
                            if u == index {
                                if ctx.search == field.map[elem].tags[0] && 
                                    ui.input(|i| i.key_pressed(Key::Enter)) {
                                        click();
                                    }
                                resp.request_focus();
                            }
                        }
                        if resp.clicked() {
                            click();
                        }
                    }
                );
            };
            egui::popup_below_widget(
                ui, 
                popup_id, 
                &resp, 
                PopupCloseBehavior::CloseOnClick, 
                |ui| {
                    use egui_extras::{TableBuilder,Column};
                    TableBuilder::new(ui)
                        .max_scroll_height(18.0*8.0)
                        .column(Column::auto())
                        .body(|body| {
                            body.rows(18.0, spw.len() , unit)
                        });
                },
            );
        } else {
            let Spawner {spw, elem, drv} = drv;
            Component {
                spw,elem,drv,field,ctx,id,
            }.ui(ui);
            resp = ui.button(
                "/"
            );
            if resp.clicked() || *ctx.focus.on_button {
                ctx.focus.on_button.off();
                ctx.open_popup.on();
                ctx.focus.edit_once.on();
                ctx.search.clear();
            }
        }
        
        if !ctx.open_popup.run(|| {
            ui.memory_mut(|mem| mem.open_popup(popup_id));
        }) {
            ctx.close_popup.run(|| {
                ui.memory_mut(|mem| mem.close_popup())
            });
        }

        resp
    }
}