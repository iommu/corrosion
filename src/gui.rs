use egui_macroquad::egui::{CollapsingHeader, DragValue, Ui, WidgetText};

use libcorr::{quaternion::Quaternion, transform::Transform, vector::Vector4F};

pub struct DisplayTransform {
    transform: Transform,
    rot_vec: Vector4F,
    init_pos: Vector4F,
}

impl DisplayTransform {
    pub fn default() -> Self {
        Self {
            transform: Transform::default(),
            rot_vec: Vector4F::new(0.0, 0.0, 0.0, 1.0),
            init_pos: Vector4F::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn from_pos(pos: Vector4F) -> Self {
        Self {
            transform: Transform::from_pos(pos),
            rot_vec: Vector4F::new(0.0, 0.0, 0.0, 1.0),
            init_pos: pos,
        }
    }
}

impl std::ops::Deref for DisplayTransform {
    type Target = Transform;
    fn deref(&self) -> &Self::Target {
        &self.transform
    }
}

impl DisplayTransform {
    fn v4f_ui(vec: &mut Vector4F, ui: &mut Ui, reset: Vector4F) {
        ui.horizontal(|ui| {
            ui.label("x:");
            ui.add(DragValue::new(vec.x_mut()).speed(0.1)).changed();
            ui.label("y:");
            ui.add(DragValue::new(vec.y_mut()).speed(0.1)).changed();
            ui.label("z:");
            ui.add(DragValue::new(vec.z_mut()).speed(0.1)).changed();
            if ui.button("⟲").clicked() {
                *vec = reset;
            }
        });
    }

    fn quat_v4f_ui(vec: &mut Vector4F, ui: &mut Ui, reset: Vector4F) -> bool /*changed */ {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("x:");
            changed |= ui
                .add(DragValue::new(vec.x_mut()).speed(1.0).range(-180.0..=180.0))
                .changed();
            ui.label("y:");
            changed |= ui
                .add(DragValue::new(vec.y_mut()).speed(1.0).range(-180.0..=180.0))
                .changed();
            ui.label("z:");
            changed |= ui
                .add(DragValue::new(vec.z_mut()).speed(1.0).range(-180.0..=180.0))
                .changed();
            if ui.button("⟲").clicked() {
                *vec = reset;
                changed = true;
            }
        });
        changed
    }

    pub fn quat_ui(quat: &mut Quaternion, ui: &mut Ui, alias: &mut Vector4F) {
        if Self::quat_v4f_ui(alias, ui, Vector4F::default()) {
            let rads = Vector4F::new(
                alias.x().to_radians(),
                alias.y().to_radians(),
                alias.z().to_radians(),
                1.0,
            );
            quat.from_euler(rads);
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, heading: impl Into<WidgetText>) {
        CollapsingHeader::new(heading)
            .default_open(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("pos");
                    Self::v4f_ui(self.transform.pos_mut(), ui, self.init_pos);
                    ui.label("rot");
                    Self::quat_ui(self.transform.rot_mut(), ui, &mut self.rot_vec);
                });
            });
    }
}
