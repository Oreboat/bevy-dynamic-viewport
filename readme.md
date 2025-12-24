### Bevy Dynamic Viewport
## a simple Dynaimc viewport for the bevy engine

Bevy Dynamic Viewport adds a simple, easy to use Dynamic viewport, to use it you can either manually add the new `DynamicViewport` component or you can utilize the `Viewport Bundle` to add it and and a Camera component

it also provides two helper functions for creating a Dynamic Viewport component

`ScalableViewport::from_resolution(resolution: UVec2,mode: AspectRatioMode)`

or

`ScalableViewport::from_aspect_ratio(ratio: UVec2,mode: AspectRatioMode)`

there is also AspectRatioMode which controls how the scaling logic works

```rust
pub enum AspectRatioMode {
    Keep,
    KeepWidth,
    KeepHeight,
    Scale,
}
```
an example of creating a scaleable viewport is

```rust
fn create_viewport(mut commands: Commands, window: Single<&Window>){
        let window_size = window.resolution.physical_size().as_vec2();
        let viewport_ratio = UVec2{x: 21,y: 9};
        let viewport_size = ViewportPlugin::get_new_resolution(window.resolution.physical_size(), viewport_ratio, &AspectRatioMode::Keep);

        let viewport_position = ((window_size - viewport_size.as_vec2()) / 2.0).as_uvec2();
        commands.spawn(ViewportBundle{
            camera: Camera { 
                viewport: Some(Viewport{ 
                    physical_position: viewport_position, 
                    physical_size: viewport_size, 
                    ..default()
            }),
            clear_color: ClearColorConfig::Custom(Color::srgb(0.96, 0.76, 0.91)), 
            ..Default::default() 
            },
            scaleable_viewport: ScaleableViewport::from_ratio(viewport_ratio, AspectRatioMode::Keep),
        })
        .insert(Camera3d{..Default::default()});


}
```