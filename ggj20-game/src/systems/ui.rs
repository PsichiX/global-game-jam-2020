#![allow(clippy::type_complexity)]

use crate::components::ui_element::UiElement;
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct UiSystem {
    last_screen_size: Vec2,
}

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        ReadExpect<'s, WebCompositeRenderer>,
        WriteStorage<'s, UiElement>,
        WriteStorage<'s, CompositeRenderable>,
        ReadStorage<'s, CompositeCamera>,
        ReadStorage<'s, CompositeTransform>,
        ReadStorage<'s, Name>,
    );

    fn run(
        &mut self,
        (renderer, mut ui_elements, mut renderables, cameras, transforms, names): Self::SystemData,
    ) {
        let screen_size = renderer.view_size();
        let force_update = (self.last_screen_size - screen_size).sqr_magnitude() > 1.0e-4;
        self.last_screen_size = screen_size;

        for (mut ui_element, mut renderable) in (&mut ui_elements, &mut renderables).join() {
            if ui_element.dirty || force_update {
                if let Some(rect) = (&cameras, &names, &transforms)
                    .join()
                    .find_map(|(c, n, t)| {
                        if ui_element.camera_name == n.0 {
                            if let Some(inv_mat) = !c.view_matrix(t, screen_size) {
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
                    ui_element.dirty = false;
                }
            }
        }
    }
}
