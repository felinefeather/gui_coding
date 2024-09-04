use egui::{pos2, vec2, Align2, Color32, FontId, Id, IdMap, Key, Pos2, RichText, Widget};
use context::Context;
use template::{Component, Driver, DriverField, Element, Field, Spawner};
#[cfg(test)]
mod ui_test;
pub mod context;
pub mod utils;
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

    // elem: Element,
    // drv: Driver,
    spw: template::Spawn,
    
    ctx: Context,
    field: Field,
    dfield: DriverField,
}


impl Default for MyApp {
    fn default() -> Self {
        let mut field = Field::default();
        let enum_default = template::enumset::Enum::enum_default(&mut field);

        

        let mut ret = Self {
            ctx: Default::default(),
            field,

            /* elem: Element::Driven(
                template::Driven::Enum(template::enumset::Enum { 
                    cond: template::enumset::Cond::Elem(vec!["Noel".into()]), 
                    default: enum_default,
                })),
            drv: Driver::Enum("Noel".into()), */
            spw: Default::default(),
            dfield: DriverField { map: Default::default() },
        };

        let elem_table = Element::Driven(template::Driven::TableH(Box::new(Element::Driven(template::Driven::CheckBoxRT("try".into())))));
        let drv_table = Driver::List(vec![Driver::CheckBoxRT(true),Driver::CheckBoxRT(false),Driver::CheckBoxRT(true)]);
        let spawn = template::Spawn::new(&elem_table, &drv_table, &mut ret.ctx, &ret.field);

        ret.spw = spawn.clone();
        ret.field.insert_spawner("Table_Example".into(), (spawn,elem_table,drv_table.clone()).into(), vec![]);
        ret.dfield.map.insert("Table_Example".into(), drv_table);
        /*
        let elem = Element::Static(template::Static::LabelRT("hello world".into()));
        let drv = Driver::None;
        let spawn = template::Spawn::new(
                &elem,
                &drv,
                &mut ret.ctx,
                &ret.field,
            );
        ret.field.insert_spawner("Noel".into(), (
            spawn,elem.clone(),drv
        ).into(),vec![]);
        ret.spw = template::Spawn::new(&ret.elem, &ret.drv, &mut ret.ctx, &ret.field);*/
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
            ui.label("hello\nw\na\no\nr\nd\na\no\nj\nd\ns\na");
            /* let elem = &Element::Static(template::Static::LabelRT("hello world".into()));
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

                id: Id::new("hello world")
            }.ui(ui);

            self.field.insert_spawner("Noel".into(), (
                template::Spawn::new(elem, drv, &mut self.ctx, &self.field),
                elem.clone(),
                Driver::None
            ).into(),vec![]);  */

            Component {
                spw: &mut self.spw,
                elem: &self.field.map[&"Table_Example".into()].elem,
                drv: &mut self.dfield.map.get_mut(&"Table_Example".into()).unwrap(),
                field: &self.field,
                ctx: &mut self.ctx,

                id: "Noel".into()
            }.ui(ui);
        });
    }
}