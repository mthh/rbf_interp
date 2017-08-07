use num_traits::Float;
use utils::PtValue;

#[derive(Debug, Clone, Copy)]
pub struct Bbox<T> {
    pub min_x: T,
    pub max_x: T,
    pub min_y: T,
    pub max_y: T,
}

impl<T> Bbox<T>
    where T: Float
{
    pub fn new(min_x: T, max_x: T, min_y: T, max_y: T) -> Self {
        Bbox {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }

    pub fn from_points(obs_points: &[PtValue<T>]) -> Self {
        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (T::infinity(), T::neg_infinity(), T::infinity(), T::neg_infinity());
        for pt in obs_points {
            let (pt_x, pt_y) = pt.get_coordinates();
            if pt_x > max_x {
                max_x = pt_x;
            } else if pt_x < min_x {
                min_x = pt_x;
            }
            if pt_y > max_y {
                max_y = pt_y;
            } else if pt_y < min_y {
                min_y = pt_y;
            }
        }
        Bbox {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }
}
