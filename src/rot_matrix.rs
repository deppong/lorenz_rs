// https://en.wikipedia.org/wiki/Rotation_matrix

pub fn rot_x(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    /*
        x axis rotation matrix
        [ 1       0           0     ] 
        [ 0 angle.cos() -angle.sin()] 
        [ 0 angle.sin()  angle.cos()]
    */
    ( 
        x, 
        x + y * angle.cos() + z * -angle.sin(),
        x + y * angle.sin() + z *  angle.cos()
    )
}

pub fn rot_y(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    /*
        y axis rotation matrix
        [ angle.cos()  0 angle.sin()] 
        [      0       1      0     ] 
        [ -angle.sin() 0 angle.cos()]
    */
    (
        x * angle.cos() + z * angle.sin(), 
        y,
        x * -angle.sin() + z * angle.cos()
    )
}

pub fn rot_z(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    /*
        z axis rotation matrix
        [ angle.cos()  -angle.sin()  0 ] 
        [ angle.sin()  angle.cos()  0 ] 
        [      0       0            1 ] 
    */
    (
        x * angle.cos() + y * -angle.sin(),
        x * angle.sin() + y * angle.cos(),
        z
    )
}
