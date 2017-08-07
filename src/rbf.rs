use errors::*;
use utils::PtValue;
use bbox::Bbox;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use num_traits::{Float, Zero, FromPrimitive};


#[derive(Debug, Clone)]
pub struct Rbf<'a, T: 'a> {
    obs_points: &'a [PtValue<T>],
    weights: Vector<T>,
    distance_function: fn(T, T) -> T,
    epsilon: T,
}

impl<'a, T> Rbf<'a, T>
    where T: Float + Zero + FromPrimitive + 'static
{
    pub fn new(obs_points: &'a [PtValue<T>], distance_function: &str, epsilon: Option<T>) -> Self {
        let distance_func = match distance_function {
            "linear" => distance_linear,
            "cubic" => distance_cubic,
            "thin_plate" => distance_thin_plate,
            "quintic" => distance_quintic,
            "gaussian" => distance_gaussian,
            "multiquadratic" => distance_multiquadratic,
            "inverse_multiquadratic" => distance_inverse_multiquadratic,
            &_ => panic!("Invalid function name!"),
        };
        let nb_pts = obs_points.len();
        let mut mat = vec![Zero::zero(); nb_pts * nb_pts];
        for j in 0..nb_pts {
            for i in 0..nb_pts {
                mat[j * nb_pts + i] = _norm(&obs_points[i], &obs_points[j]);
            }
        }
        let eps = if epsilon.is_some() {
            epsilon.unwrap()
        } else {
            let _nb = T::from_usize(nb_pts).unwrap();
            sum_all(&mat) / (_nb.powi(2) - _nb)
        };
        // for j in 0..nb_pts {
        //     for i in 0..nb_pts {
        //         mat[j * nb_pts + i] = distance_func(mat[j * nb_pts + i], eps);
        //     }
        // }
        for ix in 0..(nb_pts * nb_pts) {
            mat[ix] = distance_func(mat[ix], eps);
        }
        let mut values: Vec<T> = Vec::with_capacity(nb_pts);
        for i in 0..nb_pts {
            values.push(obs_points[i].get_value());
        }
        let mat = Matrix::new(nb_pts, nb_pts, mat);
        let vec = Vector::new(values);
        // let weights = mat.solve(vec).unwrap().into_iter().collect::<Vec<Float>>();
        let weights = mat.solve(vec).unwrap();
        Rbf {
            obs_points: obs_points,
            distance_function: distance_func,
            epsilon: eps,
            weights: weights,
        }
    }

    pub fn interp_point(&self, pt: (T, T)) -> T {
        let _pt = PtValue::new(pt.0, pt.1, Zero::zero());
        let mut distances: Vec<T> = Vec::with_capacity(self.obs_points.len());
        for point in self.obs_points {
            let a = _norm(&_pt, point);
            distances.push((self.distance_function)(a, self.epsilon));
        }
        let dist = Vector::new(distances);
        let r = &dist.elemul(&self.weights);
        r.sum()
    }
}

/// Function allowing to compute the choosen radial basis interpolation
/// from scattered data on a grid defined by its Bbox and by its resolution
/// on the x and y axis.
pub fn rbf_interpolation<T>(reso_x: u32,
                            reso_y: u32,
                            bbox: &Bbox<T>,
                            obs_points: &[PtValue<T>],
                            func_name: &str,
                            epsilon: Option<T>)
                            -> Result<Vec<PtValue<T>>>
    where T: Float + Zero + FromPrimitive + 'static
{
    let rx: T = T::from_u32(reso_x).unwrap();
    let ry: T = T::from_u32(reso_y).unwrap();
    let x_step = (bbox.max_x - bbox.min_x) / rx;
    let y_step = (bbox.max_y - bbox.min_y) / ry;
    let mut plots = Vec::with_capacity((reso_x * reso_y) as usize);
    let rbf = Rbf::new(obs_points, func_name, epsilon);
    for i in 0..reso_x {
        for j in 0..reso_y {
            let x = bbox.min_x + x_step * T::from_u32(i).unwrap();
            let y = bbox.min_y + y_step * T::from_u32(j).unwrap();
            let value = rbf.interp_point((x, y));
            plots.push(PtValue::new(x, y, value));
        }
    }
    Ok(plots)
}

fn sum_all<T>(mat: &Vec<T>) -> T
    where T: Float
{
    let mut s: T = Zero::zero();
    for &v in mat {
        s = s + v;
    }
    s
}


fn _norm<T>(pa: &PtValue<T>, pb: &PtValue<T>) -> T
    where T: Float + Zero
{
    let ca = pa.get_coordinates();
    let cb = pb.get_coordinates();
    ((ca.0 - cb.0).powi(2) + (ca.1 - cb.1).powi(2)).sqrt()
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_linear<T>(r: T, epsilon: T) -> T
    where T: Float
{
    r
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_cubic<T>(r: T, epsilon: T) -> T
    where T: Float
{
    r.powi(3)
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_quintic<T>(r: T, epsilon: T) -> T
    where T: Float
{
    r.powi(5)
}

#[inline(always)]
#[allow(unused_variables)]
fn distance_thin_plate<T>(r: T, epsilon: T) -> T
    where T: Float
{
    if r == Zero::zero() {
        Zero::zero()
    } else {
        r.powi(2) * r.ln()
    }
}

#[inline(always)]
fn distance_gaussian<T>(r: T, epsilon: T) -> T
    where T: Float
{
    T::one() / ((r / epsilon).powi(2) + T::one()).exp()
}

#[inline(always)]
fn distance_inverse_multiquadratic<T>(r: T, epsilon: T) -> T
    where T: Float
{
    T::one() / ((r / epsilon).powi(2) + T::one()).sqrt()
}

#[inline(always)]
fn distance_multiquadratic<T>(r: T, epsilon: T) -> T
    where T: Float
{
    ((r / epsilon).powi(2) + T::one()).sqrt()
}
