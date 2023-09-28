#![warn(clippy::all)]
#![allow(unused)]

use crate::gui::{
    first_column, plot_durations, plot_timeframes, second_column, third_column, UpdateOutputsTrait,
};
use crate::processing::{AsMyStringTrait, ProjectType, TimeframeTrait, Update};
use crate::MainApp;
use chrono::{Duration, Timelike};
use egui::Ui;

use egui_plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot, PlotPoints};

pub fn central_panel(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    // screen size:
    let available_width = ui.available_width();
    let available_height = ui.available_height();

    // layout:
    let num_columns = 3u32;

    app.state.update();
    app.outputs.update(&app.state);

    // ui:
    egui::ScrollArea::horizontal()
        .min_scrolled_width(available_width)
        .show(ui, |ui| {
            ui.style_mut().wrap = Some(false);

            ui.horizontal(|ui| {
                egui::ScrollArea::vertical()
                    .min_scrolled_height(available_height)
                    .show(ui, |ui| {
                        ui.style_mut().wrap = Some(false);

                        first_column(
                            app,
                            ctx,
                            frame,
                            ui,
                            available_width,
                            available_height,
                            num_columns,
                        );
                        second_column(
                            app,
                            ctx,
                            frame,
                            ui,
                            available_width,
                            available_height,
                            num_columns,
                        );
                        third_column(
                            app,
                            ctx,
                            frame,
                            ui,
                            available_width,
                            available_height,
                            num_columns,
                        );
                    });
            });
        });
}
