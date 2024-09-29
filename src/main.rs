use eframe::{Frame, egui, NativeOptions};
use egui::{CentralPanel, Context, Vec2, Color32, Sense, Pos2};

#[derive(Default)]
struct LaserPointer {
    position: Pos2,
}

impl LaserPointer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            position: Pos2 { x: 0.0, y: 0.0 }, // We'll put it in the top left corner to start
        }
    }
}

impl eframe::App for LaserPointer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let rect = ctx.screen_rect();
            let screen_size = Vec2 {
                x: rect.width(),
                y: rect.height(),
            };
            let (response, painter) = ui.allocate_painter(screen_size, Sense::hover());
            if response.hovered() {
                let Pos2 { x, y } = self.position;
                let Pos2 { x: x2, y: y2 } = ctx.pointer_hover_pos().unwrap_or_default();

                if (x - x2).abs() < 10.0 && (y - y2).abs() < 10.0 {
                    if fastrand::bool() {
                        self.position.x += fastrand::f32() * 20.0;
                    } else {
                        self.position.x -= fastrand::f32() * 20.0;
                    }

                    if fastrand::bool() {
                        self.position.y += fastrand::f32() * 20.0;
                    } else {
                        self.position.y -= fastrand::f32() * 20.0;
                    }
                }
            }
            self.position.x += 0.5;
            self.position.y += 0.5;
            let radius = 10.0;
            painter.circle_filled(self.position, radius, Color32::RED);
        });
    }
}

fn main() {
    let native_options = NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(LaserPointer::new(cc)))),
    );
}
