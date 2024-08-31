use egui::{Pos2,Color32};

pub struct LinkerSpawn {
    start: Pos2,
    end: Pos2,
    on_link: bool,
    on_operating: bool,
}

impl Default for LinkerSpawn {
    fn default() -> Self {
        Self {
            start: Pos2 { x: 20., y: 20. },
            end: Pos2 { x: 200., y: 200. },
            on_link: false,
            on_operating: false,
        }
    }
}

impl LinkerSpawn {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { start, end, on_link, on_operating } = self;
        let painter = ui.painter();

        let stroke = egui::Stroke::new(2.0, Color32::GRAY);
        let stroke_circle = egui::Stroke::new(2.0, Color32::BLUE);
        
        let radius = 5.0;

        painter.circle_stroke(*start, radius, stroke_circle);
        painter.circle_stroke(*end, radius, stroke_circle);

        if ui.input(|i| {
            let p = &i.pointer;
            (p.button_clicked(egui::PointerButton::Primary) && (p.interact_pos().unwrap_or_default() - *start).length() < radius*2.0)
            || p.button_clicked(egui::PointerButton::Secondary)
        }) {
            *on_operating = !*on_operating;
        }

        if ui.input(|i| {
            let p = &i.pointer;
            p.button_clicked(egui::PointerButton::Primary) && (p.interact_pos().unwrap_or_default() - *end).length() < radius*2.0
        }) {
            if *on_operating {
                *on_operating = false;
                *on_link = true;
            }
            else if *on_link {
                *on_operating = true;
                *on_link = false;
            }
            
        }
        

        if !*on_operating && !*on_link { return; }
        let Some(end) = (
            if !*on_link {
                ui.input(|i| i.pointer.latest_pos())
            } else {Some(*end)}
        ) else { return; };

        let mid_point = Pos2::new(
            (start.x + end.x) / 2.0,
            (start.y + end.y) / 2.0,
        );
    
        let control_point1 = Pos2 { x: mid_point.x, y: start.y };
        let control_point2 = Pos2 { x: mid_point.x, y: end.y };
    
        // Draw a cubic Bezier curve to make it symmetric
        painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
            [
                *start, control_point1, control_point2, end
            ],
            false,
            Color32::TRANSPARENT,
            stroke,
        ));
    }
}