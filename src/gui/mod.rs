use std::time::Instant;

use eframe::egui;

use crate::{input::InputBindingMap, aircraft::{Aircraft, AircraftCatalog}, fdm::FlightState, instrumentation::InstrumentationState, input::InputState, input::JoystickInput, time::SimTime};

#[derive(Debug, Clone, Default)]
pub struct StartupState {
    pub selected_aircraft: usize,
    pub airport_icao: String,
    pub speed_kts: f64,
    pub altitude_ft: f64,
    pub heading_deg: f64,
    pub tab: StartupTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StartupTab {
    #[default]
    Location,
    Aircraft,
    Conditions,
}

#[derive(Debug)]
pub struct RustGearApp {
    pub catalog: AircraftCatalog,
    pub selected_index: usize,
    pub plane: Aircraft,
    pub state: FlightState,
    pub input: InputState,
    pub instruments: InstrumentationState,
    pub time: SimTime,
    pub accum: f64,
    pub last: Instant,
    pub autopilot_heading: f64,
    pub autopilot_altitude: f64,
    pub autopilot_speed: f64,
    pub ap_heading: bool,
    pub ap_altitude: bool,
    pub ap_speed: bool,
    pub bindings: InputBindingMap,
    pub joystick: JoystickInput,
    pub keys_held: std::collections::HashSet<egui::Key>,
    pub paused: bool,
    pub startup: Option<StartupState>,
    pub view3d: bool,
    pub roll_angle: f32,
    pub pitch_angle: f32,
    pub yaw_angle: f32,
    pub prev_p_pressed: bool,
    pub prev_r_pressed: bool,
    pub debug: bool,
}

impl RustGearApp {
    pub fn new(catalog: AircraftCatalog, selected_index: usize, debug: bool) -> Self {
        let cfg = &catalog.entries[selected_index];
        let plane = Aircraft::new(cfg.id.as_str(), cfg.name.as_str(), cfg.model);
        Self {
            catalog,
            selected_index,
            plane,
            input: InputState {
                throttle: 0.0,
                pitch: 0.0,
                roll: 0.0,
                yaw: 0.0,
            },
            last: Instant::now(),
            accum: 0.0,
            autopilot_heading: 0.0,
            autopilot_altitude: 0.0,
            autopilot_speed: 0.0,
            ap_heading: false,
            ap_altitude: false,
            ap_speed: false,
            bindings: InputBindingMap::default(),
            joystick: JoystickInput::new(),
            state: FlightState::default(),
            instruments: InstrumentationState::default(),
            time: SimTime::new(),
            keys_held: std::collections::HashSet::default(),
            paused: false,
            startup: Some(StartupState {
                selected_aircraft: selected_index,
                airport_icao: String::new(),
                speed_kts: 0.0,
                altitude_ft: 0.0,
                heading_deg: 0.0,
                tab: StartupTab::default(),
            }),
            view3d: true,
            roll_angle: 0.0,
            pitch_angle: 0.0,
            yaw_angle: 0.0,
            prev_p_pressed: false,
            prev_r_pressed: false,
            debug,
        }
    }

    pub fn reset_to_runway(&mut self) {
        self.input = InputState::default();
        if let Some(startup) = &self.startup {
            self.state.speed_kts = startup.speed_kts;
            self.state.altitude_ft = startup.altitude_ft;
            self.state.heading_deg = startup.heading_deg;
        }
        self.state.pitch_deg = 6.0;
        self.state.roll_deg = 0.0;
        self.state.gamma_rad = 0.0;
        self.state.vertical_speed_fpm = 0.0;
        self.state.turn_rate_deg_per_s = 0.0;
        self.state.pitch_rate_deg_per_s = 0.0;
        self.time = SimTime::new();
        self.instruments = InstrumentationState::default();
    }

    fn step(&mut self) {
        if self.paused {
            return;
        }
        self.plane.step(crate::fdm::flight_model::FIXED_DT, &self.input, &mut self.state);
        self.instruments.update(&self.state, crate::fdm::flight_model::FIXED_DT);
        self.time.update(crate::fdm::flight_model::FIXED_DT);
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        let mut pressed = Vec::new();
        let mut released = Vec::new();

        ctx.input(|i| {
            for key in self.keys_held.iter() {
                if !i.key_down(*key) {
                    released.push(*key);
                }
            }
            for key in [
                egui::Key::ArrowUp,
                egui::Key::ArrowDown,
                egui::Key::ArrowLeft,
                egui::Key::ArrowRight,
                egui::Key::Q,
                egui::Key::E,
                egui::Key::PageUp,
                egui::Key::PageDown,
                egui::Key::Enter,
                egui::Key::Num0,
                egui::Key::P,
                egui::Key::R,
            ] {
                if i.key_down(key) && !self.keys_held.contains(&key) {
                    pressed.push(key);
                }
            }
        });

        for key in &released {
            self.keys_held.remove(key);
        }
        for key in &pressed {
            self.keys_held.insert(*key);
        }

        let throttle_rate = 1.0 * crate::fdm::flight_model::FIXED_DT;
        let pitch_rate = 1.0 * crate::fdm::flight_model::FIXED_DT;
        let roll_rate = 1.0 * crate::fdm::flight_model::FIXED_DT;
        let yaw_rate = 1.0 * crate::fdm::flight_model::FIXED_DT;

        if self.keys_held.contains(&egui::Key::ArrowUp) {
            self.input.pitch = (self.input.pitch - pitch_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::ArrowDown) {
            self.input.pitch = (self.input.pitch + pitch_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::ArrowLeft) {
            self.input.roll = (self.input.roll - roll_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::ArrowRight) {
            self.input.roll = (self.input.roll + roll_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::Q) {
            self.input.yaw = (self.input.yaw - yaw_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::E) {
            self.input.yaw = (self.input.yaw + yaw_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::Enter) {
            self.input.yaw = (self.input.yaw - yaw_rate).clamp(-1.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::Num0) {
            self.input.yaw = (self.input.yaw + yaw_rate).clamp(-1.0, 1.0);
        }
        let p_now = ctx.input(|i| i.key_down(egui::Key::P));
        let r_now = ctx.input(|i| i.key_down(egui::Key::R));
        if p_now && !self.prev_p_pressed {
            self.paused = !self.paused;
        }
        if r_now && !self.prev_r_pressed {
            self.reset_to_runway();
        }
        self.prev_p_pressed = p_now;
        self.prev_r_pressed = r_now;
        if self.keys_held.contains(&egui::Key::PageUp) {
            self.input.throttle = (self.input.throttle + throttle_rate).clamp(0.0, 1.0);
        }
        if self.keys_held.contains(&egui::Key::PageDown) {
            self.input.throttle = (self.input.throttle - throttle_rate).clamp(0.0, 1.0);
        }
        if !self.keys_held.contains(&egui::Key::ArrowUp) && !self.keys_held.contains(&egui::Key::ArrowDown) {
            self.input.pitch *= 0.9;
        }
        if !self.keys_held.contains(&egui::Key::ArrowLeft) && !self.keys_held.contains(&egui::Key::ArrowRight) {
            self.input.roll *= 0.9;
        }
        if !self.keys_held.contains(&egui::Key::Q) && !self.keys_held.contains(&egui::Key::E) {
            self.input.yaw *= 0.9;
        }
    }
}

impl eframe::App for RustGearApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt_real = now.duration_since(self.last).as_secs_f64().min(0.1);
        self.last = now;
        self.accum += dt_real;
        while self.accum >= crate::fdm::flight_model::FIXED_DT {
            self.accum -= crate::fdm::flight_model::FIXED_DT;
            self.step();
        }
        if ctx.input(|i| i.key_down(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.handle_keyboard_input(ctx);
        self.joystick.poll();
        if self.joystick.has_fresh_data() {
            self.joystick.state.apply(&mut self.input, &self.bindings);
            self.joystick.clear_fresh_data();
        }

        if let Some(startup) = &mut self.startup {
            if draw_startup(ctx, startup, &mut self.selected_index, &mut self.plane, &mut self.catalog) {
                self.state.speed_kts = startup.speed_kts;
                self.state.altitude_ft = startup.altitude_ft;
                self.state.heading_deg = startup.heading_deg;
                self.state.pitch_deg = 6.0;
                self.state.roll_deg = 0.0;
                self.state.gamma_rad = 0.0;
                self.state.vertical_speed_fpm = 0.0;
                self.state.turn_rate_deg_per_s = 0.0;
                self.state.pitch_rate_deg_per_s = 0.0;
                self.input = InputState::default();
                self.time = SimTime::new();
                self.instruments = InstrumentationState::default();
                self.startup = None;
            }
            ctx.request_repaint();
            return;
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.label(format!(
                    "   {} | {} | T {:.2} | {}",
                    self.plane.id, self.plane.name, self.input.throttle as f32, if self.paused { "PAUSED" } else { "" }
                ));
                ui.separator();
                ui.checkbox(&mut self.view3d, "3D");
                if self.debug {
                    ui.checkbox(&mut self.debug, "Debug mode");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.view3d {
                let rect = ui.available_rect_before_wrap();
                draw_3d_view(ui.painter(), rect, &self.state, &mut self.roll_angle, &mut self.pitch_angle, self.debug);
            } else {
                let painter = ui.painter();
                let rect = ui.available_rect_before_wrap();
                draw_hud(painter, rect, &self.state, &self.instruments, &self.plane.systems, &self.time);
            }

            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Controls");
                    ui.separator();
                    egui::Grid::new("controls").striped(true).show(ui, |ui| {
                        ui.label("Throttle");
                        ui.add(egui::Slider::new(&mut self.input.throttle, 0.0..=1.0).text("T"));
                        ui.end_row();

                        ui.label("Pitch");
                        ui.add(egui::Slider::new(&mut self.input.pitch, -1.0..=1.0).text("P"));
                        ui.end_row();

                        ui.label("Roll");
                        ui.add(egui::Slider::new(&mut self.input.roll, -1.0..=1.0).text("R"));
                        ui.end_row();

                        ui.label("Yaw");
                        ui.add(egui::Slider::new(&mut self.input.yaw, -1.0..=1.0).text("Y"));
                        ui.end_row();
                    });
                });
            });
        });
        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        eprintln!("RustGearApp exiting");
    }
}

fn draw_startup(
    ctx: &egui::Context,
    startup: &mut StartupState,
    _selected_index: &mut usize,
    _plane: &mut Aircraft,
    catalog: &mut AircraftCatalog,
) -> bool {
    let mut launched = false;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add_space(4.0);
                ui.heading("RustGear");
                ui.separator();
                ui.selectable_value(&mut startup.tab, StartupTab::Location, "Location");
                ui.selectable_value(&mut startup.tab, StartupTab::Aircraft, "Aircraft");
                ui.selectable_value(&mut startup.tab, StartupTab::Conditions, "Conditions");
                ui.add_space(8.0);
                if ui.button("Fly!").clicked() {
                    launched = true;
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                match startup.tab {
                    StartupTab::Location => {
                        ui.label("Location");
                        ui.label("Airport ICAO");
                        ui.text_edit_singleline(&mut startup.airport_icao);
                    }
                    StartupTab::Aircraft => {
                        ui.label("Aircraft");
                        egui::ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                            for (i, entry) in catalog.entries.iter().enumerate() {
                                if ui.selectable_label(startup.selected_aircraft == i, entry.name.as_str()).clicked() {
                                    startup.selected_aircraft = i;
                                }
                            }
                        });
                        ui.label(format!("Selected: {}", catalog.entries[startup.selected_aircraft].name));
                    }
                    StartupTab::Conditions => {
                        ui.label("Startup Conditions");
                        egui::Grid::new("startup_fields").striped(true).show(ui, |ui| {
                            ui.label("Speed kt");
                            ui.add(egui::DragValue::new(&mut startup.speed_kts).speed(1.0).range(0.0..=300.0));
                            ui.end_row();

                            ui.label("Altitude ft");
                            ui.add(egui::DragValue::new(&mut startup.altitude_ft).speed(10.0).range(0.0..=50000.0));
                            ui.end_row();

                            ui.label("Heading °");
                            ui.add(egui::DragValue::new(&mut startup.heading_deg).speed(1.0).range(0.0..=360.0));
                            ui.end_row();
                        });
                    }
                }
            });
        });
    });
    launched
}

fn draw_3d_view(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &FlightState,
    roll_angle: &mut f32,
    pitch_angle: &mut f32,
    debug: bool,
) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let green = egui::Color32::from_rgb(0, 255, 0);
    let dim = egui::Color32::from_rgb(0, 180, 0);

    *roll_angle = (state.roll_deg as f32).to_radians();
    *pitch_angle = (state.pitch_deg as f32).to_radians();
    let scale = rect.width().min(rect.height()) * 0.25;

    let body_len = scale * 2.2;
    let wing_half = scale * 1.4;
    let wing_span = scale * 0.6;
    let tail_len = scale * 0.6;

    let nose = egui::pos2(cx - body_len, cy);
    let tail_center = egui::pos2(cx + body_len, cy);

    painter.line_segment([nose, tail_center], (2.5, dim));
    painter.line_segment([tail_center, egui::pos2(tail_center.x + tail_len, tail_center.y)], (2.0, dim));
    painter.line_segment([egui::pos2(cx, cy), egui::pos2(cx - wing_half, cy - wing_span)], (1.5, dim));
    painter.line_segment([egui::pos2(cx, cy), egui::pos2(cx + wing_half, cy - wing_span)], (1.5, dim));

    let dir = egui::Vec2::new(pitch_angle.cos(), pitch_angle.sin());
    painter.line_segment(
        [nose, egui::pos2(nose.x - dir.x * scale * 0.5, nose.y - dir.y * scale * 0.5)],
        (2.0, green),
    );

    if debug {
        let text = format!(
            "SPD: {:.1} kt\nALT: {:.0} ft\nVS: {:.0} fpm",
            state.speed_kts, state.altitude_ft, state.vertical_speed_fpm
        );
        painter.text(
            egui::pos2(rect.left() + 10.0, rect.bottom() - 60.0),
            egui::Align2::LEFT_BOTTOM,
            text,
            egui::FontId::monospace(14.0),
            green,
        );
    }
}

fn draw_hud(
    painter: &egui::Painter,
    rect: egui::Rect,
    _state: &FlightState,
    instruments: &InstrumentationState,
    systems: &crate::systems::SystemsState,
    time: &SimTime,
) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width();
    let h = rect.height();

    let green = egui::Color32::from_rgb(0, 255, 0);
    let dim = egui::Color32::from_rgb(0, 180, 0);

    let ai_cx = cx;
    let ai_cy = cy - 80.0;
    let ai_r = (h * 0.38).min(w * 0.38).min(170.0);
    let pitch_rad = (instruments.pitch_deg as f32).to_radians();
    let roll_rad = (instruments.roll_deg as f32).to_radians();
    let pitch_offset = pitch_rad.sin() * ai_r * 0.8;
    let horizon_y = ai_cy - pitch_offset;

    let speed = instruments.airspeed_kt as f32;
    let speed_x = cx - w * 0.32;
    let speed_y = ai_cy - ai_r - 35.0;
    painter.text(
        egui::pos2(speed_x, speed_y),
        egui::Align2::CENTER_TOP,
        format!("{:.0} kt", speed),
        egui::FontId::monospace(18.0),
        green,
    );

    let sky_top = if roll_rad.cos() > 0.0 { horizon_y - ai_r } else { horizon_y + ai_r };
    let ground_top = if roll_rad.cos() > 0.0 { horizon_y + ai_r } else { horizon_y - ai_r };
    let clipped = egui::Rect::from_center_size(egui::pos2(ai_cx, ai_cy), egui::vec2(ai_r * 2.0, ai_r * 2.0));
    let _ai_clipped = painter.with_clip_rect(clipped);
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(ai_cx, sky_top + ai_r * 0.5), egui::vec2(ai_r * 2.2, ai_r * 1.1)),
        0.0,
        egui::Color32::from_rgb(20, 40, 80),
    );
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(ai_cx, ground_top + ai_r * 0.5), egui::vec2(ai_r * 2.2, ai_r * 1.1)),
        0.0,
        egui::Color32::from_rgb(80, 50, 20),
    );

    let hx1 = ai_cx - roll_rad.cos() * ai_r;
    let hy1 = horizon_y - roll_rad.sin() * ai_r;
    let hx2 = ai_cx + roll_rad.cos() * ai_r;
    let hy2 = horizon_y + roll_rad.sin() * ai_r;
    painter.line_segment([egui::pos2(hx1, hy1), egui::pos2(hx2, hy2)], (2.0, green));

    let pitch_marks: [i32; 5] = [-20, -10, 0, 10, 20];
    for mark in pitch_marks {
        let m_rad = (mark as f32).to_radians();
        let dy = -m_rad.sin() * ai_r * 0.8;
        let y = ai_cy + dy;
        if y < clipped.top() + 10.0 || y > clipped.bottom() - 10.0 {
            continue;
        }
        let half_width = 20.0;
        let dx = roll_rad.sin() * half_width;
        let width = if mark == 0 { 2.5 } else { 1.5 };
        painter.line_segment(
            [egui::pos2(ai_cx - dx - half_width, y - roll_rad.cos() * half_width), egui::pos2(ai_cx - dx + 8.0, y - roll_rad.cos() * half_width)],
            (width, dim),
        );
        painter.line_segment(
            [egui::pos2(ai_cx + dx - 8.0, y - roll_rad.cos() * half_width), egui::pos2(ai_cx + dx + half_width, y - roll_rad.cos() * half_width)],
            (width, dim),
        );
        if mark != 0 {
            let sign = if mark > 0 { "" } else { "-" };
            painter.text(
                egui::pos2(ai_cx - dx + 12.0, y - roll_rad.cos() * 8.0),
                egui::Align2::LEFT_CENTER,
                format!("{}{}", sign, mark.abs()),
                egui::FontId::monospace(11.0),
                dim,
            );
        }
    }

    let pointer_dist = ai_r - 15.0;
    let px = ai_cx + roll_rad.sin() * pointer_dist;
    let py = ai_cy - roll_rad.cos() * pointer_dist;
    painter.line_segment([egui::pos2(px - 5.0, py - 5.0), egui::pos2(px, py)], (2.0, green));
    painter.line_segment([egui::pos2(px + 5.0, py - 5.0), egui::pos2(px, py)], (2.0, green));

    let wing = 25.0;
    painter.line_segment([egui::pos2(ai_cx - wing, ai_cy), egui::pos2(ai_cx - 6.0, ai_cy)], (2.5, green));
    painter.line_segment([egui::pos2(ai_cx + 6.0, ai_cy), egui::pos2(ai_cx + wing, ai_cy)], (2.5, green));
    painter.line_segment([egui::pos2(ai_cx, ai_cy + 4.0), egui::pos2(ai_cx, ai_cy + 14.0)], (2.5, green));
    painter.circle_filled(egui::pos2(ai_cx, ai_cy), 3.0, green);

    let _rect_clipped = painter.with_clip_rect(rect);

    let tb_w = 120.0;
    let tb_h = 28.0;
    let tb_x = cx;
    let tb_y = rect.top() + tb_h / 2.0 + 10.0;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(tb_x, tb_y), egui::vec2(tb_w, tb_h)),
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 20, 0, 180),
    );
    painter.rect_stroke(
        egui::Rect::from_center_size(egui::pos2(tb_x, tb_y), egui::vec2(tb_w, tb_h)),
        0.0,
        (1.0, green),
    );

    let tape_h = 28.0;
    let tape_y = rect.bottom() - tape_h / 2.0 - 10.0;
    let status_y = tape_y - tape_h / 2.0 - 24.0;
    painter.text(
        egui::pos2(cx, status_y),
        egui::Align2::CENTER_BOTTOM,
        format!(
            "HDG {:.0}°  VS {:.0} fpm  T {:.1}s  Fuel {:.1}kg  IAS {:.0} kt  ALT {:.0} ft",
            instruments.heading_deg,
            instruments.vertical_speed_fpm,
            time.sim_time,
            systems.fuel_remaining_kg,
            instruments.airspeed_kt,
            instruments.altitude_ft
        ),
        egui::FontId::monospace(13.0),
        green,
    );

    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx, tape_y), egui::vec2(w * 0.6, tape_h)),
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 20, 0, 180),
    );
    painter.rect_stroke(
        egui::Rect::from_center_size(egui::pos2(cx, tape_y), egui::vec2(w * 0.6, tape_h)),
        0.0,
        (1.0, green),
    );

    let heading_f = instruments.heading_deg as f32;
    let tape_w = w * 0.6;
    let pixels_per_deg = tape_w / 60.0;

    for deg in (heading_f as i32 - 30)..=(heading_f as i32 + 30) {
        let h = ((deg % 360) + 360) % 360;
        let x = cx + (deg as f32 - heading_f) * pixels_per_deg;
        if x < rect.left() + 20.0 || x > rect.right() - 20.0 {
            continue;
        }

        let is_major = h % 30 == 0;
        let tick_h = if is_major { 10.0 } else { 5.0 };
        painter.line_segment(
            [egui::pos2(x, tape_y - tick_h), egui::pos2(x, tape_y + tick_h)],
            (1.5, if is_major { green } else { dim }),
        );

        if is_major {
            painter.text(
                egui::pos2(x, tape_y - tick_h - 10.0),
                egui::Align2::CENTER_BOTTOM,
                format!("{:03}", h),
                egui::FontId::monospace(12.0),
                green,
            );
        }
    }

    let tape_w2 = 70.0;
    let tape_h2 = h * 0.5;
    let tape_x = rect.left() + tape_w2 / 2.0 + 10.0;
    let tape_y2 = cy + 20.0;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(tape_x, tape_y2), egui::vec2(tape_w2, tape_h2)),
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 20, 0, 180),
    );
    painter.rect_stroke(
        egui::Rect::from_center_size(egui::pos2(tape_x, tape_y2), egui::vec2(tape_w2, tape_h2)),
        0.0,
        (1.0, green),
    );

    let speed = instruments.airspeed_kt as f32;
    let speed_range = 40.0_f32..=250.0_f32;
    let marker_x = tape_x + tape_w2 / 2.0 - 6.0;
    painter.line_segment(
        [egui::pos2(marker_x, tape_y2 - 12.0), egui::pos2(marker_x + 10.0, tape_y2)],
        (2.0, green),
    );
    painter.line_segment(
        [egui::pos2(marker_x, tape_y2 + 12.0), egui::pos2(marker_x + 10.0, tape_y2)],
        (2.0, green),
    );

    for i in (0..=10).rev() {
        let y = tape_y2 + tape_h2 / 2.0 - (i as f32 / 10.0) * tape_h2;
        let val = *speed_range.start() + (i as f32 / 10.0) * (*speed_range.end() - *speed_range.start());
        painter.line_segment(
            [egui::pos2(tape_x - tape_w2 / 2.0 + 4.0, y), egui::pos2(tape_x + tape_w2 / 2.0 - 4.0, y)],
            (1.0, dim),
        );
        if i % 2 == 0 {
            painter.text(
                egui::pos2(tape_x - tape_w2 / 2.0 + 6.0, y),
                egui::Align2::LEFT_CENTER,
                format!("{:.0}", val),
                egui::FontId::monospace(11.0),
                green,
            );
        }
    }

    painter.text(
        egui::pos2(tape_x - tape_w2 / 2.0 + 6.0, tape_y2 + tape_h2 / 2.0 + 14.0),
        egui::Align2::LEFT_TOP,
        format!("{:.0} kt", speed),
        egui::FontId::monospace(14.0),
        green,
    );

    let tape_w3 = 70.0;
    let tape_x3 = rect.right() - tape_w3 / 2.0 - 10.0;
    let marker_x3 = tape_x3 - tape_w3 / 2.0 + 6.0;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(tape_x3, tape_y2), egui::vec2(tape_w3, tape_h2)),
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 20, 0, 180),
    );
    painter.rect_stroke(
        egui::Rect::from_center_size(egui::pos2(tape_x3, tape_y2), egui::vec2(tape_w3, tape_h2)),
        0.0,
        (1.0, green),
    );

    let vs = instruments.vertical_speed_fpm as f32;
    let vs_range = -20.0_f32..=20.0_f32;
    painter.line_segment(
        [egui::pos2(marker_x3, tape_y2 - 12.0), egui::pos2(marker_x3 - 10.0, tape_y2)],
        (2.0, green),
    );
    painter.line_segment(
        [egui::pos2(marker_x3, tape_y2 + 12.0), egui::pos2(marker_x3 - 10.0, tape_y2)],
        (2.0, green),
    );

    for i in 0..=10 {
        let y = tape_y2 + tape_h2 / 2.0 - (i as f32 / 10.0) * tape_h2;
        let val = *vs_range.start() + (i as f32 / 10.0) * (*vs_range.end() - *vs_range.start());
        painter.line_segment(
            [egui::pos2(tape_x3 - tape_w3 / 2.0 + 4.0, y), egui::pos2(tape_x3 + tape_w3 / 2.0 - 4.0, y)],
            (1.0, dim),
        );
        if i % 2 == 0 {
            painter.text(
                egui::pos2(tape_x3 + tape_w3 / 2.0 - 6.0, y),
                egui::Align2::RIGHT_CENTER,
                format!("{:.0}", val),
                egui::FontId::monospace(11.0),
                green,
            );
        }
    }

    painter.text(
        egui::pos2(tape_x3 + tape_w3 / 2.0 - 6.0, tape_y2 + tape_h2 / 2.0 + 14.0),
        egui::Align2::RIGHT_TOP,
        format!("{:.0} fpm", vs),
        egui::FontId::monospace(14.0),
        green,
    );
}
