use kiss3d::prelude::*;

use crate::planet::{Planet, PlanetAppearance};

mod planet;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Solar system").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();


    let sun_light = Light::directional(Vec3::new(-1.0, -1.0, 0.0))
    .with_color(Color::new(1.0, 0.95, 0.8, 1.0))
    .with_intensity(2.0);
    scene.add_light(sun_light);

    let mut sun = scene.add_sphere(0.15);
    sun.set_color(YELLOW);

    let mut earth = Planet::new(
        &mut scene,
        0.05,
        0.3,
        PlanetAppearance::Texture { 
            path: "textures/earth.jpg".to_string(), 
            name: "earth".to_string() 
        },
        0.02,
        0.01);
    
    let mut mars = Planet::new(
        &mut scene, 
        0.8, 
        5.0, 
        PlanetAppearance::Color(0.8, 0.3, 0.2),
        0.008,
        0.09
    );

    while window.render_3d(&mut scene, &mut camera).await {

        earth.update();
        mars.update();

    }
}