use crate::app::user_actions::UserActions;
use crate::pages::games::GamePage;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::{*, Switch, Redirect};

mod about;
mod state;
mod user_actions;

pub use state::AppState;
use crate::components::NavLinkItem;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Index,
    #[at("/game/:game")]
    Game { game: String },
}

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <BrowserRouter>
                    <Switch<AppRoute> render={switch_app_route} />
                </BrowserRouter>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Index => html! { <Redirect<AppRoute> to={AppRoute::Game { game: "Factorio".to_owned() }} /> },
        AppRoute::Game { game } => html! {<AppPage><GamePage key={game.clone()} game={game.clone()} /></AppPage>},
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
    let games = [
        "Factorio",
        "Generic"
    ];
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavExpandable title="Games">
                        {for games.iter().map(|&game| html_nested! {
                            <NavLinkItem<AppRoute> to={AppRoute::Game { game: game.to_owned() }}>
                                {&game}
                            </NavLinkItem<AppRoute>>
                        })}
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    let brand = html! (
        <MastheadBrand>
            // <Brand src="assets/images/pf-logo.svg" alt="Patternfly Logo" style="--pf-v5-c-brand--Height: 36px;"/>
            {"Homelab Servers"}
        </MastheadBrand>
    );

    let backdropper = use_backdrop();

    let onabout = use_callback((), move |_, ()| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(html!(<about::About/>));
        }
    });

    // track dark mode state
    let darkmode = use_state_eq(|| {
        gloo_utils::window()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map(|m| m.matches())
            .unwrap_or_default()
    });

    // apply dark mode
    use_effect_with(*darkmode, |state| match state {
        true => gloo_utils::document_element().set_class_name("pf-v5-theme-dark"),
        false => gloo_utils::document_element().set_class_name(""),
    });

    // toggle dark mode
    let onthemeswitch = use_callback(darkmode.setter(), |state, setter| setter.set(state));

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <patternfly_yew::prelude::Switch checked={*darkmode} onchange={onthemeswitch} label="Dark Theme" />
                    </ToolbarItem>
                    <ToolbarItem>
                        <a href="https://github.com/scottbot95/homelab-server-manager" target="_blank">
                            <Button variant={ButtonVariant::Plain} icon={Icon::Github}/>
                        </a>
                    </ToolbarItem>
                    <ToolbarItem>
                        <Dropdown
                            position={Position::Right}
                            icon={Icon::QuestionCircle}
                            variant={MenuToggleVariant::Plain}
                        >
                            <MenuAction onclick={onabout}>{"About"}</MenuAction>
                        </Dropdown>
                    </ToolbarItem>
                    <ToolbarItem>
                        <UserActions />
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! (
        <Page {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </Page>
    )
}
