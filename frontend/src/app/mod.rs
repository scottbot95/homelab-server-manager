use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;
use crate::app::user_actions::UserActions;
use crate::pages::games::Factorio;

mod about;
mod user_actions;
mod state;

pub use state::{AppState, AppAction};

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
}

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <RouterSwitch<AppRoute> render={switch_app_route} />
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Index => html! {<AppPage><Factorio/></AppPage>},
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavExpandable title="Games">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>{"Factorio"}</NavRouterItem<AppRoute>>
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
