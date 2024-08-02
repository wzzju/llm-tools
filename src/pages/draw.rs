use crate::components::*;
use csv::ReaderBuilder;
use leptonic::{components::prelude::*, prelude::*};
use leptos::html::Div;
use leptos::*;
use leptos_chartistry::*;
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{Event, File, FileReader};

#[derive(Copy, Clone, Debug, Deserialize)]
struct Loss {
    step: f64,
    xpu: f64,
    gpu: f64,
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct LossDiff {
    step: f64,
    abs: f64,
    rel: f64,
}

#[derive(Copy, Clone, Debug, Deserialize, Default)]
pub struct Feature {
    max_diff: (f64, usize),
    min_diff: (f64, usize),
    mean_diff: f64,
    max_abs_diff: (f64, usize),
    min_abs_diff: (f64, usize),
    mean_abs_diff: f64,
    min_p_diff: (f64, usize),
    max_n_diff: (f64, usize),
}

impl Feature {
    pub fn new(
        max_diff: (f64, usize),
        min_diff: (f64, usize),
        mean_diff: f64,
        max_abs_diff: (f64, usize),
        min_abs_diff: (f64, usize),
        mean_abs_diff: f64,
        min_p_diff: (f64, usize),
        max_n_diff: (f64, usize),
    ) -> Self {
        Self {
            max_diff,
            min_diff,
            mean_diff,
            max_abs_diff,
            min_abs_diff,
            mean_abs_diff,
            min_p_diff,
            max_n_diff,
        }
    }
}

fn calculate_feature(diffs: &Vec<LossDiff>) -> Feature {
    let mut max_diff = f64::MIN;
    let mut max_diff_step: usize = 0;
    let mut min_diff = f64::MAX;
    let mut min_diff_step: usize = 0;
    let mut sum_diff = 0.0;
    let mut max_abs_diff: f64 = f64::MIN;
    let mut max_abs_diff_step: usize = 0;
    let mut min_abs_diff: f64 = f64::MAX;
    let mut min_abs_diff_step: usize = 0;
    let mut sum_abs_diff = 0.0;
    let mut min_p_diff = f64::MAX;
    let mut min_p_diff_step: usize = 0;
    let mut max_n_diff = f64::MIN;
    let mut max_n_diff_step: usize = 0;
    let len = diffs.len();
    for diff in diffs {
        if max_diff < diff.abs {
            max_diff = diff.abs;
            max_diff_step = diff.step as usize;
        }
        if min_diff > diff.abs {
            min_diff = diff.abs;
            min_diff_step = diff.step as usize;
        }
        if max_abs_diff < diff.abs.abs() {
            max_abs_diff = diff.abs.abs();
            max_abs_diff_step = diff.step as usize;
        }
        if min_abs_diff > diff.abs.abs() {
            min_abs_diff = diff.abs.abs();
            min_abs_diff_step = diff.step as usize;
        }
        sum_diff += diff.abs;
        sum_abs_diff += diff.abs.abs();
        if diff.abs >= 0.0 && min_p_diff > diff.abs {
            min_p_diff = diff.abs;
            min_p_diff_step = diff.step as usize;
        }
        if diff.abs < 0.0 && max_n_diff < diff.abs {
            max_n_diff = diff.abs;
            max_n_diff_step = diff.step as usize;
        }
    }

    if len > 0 {
        Feature::new(
            (max_diff, max_diff_step),
            (min_diff, min_diff_step),
            sum_diff / len as f64,
            (max_abs_diff, max_abs_diff_step),
            (min_abs_diff, min_abs_diff_step),
            sum_abs_diff / len as f64,
            (min_p_diff, min_p_diff_step),
            (max_n_diff, max_n_diff_step),
        )
    } else {
        Feature::default()
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn DrawPage() -> impl IntoView {
    let (file_name, set_file_name) = create_signal("[ step, xpu, gpu ]".to_string());
    let (loss, set_loss) = create_signal(vec![]);
    let (diff, set_diff) = create_signal(vec![]);
    let (global_loss, set_global_loss) = create_signal(vec![]);
    let (global_diff, set_global_diff) = create_signal(vec![]);
    let (global_len, set_global_len) = create_signal(0.0);
    let (start, set_start) = create_signal(0.0);
    let (end, set_end) = create_signal(-1.0);
    let (feature, set_feature) = create_signal(Feature::default());

    let loss_series = Series::new(|loss: &Loss| loss.step)
        .line(
            Line::new(|loss: &Loss| loss.xpu)
                .with_name("XPU")
                .with_interpolation(Interpolation::Linear),
        )
        .line(
            Line::new(|loss: &Loss| loss.gpu)
                .with_name("GPU")
                .with_interpolation(Interpolation::Linear),
        );

    // draw loss diff curve
    let diff_series = Series::new(|diff: &LossDiff| diff.step)
        .line(
            Line::new(|diff: &LossDiff| diff.abs)
                .with_name("Abs")
                .with_interpolation(Interpolation::Linear),
        )
        .line(
            Line::new(|diff: &LossDiff| diff.rel)
                .with_name("Rel")
                .with_interpolation(Interpolation::Linear),
        );

    // set axis ticks
    let x_ticks = TickLabels::default();
    let y_ticks = TickLabels::aligned_floats();

    // drag and drop files
    let (drag_color, set_drag_color) = create_signal("#e66956");
    let (chart_visibility, set_chart_visibility) = create_signal("none");
    let drop_zone_el = create_node_ref::<Div>();
    let update_data = create_action(move |file: &File| {
        let file = file.to_owned();
        set_file_name(format!("[ {} ]", file.name()));
        logging::log!("File name: {}", file.name());
        logging::log!("File size: {}", file.size());
        logging::log!("File type: {}", file.type_());
        logging::log!("File last modified time: {}", file.last_modified());
        async move {
            let file_reader = FileReader::new().unwrap();
            let onloadend = Closure::wrap(Box::new(move |event: Event| {
                let file_reader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                let content = file_reader.result().unwrap();
                let raw_data = js_sys::Uint8Array::from(content).to_string();
                let data = format!("step,xpu,gpu\n{}", raw_data);
                let mut csv_reader = ReaderBuilder::new().from_reader(data.as_bytes());
                let mut loss = vec![];
                let mut diff = vec![];
                for result in csv_reader.deserialize() {
                    let record: Loss = result.unwrap();
                    let rel_diff = (record.xpu - record.gpu) / record.gpu;
                    diff.push(LossDiff {
                        step: record.step,
                        abs: record.xpu - record.gpu,
                        rel: match rel_diff.is_nan() {
                            true => (record.xpu - record.gpu) / (record.gpu + 1e-8),
                            false => rel_diff,
                        },
                    });
                    loss.push(record);
                }
                let data_len = loss.len();
                set_feature(calculate_feature(&diff));
                set_start(0.0);
                set_end(data_len as f64);
                set_loss(loss.clone());
                set_diff(diff.clone());
                set_global_loss(loss);
                set_global_diff(diff);
                set_global_len(data_len as f64);
                set_chart_visibility("block");
            }) as Box<dyn FnMut(_)>);

            file_reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
            file_reader.read_as_text(&file).unwrap();
            // prevent the callback from being dropped
            onloadend.forget();
        }
    });
    let UseDropZoneReturn {
        is_over_drop_zone, ..
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default().on_drop(move |event| {
            let files = event.files;
            for file in files {
                update_data.dispatch(file);
            }
        }),
    );
    // change drag and drop text color
    create_effect(move |_| match is_over_drop_zone() {
        true => set_drag_color("#21a675"),
        false => set_drag_color("#e66956"),
    });

    let replot = move |_| {
        let start = usize::max(start.get_untracked() as usize, 0);
        let mut end = usize::min(
            end.get_untracked() as usize,
            global_len.get_untracked() as usize,
        );
        end = end.max(start);
        let current_loss = global_loss.get_untracked()[start..end].to_owned();
        let current_diff = global_diff.get_untracked()[start..end].to_owned();
        set_feature(calculate_feature(&current_diff));
        set_loss(current_loss);
        set_diff(current_diff);
    };

    view! {
        <PageTitle text="Draw Loss Curve"/>

        <div
            node_ref=drop_zone_el
            class="flex flex-col items-center p-4 border-2 border-dashed border-gray-300 rounded-lg mt-10 mb-5"
        >
            <p class="text-lg font-semibold" style=move || format!("color: {};", drag_color())>
                "Drag and Drop CSV Files"
            </p>
            <p class="text-sm font-thin" style=move || format!("color: {};", drag_color())>
                {file_name}
            </p>
        </div>

        <div
            class="container flex flex-col items-center mt-10 mb-20"
            style=move || format!("display: {};", chart_visibility())
        >
            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(3.0)>
                <FormControl class="flex flex-row">
                    <Label class="mr-2">"Start"</Label>
                    <NumberInput
                        min=0.0
                        max=global_len
                        step=1.0
                        get=start
                        set=set_start
                        class="h-10"
                    />
                </FormControl>

                <FormControl class="flex flex-row">
                    <Label class="mr-2">"End"</Label>
                    <NumberInput min=1.0 max=global_len step=1.0 get=end set=set_end class="h-10"/>
                </FormControl>

                <button
                    on:click=replot
                    class="hover:bg-cyan-600 rounded-md bg-red-400 text-white text-m font-medium pl-2 pr-3 py-2 h-10 shadow-sm ml-2"
                >
                    "RePlot"
                </button>
            </Stack>

            <div class="chart-theme mt-7">
                <Chart
                    debug=false
                    aspect_ratio=AspectRatio::from_env_width(400.0)
                    left=y_ticks.clone()
                    bottom=RotatedLabel::middle("Step")
                    // bottom=x_ticks.clone()
                    top=RotatedLabel::middle("Loss Curve")
                    right=Legend::end()

                    inner=[
                        XGridLine::from_ticks(x_ticks.clone()).into_inner(),
                        YGridLine::from_ticks(y_ticks.clone()).into_inner(),
                        AxisMarker::left_edge().into_inner(),
                        AxisMarker::bottom_edge().into_inner(),
                        YGuideLine::over_mouse().into_inner(),
                        XGuideLine::over_data().into_inner(),
                    ]

                    tooltip=Tooltip::left_cursor().show_x_ticks(true).skip_missing(true)
                    series=loss_series
                    data=loss
                />

                <hr class="border-t border-dotted border-gray-300 mt-5 mb-5 w-full"/>

                <Chart
                    debug=false
                    aspect_ratio=AspectRatio::from_env_width(400.0)
                    left=y_ticks.clone()
                    bottom=RotatedLabel::middle("Step")
                    // bottom=x_ticks.clone()
                    top=RotatedLabel::middle("Loss Diff Curve")
                    right=Legend::end()

                    inner=[
                        XGridLine::from_ticks(x_ticks.clone()).into_inner(),
                        YGridLine::from_ticks(y_ticks.clone()).into_inner(),
                        AxisMarker::left_edge().into_inner(),
                        AxisMarker::bottom_edge().into_inner(),
                        YGuideLine::over_mouse().into_inner(),
                        XGuideLine::over_data().into_inner(),
                    ]

                    tooltip=Tooltip::left_cursor().show_x_ticks(true).skip_missing(true)
                    series=diff_series
                    data=diff
                />
            </div>

            <div class="flex flex-row justify-center mt-5">
                <Grid
                    gap=Size::Em(0.5)
                    class="border border-red-300 border-dashed rounded-md p-4 w-7/12"
                >
                    <Row>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Max Diff: "
                                {move || {
                                    let f = feature();
                                    format!("{:.6} (step={})", f.max_diff.0, f.max_diff.1)
                                }}

                            </P>
                        </Col>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Min Diff: "
                                {move || {
                                    let f = feature();
                                    format!("{:.6} (step={})", f.min_diff.0, f.min_diff.1)
                                }}

                            </P>
                        </Col>
                    </Row>
                    <Row>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Max Abs Diff: "
                                {move || {
                                    let f = feature();
                                    format!("{:.6} (step={})", f.max_abs_diff.0, f.max_abs_diff.1)
                                }}

                            </P>
                        </Col>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Min Abs Diff: "
                                {move || {
                                    let f = feature();
                                    format!("{:.6} (step={})", f.min_abs_diff.0, f.min_abs_diff.1)
                                }}

                            </P>
                        </Col>
                    </Row>
                    <Row>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Max N-Diff: "
                                {move || {
                                    let f = feature();
                                    if f.max_n_diff.0 == f64::MIN {
                                        format!("null (step=null)")
                                    } else {
                                        format!("{:.6} (step={})", f.max_n_diff.0, f.max_n_diff.1)
                                    }
                                }}

                            </P>
                        </Col>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Min P-Diff: "
                                {move || {
                                    let f = feature();
                                    if f.min_p_diff.0 == f64::MAX {
                                        format!("null (step=null)")
                                    } else {
                                        format!("{:.6} (step={})", f.min_p_diff.0, f.min_p_diff.1)
                                    }
                                }}

                            </P>
                        </Col>
                    </Row>
                    <Row>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Mean Diff: " {move || { format!("{:.6}", feature().mean_diff) }}
                            </P>
                        </Col>
                        <Col xs=3>
                            <P class="text-cyan-700">
                                "Mean Abs Diff: "
                                {move || { format!("{:.6}", feature().mean_abs_diff) }}
                            </P>
                        </Col>
                    </Row>
                </Grid>
            </div>
        </div>
    }
}
