use egui::{pos2, Align2, Color32, Response, Ui};

pub fn bra_ket(ui: &mut Ui, resp: Response, pair:(&'static str,&'static str)) {
    let egui::Rect { min, max } = resp.rect;
    let size = resp.rect.size().y;
    let painter = ui.painter();
    painter.text(
        pos2(min.x - size/3.,max.y+size*0.222222), 
        Align2::LEFT_BOTTOM, pair.0, 
        egui::FontId::monospace(size*1.222222), 
        Color32::from_gray(120)
    );
    painter.text(
        pos2(max.x + size/3.,max.y+size*0.222222), 
        Align2::RIGHT_BOTTOM, pair.1, 
        egui::FontId::monospace(size*1.2222222), 
        Color32::from_gray(120)
    );
}