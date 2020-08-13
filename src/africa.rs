#[macro_use]
extern crate geo;
use geo_types::{LineString, Polygon};

pub fn get_polygon() -> Polygon {
    let exterior = geo::line_string![
        (11.602070, 37.778170),
        (3.525989, 37.764440),
        (-1.967826, 36.321710),
        (-4.287849, 36.200820),
        (-5.602940, 35.987700),
        (-9.618688, 35.981020),
        (-15.514733, 29.500826),
        (-27.262032, 30.814000),
        (-23.245360, -60.316700),
        (44.639420, -57.087980),
        (66.722766, -14.903707),
        (51.630250, 12.550150),
        (44.207750, 11.678600),
        (43.654172, 12.549204),
        (43.357541, 12.634981),
        (43.338315, 12.790377),
        (43.107602, 13.210537),
        (42.679135, 13.592602),
        (42.517084, 14.088635),
        (42.044667, 14.711145),
        (39.813119, 18.162296),
        (37.902821, 22.238270),
        (34.741261, 27.031591),
        (34.475784, 28.006527),
        (34.705809, 28.576081),
        (34.937410, 29.425190),
        (34.879703, 29.557033),
        (34.885883, 29.642857),
        (34.849240, 29.786660),
        (34.242840, 31.296815),
        (32.706293, 33.975258),
        (11.600920, 33.998750),
    ];
    Polygon::new(interior, vec![])
}
