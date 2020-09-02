const EXTENT: u32 = 4096;

fn project_point_to_tile(lng: f64, lat: f64, z: u32, x: u32, y: u32) -> (u32, u32) {
    let (px, py) = to_planar(lng, lat, z + 12);
    let miny: f64 = (y << 12).into();
    let minx: f64 = (x << 12).into();
    ((px - minx).floor() as u32, (py - miny).floor() as u32)
}

fn to_planar(lng: f64, lat: f64, level: u32) -> (f64, f64) {
    let max_tiles = (1 << level) as f64;
    let x = ((lng / 360f64) + 0.5) * (max_tiles).floor();
    let siny = ((PI * lat) / 180f64).sin();
    let y = match siny {
        d if d < -0.9999 => 0.0,
        d if d > 0.9999 => max_tiles - 1.0,
        _ => (((((1.0 + siny) / (1.0 - siny)).ln()) * 0.5) / (-2.0 * PI) + 0.5) * max_tiles,
    };
    (x, y)
}
