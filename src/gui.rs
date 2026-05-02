use egui_macroquad::egui::{
    CollapsingHeader, DragValue, Rgba, Slider, Ui, WidgetText,
    color_picker::{self, Alpha},
};

use libcorr::{
    lightsource::LightSource, matrix::Matrix4F, quaternion::Quaternion, transform::Transform,
    vector::Vector4F,
};

use crate::camera::Camera;

fn v4f_ui(vec: &mut Vector4F, ui: &mut Ui, reset: Option<Vector4F>) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label("x:");
        changed |= ui.add(DragValue::new(vec.x_mut()).speed(0.1)).changed();
        ui.label("y:");
        changed |= ui.add(DragValue::new(vec.y_mut()).speed(0.1)).changed();
        ui.label("z:");
        changed |= ui.add(DragValue::new(vec.z_mut()).speed(0.1)).changed();
        if let Some(reset) = reset {
            if ui.button("⟲").clicked() {
                *vec = reset;
                changed = true;
            }
        }
    });
    changed
}

fn quat_v4f_ui(vec: &mut Vector4F, ui: &mut Ui, reset: Option<Vector4F>) -> bool /*changed */ {
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
        if let Some(reset) = reset {
            if ui.button("⟲").clicked() {
                *vec = reset;
                changed = true;
            }
        }
    });
    changed
}

fn quat_ui(quat: &mut Quaternion, ui: &mut Ui, alias: &mut Vector4F) {
    if quat_v4f_ui(alias, ui, Some(Vector4F::default())) {
        let rads = Vector4F::new(
            alias.x().to_radians(),
            alias.y().to_radians(),
            alias.z().to_radians(),
            1.0,
        );
        quat.from_euler(rads);
    }
}

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
    pub fn ui(&mut self, ui: &mut Ui, heading: impl Into<WidgetText>) {
        CollapsingHeader::new(heading)
            .default_open(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("pos");
                    v4f_ui(self.transform.pos_mut(), ui, Some(self.init_pos));
                    ui.label("rot");
                    quat_ui(self.transform.rot_mut(), ui, &mut self.rot_vec);
                });
            });
    }
}

pub struct DisplayCamera {
    camera: Camera,
    // settings storage
    pub fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl DisplayCamera {
    pub fn new(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            camera: Camera::new(Matrix4F::new_perspective(
                fov.to_radians(),
                aspect_ratio,
                z_near,
                z_far,
            )),
            fov,
            aspect_ratio,
            z_near,
            z_far,
        }
    }

    pub fn reproject(&mut self) {
        self.camera.reproject(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("Camera")
            .default_open(true)
            .show(ui, |ui| {
                let mut changed = false;
                let mut changed_near = false;
                let mut changed_far = false;
                ui.horizontal(|ui| {
                    ui.label("FOV :");
                    changed |= ui
                        .add(DragValue::new(&mut self.fov).speed(3.0).range(30.0..=120.0))
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("z_near :");
                    changed_near |= ui.add(Slider::new(&mut self.z_near, 0.1..=100.0)).changed();
                });
                ui.horizontal(|ui| {
                    ui.label("z_far    :");
                    changed_far |= ui.add(Slider::new(&mut self.z_far, 0.1..=100.0)).changed();
                });

                if changed_near {
                    self.z_far = self.z_far.max(self.z_near);
                }
                if changed_far {
                    self.z_near = self.z_near.min(self.z_far);
                }
                if changed || changed_near || changed_far {
                    self.camera.reproject(
                        self.fov.to_radians(),
                        self.aspect_ratio,
                        self.z_near,
                        self.z_far,
                    );
                }
            });
    }
}

impl std::ops::Deref for DisplayCamera {
    type Target = Camera;
    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for DisplayCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}

pub struct DisplayLightSource {
    light: LightSource,
    quat: Quaternion,
    color: [f32; 3],
}

impl DisplayLightSource {
    pub fn new(direction: Vector4F, color: [u8; 4], intensity: u8) -> Self {
        Self {
            light: LightSource::new(direction, color, intensity),
            quat: Quaternion::default(),
            color: [
                color[0].clamp(0, 255) as f32,
                color[1].clamp(0, 255) as f32,
                color[2].clamp(0, 255) as f32,
            ],
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("Light")
            .default_open(true)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("rot : ");

                        if v4f_ui(self.light.direction_mut(), ui, None) {
                            *self.light.direction_mut() = self.light.direction().normalized();
                        }
                    });
                    // intensity
                    ui.horizontal(|ui| {
                        ui.label("color : ");
                        if color_picker::color_edit_button_rgb(ui, &mut self.color).changed() {
                            *self.light.color_mut() = [
                                (self.color[0] * 255.0) as u8,
                                (self.color[1] * 255.0) as u8,
                                (self.color[2] * 255.0) as u8,
                                255,
                            ];
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("intensity :");
                        let mut intensity = self.light.intensity() as f32 / 128.0;
                        if ui.add(Slider::new(&mut intensity, 0.0..=2.0)).changed() {
                            *self.light.intensity_mut() = (intensity * 128.0) as u8;
                        }
                    });
                });
            });
    }
}

impl std::ops::Deref for DisplayLightSource {
    type Target = LightSource;
    fn deref(&self) -> &Self::Target {
        &self.light
    }
}

impl std::ops::DerefMut for DisplayLightSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.light
    }
}
