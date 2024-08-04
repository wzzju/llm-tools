use crate::components::*;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use uuid::Uuid;

use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[server(input = MultipartFormData)]
pub async fn save_files(data: MultipartData) -> Result<usize, ServerFnError> {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let mut data = data.into_inner().unwrap();

    let current_dir =
        std::env::var("LEPTOS_SITE_ROOT").map_err(|e| ServerFnError::new(e.to_string()))?;
    let upload_dir = Path::new(&current_dir).join("static/upload");

    if !upload_dir.exists() {
        std::fs::create_dir_all(upload_dir.clone())
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    let mut num_files = 0usize;
    while let Ok(Some(mut field)) = data.next_field().await {
        let file_name = field.file_name().unwrap_or_default();
        let file_path = upload_dir.join(&file_name);
        let mut file = File::create(&file_path).map_err(|e| ServerFnError::new(e.to_string()))?;

        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk)
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        num_files += 1;
    }
    tracing::info!("Save {num_files} file(s).");

    Ok(num_files)
}

#[component]
pub fn ChatPage() -> impl IntoView {
    let toasts = expect_context::<Toasts>();

    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        // `MultipartData` implements `From<FormData>`
        save_files(data.into())
    });

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();
        upload_action.dispatch(form_data);
    };

    create_effect(move |_| {
        if let Some(save_files_res) = upload_action.value().get() {
            match save_files_res {
                Ok(num_files) => {
                    let body_str = if num_files > 1 {
                        format!("{} files are uploaded.", num_files)
                    } else {
                        format!("{} file is uploaded.", num_files)
                    };
                    toasts.push(Toast {
                        id: Uuid::new_v4(),
                        created_at: time::OffsetDateTime::now_utc(),
                        variant: ToastVariant::Success,
                        header: "Upload Successfully!".to_owned().into_view(),
                        body: body_str.into_view(),
                        timeout: ToastTimeout::CustomDelay(time::Duration::seconds(5)),
                    });
                }
                Err(err) => {
                    toasts.push(Toast {
                        id: Uuid::new_v4(),
                        created_at: time::OffsetDateTime::now_utc(),
                        variant: ToastVariant::Error,
                        header: "Failed to upload!".to_owned().into_view(),
                        body: err.to_string().into_view(),
                        timeout: ToastTimeout::CustomDelay(time::Duration::seconds(5)),
                    });
                }
            }
        }
    });

    view! {
        <PageTitle text="Chat With AI"/>

        <Stack orientation=StackOrientation::Vertical spacing=Size::Em(3.0) class="mt-20">
            <form
                on:submit=on_submit
                class="w-full flex flex-row justify-center space-x-4 p-4 border-2 border-dashed border-gray-300 rounded-lg"
            >
                <label for="open-files" class="cursor-pointer">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="40"
                        height="40"
                        viewBox="0 0 40 40"
                        fill="none"
                    >
                        <g id="Folder Open">
                            <path
                                id="icon"
                                d="M5 28.3333V14.8271C5 10.2811 5 8.00803 6.36977 6.56177C6.43202 6.49604 6.49604 6.43202 6.56177 6.36977C8.00803 5 10.2811 5 14.8271 5H15.3287C16.5197 5 17.1151 5 17.6492 5.18666C17.9753 5.30065 18.2818 5.46465 18.5575 5.67278C19.0091 6.0136 19.3394 6.50907 20 7.5C20.6606 8.49093 20.9909 8.9864 21.4425 9.32722C21.7182 9.53535 22.0247 9.69935 22.3508 9.81334C22.8849 10 23.4803 10 24.6713 10H28.3333C31.476 10 33.0474 10 34.0237 10.9763C35 11.9526 35 13.524 35 16.6667V17.5M16.2709 35H25.8093C28.2565 35 29.4801 35 30.3757 34.3164C31.2714 33.6328 31.5942 32.4526 32.2398 30.0921L32.6956 28.4254C33.7538 24.5564 34.2829 22.622 33.2823 21.311C32.2817 20 30.2762 20 26.2651 20H16.7339C14.2961 20 13.0773 20 12.1832 20.6796C11.2891 21.3591 10.9629 22.5336 10.3105 24.8824L9.84749 26.549C8.76999 30.428 8.23125 32.3675 9.23171 33.6838C10.2322 35 12.2451 35 16.2709 35Z"
                                stroke="#e66956"
                                stroke-width="1.6"
                                stroke-linecap="round"
                            ></path>
                        </g>
                    </svg>

                    <input id="open-files" name="open_files" type="file" multiple class="hidden"/>
                </label>

                <button
                    type="submit"
                    class="hover:bg-cyan-600 rounded-md bg-red-400 text-white text-m font-medium pl-2 pr-3 py-2 shadow-sm"
                >
                    "Upload"
                </button>
            </form>
        </Stack>
    }
}
