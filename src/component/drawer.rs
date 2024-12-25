use leptos::prelude::*;
use leptos::{view, IntoView};
use thaw::*;

#[component]
pub fn AppSetting(open_settings: RwSignal<bool>) -> impl IntoView {
    view! {
        <OverlayDrawer open=open_settings position=DrawerPosition::Left>
            <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            icon=icondata::CgClose
                            appearance=ButtonAppearance::Subtle
                            on_click=move |_| open_settings.set(false)
                        />
                    </DrawerHeaderTitleAction>
                "App Settings"
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>
                <Flex vertical=true>
                    <p><b>"Language"</b></p>
                    <Select>
                        <option>"English"</option>
                        <option>"日本語"</option>
                        <option>"简体中文"</option>
                    </Select>
                </Flex>
                <Flex vertical=true>
                    <p><b>"Direct Generation"</b></p>
                    <Switch/>
                    <p>
                        <b>"Note: "</b>
                        "Turn on Direct Generation will generate markdown at selected output path "
                        <b>"instantly "</b>
                        "when worksheet with name matching today's date has been found."
                    </p>
                </Flex>
                <Flex justify=FlexJustify::End>
                    <Button>"Save"</Button>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}
