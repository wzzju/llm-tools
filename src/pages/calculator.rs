use crate::components::*;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Model {
    Llama2_7B,
    Llama2_13B,
    Llama2_70B,
    Llama3_8B,
    Llama3_70B,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Llama2_7B => "Llama2 7B",
            Self::Llama2_13B => "Llama2 13B",
            Self::Llama2_70B => "Llama2 70B",
            Self::Llama3_8B => "Llama3 8B",
            Self::Llama3_70B => "Llama3 70B",
        };
        f.write_str(name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TrainMode {
    FP16,
    BF16,
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn CalculatorPage() -> impl IntoView {
    let toasts = expect_context::<Toasts>();

    let (sp, set_sp) = create_signal(true);
    let (zero_level, set_zero_level) = create_signal(1);

    let (model, set_model) = create_signal(Model::Llama3_70B);
    let (train_mode, set_train_mode) = create_signal(TrainMode::BF16);
    let (params, set_params) = create_signal(70.0);
    let (layers, set_layers) = create_signal(80.0);
    let (hidden_size, set_hidden_size) = create_signal(8192.0);
    let (mem_useage, set_mem_useage) = create_signal(Option::<f64>::None);

    let calculate = move |_| {
        toasts.push(Toast {
            id: Uuid::new_v4(),
            created_at: time::OffsetDateTime::now_utc(),
            variant: ToastVariant::Info,
            header: "Calculated!".to_owned().into_view(),
            body: format!(
                "Sequence Parallel: {}, Zero Level: {}, Model Type: {}, Layer Number: {}, Hidden Size: {}",
                sp.get_untracked(),
                zero_level.get_untracked(),
                model.get_untracked(),
                layers.get_untracked() as i64,
                hidden_size.get_untracked() as i64,
            )
            .into_view(),
            timeout: ToastTimeout::DefaultDelay,
        });
        set_mem_useage(Some(86.3689));
    };

    // Adjust model parameters when the model type changes.
    create_effect(move |_| match model() {
        Model::Llama2_7B => {
            set_train_mode(TrainMode::FP16);
            set_layers(32.0);
            set_params(7.0);
            set_hidden_size(4096.0);
            set_mem_useage(None);
        }
        Model::Llama2_13B => {
            set_train_mode(TrainMode::FP16);
            set_layers(40.0);
            set_params(13.0);
            set_hidden_size(5120.0);
            set_mem_useage(None);
        }
        Model::Llama2_70B => {
            set_train_mode(TrainMode::FP16);
            set_layers(80.0);
            set_params(70.0);
            set_hidden_size(8192.0);
            set_mem_useage(None);
        }
        Model::Llama3_8B => {
            set_train_mode(TrainMode::BF16);
            set_layers(32.0);
            set_params(8.0);
            set_hidden_size(4096.0);
            set_mem_useage(None);
        }
        Model::Llama3_70B => {
            set_train_mode(TrainMode::BF16);
            set_layers(80.0);
            set_params(70.0);
            set_hidden_size(8192.0);
            set_mem_useage(None);
        }
    });

    view! {
        <PageTitle text="Memory Usage Calculator"/>

        <Grid gap=Size::Em(0.5) class="mt-20">
            <Row>
                <Col xs=6 class="border border-gray-300 rounded-md p-2">
                    <div class="flex flex-col gap-2">
                        <FormControl class="flex flex-row">
                            <Label class="w-28 mr-1">"Model Type"</Label>
                            <Select
                                options=vec![
                                    Model::Llama2_7B,
                                    Model::Llama2_13B,
                                    Model::Llama2_70B,
                                    Model::Llama3_8B,
                                    Model::Llama3_70B,
                                ]

                                search_text_provider=move |option| format!("{option}")
                                render_option=move |option| format!("{option}")
                                selected=model
                                set_selected=set_model
                                class="w-36"
                            />
                        </FormControl>

                        <FormControl class="flex flex-row">
                            <Label class="w-28 mr-1">"Train Mode"</Label>
                            <Select
                                options=vec![TrainMode::FP16, TrainMode::BF16]

                                search_text_provider=move |option| format!("{:?}", option)
                                render_option=move |option| format!("{:?}", option)
                                selected=train_mode
                                set_selected=set_train_mode
                                class="w-36"
                            />
                        </FormControl>

                        <FormControl class="flex flex-row">
                            <Label class="w-28 mr-1">"Parameters"</Label>
                            <NumberInput
                                min=0.0
                                max=1000.0
                                step=0.001
                                get=params
                                set=set_params
                                class="w-36"
                            />
                            <Label class="ml-1">"B"</Label>
                        </FormControl>

                        <FormControl class="flex flex-row">
                            <Label class="w-28 mr-1">"Layer Number"</Label>
                            <NumberInput
                                min=1.0
                                max=1024.0
                                step=1.0
                                get=layers
                                set=set_layers
                                class="w-36"
                            />
                        </FormControl>

                        <FormControl class="flex flex-row">
                            <Label class="w-28 mr-1">"Hidden Size"</Label>
                            <NumberInput
                                min=1.0
                                max=102400.0
                                step=1.0
                                get=hidden_size
                                set=set_hidden_size
                                class="w-36"
                            />
                        </FormControl>
                    </div>
                </Col>

                <Col xs=6 class="border border-gray-300 rounded-md p-2">
                    <div class="flex flex-col gap-2">
                        <FormControl class="flex flex-row">
                            <Checkbox checked=sp set_checked=set_sp/>
                            <Label class="ml-1">"Sequence Parallel"</Label>
                        </FormControl>

                        <RadioGroup class="flex flex-row gap-4">
                            <FormControl class="flex flex-row">
                                <Radio
                                    checked=Signal::derive(move || { zero_level() == 1 })

                                    set_checked=move |checked| {
                                        if checked {
                                            set_zero_level.set(1)
                                        }
                                    }
                                />

                                <Label class="ml-1">"Zero-1"</Label>
                            </FormControl>
                            <FormControl class="flex flex-row">
                                <Radio
                                    checked=Signal::derive(move || { zero_level() == 2 })

                                    set_checked=move |checked| {
                                        if checked {
                                            set_zero_level.set(2)
                                        }
                                    }
                                />

                                <Label class="ml-1">"Zero-2"</Label>
                            </FormControl>
                            <FormControl class="flex flex-row">
                                <Radio
                                    checked=Signal::derive(move || { zero_level() == 3 })

                                    set_checked=move |checked| {
                                        if checked {
                                            set_zero_level.set(3)
                                        }
                                    }
                                />

                                <Label class="ml-1">"Zero-3"</Label>
                            </FormControl>
                        </RadioGroup>
                    </div>
                </Col>
            </Row>

            <Row>
                <Col xs=6 class="border border-gray-300 rounded-md p-2">
                    <div class="flex flex-col gap-2">
                        <P class="text-gray-500">
                            "Model Type: " {move || { format!("{}", model()) }} ", Train Mode: "
                            {move || { format!("{:?}", train_mode()) }}
                        </P>
                        <P class="text-gray-500">
                            "Parameters: " {move || params()} " B, Layer Number: "
                            {move || layers() as i64}
                        </P>
                        <P class="text-gray-500">"Hidden Size: " {move || hidden_size() as i64}</P>
                    </div>
                </Col>

                <Col xs=6 class="border border-gray-300 rounded-md p-2">
                    <div class="flex flex-col gap-2">
                        <P class="text-gray-500">"Sequence Parallel: " {sp}</P>
                        <P class="text-gray-500">"Zero Level: " {zero_level}</P>
                    </div>
                </Col>
            </Row>

            <Show when=move || { mem_useage() != None } fallback=|| ()>
                <Row>
                    <Col xs=6 class="border border-red-300 border-dashed rounded-md p-2">
                        <div class="flex flex-col gap-2">
                            <P class="text-red-400">
                                "Memory Usage: "
                                {move || { format!("{:.2} GiB", mem_useage().unwrap_or(0.0)) }}
                            </P>
                        </div>
                    </Col>
                </Row>
            </Show>
        </Grid>

        <div class="container mx-auto flex flex-row-reverse mt-10">
            <button
                on:click=calculate
                class="hover:bg-cyan-600 rounded-md bg-red-400 text-white text-m font-medium pl-2 pr-3 py-2 shadow-sm"
            >
                "Calculate"
            </button>
        </div>
    }
}
