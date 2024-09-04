use egui::{Id, IdMap, Pos2, RichText, Widget};
use enum_table::EnumSpawn;

use crate::template::{Driver, Element};

pub mod enum_table;
pub mod linker;
pub mod catagory;

pub mod test {
    use crate::{context::Context, template};
    use egui::{pos2, vec2, Align2, Color32, FontId, Id, IdMap, Key, Pos2, RichText, Widget};
    use crate::template::{Component, Driver, Element, Field};
    use super::{catagory::CatagorySpawn, enum_table::EnumSpawn, linker::LinkerSpawn};

    enum Spawn {
        TexSpawn(EnumSpawn),
        PicSpawn(EnumSpawn),
    }

    pub struct UiTest {
        // texts: Vec<(Pos2, String, usize)>,
        es: Vec<Spawn>,
        st: Vec<String>,
        pass_focus: usize,
        passing_focus: bool,
        lk: LinkerSpawn,
        cg: CatagorySpawn,
    
        elem: Element,
        drv: Driver,
        spw: template::Spawn,
        
        ctx: Context,
        field: Field
    }

    
    impl Default for UiTest {
        fn default() -> Self {
            let mut field = Field::default();
            let enum_default = template::enumset::Enum::enum_default(&mut field);



            let mut ret = Self { 
                // texts: vec![(Pos2 { x: 100.0, y: 100.0},"awawa".into(),2)],
                es: vec![Spawn::PicSpawn(EnumSpawn::default())],
                lk: LinkerSpawn::default(),
                pass_focus: 0,
                passing_focus: false,
                st: vec![String::new()],
                cg: CatagorySpawn::default(),
                ctx: Default::default(),
                field,

                elem: Element::Driven(
                    template::Driven::Enum(template::enumset::Enum { 
                        cond: template::enumset::Cond::Elem(vec!["Noel".into()]), 
                        default: enum_default,
                    })),
                drv: Driver::None,
                spw: Default::default()
            };
            let elem = Element::Static(template::Static::LabelRT("hello world".into()));
            let drv = Driver::None;
            let spawn = template::Spawn::new(
                    &elem,
                    &drv,
                    &mut ret.ctx,
                    &ret.field,
                );
            ret.spw = template::Spawn::new(&ret.elem, &ret.drv, &mut ret.ctx, &ret.field);
            ret
        }
    }





    // 1. MSG serial.
    // {Participant: }
    //  * 复制PIC命令以继续
    //  说. => Noel Do What?

    impl eframe::App for UiTest {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            // ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.texts.len().to_string()));
            // if let Some(pos) = ctx.input(|i| i.pointer.interact_pos().filter(|_| i.pointer.any_click() && i.key_down(egui::Key::A))) {
            //     self.texts.push((pos,"newer".into(),4))
            // }
            // Display all texts
            

            egui::CentralPanel::default().show(ctx, |ui| {
                //ui.add(CatagorySpawn::default());
            /*    egui::Area::new(egui::Id::new("awa"))
                    .order(egui::Order::Middle)
                    .default_pos(pos2(1.,1.))
                    .default_size(vec2(500., 50.))
                    .show(ui.ctx(), |ui| {
                        egui::Frame::default()
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::same(8.0))
                            .stroke(ui.ctx().style().visuals.window_stroke)
                            .fill(ui.style().visuals.panel_fill)
                            .show(ui, |ui| {
                                let paint = ui.painter();
                                paint.text(Pos2::ZERO, Align2::LEFT_CENTER, "text", Default::default(), Color32::WHITE);
                                if ui.button("Lick me!").clicked() {
                                    println!("licked");
                                };
                            });
                        
                    });
                */
                //self.lk.ui(ui);
                
                let resp = ui.horizontal(|ui| {
                    // let window_layer = ui.layer_id();
                    // let (id, _) = ui.allocate_space(ui.available_size());
                    let resp = egui::Area::new(egui::Id::new("talk_ui").with(0))
                        .default_pos(Pos2::ZERO)
                        // .order(egui::Order::Middle)
                        .movable(true)
                        .show(ctx, |ui| {
                            for i in 0..self.es.len() {
                                egui::Frame::default()
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::Margin::same(8.0))
                                    .stroke(ui.style().visuals.window_stroke)
                                    .fill(ui.style().visuals.panel_fill)
                                    .show(ui, |ui| {
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                                        match &mut self.es[i] {
                                            Spawn::TexSpawn(es) => {
                                                if self.passing_focus && self.pass_focus == i {
                                                    es.focus_button = true;
                                                    self.passing_focus = false;
                                                }
                                                es.with_id(i).ui(ui);
                                                let resp = ui.text_edit_multiline(&mut self.st[i]);
                                                if resp.has_focus() {
                                                    self.pass_focus = i+1;
                                                }
                                                if es.lost_focus {
                                                    resp.request_focus();
                                                }
                                                    
                                            },
                                            Spawn::PicSpawn(es) => {
                                                ui.horizontal(|ui| {
                                                    ui.label("Picture");
                                                    es.with_id(i).ui(ui);
                                                });
                                                let resp = ui.text_edit_singleline(&mut self.st[i]);
                                                if es.lost_focus {
                                                    resp.request_focus();
                                                }
                                            },
                                        }
                                    });
                                }
                                
                                
                                ui.vertical_centered(|ui| {
                                    if ui.button(RichText::new("   +   ").font(FontId::monospace(15.0))).clicked() {
                                        self.es.push(Spawn::TexSpawn(Default::default()));
                                        self.st.push(Default::default());
                                    }
                                    if ui.button(RichText::new("   -   ").font(FontId::monospace(15.0))).clicked() {
                                        self.es.pop();
                                        self.st.pop();
                                    }
                                });
                            })
                        .response;
                    let egui::Rect { min, max } = resp.rect;
                    let size = resp.rect.size().y;
                    let painter = ui.painter();
                    painter.text(
                        pos2(min.x - size/3.,max.y+size*0.222222), 
                        Align2::LEFT_BOTTOM, "[", 
                        egui::FontId::monospace(size*1.222222), 
                        Color32::from_gray(120)
                    );
                    painter.text(
                        pos2(max.x + size/3.,max.y+size*0.222222), 
                        Align2::RIGHT_BOTTOM, "]", 
                        egui::FontId::monospace(size*1.2222222), 
                        Color32::from_gray(120)
                    );
                    // let id = resp.layer_id;
                    // ui.ctx().set_sublayer(window_layer, id);
                }).response;
                
            
                if ui.input(|i| i.modifiers.command && i.key_pressed(Key::Enter)) {
                    self.es.push(Spawn::TexSpawn(Default::default()));
                    self.st.push(Default::default());
                    self.pass_focus += 1;
                    self.passing_focus = true;
                }

                // ui.allocate_ui_at_rect(Rect { min: self.es.position, max: self.es.position }, |ui| self.es.ui(ui));
                /*for (pos, text,num) in &mut self.texts {
                    let mut child = ui.child_ui(
                        Rect::from_min_size(*pos, Default::default()), 
                        Layout::left_to_right(egui::Align::Center), None
                    );

                    let ir = child.allocate_ui_with_layout(Vec2::ZERO, Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        for i in 0..*num { 
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(text.as_str())
                                        .font(FontId::proportional(20.0))
                                        .color(
                                            if i%2 == 0 {Color32::from_rgb(50, 100, 50)}
                                            else {Color32::from_rgb(100, 50, 50)}
                                        )
                                );
                            });
                            
                        }
                        ui.horizontal_centered(|ui| {
                            ui.add_space(10.0);
                            if ui.button(RichText::new("+").font(FontId::proportional(10.0))).clicked() {
                                *num += 1;
                            }
                            if ui.button(RichText::new("-").font(FontId::proportional(15.0))).clicked() {
                                *num -= 1;
                            }
                        });
                    });
                    
                    let Rect { min, max} = ir.response.rect;
                    let height = max.y - min.y;

                    child.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        ui.allocate_ui_at_rect(
                            Rect { 
                                min: Pos2 { x: min.x - height/3.0, y: min.y - height*0.2 }, 
                                max: Pos2 { x: min.x, y: max.y } 
                            }, |ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("{")
                                        .color(Color32::from_gray(150))
                                        .font(FontId::proportional(height*1.2)),
                                ).selectable(false)
                            );
                        });
                        ui.allocate_ui_at_rect(Rect { 
                            min: Pos2 { x: min.x - height/3.0, y: min.y }, 
                            max: Pos2 { x: min.x, y: max.y } 
                        }, |ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("o")
                                        .color(Color32::from_gray(50))
                                        .font(FontId::proportional(height*0.1)),
                                ).selectable(false)
                            );
                        });
                        
                    });

                    child.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        ui.allocate_ui_at_rect(
                            Rect { 
                                min: Pos2 { x: max.x, y: min.y - height*0.2 }, 
                                max: Pos2 { x: max.x + height/3.0, y: max.y } 
                            }, |ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("}")
                                        .color(Color32::from_gray(150))
                                        .font(FontId::proportional(height*1.2)),
                                ).selectable(false)
                            );
                        });
                    });
                    /* 
                    child.vertical(|ui| {
                        ui.add(
                            Label::new(
                                RichText::new("{")
                                    .color(Color32::from_gray(150))
                                    .font(FontId::proportional(10.0 + 20.0 * *num as f32)),
                            ).selectable(false)
                        );
                        ui.horizontal_centered(|ui| {
                            ui.add_space(10.0);
                            if ui.button(RichText::new("+").font(FontId::proportional(10.0))).clicked() {
                                *num += 1;
                            }
                            if ui.button(RichText::new("-").font(FontId::proportional(15.0))).clicked() {
                                *num -= 1;
                            }
                        });
                    });
                    

                    child.horizontal_top(|ui| {
                        ui.vertical(|ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("{")
                                        .color(Color32::from_gray(150))
                                        .font(FontId::proportional(10.0 + 20.0 * *num as f32)),
                                ).selectable(false)
                            );
                            ui.horizontal_centered(|ui| {
                                ui.add_space(10.0);
                                if ui.button(RichText::new("+").font(FontId::proportional(10.0))).clicked() {
                                    *num += 1;
                                }
                                if ui.button(RichText::new("-").font(FontId::proportional(15.0))).clicked() {
                                    *num -= 1;
                                }
                            });
                        });
                        ui.vertical(|ui| {
                            ui.add_space(10.0);
                            for _ in 0..*num { 
                                ui.horizontal(|ui| {ui.label(RichText::new(text.as_str()).font(FontId::proportional(20.0)));} );
                                
                            }
                        });
                        
                        ui.label(
                            RichText::new("}")
                                .color(Color32::from_gray(150))
                                .font(FontId::proportional(10.0 + 20.0 * *num as f32)),
                        );
                    });*/
                }*/
                //my_ui(ui);
            });
        }
    }
}
