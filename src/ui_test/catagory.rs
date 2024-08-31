use egui::{pos2, vec2, Align2, Color32, FontId, Layout, Pos2, Rect, RichText, Sense, Vec2, Widget};

pub struct CatagorySpawn {
    pos: Pos2,
    size: Vec2,
    title: RichText,
    child: Vec<CatagorySpawn>,

}

impl Default for CatagorySpawn {
    fn default() -> Self {
        Self { 
            pos: Default::default(), 
            size: vec2(400.0, 300.0), 
            title: RichText::new("CatagorySpwan").size(30.), 
            child: Default::default() 
        }
    }
}

impl Widget for CatagorySpawn {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let resp = egui::Area::new(egui::Id::new(self.title.text()))
            .default_pos(self.pos)
            .movable(true)
            .default_size(self.size)
            .show(ui.ctx(), |ui| {
                ui.add(
                    egui::Label::new(self.title)
                    .selectable(false)
                    .wrap_mode(egui::TextWrapMode::Extend)
                );
                
                ui.allocate_at_least(self.size, Sense::focusable_noninteractive());
                self.child.into_iter().for_each(|x| {x.ui(ui);});
            }).response;
        let Rect { min, max } = resp.rect;
        let size = resp.rect.size().y;
        let painter = ui.painter();
        painter.text(
            pos2(min.x - size/2.,max.y), 
            Align2::LEFT_BOTTOM, "[", 
            FontId::monospace(resp.rect.size().y), 
            Color32::from_gray(120)
        );
        painter.text(
            pos2(max.x + size/2.,max.y), 
            Align2::RIGHT_BOTTOM, "]", 
            FontId::monospace(resp.rect.size().y), 
            Color32::from_gray(120)
        );

        resp
    }
}