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

#[component]
pub fn DrawPage() -> impl IntoView {
    // draw loss curve
    let (loss, set_loss) = create_signal(vec![]);
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
    let (diff, set_diff) = create_signal(Vec::<LossDiff>::new());
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
        let file = file.clone();
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
                set_loss(loss);
                set_diff(diff);
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

    view! {
        <PageTitle text="Draw Loss Curve"/>

        <div class="container items-center flex flex-col mt-10">
            <div
                node_ref=drop_zone_el
                class="p-4 border-2 border-dashed border-gray-300 rounded-lg"
            >
                <p class="text-lg font-semibold" style=move || format!("color: {};", drag_color())>
                    "Drag and Drop CSV Files"
                </p>
            </div>
        </div>

        <div
            class="container items-center mt-10 mb-10 chart-theme"
            style=move || format!("display: {};", chart_visibility())
        >
            <Stack spacing=Size::Em(2.0)>
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

                <hr class="border-t border-dotted border-gray-300" style="width: 100%;"/>

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
            </Stack>
        </div>
    }
}
