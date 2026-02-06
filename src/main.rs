use kiss3d::prelude::*;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Kiss3d: cube").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();


    let sun_light = Light::directional(Vec3::new(-1.0, -1.0, 0.0))
    .with_color(Color::new(1.0, 0.95, 0.8, 1.0))
    .with_intensity(2.0);
    scene.add_light(sun_light);

    let mut sun = scene.add_sphere(0.15);
    sun.set_color(YELLOW);

    let mut earth_pivot = scene.add_group();

    let mut earth = earth_pivot.add_sphere(0.05);
    earth.set_color(BLUE);

    earth.translate(Vec3 { x: 0.5, y: 0.0, z: 0.0 });

    while window.render_3d(&mut scene, &mut camera).await {

        earth_pivot.prepend_rotation(Quat::from_axis_angle(
            Vec3::Y, 
            0.01
        ));

    }
}