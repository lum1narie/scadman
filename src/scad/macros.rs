#[macro_export]
macro_rules! any_scads {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject>),*]
    };
}
#[macro_export]
macro_rules! any_scads2d {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject2D>),*]
    };
}
#[macro_export]
macro_rules! any_scads3d {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject3D>),*]
    };
}
