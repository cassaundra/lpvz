use std::ops::Neg;

pub fn rect(width: u8, height: u8) -> Vec<(u8, u8)> {
    let mut coords = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            coords.push((x, y));
        }
    }

    coords
}

pub fn ring(size: u8) -> Vec<(u8, u8)> {
    let length = (size as usize - 1) * 4;

    let mut coords = Vec::with_capacity(length);

    let (mut x, mut y) = (0i8, 0i8);
    let (mut dx, mut dy) = (1i8, 0i8);

    let mut side_length = 0;

    for _ in 0..length {
        coords.push((x as u8, y as u8));

        x += dx;
        y += dy;
        side_length += 1;

        if side_length == size - 1 {
            side_length = 0;

            std::mem::swap(&mut dx, &mut dy);
            dx = dx.neg();
        }
    }

    coords
}
