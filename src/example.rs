use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_default]
    pub experimental: bool,
    #[prop_or_default]
    pub subtitle: Children,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ExamplePage)]
pub fn example_page(props: &Props) -> Html {
    html! (
        <PageSectionGroup>
            <PageSection
                r#type={PageSectionType::Default}
                variant={PageSectionVariant::Light}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >
                <Content>
                    <Title size={Size::XXXXLarge}>
                        { props.title.clone() }
                    </Title>
                    { for props.subtitle.iter() }
                </Content>
            </PageSection>

            if props.experimental {
                <PageSection>
                    <Alert inline=true title="Experimental feature" r#type={AlertType::Info}>
                        { Html::from_html_unchecked(r#"
<p>
This functionality is considered experimental. This means that the whole idea of it might be
changed or removed in future versions. It will also will not be considered for semantic versioning.
</p>
<p>
In order to prevent people from accidentially using such features, they are gated using Rust's
features and are not part of the <code>default</code> feature. In order to enable all experimental
features, enable the <code>experimental</code> feature. Individual experimental components can
be enabled using individual features. See the Rust docmentation for more details on which features
exist.
</p>"#.into()) }
                    </Alert>
                </PageSection>
            }

            { for props.children.iter().map(|child|{
                html!(<PageSection>{child}</PageSection>)
            })}
        </PageSectionGroup>
    )
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ExampleProps {
    pub title: String,
    pub children: Children,
    pub code: String,
}
