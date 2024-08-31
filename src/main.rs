use std::{collections::HashMap, default, time::SystemTime};

use context::Context;
use egui::{pos2, vec2, Align2, Color32, FontId, Id, IdMap, Key, Pos2, RichText, Widget};
use template::{Component, Driver, Element, Field};
mod ui_test;
pub mod context;
mod template;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "⭐Editor⭐",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[cfg(test)]
mod test {
    use crate::ui_test;

    #[test]
    fn test() -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "⭐Editor⭐",
            options,
            Box::new(|_cc| 
                Ok(Box::new(ui_test::test::UiTest::default()
            ))),
        )
    }
}

struct MyApp {

    elem: Element,
    drv: Driver,
    spw: template::Spawn,
    
    ctx: Context,
    field: Field
}


impl Default for MyApp {
    fn default() -> Self {
        let mut field = Field { map: <IdMap<(template::Spawn,Element,Driver)> as egui::ahash::HashMapExt>::new(), 
            tag: <egui::ahash::HashMap<String,Vec<Id>> as egui::ahash::HashMapExt>::new() };
        let enum_default = template::enumset::Enum::enum_default(&mut field);



        let mut ret = Self {
            ctx: Default::default(),
            field,

            elem: Element::Driven(
                template::Driven::Enum(template::enumset::Enum { 
                    cond: template::enumset::Cond::Elem(vec![("Noel".into(),"Alma".into())]), 
                    default: enum_default,
                })),
            drv: Driver::Enum("Noel".into()),
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
        ret.field.map.insert("Noel".into(), (
            spawn,elem.clone(),drv
        ));
        ret.spw = template::Spawn::new(&ret.elem, &ret.drv, &mut ret.ctx, &ret.field);
        ret
    }
}





// 1. MSG serial.
// {Participant: }
//  * 复制PIC命令以继续
//  说. => Noel Do What?

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let elem = &Element::Static(template::Static::LabelRT("hello world".into()));
            let drv = &mut Driver::None;
            Component {
                spawn: &mut template::Spawn::new(
                    elem,
                    drv,
                    &mut self.ctx,
                    &self.field,
                ),
                elem,
                drv,
                field: &self.field,
                ctx: &mut self.ctx,
            }.ui(ui);

            self.field.map.insert("Noel".into(), (
                template::Spawn::new(elem, drv, &mut self.ctx, &self.field).with_name("Ixia".into()),
                elem.clone(),
                Driver::None
            ));

            Component {
                spawn: &mut self.spw,
                elem: &self.elem,
                drv: &mut self.drv,
                field: &self.field,
                ctx: &mut self.ctx,
            }.ui(ui);
        });
    }
}