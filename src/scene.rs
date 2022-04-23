use crate::bxdf::{BxDF, Lambert};
use crate::core::{IntersectInfoGlobal, IntersectableGlobal, IntersectableLocal, Ray, ShadingInfo};
use crate::intersector::Intersector;
use crate::vec3::{build_orthonormal_basis, Vec3};

use std::rc::Rc;

pub struct Material {
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub emission: Vec3,
}

// TODO: make intersector selectable
pub struct Scene {
    _primitives: Rc<Vec<Box<dyn IntersectableLocal>>>,
    materials: Vec<Material>,
    intersector: Intersector,
}

impl Scene {
    pub fn new(primitives: Vec<Box<dyn IntersectableLocal>>, materials: Vec<Material>) -> Self {
        let primitives = Rc::new(primitives);
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

    pub fn get_bxdf(&self, prim_idx: u32) -> Box<dyn BxDF> {
        let material = &self.materials[prim_idx as usize];
        Box::new(Lambert::new(material.diffuse))
    }
}

impl IntersectableGlobal for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfoGlobal> {
        self.intersector.intersect(ray)
    }
}
