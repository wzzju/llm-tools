use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use leptos_router::*;

use crate::app::{AppLayoutContext, AppRoutes, APP_BAR_HEIGHT};

#[component]
#[allow(clippy::too_many_lines)]
pub fn SideLayout() -> impl IntoView {
    let app_layout_ctx = expect_context::<AppLayoutContext>();
    let AppLayoutContext {
        is_small,
        is_medium,
        side_drawer_closed,
        ..
    } = app_layout_ctx;

    let drawer_class = move || match is_small() {
        true => "mobile",
        false => "",
    };

    let close_side_drawer_on_mobile = move || {
        if is_small.get_untracked() {
            app_layout_ctx.close_side_drawer();
        }
    };

    let drawer_content = view! {
        <DrawerSection
            level=1
            header=move || {
                view! {
                    <Icon icon=icondata::BsTools margin=Margin::Right(Size::Em(1.0))/>
                    "Tools"
                }
            }
        >

            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link
                    href=AppRoutes::Draw
                    class="item"
                    on:click=move |_| close_side_drawer_on_mobile()
                >
                    "Loss Plotter"
                </Link>
                <Link
                    href=AppRoutes::Calculator
                    class="item"
                    on:click=move |_| close_side_drawer_on_mobile()
                >
                    "Memory Calculator"
                </Link>
            </Stack>
        </DrawerSection>

        <DrawerSection
            level=1
            header=move || {
                view! {
                    <Icon icon=icondata::BsChatSquare margin=Margin::Right(Size::Em(1.0))/>
                    "Conversations"
                }
            }
        >

            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link
                    href=AppRoutes::Chat
                    class="item"
                    on:click=move |_| close_side_drawer_on_mobile()
                >
                    "New Chat"
                </Link>
            </Stack>
        </DrawerSection>
    };

    view! {
        <Box
            id="side-layout"
            style=move || {
                format!(
                    "margin-left: {}em; margin-right: {}em;",
                    match side_drawer_closed() {
                        true => 0,
                        false => 16,
                    },
                    match is_medium() {
                        true => 0,
                        false => 12,
                    },
                )
            }
        >

            <Drawer
                side=DrawerSide::Left
                id="side-drawer"
                shown=Signal::derive(move || !side_drawer_closed())
                class=drawer_class
                style=format!("position: fixed; left: 0; top: {APP_BAR_HEIGHT}; bottom: 0;")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="menu">
                    {drawer_content}
                </Stack>
            </Drawer>

            // <Outlet/> will show nested child routes.
            <Outlet/>
        </Box>
    }
}

#[component]
pub fn DrawerSection<H, IV>(level: u32, header: H, children: Children) -> impl IntoView
where
    H: Fn() -> IV + 'static,
    IV: IntoView + 'static,
{
    view! {
        <div class="drawer-section" data-level=level>
            <div class="section-header">{header()}</div>
            {children()}
        </div>
    }
}
