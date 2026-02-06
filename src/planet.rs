use kiss3d::prelude::*;
use std::path::Path;

pub struct Planet {
    orbit_pivot: SceneNode3d,  // pivot around sun
    body: SceneNode3d,         // the planet
    orbit_speed: f32,          // speed around sun
    rotation_speed: f32,       // speed rotation on itself
}

pub enum PlanetAppearance {
    Texture { path: String, name: String },
    Color(f32, f32, f32),
}

impl Planet {
    pub fn new(
        scene: &mut SceneNode3d,    
        radius: f32,                    // size of the planet
        distance: f32,                  // distance to the sun
        appearance : PlanetAppearance,  // texture or rgb color
        orbit_speed: f32,               // year speed
        rotation_speed: f32             // day speed
    ) -> Self {
        
        let mut pivot = scene.add_group();
        
        let mut location = pivot.add_group();
        location.translate(Vec3::new(distance, 0.0, 0.0));

        let mut sphere = location.add_sphere(radius);
        
        match appearance {
            PlanetAppearance::Texture { path, name } => {
                sphere.set_texture_from_file(Path::new(&path), &name);
            },
            PlanetAppearance::Color(r, g, b) => {
                sphere.set_color(Color::new(r, g, b, 1.0));
            }
        }

        Planet {
            orbit_pivot: pivot,
            body: sphere,
            orbit_speed,
            rotation_speed,
        }
    }

    /// update planet position 
    pub fn update(&mut self) {
        // turn around sun
        self.orbit_pivot.prepend_rotation(Quat::from_axis_angle(
            Vec3::Y, 
            self.orbit_speed
        ));

        // turn on itself
        self.body.prepend_rotation(Quat::from_axis_angle(
            Vec3::Y, 
            self.rotation_speed
        ));
    }
}