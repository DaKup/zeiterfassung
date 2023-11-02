#![warn(clippy::all)]

use crate::MainApp;
use chrono::Timelike;
use egui::Ui;
use egui_plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Legend, Plot};

pub fn plot_timeframes(
    app: &mut MainApp,
    _ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
    width: f32,
    height: f32,
    _rounded: bool,
) {
    let bars: Vec<BoxElem> = app
        .state
        .tasks
        .iter()
        .enumerate()
        .map(|(_i, task)| {
            // let project_id = i;
            let project_id = 0;

            // let lower_whisker= task.timeframe.begin().timestamp() as f64;
            // let upper_whisker= task.timeframe.end().timestamp() as f64;

            // let (lower_whisker, upper_whisker) = match app.state.rounded_plots {
            //     false => (task.timeframe.begin().timestamp() as f64, task.timeframe.end().timestamp() as f64),
            //     true => (task.timeframe.round().begin().timestamp() as f64, task.timeframe.round().end().timestamp() as f64)
            // };

            let (begin, end) = match app.state.rounded_plots {
                false => (task.timeframe.begin(), task.timeframe.end()),
                true => (task.timeframe.round().begin(), task.timeframe.round().end()),
            };

            let lower_whisker = (begin.hour() * 60 + begin.minute()) as f64;
            let upper_whisker = (end.hour() * 60 + end.minute()) as f64;

            let (lower_whisker, upper_whisker) = match app.state.log_scale {
                false => (lower_whisker, upper_whisker),
                true => (lower_whisker.log10(), upper_whisker.log10()),
            };

            let quartille1 = lower_whisker;
            let median = (lower_whisker + upper_whisker) / 2.0;
            let quartille3 = upper_whisker;

            BoxElem::new(
                project_id as f64,
                BoxSpread::new(lower_whisker, quartille1, median, quartille3, upper_whisker),
            )
            .name(task.description.clone())
        })
        .collect();

    // let data_aspect = match app.state.log_scale {
    //     false => 1.01f64,
    //     true => (1.0f64).log10(),
    // };

    Plot::new("Timeframes")
        .view_aspect(0.5)
        .width(width)
        .height(height)
        .legend(Legend::default())
        .data_aspect(0.1)
        .show(ui, |plot_ui| {
            for box_elem in bars.iter() {
                let box_elem = box_elem.clone();
                let boxplot = BoxPlot::new(vec![box_elem]).horizontal();
                plot_ui.box_plot(boxplot);
            }
        });
}

pub fn plot_durations(
    app: &mut MainApp,
    _ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
    width: f32,
    height: f32,
) {
    let bars: Vec<Bar> = app
        .state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let project_id = i;
            let mut height = 0f64;

            let duration = match app.state.rounded_plots {
                true => task.timeframe.round().duration().num_minutes(),
                false => task.timeframe.duration().num_minutes(),
            };

            if duration > 0 && app.state.log_scale {
                height = (duration as f64).log10();
            }

            Bar::new(project_id as f64, height).name(task.description.clone())
        })
        .collect();

    Plot::new("Durations")
        .view_aspect(0.5)
        .width(width)
        .height(height)
        .legend(Legend::default())
        .data_aspect(0.3)
        .show(ui, |plot_ui| {
            for bar in bars.iter() {
                let barchart = BarChart::new(vec![bar.clone()]);
                plot_ui.bar_chart(barchart);
            }
        });
}
