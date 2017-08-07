extern crate gnuplot;
extern crate rbf_interp;

use gnuplot::*;
use rbf_interp::{rbf_interpolation, Bbox, PtValue};

type Pt = PtValue<f64>;

fn example(c: Common) {
    let obs_points_two_stocks = vec![Pt::new(3.5, 3.5, 100.0), Pt::new(6.5, 6.5, 100.0)];
    let bbox = Bbox::new(0.0, 10.0, 0.0, 10.0);
    let (reso_x, reso_y) = (40, 40);
    let res_rbf = rbf_interpolation(reso_x,
                                    reso_y,
                                    &bbox,
                                    &obs_points_two_stocks,
                                    "inverse_multiquadratic",
                                    Some(1.66))
            .unwrap();
    let mut z1 = Vec::with_capacity(res_rbf.len());
    for pt in res_rbf {
        z1.push(pt.get_value());
    }

    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes3d()
        .set_title("Two stocks. Inverse multiquadratic RBF interpolation (epsilon: 1.66).",
                   &[])
        .surface(z1.iter(),
                 reso_x as usize,
                 reso_y as usize,
                 Some((0.0, 0.0, 10.0, 10.0)),
                 &[])
        .set_x_label("X", &[])
        .set_y_label("Y", &[])
        .set_z_label("Z", &[])
        .set_z_range(Auto, Auto)
        .set_palette(HELIX)
        .set_view(45.0, 175.0);
    c.show(&mut fg, None);

    let res_rbf = rbf_interpolation(reso_x,
                                    reso_y,
                                    &bbox,
                                    &obs_points_two_stocks,
                                    "gaussian",
                                    Some(1.66))
            .unwrap();
    let mut z1 = Vec::with_capacity(res_rbf.len());
    for pt in res_rbf {
        z1.push(pt.get_value());
    }

    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes3d()
        .set_title("Two stocks. Gaussian RBF interpolation (epsilon: 1.66).",
                   &[])
        .surface(z1.iter(),
                 reso_x as usize,
                 reso_y as usize,
                 Some((0.0, 0.0, 10.0, 10.0)),
                 &[])
        .set_x_label("X", &[])
        .set_y_label("Y", &[])
        .set_z_label("Z", &[])
        .set_z_range(Auto, Auto)
        .set_palette(HELIX)
        .set_view(45.0, 175.0);
    c.show(&mut fg, None);


}

pub struct Common {
    pub no_show: bool,
    pub term: Option<String>,
}

impl Common {
    pub fn new() -> Option<Common> {
        Some(Common {
                 no_show: false,
                 term: None,
             })
    }

    pub fn show(&self, fg: &mut Figure, filename: Option<&str>) {
        if !self.no_show {
            fg.show();
        }
        if filename.is_some() {
            fg.echo_to_file(filename.unwrap());
        }
    }

    pub fn set_term(&self, fg: &mut Figure) {
        self.term.as_ref().map(|t| { fg.set_terminal(&t[..], ""); });
    }
}


fn main() {
    Common::new().map(|c| example(c));
}


// x,y,z
// 3.5,3.5,100
// 6.5,6.5,100
