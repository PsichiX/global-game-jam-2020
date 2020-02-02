#![allow(clippy::type_complexity)]

use crate::components::ui_element::UiElement;
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct UiSystem;

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        ReadExpect<'s, WebCompositeRenderer>,
        ReadStorage<'s, UiElement>,
        WriteStorage<'s, CompositeRenderable>,
        ReadStorage<'s, CompositeCamera>,
        ReadStorage<'s, CompositeTransform>,
        ReadStorage<'s, Tag>,
    );

    fn run(
        &mut self,
        (renderer, ui_elements, mut renderables, cameras, transforms, tags): Self::SystemData,
    ) {
        let screen_size = renderer.view_size();

        for (ui_element, mut renderable) in (&ui_elements, &mut renderables).join() {
            if let Some(rect) = (&cameras, &tags, &transforms)
                .join()
                .find_map(|(c, tg, tr)| {
                    if ui_element.camera_tag == tg.0 {
                        if let Some(inv_mat) = !c.view_matrix(tr, screen_size) {
                            let size = screen_size * inv_mat;
                            Some(Rect {
                                x: 0.0,
                                y: 0.0,
                                w: size.x,
                                h: size.y,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            {
                let commands = ui_element.build_commands(rect);
                renderable.0 = Renderable::Commands(commands);
            }
        }
    }
}
