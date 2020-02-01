#[macro_use]
extern crate oxygengine;

use crate::{
    assets::tiled_map_asset_protocol::TiledMapAssetProtocol,
    components::{
        airplane::Airplane, city::City, infection_rate::InfectionRate, letter::Letter,
        MainCameraTag, MenuTrackSelectedTag,
    },
    resources::wave::Wave,
    states::loading::LoadingState,
    // systems::keyboard_movement::KeyboardMovementSystem,
    systems::{
        airplane_land::AirplaneLandSystem, airplane_move::AirplaneMoveSystem,
        airplane_return::AirplaneReturnSystem, view::ViewSystem, wave::WaveSystem,
    },
};
use oxygengine::prelude::*;
use wasm_bindgen::prelude::*;

mod assets;
mod components;
mod resources;
mod states;
mod systems;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // initialize logger to see logs in web browser (debug only).
    #[cfg(debug_assertions)]
    logger_setup(WebLogger);

    // Application build phase - install all systems and resources and setup them.
    let app = App::build()
        // install core module assets managment.
        .with_bundle(
            oxygengine::core::assets::bundle_installer,
            (WebFetchEngine::default(), |assets| {
                // register assets loading error reporter that shows errors in console.
                #[cfg(debug_assertions)]
                assets.register_error_reporter(LoggerAssetsDatabaseErrorReporter);
                // register assets protocols from composite renderer module.
                oxygengine::composite_renderer::protocols_installer(assets);
                // register assets protocols from audio module.
                oxygengine::audio::protocols_installer(assets);
                assets.register(TiledMapAssetProtocol);
            }),
        )
        // install core module prefabs management.
        .with_bundle(oxygengine::core::prefab::bundle_installer, |prefabs| {
            // install composite renderer prefabs.
            oxygengine::composite_renderer::prefabs_installer(prefabs);
            // install audio prefabs.
            oxygengine::audio::prefabs_installer(prefabs);
            // register game prefabs component factories.
            prefabs.register_component_factory::<Airplane>("Airplane");
            prefabs.register_component_factory::<City>("City");
            prefabs.register_component_factory::<InfectionRate>("InfectionRate");
            prefabs.register_component_factory::<MainCameraTag>("MainCameraTag");
            prefabs.register_component_factory::<MenuTrackSelectedTag>("MenuTrackSelectedTag");
            prefabs.register_component_factory::<Letter>("Letter");
        })
        // install input managment.
        .with_bundle(oxygengine::input::bundle_installer, |input| {
            // register input devices.
            input.register(WebKeyboardInputDevice::new(get_event_target_document()));
            input.register(WebMouseInputDevice::new(get_event_target_by_id("screen")));
            // map input axes and triggers to devices.
            for c in b'a'..=b'z' {
                let c = c as char;
                let id = format!("key-{}", c);
                let mapping = format!("Key{}", c.to_uppercase());
                input.map_trigger(&id, "keyboard", &mapping);
            }
            input.map_axis("mouse-x", "mouse", "x");
            input.map_axis("mouse-y", "mouse", "y");
            input.map_trigger("mouse-left", "mouse", "left");
        })
        // install composite renderer.
        .with_bundle(
            oxygengine::composite_renderer::bundle_installer,
            WebCompositeRenderer::with_state(
                get_canvas_by_id("screen"), // canvas target.
                RenderState::new(Some(Color::black())),
            ),
        )
        // install audio support.
        .with_bundle(oxygengine::audio::bundle_installer, WebAudio::default())
        // .with_system(KeyboardMovementSystem, "keyboard_movement", &[])
        .with_component::<Airplane>()
        .with_component::<City>()
        .with_component::<InfectionRate>()
        .with_resource(Wave::new(20, 1.0))
        .with_system(WaveSystem::default(), "wave", &[])
        .with_system(AirplaneReturnSystem::default(), "airplane_return", &[])
        .with_system(AirplaneLandSystem::default(), "airplane_land", &[])
        .with_system(AirplaneMoveSystem::default(), "airplane_move", &[])
        .with_system(ViewSystem::default(), "view", &[])
        .build(LoadingState::default(), WebAppTimer::default());

    // Application run phase - spawn runner that ticks our app.
    AppRunner::new(app).run(WebAppRunner)?;

    Ok(())
}
