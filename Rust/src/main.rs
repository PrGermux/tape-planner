#![windows_subsystem = "windows"]

use eframe::egui::{self, TextStyle, FontId, RichText, Color32};
use eframe::IconData;
use std::fs::File;
use std::io::Read;

struct TapePlanner {
    tapes: Vec<String>,
    result: Vec<Vec<RichText>>, // Change result to store a vector of RichText vectors
}

impl Default for TapePlanner {
    fn default() -> Self {
        Self {
            tapes: Vec::new(),
            result: Vec::new(), // Initialize as an empty vector
        }
    }
}

impl eframe::App for TapePlanner {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the global font size
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::proportional(24.0)),
            (TextStyle::Body, FontId::proportional(14.0)),
            (TextStyle::Monospace, FontId::monospace(14.0)),
            (TextStyle::Button, FontId::proportional(14.0)),
            (TextStyle::Small, FontId::proportional(12.0)),
        ].into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Add tape button
            if ui.add_sized([100.0, 30.0], egui::Button::new("Add tape")).clicked() {
                self.tapes.push(String::new());
            }

            // Delete tape button
            if ui.add_sized([100.0, 30.0], egui::Button::new("Delete tape")).clicked() {
                self.tapes.pop();
            }

            // Display tape length inputs with placeholder
            for tape in &mut self.tapes {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(tape)
                            .hint_text("Enter tape length min 300 m")
                    );
                });
            }

            // Calculate button
            if ui.add_sized([100.0, 30.0], egui::Button::new("Calculate")).clicked() {
                self.calculate();
            }

            // Display result
            ui.separator();
            for result_line in &self.result {
                ui.horizontal(|ui| {
                    for rich_text in result_line {
                        ui.label(rich_text.clone()); // Display each part of the line
                    }
                });
            }
        });
    }
}

impl TapePlanner {
    fn calculate(&mut self) {
        let lengths: Vec<f64> = self.tapes.iter()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        if lengths.is_empty() {
            self.result = vec![vec![RichText::new("Please enter at least one valid tape length.").color(Color32::BLACK)]];
            return;
        }

        // Perform the calculation logic
        self.result = self.find_combinations(&lengths);
    }

    fn find_combinations(&self, lengths: &[f64]) -> Vec<Vec<RichText>> {
        let sort_1_range: Vec<f64> = (3000..=3600).map(|x| x as f64 / 10.0).collect();
        let sort_2_range: Vec<f64> = (3610..=6000).map(|x| x as f64 / 10.0).collect();

        let mut results = Vec::new();

        results.push(vec![RichText::new(format!("Input is {:?} m, sum {:.1} m\n", lengths, lengths.iter().sum::<f64>())).color(Color32::BLACK)]);

        let (details, ratio) = self.backtrack(0, 0.0, 0.0, Vec::new(), lengths, &sort_1_range, &sort_2_range);

        if let Some((details, ratio)) = details.zip(ratio) {
            for detail in details {
                results.push(detail);
            }
            results.push(vec![RichText::new(format!("\nRatio {}", ratio)).color(Color32::BLACK)]);
        } else {
            results.push(vec![RichText::new("No valid combinations found. Please add more tapes.").color(Color32::BLACK)]);
        }

        results
    }

    fn backtrack(
        &self,
        index: usize,
        s1_count: f64,
        s2_count: f64,
        details: Vec<Vec<RichText>>,
        lengths: &[f64],
        sort_1_range: &[f64],
        sort_2_range: &[f64],
    ) -> (Option<Vec<Vec<RichText>>>, Option<String>) {
        if index == lengths.len() {
            let ratio = self.simplify_ratio(s1_count, s2_count);
            if ["2:1", "5:2", "3:2"].contains(&ratio.as_str()) {
                return (Some(details), Some(ratio));
            }
            return (None, None);
        }

        let length = lengths[index];

        // Try splitting into Sort 1 and Sort 2 combinations
        for &s1 in sort_1_range {
            for &s2 in sort_2_range {
                if (s1 + s2 - length).abs() < 1e-3 {
                    let s1_text = RichText::new(format!("1x {} m,", s1)).color(Color32::DARK_GREEN);
                    let s2_text = RichText::new(format!("1x {} m", s2)).color(Color32::DARK_RED);
                    let mut new_details = details.clone();
                    new_details.push(vec![
                        s1_text,
                        s2_text,
                        RichText::new(format!("from {} m tape", length)).color(Color32::BLACK),
                    ]);
                    if let (Some(d), Some(r)) = self.backtrack(
                        index + 1,
                        s1_count + 1.0,
                        s2_count + 1.0,
                        new_details,
                        lengths,
                        sort_1_range,
                        sort_2_range,
                    ) {
                        return (Some(d), Some(r));
                    }
                }
            }
        }

        // Try multiple Sort 1 segments
        for &s1 in sort_1_range {
            if (length % s1).abs() < 1e-3 {
                let num_s1_segments = (length / s1).round();
                let s1_text = RichText::new(format!("{}x {} m", num_s1_segments, s1)).color(Color32::DARK_GREEN);
                let mut new_details = details.clone();
                new_details.push(vec![
                    s1_text,
                    RichText::new(format!("from {} m tape", length)).color(Color32::BLACK),
                ]);
                if let (Some(d), Some(r)) = self.backtrack(
                    index + 1,
                    s1_count + num_s1_segments,
                    s2_count,
                    new_details,
                    lengths,
                    sort_1_range,
                    sort_2_range,
                ) {
                    return (Some(d), Some(r));
                }
            }
        }

        // Try multiple Sort 2 segments
        for &s2 in sort_2_range {
            if (length % s2).abs() < 1e-3 {
                let num_s2_segments = (length / s2).round();
                let s2_text = RichText::new(format!("{}x {} m", num_s2_segments, s2)).color(Color32::DARK_RED);
                let mut new_details = details.clone();
                new_details.push(vec![
                    s2_text,
                    RichText::new(format!("from {} m tape", length)).color(Color32::BLACK),
                ]);
                if let (Some(d), Some(r)) = self.backtrack(
                    index + 1,
                    s1_count,
                    s2_count + num_s2_segments,
                    new_details,
                    lengths,
                    sort_1_range,
                    sort_2_range,
                ) {
                    return (Some(d), Some(r));
                }
            }
        }

        // No valid combination found
        (None, None)
    }

    fn simplify_ratio(&self, s1_count: f64, s2_count: f64) -> String {
        let gcd = gcd((s1_count * 10.0) as i64, (s2_count * 10.0) as i64);
        format!(
            "{}:{}",
            (s1_count * 10.0 / gcd as f64).round(),
            (s2_count * 10.0 / gcd as f64).round()
        )
    }
}

// Helper function to calculate the greatest common divisor (GCD)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn load_icon(path: &str) -> IconData {
    let mut file = File::open(path).expect("Failed to open icon file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read icon file");

    let image = image::load_from_memory(&buf).expect("Failed to load image from memory");
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    IconData {
        width,
        height,
        rgba,
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(350.0, 400.0)),
        icon_data: Some(load_icon("res/icon.png")), // Load the icon
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Tape Planner",
        native_options,
        Box::new(|_cc| Box::new(TapePlanner::default())),
    );
}
