use crate::layouts::*;
use crate::pages::*;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use leptos_meta::{provide_meta_context, Link as MetaLink, Meta, Stylesheet, Title};
use leptos_router::*;
use leptos_use::use_media_query;

pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);
pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

#[derive(Debug, Copy, Clone)]
pub enum AppRoutes {
    Home,
    Draw,
    Calculator,
    Chat,
}

impl AppRoutes {
    pub const fn route(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Draw => "/draw",
            Self::Calculator => "/calculator",
            Self::Chat => "/chat",
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl std::fmt::Display for AppRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for AppRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || self.route().to_string())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_small: Signal<bool>,
    pub is_medium: Signal<bool>,
    pub main_drawer_closed: ReadSignal<bool>,
    pub set_main_drawer_closed: WriteSignal<bool>,
    pub side_drawer_closed: ReadSignal<bool>,
    pub set_side_drawer_closed: WriteSignal<bool>,
}

impl AppLayoutContext {
    #[allow(unused)]
    pub fn close_main_drawer(&self) {
        (self.set_main_drawer_closed)(true);
    }

    pub fn close_side_drawer(&self) {
        (self.set_side_drawer_closed)(true);
    }

    pub fn toggle_main_drawer(&self) {
        let currently_closed = self.main_drawer_closed.get_untracked();
        (self.set_main_drawer_closed)(!currently_closed);
        if currently_closed {
            self.close_side_drawer();
        }
    }

    pub fn toggle_side_drawer(&self) {
        let currently_closed = self.side_drawer_closed.get_untracked();
        (self.set_side_drawer_closed)(!currently_closed);
        if currently_closed {
            self.close_main_drawer();
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let is_small = use_media_query("(max-width: 800px)");
    let is_medium = use_media_query("(max-width: 1200px)");
    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = create_signal(true);
    let (side_drawer_closed, set_side_drawer_closed) = create_signal(false);

    let app_layout_ctx = AppLayoutContext {
        is_small,
        is_medium,
        main_drawer_closed: main_drawer_closed,
        set_main_drawer_closed,
        side_drawer_closed: side_drawer_closed,
        set_side_drawer_closed,
    };
    provide_context(app_layout_ctx);

    view! {
        <Meta name="description" content="llm-tools"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css")/>

        <MetaLink rel="icon" href="/images/favicon.ico"/>
        <MetaLink rel="apple-touch-icon" href="/images/logo.png"/>

        <Title text="Welcome to llm-tools"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router
                trailing_slash=TrailingSlash::Redirect
                fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorPage outside_errors/> }.into_view()
                }
            >

                <MainLayout>
                    <Routes>
                        <Route path="/" view=|| view! { <SideLayout/> }>
                            <Route path=AppRoutes::Home view=HomePage/>
                            <Route path=AppRoutes::Draw view=DrawPage/>
                            <Route path=AppRoutes::Calculator view=CalculatorPage/>
                            <Route path=AppRoutes::Chat view=ChatPage/>
                        </Route>
                    </Routes>
                </MainLayout>
            </Router>
        </Root>
    }
}
