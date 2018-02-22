use std::cmp;

#[allow(dead_code)]
pub fn compute_final_alpha(fg: &[u8; 4], bg: &[u8; 4]) -> u8 {
    let fg_alpha = fg[3] as f32 / 255.0;
    let bg_alpha = bg[3] as f32 / 255.0;
    let final_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);
    (final_alpha * 255.0) as u8
}

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
    let x1 = x1 as u16;
    let x2 = x2 as u16;
    ((x1 * x2) / 255) as u8
}

#[allow(dead_code)]
pub fn blend_color_burn(x1: u8, x2: u8) -> u8 {
    if x2 == 0 {
        x2
    } else {
        let x1 = x1 as u16;
        let x2 = x2 as u16;
        let max = 255 as u16;
        let rhs = max.wrapping_sub(((max - x1)<<8)/x2);
        if rhs > 0 {
            rhs as u8
        } else {
            0
        }
    }
}

#[allow(dead_code)]
pub fn blend_subtract(x1: u8, x2: u8) -> u8 {
    let x1 = x1 as u16;
    let x2 = x2 as u16;
    let lhs = x1 + x2;
    if lhs < 255 {
        0
    } else {
        (lhs - 255) as u8
    }
}

#[allow(dead_code)]
pub fn blend_linear_burn(x1: u8, x2: u8) -> u8 {
    blend_subtract(x2, x1)
}

#[allow(dead_code)]
pub fn blend_color_dodge(x1: u8, x2: u8) -> u8 {
    if x2 == 255 {
        x2
    } else {
        let x1: u16 = x1 as u16;
        let x2: u16 = x2 as u16;
        let rhs = (x1<<8)/(255-x2);
        if 255 < rhs {
            255
        } else {
            rhs as u8
        }
    }
}

#[allow(dead_code)]
pub fn blend_add(x1: u8, x2: u8) -> u8 {
    let rhs = x1.wrapping_add(x2);
    rhs
}

#[allow(dead_code)]
pub fn blend_linear_dodge(x1: u8, x2: u8) -> u8 {
    blend_add(x2, x1)
}

#[allow(dead_code)]
pub fn blend_overlay(x1: u8, x2: u8) -> u8 {
    let x1 = x1 as u16;
    let x2 = x2 as u16;
    if x2 < 128 {
        (2 * x1 * x2 / 255) as u8
    } else {
        (255 - 2 * (255 - x1) * (255 - x2) / 255) as u8
    }
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
    blend_overlay(x2, x1)
}

#[allow(dead_code)]
pub fn blend_vivid_light(x1: u8, x2: u8) -> u8 {
    if x2 < 128 {
        blend_color_burn(x1, x2 * 2)
    } else {
        blend_color_dodge(x1, 2 * (x2 - 128))
    }
}

#[allow(dead_code)]
pub fn blend_linear_light(x1: u8, x2: u8) -> u8 {
    if x2 < 128 {
        blend_linear_burn(x1, 2 * x2)
    } else {
        blend_linear_dodge(x1, 2 * (x2 - 128))
    }
}

#[allow(dead_code)]
pub fn blend_pin_light(x1: u8, x2: u8) -> u8 {
    if x2 < 128 {
        blend_darken(x1, 2 * x2)
    } else {
        blend_lighten(x1, 2 * (x2 - 128))
    }
}

#[allow(dead_code)]
pub fn blend_difference(x1: u8, x2: u8) -> u8 {
    let x1 = x1 as i16;
    let x2 = x2 as i16;
    (x1 - x2).abs() as u8
}

#[allow(dead_code)]
pub fn blend_exclusion(x1: u8, x2: u8) -> u8 {
    let x1 = x1 as u32;
    let x2 = x2 as u32;
    (x1 + x2 - 2 * x1 * x2 / 255) as u8
}