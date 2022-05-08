use crate::bxdf::Lambert;
use crate::core::{IntersectInfoGlobal, IntersectableGlobal, Ray, ShadingInfo};
use crate::intersector::Intersector;
use crate::shape::Shape;
use crate::vec3::{build_orthonormal_basis, Vec3};

use std::sync::Arc;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub emission: Vec3,
}

impl Material {
    pub fn new(diffuse: Vec3, specular: Vec3, emission: Vec3) -> Self {
        Material {
            diffuse,
            specular,
            emission,
        }
    }
}

// TODO: make intersector selectable
pub struct Scene {
    _primitives: Arc<Vec<Shape>>,
    materials: Vec<Material>,
    intersector: Intersector,
}

impl Scene {
    pub fn new(primitives: Vec<Shape>, materials: Vec<Material>) -> Self {
        if primitives.len() != materials.len() {
            panic!("number of primitives does not equal to the number of materials.");
        }

        let primitives = Arc::new(primitives);
        Scene {
            _primitives: primitives.clone(),
            materials,
            intersector: Intersector::new(primitives),
        }
    }

    pub fn has_emission(&self, prim_idx: u32) -> bool {
        let emission = self.materials[prim_idx as usize].emission;
        emission.x() > 0.0 && emission.y() > 0.0 && emission.z() > 0.0
    }

    pub fn get_emission(&self, prim_idx: u32) -> Vec3 {
        let material = &self.materials[prim_idx as usize];
        material.emission
    }

    pub fn get_shading_info(&self, wo_global: Vec3, info: &IntersectInfoGlobal) -> ShadingInfo {
        let (t, n, b) = build_orthonormal_basis(info.normal);
        ShadingInfo {
            x: info.pos,
            n,
            wo: wo_global.world_to_local(t, n, b),
            t,
            b,
        }
    }

    pub fn get_bxdf(&self, prim_idx: u32) -> Lambert {
        let material = &self.materials[prim_idx as usize];
        Lambert::new(material.diffuse)
    }
}

impl IntersectableGlobal for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoGlobal> {
        self.intersector.intersect(ray)
    }
}
