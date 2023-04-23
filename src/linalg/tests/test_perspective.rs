use super::super::prelude::*;

#[test]
fn test_perspective()
{
    test_perspective_helper(-1.0, 1.0, -1.0, 1.0, 1.0, 2.0);

    test_perspective_helper(-2.0, 2.0, -1.0, 1.0, 1.0, 2.0);

    test_perspective_helper(-1.0, 1.0, -2.0, 2.0, 1.0, 2.0);
    test_perspective_helper(0.0,  2.0, -1.0, 1.0, 1.0, 2.0);
    test_perspective_helper(-1.0, 2.0, -1.0, 1.0, 1.0, 2.0);

    test_perspective_helper(-1.0, 2.0, -3.0, 4.0, 5.0, 6.0);
}

fn test_perspective_helper(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32)
{
    let m = Mat4::persp_lrtbnf(l, r, t, b, n, f);

    let tn = m * Vec3 { x: 0.0, y: 0.0, z: n };
    let tf = m * Vec3 { x: 0.0, y: 0.0, z: f };
    let tl = m * Vec3 { x: l,   y: 0.0, z: n };
    let tr = m * Vec3 { x: r,   y: 0.0, z: n };
    let tt = m * Vec3 { x: 0.0, y: t,   z: n };
    let tb = m * Vec3 { x: 0.0, y: b,   z: n };

    assert_eq!(tn.z,  0.0);
    assert_eq!(tf.z,  1.0);
    assert_eq!(tl.x, -1.0);
    assert_eq!(tr.x,  1.0);
    assert_eq!(tt.y, -1.0);
    assert_eq!(tb.y,  1.0);
}