#[macro_export]
macro_rules! any_scads {
    [ $($scad:expr_2021),* $(,)? ] => {
        {
            let v: Vec<Box<dyn ScadObject>> = vec![$(Box::new($scad)),*];
            v
        }
    };
}
#[macro_export]
macro_rules! any_scads2d {
    [ $($scad:expr_2021),* $(,)? ] => {
        {
            let v: Vec<Box<dyn ScadObject2D>> = vec![$(Box::new($scad)),*];
            v
        }
    };
}
#[macro_export]
macro_rules! any_scads3d {
    [ $($scad:expr_2021),* $(,)? ] => {
        {
            let v: Vec<Box<dyn ScadObject3D>> = vec![$(Box::new($scad)),*];
            v
        }
    };
}
