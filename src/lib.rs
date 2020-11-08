#![allow(unused)]

use std::sync::Arc;

pub struct Instance {
    context: Arc<egui::Context>,
    raw_input: egui::RawInput,
    start_time: std::time::Instant,
    paint_jobs: egui::PaintJobs,
}

impl Instance {
    pub fn new(screen_size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) -> Self {
        let context = egui::Context::new();
        let start_time = std::time::Instant::now();
        let raw_input = egui::RawInput {
            mouse_down: false,
            mouse_pos: None,
            scroll_delta: egui::Vec2::default(),
            screen_size: egui::vec2(screen_size.width as f32, screen_size.height as f32),
            pixels_per_point: Some(scale_factor as f32),
            time: 0.0,
            events: vec![],
        };

        Self {
            context,
            raw_input,
            start_time,
            paint_jobs: vec![],
        }
    }

    pub fn update_time(&mut self) {
        self.raw_input.time = self.start_time.elapsed().as_secs_f64();
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(new_inner_size) => {
                self.raw_input.screen_size =
                    egui::vec2(new_inner_size.width as f32, new_inner_size.height as f32);
            }
            winit::event::WindowEvent::Moved(_) => {}
            winit::event::WindowEvent::CloseRequested => {}
            winit::event::WindowEvent::Destroyed => {}
            winit::event::WindowEvent::DroppedFile(_) => {}
            winit::event::WindowEvent::HoveredFile(_) => {}
            winit::event::WindowEvent::HoveredFileCancelled => {}
            winit::event::WindowEvent::ReceivedCharacter(_) => {}
            winit::event::WindowEvent::Focused(_) => {}
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => {}
            winit::event::WindowEvent::ModifiersChanged(_) => {}
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.raw_input.mouse_pos = Some(egui::pos2(position.x as f32, position.y as f32));
            }
            winit::event::WindowEvent::CursorEntered { device_id } => {}
            winit::event::WindowEvent::CursorLeft { device_id } => {
                self.raw_input.mouse_pos = None;
            }
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => {}
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                match state{
                    winit::event::ElementState::Pressed => {
                        self.raw_input.mouse_down = true;
                    }
                    winit::event::ElementState::Released => {
                        self.raw_input.mouse_down = false;
                    }
                }
            }
            winit::event::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => {}
            winit::event::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => {}
            winit::event::WindowEvent::Touch(_) => {}
            winit::event::WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.raw_input.pixels_per_point = Some(scale_factor.clone() as f32);
                self.raw_input.screen_size =
                    egui::vec2(new_inner_size.width as f32, new_inner_size.height as f32);
            }
            winit::event::WindowEvent::ThemeChanged(_) => {}
        }
    }

    pub fn paint_jobs(&self) -> &egui::PaintJobs {
        &self.paint_jobs
    }

    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub fn begin_frame(&mut self) -> egui::Ui {
        self.context.begin_frame(self.raw_input.clone())
    }

    pub fn end_frame(&mut self) {
        let (_, paint_jobs) = self.context.end_frame();
        self.paint_jobs = paint_jobs;
    }
}
