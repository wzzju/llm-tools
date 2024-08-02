use leptos::*;
use leptos_meta::Title;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Welcome to LLM-Tools"/>

        <div class="container flex items-center mt-20 flex-col">
            <h1 class="text-4xl font-bold text-red-400 mb-4">"Welcome to LLM-Tools!"</h1>
            <div class="flex gap-2 items-center mt-2">
                <img src="/images/logo.png" id="pic" alt="picture" width="320" height="320"/>
            </div>
        </div>
    }
}
