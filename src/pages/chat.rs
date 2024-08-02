use crate::components::*;
use leptonic::components::prelude::*;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn ChatPage() -> impl IntoView {
    let toasts = expect_context::<Toasts>();

    view! {
        <PageTitle text="Chat With AI"/>

        <div class="container flex items-center mt-20 flex-col">
            <h1 class="text-4xl font-bold text-red-400 mb-4">"New Chat"</h1>

            <div class="flex gap-2 items-center mt-2">
                <button
                    on:click=move |_| {
                        toasts
                            .push(Toast {
                                id: Uuid::new_v4(),
                                created_at: time::OffsetDateTime::now_utc(),
                                variant: ToastVariant::Success,
                                header: "Sent!".to_owned().into_view(),
                                body: "Your response will be sent shortly!".to_owned().into_view(),
                                timeout: ToastTimeout::DefaultDelay,
                            });
                    }

                    class="hover:bg-cyan-600 rounded-md bg-red-400 text-white text-m font-medium pl-2 pr-3 py-2 shadow-sm"
                >
                    "Send"
                </button>
            </div>
        </div>
    }
}
