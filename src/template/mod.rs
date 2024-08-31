use std::{default, ops::{Deref, DerefMut}};
use egui::{ahash::{HashMap, HashSet}, Button, FontId, Id, Key, Layout, Modifiers, Pos2, Response, RichText, TextEdit, Ui, Widget};
use enumset::EnumSpawn;

use crate::{context::Context};

pub mod enumset;

// what about 'MadeUped Template', I mean, a function defined by user for instance.
// it IS an element; I know for sure. However, it is driven by an unstable driver.

struct Template {
    id: String,
    version: String,
    element: Vec<ElemSpace>,
    shortcut: Vec<ShortCutMNG>,
}
// there has to be a shortcut to put the first block onto the window.

impl Template {
    fn init(&mut self) {
        let mut action = ActionMNG::new();
        for sc in &mut self.shortcut {
            match sc.short {
                ShortCut::OnceStart => {
                    action.merge(&mut sc.action)
                },
                _ => ()
            }
        }
        action.run();
    }
}


struct ShortCutMNG {
    short: ShortCut,
    action: ActionMNG,
    modifier: Modifiers
}

struct ActionMNG {
    action: Vec<Action>
}

enum Action {
    
}

impl ActionMNG {
    fn new() -> Self { ActionMNG { action: Vec::new()} }
    fn merge(&mut self,rhs: &mut Self) { self.action.append(&mut rhs.action)}
    fn run(&mut self) {}
}

enum ShortCut {
    KeyPressed(Key),
    OnceStart,
}

#[derive(Clone)]
struct ElemSpace {
    elem: Element,
    tags: HashSet<String>,
    id: Id,
} // -> into field. Is ElemSpace Important?
// 保存文件的id使用情况？

#[derive(Clone)]
pub struct Spawn {
    child: Vec<Spawn>,
    cache: Cache,
    pub name: String,
    pub id: Id,
}


#[derive(Clone)]
pub enum Element {
    Horizontal(Vec<Element>),
    Vertical(Vec<Element>),
    Static(Static),
    Driven(Driven),
}


pub struct Field {
    pub map: egui::IdMap<(Spawn,Element,Driver)>, 
    pub tag: egui::ahash::HashMap<String,Vec<Id>>
    // every spawn has its id.
    // when refering to a spawn, we call it using the map
}
// Wait ——
// should an element that has no focus owning a ...

// I mean: Context, 启动！


impl Default for Spawn {
    fn default() -> Self {
        Self { child: Default::default(), cache: Cache::None, name: Default::default(), id: Id::NULL }
    }
}

impl Spawn {
    #[inline]
    pub fn with_name(mut self, str: String) -> Self {
        self.name = str;
        self
    } 
    pub fn new(elem: &Element, drv: &Driver, ctx: &mut Context, field: &Field) -> Self {
        let mut ret: Self;
        match elem {
            Element::Horizontal(elem) | Element::Vertical(elem) => {
                let Driver::List(drv) = drv else { panic!("drv not correspond"); };
                ret = Self {
                    child: Vec::with_capacity(elem.len()),
                    cache: Cache::None,
                    name: String::new(),
                    id: Id::new("Horizontal").with(ctx.count.get()),
                };

                Iterator::zip(elem.iter(), drv.iter()).for_each(|(elem,drv)| {
                    ret.child.push(Spawn::new(elem, drv, ctx, field));
                });
                
            },
            Element::Static(elem) => {
                match elem {
                    Static::LabelRT(_) => ret = Default::default(),
                }
            },
            Element::Driven(elem) => {
                match elem {
                    Driven::Enum(elem) => {
                        let Driver::Enum(drv) = drv else { panic!("drv not correspond"); };
                        let mut searcher =  simsearch::SimSearch::new();
                        let vec = match &elem.cond {
                            enumset::Cond::Elem(elem) => {
                                let capacity = elem.len();
                                elem.iter().enumerate()
                                    .fold(
                                        (Vec::with_capacity(capacity),&mut searcher), 
                                        |(mut e,s),(id,(ex,sx))| {
                                    e.push((ex.clone(),sx.clone()));
                                    s.insert(id, sx);

                                    (e,s)
                                }).0
                            },
                            enumset::Cond::Tags(tag) => {
                                //field
                                let mut ret = Vec::new();
                                for tag in tag {
                                    let mut vec = Vec::with_capacity(field.tag[tag].len());
                                    for id in &field.tag[tag] {
                                        let (spw,elem,drv) = &field.map[id];
                                        vec.push((*id,spw.name.clone()));
                                    }
                                    ret.append(&mut vec);
                                }
                                ret
                            },
                        };

                        ret = Self {
                            child: (vec.iter().map(|(id,str)| {
                                field.map[id].clone().0
                            })).collect(),
                            name: String::new(),
                            id: Id::new("Enum").with(ctx.count.get()),
                            cache: Cache::Enum(enumset::Cache { 
                                searcher, 
                                selected: Box::new(
                                    (
                                        Spawn::new(&Element::Static(Static::LabelRT("unselected".into())), &Driver::None, ctx, field),
                                        Element::Static(Static::LabelRT("unselected".into())),
                                        Driver::None, // 这里我们需要输入本身包含有一个default，否则我们就用上面这个。
                                    )),
                                selectable: vec, // elem should be a 
                            }),
                        };
                    },
                    Driven::CheckBoxRT(_) => todo!(),
                    Driven::TableH(_) => todo!(),
                    Driven::TextBoxEditable(_) => todo!(),
                }
            },
        };

        ret
    }
}

pub struct Component<'a> {
    pub spawn: &'a mut Spawn,
    pub elem: &'a Element,
    pub drv: &'a mut Driver,
    pub field: &'a Field,
    pub ctx: &'a mut Context,
}

impl<'a> Widget for Component<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let Component { spawn, elem, drv, field, ctx } = self;
        ctx.depth += 1;
        match elem {
            Element::Horizontal(ele) => {
                let Driver::List(drv) = drv else { panic!("drv not correspond"); };
                ui.horizontal(|ui| {
                    spawn.child.iter_mut()
                        .zip(ele.iter())
                        .zip(drv).for_each(|((spawn,elem),drv)| {
                        Component {
                            spawn,elem,drv,field,ctx
                        }.ui(ui);
                    });
                }).response
            },
            Element::Vertical(ele) => {
                let Driver::List(drv) = drv else { panic!("drv not correspond"); };
                ui.vertical(|ui| {
                    spawn.child.iter_mut()
                        .zip(ele.iter())
                        .zip(drv).for_each(|((spawn,elem),drv)| {
                        Component {
                            spawn,elem,drv,field,ctx
                        }.ui(ui);
                    });
                }).response
            },
            Element::Static(widget) => match widget {
                Static::LabelRT(rt) => { ui.label(rt.clone()) },
            },
            Element::Driven(dr) => match dr {
                Driven::Enum(cond) => {
                    let Driver::Enum(drv) = *drv else { panic!("drv not correspond "); };
                    let Cache::Enum(cache) = &mut spawn.cache else { panic!("cache not correspond ") };
                    EnumSpawn {
                        id: Id::new("enum").with(spawn.id),
                        drv, cache, ctx, field,
                        spw: &mut spawn.child,
                        elem: &cond.cond,
                    }.ui(ui)
                    // here is where the Spawn works. In short, if we have an elem & a drv, we shall render the ui through 
                    // using the spawn. Drv + Elem = ... ! <- nye. There's context for focus changing. We pack it and send to anyone in need. 
                },
                Driven::CheckBoxRT(rt) => {
                    let Driver::CheckBoxRT(b) = drv else { panic!("drv not correspond "); };
                    ui.checkbox(b, rt.clone())
                },
                Driven::TableH(elem) => {
                    let Driver::List(drv) = drv else { panic!("drv not correspond"); };
                    ctx.inc_table.run(|| {drv.push(Driver::None);});
                    ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                        let button_add = ui.button(RichText::new("   +   ").font(FontId::monospace(15.0)));
                        let button_sub = ui.button(RichText::new("   -   ").font(FontId::monospace(15.0)));
                        if button_add.clicked() {
                            drv.push(Driver::None);
                        }
                        else if button_sub.clicked() {
                            drv.pop();
                        }
                        ui.horizontal( |ui| {
                            spawn.child.iter_mut()
                                .zip(drv).for_each(|(spawn,drv)| {
                                Component {
                                    spawn,elem,drv,field,ctx
                                }.ui(ui);
                            });
                        });
                    }).response
                },
                Driven::TextBoxEditable(_) => todo!(),
            },
        }
    }
}

#[derive(Clone)]
enum Cache {
    Enum(enumset::Cache),
    None
}

impl Default for Cache {
    fn default() -> Self {
        Self::None
    }
}

/*
const ELE: fn() -> Element = || Element::Vertical(vec![
    Element::Static(
        Static::LabelRT(RichText::new("Story Module"))
    ),
    Element::Driven(Driven::TableH(
        Box::new(Element::Horizontal(vec![
            Element::Driven(
                Driven::Enum(
                    Enum {
                        cond: enumset::Cond::Elem(
                            vec![
                                (Element::Static(
                                    Static::LabelRT(
                                        RichText::new("Noel")
                                    )
                                ),"Noel".into()),
                                (Element::Static(
                                    Static::LabelRT(
                                        RichText::new("Alma")
                                    )
                                ),"Alma".into()),
                            ]
                        ), 
                        default: Static::LabelRT(
                            RichText::new("Alma")
                        ),
                        id: Id::new(0)
                        }
                    )
                )
            ,
            Element::Driven(
                Driven::TextBoxEditable(
                    egui::FontId { size: 15.0, ..Default::default() }
                )
            )
        ]))
    ))
]);*/



#[derive(Clone)]
struct TagCache {
    pub vec: Vec<(Id, String)> // 但是这里没问题。Drv或者是usize？会乱掉的吧。
}



struct Data {
    
}

#[derive(Clone)]
pub enum Static {
    LabelRT(egui::RichText)
}

#[derive(Clone)]
pub enum Driver {
    None,
    List(Vec<Driver>),

    // Following for Driven
    Enum(Id), // the element you select.
    // not that.
    CheckBoxRT(bool),
    TableH(Vec<Driver>),
    TextBoxEditable(String)

    // ... 无语了 ...
    // 根据element的类型，按序输入Driver
    // 总觉得不太好，但是按序是好文明吧
}

impl Driver {
    pub fn default_for(elem: &Element) -> Self {
        match elem {
            Element::Horizontal(_) => todo!(),
            Element::Vertical(_) => todo!(),
            Element::Static(_) => {
                Self::None
            },
            Element::Driven(elem) => {
                match elem {
                    Driven::Enum(elem) => { Self::Enum(Id::new("default")) },
                    Driven::CheckBoxRT(_) => Self::CheckBoxRT(false),
                    Driven::TableH(elem) => {Self::TableH(vec![])},
                    Driven::TextBoxEditable(elem) => {Self::TextBoxEditable(String::new())},
                }
            },
        }
    }
}

#[derive(Clone)]
pub enum Driven {
    Enum(enumset::Enum),
    CheckBoxRT(egui::RichText),
    TableH(Box<Element>),
    TextBoxEditable(FontId),
    //Button(Box<Button<'a>>),
}

// Driver input for -> Driven drive -> spawn.
// Action -> Works.


/*
pub fn merge<'a>(ui: &mut Ui, elem: &Element, drv: &mut impl Iterator<Item = &'a mut Driver>) {
    match elem {
        Element::Horizontal(vec) => {
            ui.horizontal(|ui| {
                for elem in vec {
                    merge(ui, elem, drv)
                }
            });
        },
        Element::Vertical(vec) => {
            ui.vertical(|ui| {
                for elem in vec {
                    merge(ui, elem, drv)
                }
            });
        },
        Element::Static(widget) => match widget {
            Static::LabelRT(rt) => { ui.label(rt.clone()); },
        },
        Element::Driven(dr) => match dr {
            Driven::Enum(e) => {
                let Some(Driver::Enum(elem)) = drv.next() else { panic!("drv not correspond "); };
                // here is where the Spawn works. In short, if we have an elem & a drv, we shall render the ui through 
                // using the spawn. Drv + Elem = ... ! <- nye. There's context for focus changing. We pack it and send to anyone in need. 
            },
            Driven::CheckBoxRT(rt) => {
                let Some(Driver::CheckBoxRT(b)) = drv.next() else { panic!("drv not correspond "); };
                ui.checkbox(b, rt.clone());
            },
            Driven::TableH(single) => {
                let Some(Driver::TableH(vec)) = drv.next() else { panic!("drv not correspond "); };
                ui.horizontal(|ui| {
                    for driver in vec {
                        merge(ui, single, &mut driver.iter_mut());
                    }
                });
            },
            Driven::TextBoxEditable(_) => todo!(),
        },
    }
}
    */