use std::ops;

use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Mat4
{
    pub v00: f32,
    pub v01: f32,
    pub v02: f32,
    pub v03: f32,

    pub v10: f32,
    pub v11: f32,
    pub v12: f32,
    pub v13: f32,

    pub v20: f32,
    pub v21: f32,
    pub v22: f32,
    pub v23: f32,

    pub v30: f32,
    pub v31: f32,
    pub v32: f32,
    pub v33: f32,
}

impl ops::Mul<Mat4> for Mat4
{
    type Output = Mat4;

    fn mul(self, r: Mat4) -> Mat4
    {
        Mat4
        {
            v00: self.v00 * r.v00 + self.v01 * r.v10 + self.v02 * r.v20 + self.v03 * r.v30,
            v01: self.v00 * r.v01 + self.v01 * r.v11 + self.v02 * r.v21 + self.v03 * r.v31,
            v02: self.v00 * r.v02 + self.v01 * r.v12 + self.v02 * r.v22 + self.v03 * r.v32,
            v03: self.v00 * r.v03 + self.v01 * r.v13 + self.v02 * r.v23 + self.v03 * r.v33,

            v10: self.v10 * r.v00 + self.v11 * r.v10 + self.v12 * r.v20 + self.v13 * r.v30,
            v11: self.v10 * r.v01 + self.v11 * r.v11 + self.v12 * r.v21 + self.v13 * r.v31,
            v12: self.v10 * r.v02 + self.v11 * r.v12 + self.v12 * r.v22 + self.v13 * r.v32,
            v13: self.v10 * r.v03 + self.v11 * r.v13 + self.v12 * r.v23 + self.v13 * r.v33,

            v20: self.v20 * r.v00 + self.v21 * r.v10 + self.v22 * r.v20 + self.v23 * r.v30,
            v21: self.v20 * r.v01 + self.v21 * r.v11 + self.v22 * r.v21 + self.v23 * r.v31,
            v22: self.v20 * r.v02 + self.v21 * r.v12 + self.v22 * r.v22 + self.v23 * r.v32,
            v23: self.v20 * r.v03 + self.v21 * r.v13 + self.v22 * r.v23 + self.v23 * r.v33,

            v30: self.v30 * r.v00 + self.v31 * r.v10 + self.v32 * r.v20 + self.v33 * r.v30,
            v31: self.v30 * r.v01 + self.v31 * r.v11 + self.v32 * r.v21 + self.v33 * r.v31,
            v32: self.v30 * r.v02 + self.v31 * r.v12 + self.v32 * r.v22 + self.v33 * r.v32,
            v33: self.v30 * r.v03 + self.v31 * r.v13 + self.v32 * r.v23 + self.v33 * r.v33,
        }
    }
}

impl ops::Mul<Vec4> for Mat4
{
    type Output = Vec4;

    fn mul(self, r: Vec4) -> Vec4
    {
        Vec4
        {
            x: self.v00 * r.x + self.v01 * r.y + self.v02 * r.z + self.v03 * r.w,
            y: self.v10 * r.x + self.v11 * r.y + self.v12 * r.z + self.v13 * r.w,
            z: self.v20 * r.x + self.v21 * r.y + self.v22 * r.z + self.v23 * r.w,
            w: self.v30 * r.x + self.v31 * r.y + self.v32 * r.z + self.v33 * r.w,
        }
    }
}

impl ops::Mul<Vec3> for Mat4
{
    type Output = Vec3;

    fn mul(self, r: Vec3) -> Vec3
    {
        let u = Vec4 { x: r.x, y: r.y, z: r.z, w: 1.0 };
        let v = self * u;
        Vec3 { x: v.x / v.w, y: v.y / v.w, z: v.z / v.w }
    }
}

impl Mat4
{
    pub const IDENTITY: Mat4 = Mat4
    {
        v00: 1.0, v01: 0.0, v02: 0.0, v03: 0.0,
        v10: 0.0, v11: 1.0, v12: 0.0, v13: 0.0,
        v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
        v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
    };

    pub fn translation_vec2(v: Vec2) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: v.x,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: v.y,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn translation_vec3(v: Vec3) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: v.x,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: v.y,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: v.z,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn translation_x(x: f32) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: x,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn translation_y(y: f32) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: y,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn translation_z(z: f32) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: z,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn translation_xy(x: f32, y: f32) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: x,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: y,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn translation_xyz(x: f32, y: f32, z: f32) -> Mat4
    {
        Mat4
        {
            v00: 1.0, v01: 0.0, v02: 0.0, v03: x,
            v10: 0.0, v11: 1.0, v12: 0.0, v13: y,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: z,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn rotation(q: Quat) -> Mat4
    {
        Mat4::axis
        (
            q * Vec3 { x: 1.0, y: 0.0, z: 0.0 },
            q * Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            q * Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        )
    }
    
    pub fn rotation_c(c: Complex) -> Mat4
    {
        Mat4
        {
            v00: c.r, v01:-c.i, v02: 0.0, v03: 0.0,
            v10: c.i, v11: c.r, v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0
        }
    }
    
    pub fn orienation(o: Mat3) -> Mat4
    {
        Mat4
        {
            v00: o.v00, v01: o.v01, v02: o.v02, v03: 0.0,
            v10: o.v10, v11: o.v11, v12: o.v12, v13: 0.0,
            v20: o.v20, v21: o.v21, v22: o.v22, v23: 0.0,
            v30: 0.0,   v31: 0.0,   v32: 0.0,   v33: 1.0,
        }
    }

    pub fn affine(o: Mat3, p: Vec3) -> Mat4
    {
        Mat4
        {
            v00: o.v00, v01: o.v01, v02: o.v02, v03: p.x,
            v10: o.v10, v11: o.v11, v12: o.v12, v13: p.y,
            v20: o.v20, v21: o.v21, v22: o.v22, v23: p.z,
            v30: 0.0,   v31: 0.0,   v32: 0.0,   v33: 1.0,
        }
    }

    pub fn swing(from: Vec3, to: Vec3) -> Mat4
    {
        let q = Quat::swing(from, to);
        Mat4::rotation(q)
    }

    pub fn swing_y(to: Vec3) -> Mat4
    {
        let q = Quat::swing_y(to);
        Mat4::rotation(q)
    }
    
    pub fn euler(x: f32, y: f32, z: f32) -> Mat4
    {
        Mat4::rotation(Quat::euler(x, y, z))
    }

    pub fn scale(x: f32) -> Mat4
    {
        Mat4
        {
            v00: x,   v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: x,   v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: x,   v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn scale_xy(x: f32, y: f32) -> Mat4
    {
        Mat4
        {
            v00: x,   v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: y,   v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: 1.0, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Mat4
    {
        Mat4
        {
            v00: x,   v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: y,   v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: z,   v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn scale_v(v: Vec3) -> Mat4
    {
        Mat4
        {
            v00: v.x, v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: v.y, v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: v.z, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }

    pub fn axis(i: Vec3, j: Vec3, k: Vec3) -> Mat4
    {
        return Mat4
        {
            v00: i.x, v01: j.x, v02: k.x, v03: 0.0,
            v10: i.y, v11: j.y, v12: k.y, v13: 0.0,
            v20: i.z, v21: j.z, v22: k.z, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        };
    }

    pub fn scale_vec3(v: Vec3) -> Mat4
    {
        Mat4
        {
            v00: v.x, v01: 0.0, v02: 0.0, v03: 0.0,
            v10: 0.0, v11: v.y, v12: 0.0, v13: 0.0,
            v20: 0.0, v21: 0.0, v22: v.z, v23: 0.0,
            v30: 0.0, v31: 0.0, v32: 0.0, v33: 1.0,
        }
    }
    
    pub fn trs(t: Vec3, r: Quat, s: Dim3) -> Mat4
    {
        Mat4::translation_vec3(t) * Mat4::rotation(r) * Mat4::scale_xyz(s.x, s.y, s.z)
    }

    pub fn orto_lrtbnf(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4
    {
        let width = right - left;
        let height = bottom - top;
        let depth = far - near;

        let sv = Vec3
        {
            x: 2.0 / width,
            y: 2.0 / height,
            z: 1.0 / depth,
        };

        let p = Vec3
        {
            x: (left + right) * 0.5,
            y: (bottom + top) * 0.5,
            z: near,
        };

        let t = Mat4::translation_vec3(-p);
        let s = Mat4::scale_v(sv);

        let r = s * t;
        
        r
    }

    pub fn orto_rect(rect: Span2, near: f32, far: f32) -> Mat4
    {
        Self::orto_lrtbnf(rect.x0, rect.x1, rect.y0, rect.y1, near, far)
    }
    
    pub fn orto(scale: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4
    {
        let mut left = -scale;
        let mut right = scale;
        let mut top = -scale;
        let mut bottom = scale;

        if aspect_ratio > 1.0
        {
            left *= aspect_ratio;
            right *= aspect_ratio;
        }
        else if aspect_ratio < 1.0
        {
            top /= aspect_ratio;
            bottom /= aspect_ratio;
        }

        Mat4::orto_lrtbnf(left, right, top, bottom, near, far)
    }

    pub fn persp_lrtbnf(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Mat4
    {
        let w = r - l;
        let h = b - t;

        let xa = f / (f - n);
        let xb = f * n / (n - f);

        Mat4
        {
            v00: 2.0 * n / w, v01: 0.0,         v02: -(l + r) / w, v03: 0.0,
            v10: 0.0,         v11: 2.0 * n / h, v12: -(t + b) / h, v13: 0.0,
            v20: 0.0,         v21: 0.0,         v22: xa,            v23: xb,
            v30: 0.0,         v31: 0.0,         v32: 1.0,           v33: 0.0,
        }
    }

    pub fn persp(view_angle: f32, aspect_ratio: f32, n: f32, f: f32) -> Mat4
    {
        let tan_phi = (view_angle * 0.5).tan();
    
        let mut l = -tan_phi * n;
        let mut r =  tan_phi * n;
        let mut t = -tan_phi * n;
        let mut b =  tan_phi * n;
    
        if aspect_ratio > 1.0
        {
            l *= aspect_ratio;
            r *= aspect_ratio;
        }
        else if aspect_ratio < 1.0
        {
            t /= aspect_ratio;
            b /= aspect_ratio;
        }
    
        Mat4::persp_lrtbnf(l, r, t, b, n, f)
    }

    pub fn persp_view_angels(angle_up: f32, angle_down: f32, angle_left: f32, angle_right: f32, n: f32, f: f32) -> Mat4
    {
        let tan_phi_up = angle_up.tan();
        let tan_phi_down = angle_down.tan();
        let tan_phi_left = angle_left.tan();
        let tan_phi_right = angle_right.tan();
    
        let l = tan_phi_left * n;
        let r = tan_phi_right * n;
        let t = tan_phi_up * n;
        let b = tan_phi_down * n;
    
        Mat4::persp_lrtbnf(l, r, t, b, n, f)
    }

    pub fn rect(rect: Span2) -> Mat4
    {
        Mat4::translation_xy(rect.x0, rect.y0) * Mat4::scale_xyz(rect.w(), rect.h(), 1.0)
    }

    pub fn urect(rect: USpan2) -> Mat4
    {
        Mat4::translation_xy(rect.x0 as f32, rect.y0 as f32) * Mat4::scale_xyz(rect.w() as f32, rect.h() as f32, 1.0)
    }

    pub fn transposed(&self) -> Mat4
    {
        Mat4
        {
            v00: self.v00, v01: self.v10, v02: self.v20, v03: self.v30,
            v10: self.v01, v11: self.v11, v12: self.v21, v13: self.v31,
            v20: self.v02, v21: self.v12, v22: self.v22, v23: self.v32,
            v30: self.v03, v31: self.v13, v32: self.v23, v33: self.v33,
        }
    }

    pub fn to_array(&self) -> [f32; 16]
    {
        [
            self.v00, self.v01, self.v02, self.v03,
            self.v10, self.v11, self.v12, self.v13,
            self.v20, self.v21, self.v22, self.v23,
            self.v30, self.v31, self.v32, self.v33
        ]
    }

    pub fn inverse(&self) -> Mat4
    {
        let mut inv = Mat4
        {
            v00:  self.v11 * self.v22 * self.v33
                - self.v11 * self.v23 * self.v32
                - self.v21 * self.v12 * self.v33
                + self.v21 * self.v13 * self.v32
                + self.v31 * self.v12 * self.v23
                - self.v31 * self.v13 * self.v22,

            v01: -self.v01 * self.v22 * self.v33
                + self.v01 * self.v23 * self.v32
                + self.v21 * self.v02 * self.v33
                - self.v21 * self.v03 * self.v32
                - self.v31 * self.v02 * self.v23
                + self.v31 * self.v03 * self.v22,

            v02: self.v01 * self.v12 * self.v33
                - self.v01 * self.v13 * self.v32
                - self.v11 * self.v02 * self.v33
                + self.v11 * self.v03 * self.v32
                + self.v31 * self.v02 * self.v13
                - self.v31 * self.v03 * self.v12,

            v03: -self.v01 * self.v12 * self.v23
                + self.v01 * self.v13 * self.v22
                + self.v11 * self.v02 * self.v23
                - self.v11 * self.v03 * self.v22
                - self.v21 * self.v02 * self.v13
                + self.v21 * self.v03 * self.v12,

            v10: -self.v10 * self.v22 * self.v33
                + self.v10 * self.v23 * self.v32
                + self.v20 * self.v12 * self.v33
                - self.v20 * self.v13 * self.v32
                - self.v30 * self.v12 * self.v23
                + self.v30 * self.v13 * self.v22,

            v11:  self.v00 * self.v22 * self.v33
                - self.v00 * self.v23 * self.v32
                - self.v20 * self.v02 * self.v33
                + self.v20 * self.v03 * self.v32
                + self.v30 * self.v02 * self.v23
                - self.v30 * self.v03 * self.v22,

            v12: -self.v00 * self.v12 * self.v33
                + self.v00 * self.v13 * self.v32
                + self.v10 * self.v02 * self.v33
                - self.v10 * self.v03 * self.v32
                - self.v30 * self.v02 * self.v13
                + self.v30 * self.v03 * self.v12,

            v13: self.v00 * self.v12 * self.v23
                - self.v00 * self.v13 * self.v22
                - self.v10 * self.v02 * self.v23
                + self.v10 * self.v03 * self.v22
                + self.v20 * self.v02 * self.v13
                - self.v20 * self.v03 * self.v12,

            v20:  self.v10 * self.v21 * self.v33
                - self.v10 * self.v23 * self.v31
                - self.v20 * self.v11 * self.v33
                + self.v20 * self.v13 * self.v31
                + self.v30 * self.v11 * self.v23
                - self.v30 * self.v13 * self.v21,

            v21: -self.v00 * self.v21 * self.v33
                + self.v00 * self.v23 * self.v31
                + self.v20 * self.v01 * self.v33
                - self.v20 * self.v03 * self.v31
                - self.v30 * self.v01 * self.v23
                + self.v30 * self.v03 * self.v21,

            v22:  self.v00 * self.v11 * self.v33
                - self.v00 * self.v13 * self.v31
                - self.v10 * self.v01 * self.v33
                + self.v10 * self.v03 * self.v31
                + self.v30 * self.v01 * self.v13
                - self.v30 * self.v03 * self.v11,

            v23: -self.v00 * self.v11 * self.v23
                + self.v00 * self.v13 * self.v21
                + self.v10 * self.v01 * self.v23
                - self.v10 * self.v03 * self.v21
                - self.v20 * self.v01 * self.v13
                + self.v20 * self.v03 * self.v11,

            v30: -self.v10 * self.v21 * self.v32
                + self.v10 * self.v22 * self.v31
                + self.v20 * self.v11 * self.v32
                - self.v20 * self.v12 * self.v31
                - self.v30 * self.v11 * self.v22
                + self.v30 * self.v12 * self.v21,

            v31:  self.v00 * self.v21 * self.v32
                - self.v00 * self.v22 * self.v31
                - self.v20 * self.v01 * self.v32
                + self.v20 * self.v02 * self.v31
                + self.v30 * self.v01 * self.v22
                - self.v30 * self.v02 * self.v21,

            v32: -self.v00 * self.v11 * self.v32
                + self.v00 * self.v12 * self.v31
                + self.v10 * self.v01 * self.v32
                - self.v10 * self.v02 * self.v31
                - self.v30 * self.v01 * self.v12
                + self.v30 * self.v02 * self.v11,

            v33:  self.v00 * self.v11 * self.v22
                - self.v00 * self.v12 * self.v21
                - self.v10 * self.v01 * self.v22
                + self.v10 * self.v02 * self.v21
                + self.v20 * self.v01 * self.v12
                - self.v20 * self.v02 * self.v11,
        };

        let mut det = self.v00 * inv.v00 + self.v01 * inv.v10 + self.v02 * inv.v20 + self.v03 * inv.v30;

        det = 1.0 / det;

        inv = Mat4
        {
            v00: inv.v00 * det,
            v01: inv.v01 * det,
            v02: inv.v02 * det,
            v03: inv.v03 * det,

            v10: inv.v10 * det,
            v11: inv.v11 * det,
            v12: inv.v12 * det,
            v13: inv.v13 * det,

            v20: inv.v20 * det,
            v21: inv.v21 * det,
            v22: inv.v22 * det,
            v23: inv.v23 * det,

            v30: inv.v30 * det,
            v31: inv.v31 * det,
            v32: inv.v32 * det,
            v33: inv.v33 * det,
        };

        return inv;
    }
}
