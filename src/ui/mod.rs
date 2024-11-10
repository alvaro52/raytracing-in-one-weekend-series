use eframe::{
    egui,
    egui::{Color32, ColorImage, ImageData, TextureHandle, TextureOptions},
    CreationContext,
};
use image::RgbImage;
use std::sync::Arc;

use crate::scene::Scene;

pub struct App {
    screen_texture: TextureHandle,
    scene: Scene,
}

impl App {
    pub fn run(scene: Scene) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([
                scene.camera.image_width as f32,
                scene.camera.image_height as f32,
            ]),
            ..Default::default()
        };

        eframe::run_native(
            "Raytracing in one weekend series",
            options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(App::new(cc, scene)))
            }),
        )
    }

    fn new(cc: &CreationContext, scene: Scene) -> Self {
        let image_width = scene.camera.image_width as usize;
        let image_height = scene.camera.image_height as usize;

        let image =
            RgbImage::from_vec(image_width as u32, image_height as u32, scene.render()).unwrap();

        let mut screen_texture = cc.egui_ctx.load_texture(
            "screen",
            ImageData::Color(Arc::new(ColorImage::new(
                [image_width, image_height],
                Color32::TRANSPARENT,
            ))),
            TextureOptions::default(),
        );

        screen_texture.set(
            ColorImage::from_rgb([image_width, image_height], &image.into_raw()),
            TextureOptions::default(),
        );

        Self {
            screen_texture,
            scene,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.add(
                    egui::Image::new(&self.screen_texture)
                        .max_height(self.scene.camera.image_height as f32)
                        .max_width(self.scene.camera.image_width as f32),
                );
            });
        });
    }
}
