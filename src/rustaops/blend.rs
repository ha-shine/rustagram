use std::cmp;

#[allow(dead_code)]
pub fn compute_final_alpha(fg: &[u8; 4], bg: &[u8; 4]) -> u8 {
    let fg_alpha = fg[3] as f32 / 255.0;
    let bg_alpha = bg[3] as f32 / 255.0;
    let final_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);
    (final_alpha * 255.0) as u8
}

// ((uint8)(255 - (((255 - A) * (255 - B)) >> 8)))
#[allow(dead_code)]
pub fn blend_screen(x1: u8, x2: u8) -> u8 {
    let x1: u16 = x1 as u16;
    let x2: u16 = x2 as u16;
    (255 - ((255 - x1).wrapping_mul(255 - x2)>>8)) as u8
}

#[allow(dead_code)]
pub fn blend_darken(x1: u8, x2: u8) -> u8 {
    cmp::min(x1, x2)
}

#[allow(dead_code)]
pub fn blend_lighten(x1: u8, x2: u8) -> u8 {
    cmp::max(x1, x2)
}

#[allow(dead_code)]
pub fn blend_multiply(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32;
    (f1 * f2) as u8
}

#[allow(dead_code)]
pub fn blend_color_burn(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v = 1.0 - (1.0 - f1) / f2;
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_linear_burn(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v = f1 + f2 - 255.0;
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_color_dodge(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v;
    if f2 == 255.0 {
        v = f2;
    } else {
        v = f1 * (2.0 as f32).powi(8) / (255.0 - f2)
    }
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_linear_dodge(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v = f1 + f2;
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_overlay(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v;
    if f1 > 0.5 {
        v = 1.0 - (1.0 - 2.0 * (f1 - 0.5)) * (1.0 - f2);
    } else {
        v = 2.0 * f1 * f2;
    }
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_soft_light(x1: u8, x2: u8) -> u8 {
    let f2 = x2 as f32;
    let shifted = (x1>>1) as f32;
    if x2 < 128 {
        ((2.0*((shifted)+64.0))*(f2/255.0)) as u8
    } else {
        (255.0-(2.0*(255.0-((shifted)+64.0))*(255.0-f2)/255.0)) as u8
    }
}

#[allow(dead_code)]
pub fn blend_hard_light(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let v;
    let f2 = x2 as f32 / 255.0;
    if f2 > 0.5 {
        v = 1.0 - (1.0 - f1) * (1.0 - 2.0 * (f2 - 0.5));
    } else {
        v = f1 * (f2 + 2.0);
    }
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_vivid_light(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v;
    if f2 > 0.5 {
        v = 1.0 - (1.0 - f1) / (2.0 * (f2 - 0.5));
    } else {
        v = f1 / (1.0 - 2.0 * f2);
    }
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_linear_light(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v;
    if f2 > 0.5 {
        v = f1 + 2.0 * (f2 - 0.5);
    } else {
        v = f1 + 2.0 * f2 - 0.1;
    }
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_pin_light(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v;
    if f2 > 0.5 {
        let lhs = f1;
        let rhs = 2.0 * (f2 - 0.5);
        if lhs > rhs {
            v = lhs;
        } else {
            v = rhs;
        }
    } else {
        let lhs = f1;
        let rhs = 2.0 * f2;
        if lhs < rhs {
            v = lhs;
        } else {
            v = rhs;
        }
    }
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_difference(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v = (f1 - f2).abs();
    (v * 255.0) as u8
}

#[allow(dead_code)]
pub fn blend_exclusion(x1: u8, x2: u8) -> u8 {
    let f1 = x1 as f32 / 255.0;
    let f2 = x2 as f32 / 255.0;
    let v = 0.5 - 2.0 * (f1 - 0.5) * (f2 - 0.5);
    (v * 255.0) as u8
}