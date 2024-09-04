use std::{clone, default, ops::{Deref, DerefMut}};
use egui::{ahash::{HashMap, HashSet}, style::Spacing, vec2, Button, FontId, Id, Key, Layout, Modifiers, Pos2, Response, RichText, TextEdit, Ui, Vec2, Widget};
use enumset::EnumSpawn;

use crate::{context::Context};

pub mod decorative;
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
}


#[derive(Clone)]
pub enum Element {
    Horizontal(Vec<Element>),
    Vertical(Vec<Element>),
    Static(Static),
    Driven(Driven),
}
 
// 小小地说两句，关于Field。
// 首先，显而易见，Field是全局的、默认只读的量。
// tag是一个全局的Cache系统（吗？）
// 与默认的Spawn作为Element的临时不同
// Field除去Spawn的信息，都是被建议直接存储的。
// 啊、这么说得更正一下了，所以tag也是需要被保存的……
// 坏了，没有id -> IdMap，这下犯大错了。<- 你在胡言乱语什么啊喂！！！

#[derive(Clone)]
pub struct Chunk {
    pub elem: Element,
    pub drv: Driver,

    pub tags: Vec<String>,
}


impl Default for Chunk {
    fn default() -> Self {
        Self { elem: Element::Static(Static::LabelRT(String::new().into())), drv: Driver::None, tags: Default::default() }
    }
}

impl Chunk {
    fn with_elem(elem: Element) -> Self {
        Self { elem, drv: Driver::None, tags: Default::default() }
    }
}

#[derive(Clone)]
pub struct Spawner {
    pub spw: Spawn,
    pub elem: Element,
    pub drv: Driver,
}

impl From<(Spawn,Element,Driver)> for Spawner {
    fn from(value: (Spawn,Element,Driver)) -> Self {
        Self { spw: value.0, elem: value.1, drv: value.2 }
    }
}

impl Into<(Spawn,Element,Driver)> for Spawner {
    fn into(self) -> (Spawn,Element,Driver) {
        (self.spw,self.elem,self.drv)
    }
}

#[derive(Default)]
pub struct Field {
    pub map: egui::IdMap<Chunk>, 
    pub spw: egui::IdMap<Spawn>,
    pub tag: egui::ahash::HashMap<String,Vec<Id>>
    // every spawn has its id.
    // when refering to a spawn, we call it using the map
}

pub struct DriverField {
    pub map: egui::IdMap<Driver>,
}

impl Field {

    pub fn spawn(&mut self, dfield: &mut DriverField, id: Id, ctx: &mut Context, ui: &mut Ui, cache: &mut Cache) {
        Component {
            spw: self.spw.get_mut(&id).unwrap(),
            elem: &self.map[&id].elem,
            drv: &mut self.map.get_mut(&id).unwrap().drv,
            id,
            field: todo!(),
            ctx,
        }.ui(ui);
    }

    pub fn insert_spawner(&mut self, k: Id, v: Spawner, tags: Vec<String>) {
        self.map.insert(k,Chunk { elem: v.elem, drv: v.drv, tags });
        self.spw.insert(k, v.spw);
    }
    pub fn get_spawner(&self, k: Id) -> Option<Spawner> {
        let c =self.map.get(&k)?.clone();
        let s = self.spw.get(&k)?.clone();
        Some((s,c.elem,c.drv).into())
    }
}
// Wait ——
// should an element that has no focus owning a ...

// I mean: Context, 启动！


impl Default for Spawn {
    fn default() -> Self {
        Self { child: Default::default(), cache: Cache::None }
    }
}

impl Spawn {
    /* #[inline]
    pub fn with_name(mut self, str: String) -> Self {
        self.name = str;
        self
    } */

    pub fn new(elem: &Element, drv: &Driver, ctx: &mut Context, field: &Field) -> Self {
        match elem {
            Element::Horizontal(elem) | Element::Vertical(elem) => {
                let Driver::List(drv) = drv else { panic!("drv not correspond"); };
                let mut ret =  Self {
                    child: Vec::with_capacity(elem.len()),
                    cache: Cache::None,
                    /* name: String::new(),
                    id: Id::new("Horizontal").with(ctx.count.get()), */
                };

                Iterator::zip(elem.iter(), drv.iter()).for_each(|(elem,drv)| {
                    ret.child.push(Spawn::new(elem, drv, ctx, field));
                });
                
                ret
            },
            Element::Driven(elem) => {
                match elem {
                    Driven::Enum(elem) => {
                        let Driver::Enum(drv) = drv else { panic!("drv not correspond"); };
                        let (vec,searcher) = match &elem.cond {
                            enumset::Cond::Elem(elem) => {
                                let capacity = elem.len();
                                elem.iter().enumerate()
                                    .fold(
                                        (Vec::with_capacity(capacity),simsearch::SimSearch::new()), 
                                        |(mut e,mut s),(id,ex)| {
                                            e.push(ex.clone());
                                            s.insert_tokens(
                                                id,
                                                field.map[ex].tags.iter()
                                                    .map(|s|s.as_str())
                                                    .collect::<Vec<_>>()
                                                    .as_slice());
                                            (e,s)
                                })
                            },
                            enumset::Cond::Tags(tags) => {
                                //field
                                let iter = tags.iter()
                                    .map(|tag| &field.tag[tag])
                                    .flatten();
                                let capacity = iter.clone().count();
                                iter.enumerate()
                                    .fold((Vec::with_capacity(capacity),simsearch::SimSearch::new()), 
                                        |(mut e,mut s),(id,ex)| {
                                            e.push(ex.clone());
                                            s.insert_tokens(
                                                id,
                                                tags.iter()
                                                    .map(|s|s.as_str())
                                                    .collect::<Vec<_>>()
                                                    .as_slice());
                                            (e,s)

                                    })
                            },
                        };

                        Self {
                            child: (vec.iter().map(|id| {
                                field.spw[id].clone()
                            })).collect(),
                            cache: Cache::Enum(enumset::Cache { 
                                searcher, 
                                selected: Box::new(field.get_spawner(elem.default)
                                    .unwrap_or_else(||{panic!("elem default {} not exist",elem.default.value())})
                                    .into()),
                                selectable: vec, // elem should be a 
                            }),
                        }
                    },
                    Driven::TableH(elem) => {
                        let Driver::List(drv) = drv else {panic!("drv not correspond")};
                        Self {
                            child: drv.iter().map(|drv| Self::new(elem, drv, ctx, field)).collect(), 
                            ..Default::default()
                        }
                    }
                    _ => Default::default()
                }
            },
            _ => Default::default()
        }
    }
}

pub struct Component<'a> {
    pub spw: &'a mut Spawn,
    pub elem: &'a Element,
    pub drv: &'a mut Driver,

    pub id: Id,
    pub field: &'a Field,
    pub ctx: &'a mut Context,
}

impl<'a> Widget for Component<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let Component { spw: spawn, elem, drv, field, ctx, id } = self;
        ctx.depth += 1;
        match elem {
            Element::Horizontal(ele) => {
                let Driver::List(drv) = drv else { panic!("drv not correspond"); };
                ui.horizontal(|ui| {
                    spawn.child.iter_mut()
                        .zip(ele.iter())
                        .zip(drv).for_each(|((spawn,elem),drv)| {
                        Component {
                            spw: spawn,elem,drv,field,ctx,
                            id: Id::NULL
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
                            spw: spawn,elem,drv,field,ctx,
                            id: Id::NULL
                        }.ui(ui);
                    });
                }).response
            },
            Element::Static(widget) => match widget {
                Static::LabelRT(rt) => { ui.label(rt.clone()) },
            },
            Element::Driven(dr) => match dr {
                Driven::Enum(cond) => {
                    let Driver::Enum(drv) = drv else { panic!("drv not correspond "); };
                    let Cache::Enum(cache) = &mut spawn.cache else { panic!("cache not correspond ") };
                    EnumSpawn {
                        id: Id::new("enum").with(id),
                        drv: drv, cache, ctx, field,
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

                    let resp = ui.allocate_ui(vec2(25., 25.), |ui| {
                        ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                            if Button::new(RichText::new("   +   ")
                                .font(FontId::monospace(15.0)))
                                .wrap_mode(egui::TextWrapMode::Extend)
                                .ui(ui).clicked() {
                                let new_driver = Driver::default_for(elem, field);
                                spawn.child.push(Spawn::new(elem, &new_driver, ctx, field));
                                drv.push(new_driver);
                            }
                            else if Button::new(RichText::new("   -   ")
                                .font(FontId::monospace(15.0)))
                                .wrap_mode(egui::TextWrapMode::Extend)
                                .ui(ui).clicked() {
                                spawn.child.pop();
                                drv.pop();
                            }
                            ui.add_space(10.);
                            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                                spawn.child.iter_mut()
                                    .zip(drv).for_each(|(spawn,drv)| {
                                    Component {
                                        spw: spawn,elem,drv,field,ctx,
                                        id: Id::NULL
                                    }.ui(ui);
                                });
                            });
                        }).response
                    });

                    decorative::bra_ket(ui, resp.response, ("{","}"));

                    resp.inner
                    
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
    Enum(Box<Spawner>), // the element you select.
    // not that.
    CheckBoxRT(bool),
    TableH(Vec<Driver>),
    TextBoxEditable(String)

    // ... 无语了 ...
    // 根据element的类型，按序输入Driver
    // 总觉得不太好，但是按序是好文明吧
}

impl Driver {
    pub fn default_for(elem: &Element, field: &Field) -> Self {
        match elem {
            Element::Horizontal(_) => todo!(),
            Element::Vertical(_) => todo!(),
            Element::Static(_) => {
                Self::None
            },
            Element::Driven(elem) => {
                match elem {
                    Driven::Enum(elem) => { Self::Enum(Box::new(field.get_spawner(Id::new("default")).unwrap_or_else(||panic!("`default` not found")).clone())) },
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