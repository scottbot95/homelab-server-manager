use crate::counter::*;
use crate::hook::use_open;
use crate::index::*;
use crate::{icons::Icons, panic::Panic};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;

mod about;


#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum Form {
    #[default]
    #[target(index)]
    Index,
    Checkbox,
    Radio,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum Menu {
    #[default]
    #[target(index)]
    Index,
    Select,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum Date {
    #[default]
    #[target(index)]
    Calendar,
    DatePicker,
}

#[derive(Debug, Clone, PartialEq, Eq, Target)]
pub enum FullPage {
    Login,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    Counter,
    #[default]
    Index,
    Icons,
    Panic,
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
        AppRoute::Counter => html! {<AppPage><Counter/></AppPage>},
        AppRoute::Index => html! {<AppPage><Index/></AppPage>},
        AppRoute::Icons => html! {<AppPage><Icons/></AppPage>},
        AppRoute::Panic => html! {<AppPage><Panic/></AppPage>},
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
                    <NavExpandable title="Basics">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Counter}>{"Counter"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Icons}>{"Icons"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Panic}>{"Panic"}</NavRouterItem<AppRoute>>
                        <NavLink href="https://github.com/patternfly-yew/patternfly-yew" target="_blank">{"PatternFly Yew "} {Icon::ExternalLinkAlt.with_classes(classes!("pf-v5-u-ml-sm", "pf-v5-u-color-200"))}</NavLink>
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    let brand = html! (
        <MastheadBrand>
            <Brand src="assets/images/pf-logo.svg" alt="Patternfly Logo" style="--pf-v5-c-brand--Height: 36px;"/>
        </MastheadBrand>
    );

    let callback_github = use_open(
        "https://github.com/patternfly-yew/patternfly-yew-quickstart",
        "_blank",
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
                        <Button variant={ButtonVariant::Plain} icon={Icon::Github} onclick={callback_github}/>
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
