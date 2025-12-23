use patternfly_yew::prelude::{use_expandable, use_random_id};
use yew::prelude::*;
use yew_router::hooks::use_route;
use yew_router::prelude::Link;
use yew_router::Routable;

#[derive(Clone, PartialEq, Properties)]
pub struct NavLinkItemProps<R: Routable> {
    #[prop_or_default]
    pub children: Html,
    pub to: R,
}

/// Navigation link component for routing within the application.
///
/// Replacement for [patternfly_yew::prelude::NavRouterItem] that works with yew-router not yew-nested-router
#[function_component(NavLinkItem)]
pub fn nav_link_item<R: Routable + 'static>(props: &NavLinkItemProps<R>) -> Html {
    let route = use_route::<R>().expect("Requires a Router");

    let mut classes = Classes::from("pf-v5-c-nav__link");

    let active = route == props.to;

    let id = use_random_id();
    let expandable = use_expandable();
    use_effect_with(active, move |_| {
        if let Some(expandable) = expandable {
            expandable.state(*id, active)
        }
    });

    if active {
        classes.push("pf-m-current");
    }

    html! {
        <li class="pf-v5-c-nav__item">
            <Link<R> to={props.to.clone()} classes={classes}>
                { props.children.clone() }
            </Link<R>>
        </li>
    }
}
