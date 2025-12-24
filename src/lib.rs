

use bevy::{camera::Viewport, prelude::*, window::WindowResized};

pub enum AspectRatioMode {
    Keep,
    KeepWidth,
    KeepHeight,
    Scale,
}



#[derive(Bundle)]
pub struct ViewportBundle{
    pub camera: Camera,
    pub scaleable_viewport: ScaleableViewport,
}

#[derive(Component)]
#[require(Camera)]
pub struct ScaleableViewport{
    pub aspect_ratio_mode: AspectRatioMode,
    pub aspect_ratio: UVec2,
}




impl ScaleableViewport {
    pub fn from_resolution(
        resolution: UVec2,
        mode: AspectRatioMode,
    ) -> Self {
        fn gcd(mut a: u32, mut b: u32) -> u32 {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let divisor = gcd(resolution.x, resolution.y);
        let aspect = UVec2::new(
            resolution.x / divisor,
            resolution.y / divisor,
        );

        Self {
            aspect_ratio: aspect,
            aspect_ratio_mode: mode,
        }
    }

    pub fn from_ratio(
        ratio: UVec2,
        mode: AspectRatioMode,
    ) -> Self {

        Self {
            aspect_ratio: ratio,
            aspect_ratio_mode: mode,
        }
    }
}



pub struct ViewportPlugin;

impl ViewportPlugin{

    fn resize_viewport(
        windows: Query<&Window>,
        mut window_resized_reader: MessageReader<WindowResized>,
        mut query: Query<(&mut Camera, &ScaleableViewport)>
    ){
        for window_resized in window_resized_reader.read(){
            let window = windows.get(window_resized.window).unwrap();

            let window_size = window.resolution.physical_size().as_vec2();

            

            for (mut camera, viewport) in &mut query{


                let viewport_size = ViewportPlugin::get_new_resolution(window.resolution.physical_size(), viewport.aspect_ratio, &viewport.aspect_ratio_mode);
                let viewport_position = ((window_size - viewport_size.as_vec2()) / 2.0).as_uvec2();

                camera.viewport = Some(Viewport{
                    physical_position : viewport_position,
                    physical_size : viewport_size,
                    ..default()
                });
            }
        }
    }
    pub fn get_new_resolution(
        window: UVec2,
        aspect: UVec2,
        aspect_mode: &AspectRatioMode,
    ) -> UVec2 {
        let window_aspect = window.x as f32 / window.y as f32;
        let target_aspect = aspect.x as f32 / aspect.y as f32;

        match aspect_mode {
            AspectRatioMode::Keep => {
                if window_aspect > target_aspect {
                    // Window too wide → pillarbox
                    let height = window.y;
                    let width = (height as f32 * target_aspect).round() as u32;
                    UVec2::new(width, height)
                } else {
                    // Window too tall → letterbox
                    let width = window.x;
                    let height = (width as f32 / target_aspect).round() as u32;
                    UVec2::new(width, height)
                }
            }

            AspectRatioMode::KeepWidth => {
                let width = window.x;
                let height = (width as f32 / target_aspect).round() as u32;
                UVec2::new(width, height)
            }

            AspectRatioMode::KeepHeight => {
                let height = window.y;
                let width = (height as f32 * target_aspect).round() as u32;
                UVec2::new(width, height)
            }

            AspectRatioMode::Scale => window,
        }
    }


    }




impl Plugin for ViewportPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, ViewportPlugin::resize_viewport)
        ;
    }
}