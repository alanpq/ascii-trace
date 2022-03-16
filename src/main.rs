extern crate nalgebra_glm as glm;
extern crate pancurses;

use glm::{vec3, vec2, vec4, TVec3};
use pancurses::{endwin, initscr, noecho, Input, resize_term};
use rand::{thread_rng, Rng};
use std::rc::Rc;
use std::cell::{RefCell};

use std::sync::{Arc, RwLock};
use crate::material::Material;
use crate::renderables::{Plane, Sphere};
use crate::scene::{Scene};

mod material;
mod matrixes;
mod renderables;
mod intersect;
mod scene;
// use crate::matrixes;

const LUT: &[u8] = " .,-~:;=!*#$@".as_bytes();


fn cast_ray(source: &TVec3<f32>, dir: &TVec3<f32>, scene: Arc<RwLock<Scene>>) -> f32 {
  let scene = scene.read().unwrap();
  match &scene.intersect(source, dir) {
    Some(result) => {
      let light_dir = glm::normalize(&vec3(1., 1., 1.));
      let dot: f32 = glm::dot(&result.normal, &light_dir);
      // depth render
      // 1.-(result.dist/50.)

      // normal render
      0.1 + dot.max(0_f32) * result.obj.borrow().material().albedo
      * match &scene.intersect(&(result.hit + result.normal * 0.001), &light_dir) {
        Some(_) => 0.,
        None => 1.,
      }
    },
    None => 0.,
  }
  //1.
}

// currently using view_angles also for target_pos (cos im lazy)
fn render(win: &pancurses::Window, scene: Arc<RwLock<Scene>>, view_pos: &TVec3<f32>, view_angles: &TVec3<f32>) {
  let size = win.get_max_yx();
  let w = size.1 as f32;
  let h = size.0 as f32;
  win.mv(0, 0);
  // let fov: f32 = std::f32::consts::PI/1.2; // ortho fov
  let fov: f32 = std::f32::consts::PI/3.;
  for j in 0..size.0 {
    for i in 0..size.1 {
      // let x: f32 = (2.* (i as f32 + 0.5) / (size.1 as f32 - 1.))*(fov / 2. ).tan() * size.1 as f32/size.0 as f32;
      // let y: f32 = -(2.* (j as f32 + 0.5) / (size.0 as f32 - 1.))*(fov / 2. ).tan();
      // let dir: TVec3<f32> = glm::normalize(&vec3(x,y,-1.));
      
      // let forward = vec3(eulerA.y.sin() * eulerA.x.cos(), eulerA.x.cos(), eulerA.y.cos() * eulerA.x.cos());
      // let up      = vec3(eulerA.y.sin() * eulerA.x.cos(), eulerA.x.sin(), eulerA.y.cos() * eulerA.x.cos() );
      // //let up = vec3(0.,1.,0.);
      // let right = glm::cross(&up, &forward);
      let px = i as f32;
      let py = j as f32;

      // normalized device coords (screenspace [0,1] both axes)
      let ndcx = (px + 0.5) / w;
      let ndcy = (py + 0.5) / h;

      // screen space coords ([-1,1] both axes, origin at center)
      let fov_fac = (fov/2.).tan();
      let ssx = (2. * ndcx - 1.) * (w/h) * fov_fac;
      // inverted to flip y axis
      let ssy = (1. - 2. * ndcy) * fov_fac * 2.; // * 2, as 1 character is twice as tall as it is wide (roughly)

      // let c2w = matrixes::fps_matrix(view_pos, &view_angles.xy());

      let c2w = matrixes::look_at_matrix(view_pos, view_angles);

      //let c2w =  rotate_x(&rotate_y(&identity(), eulerA.y), eulerA.x);

      // let dir_x =  ((i as f32 + 0.5) - w/2.);
      // let dir_y = (-((j as f32 + 0.5) * 2.) + h/2.);
      // let dir_z = (-h / (2.* (fov/2.).tan() ));

      //let dir = forward * dir_z + right * dir_x + up * dir_y;

      // orthographic rendering (only works with look_at matrix)
      // let ray_start = view_pos + (c2w * vec4(ssx, ssy, 0., 0.)).xyz();
      // let ray_dir = glm::normalize(&(view_angles - view_pos));
      // let mut lum = cast_ray(&ray_start, &ray_dir, scene.clone());

      // normal perspective rays
      let mut lum = cast_ray(view_pos, &glm::normalize(&(c2w * vec4(ssx, ssy, -1., 0.)).xyz()), scene.clone());


      if lum < 0. {
        lum = 0.;
      }
      if lum > 1. {
        lum = 1.;
      }
      //lum = (i % 2) as f32;
      win.addch(LUT[(lum * 12.) as usize] as char);
    }
  }
}

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.refresh();
  window.nodelay(true);
  resize_term(20, 50);
  noecho();
  let scene = Arc::new(RwLock::new(Scene {
    objects: Vec::new()
  }));
  scene.write().unwrap().objects.push(Rc::new(RefCell::new(Plane {
    center: vec3(0.0, -5.0, 0.0),
    size: vec2(100.0, 100.0),
    material: Material {
      albedo: 0.5,
    }
  })));
  //let mut balls: Vec<Box<Sphere>> = Vec::new();
  let mut rng = thread_rng();
  for _ in 0..3 {
    scene.write().unwrap().objects.push(Rc::new(RefCell::new(Sphere {
      center: vec3(rng.gen_range(-10_f32..10_f32), rng.gen_range(-5_f32..5_f32), rng.gen_range(-10_f32..10_f32)),
      radius: rng.gen_range(2_f32..4_f32),
      material: Material {
        albedo: 1.,
      }
    })));
  }
  let sphere = Rc::new(RefCell::new(Sphere {
    center: vec3(0., 0., 0.),
    radius: 4.,
    material: Material {
      albedo: 1.,
    }
  }));
  scene.write().unwrap().objects.push(sphere);
  //let sphere = (scene.objects.last().unwrap());//.downcast::<Sphere>();
  //let mut s = sphere.as_mut();
  //let mut s = Sphere {center: vec3(0., 0., -16.), radius: 4.};
  let mut time: f32 = 0.;
  let radius:f32 = 12.;
  loop {
    render(&window, Arc::clone(&scene), &vec3(time.sin()*radius,3.,time.cos()*radius), &vec3(0.,0.,0.));
    // render(&window, Arc::clone(&scene), &vec3(10.0, 10.0, 10.0), &vec3(0., 0., 0.));

    // render(&window, Arc::clone(&scene), &vec3(7.0, 7.0, 10.0), &vec3(0., 0., 0.));


    // sphere.borrow_mut().center = vec3(0.0,0.0,time.sin());
    // sphere.borrow_mut().radius = (3. + (time*2.).sin()) * 2.0;
    time += 0.04;
    match window.getch() {
      Some(Input::KeyDC) => break,
      Some(Input::KeyResize) => {resize_term(0,0);},
      _ => {},
    }
  }
  endwin();
}
