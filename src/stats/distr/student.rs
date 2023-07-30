use crate::special::Beta;

pub struct Student;

impl Student {
    /// Calculates the Probability Density Function (PDF) of the Student's T Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Student%27s_t-distribution#Probability_density_function" target="_blank">Wikipedia TPDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use vml::stats::distr::Student;
    ///
    /// let x = 0.975;
    /// let df = 6_f64;
    ///
    /// let tpdf = Student::pdf(x, df);
    ///
    /// assert_eq!(tpdf, 0.22873968790971655);
    /// ```
    /// <hr/>
    pub fn pdf(x: f64, df: f64) -> f64 {
        let p1 = 1_f64 + (x.powi(2) / df);
        let p2 = -((df + 1_f64) / 2_f64);
        let p3 = 1_f64 / (df.sqrt() * Beta::beta(0.5, df / 2_f64));
        p3 * p1.powf(p2)
    }
    /// Calculates the Cumulative Density Function (CDF) of the Student's T Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Student%27s_t-distribution#Cumulative_distribution_function" target="_blank">Wikipedia TCDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    /// ```
    /// use vml::stats::distr::Student;
    ///
    /// let lower = 1_f64;
    /// let upper = 1.96;
    /// let df = 6_f64;
    ///
    /// let tcdf = Student::tcdf(lower, upper,df);
    ///
    /// assert_eq!(tcdf, -0.12911126566804765)
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    /// ```
    /// use vml::stats::distr::Student;
    ///
    /// let lower = -9e+99;
    /// let upper = 1.96;
    /// let df = 63_f64;
    ///
    /// let tcdf = Student::cdf(lower, upper, df);
    ///
    /// assert_eq!(tcdf, 0.9727888170834724);
    /// ```
    /// <hr/>
    pub fn cdf(lower: f64, upper: f64, df: f64) -> f64 {
        let (bound_low, bound_high) = if lower < upper { (lower, upper) } else { (upper, lower) };
        let bound_low_sq = bound_low.powi(2);
        let bound_high_sq = bound_high.powi(2);

        if bound_low <= 0_f64 {
            let bound1 = df / (bound_low_sq + df);
            let bound2 = bound_high_sq / (bound_high_sq + df);
            let pt1 = Beta::regincbeta(df / 2_f64, 0.5, bound1) / 2_f64;
            let pt2 = (Beta::regincbeta(0.5, df / 2_f64, bound2) + 1_f64) / 2_f64;
            pt1 + pt2
        } else {
            let bound1 = bound_low_sq / (bound_low_sq + df);
            let bound2 = bound_high_sq / (bound_high_sq + df);
            let pt1 = (Beta::regincbeta(0.5, df / 2_f64, bound1) + 1_f64) / 2_f64;
            let pt2 = (Beta::regincbeta(0.5, df / 2_f64, bound2) + 1_f64) / 2_f64;
            pt1 - pt2
        }
    }
    /// Calculates the Inverse of TCDF (a.k.a InvT) <br>
    /// Learn more at: <a href="https://www.mathworks.com/help/stats/tinv.html" target="_blank">MatLab T Inverse</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use vml::stats::distr::Student;
    ///
    /// let area = 0.025_f64;
    /// let df = 63_f64;
    ///
    /// let inverse_t = Student::invt(area, df);
    ///
    /// assert_eq!(inverse_t, -1.998340526962578);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    /// ```
    /// use vml::stats::distr::Student;
    ///
    /// let area = 0.5_f64;
    /// let df = 96_f64;
    ///
    /// let inverse_t = Student::inv(area, df);
    ///
    /// assert_eq!(inverse_t, 0_f64);
    /// ```
    /// <hr/>
    pub fn inv(area: f64, df: f64) -> f64 {
        if (0_f64..0.5_f64).contains(&area) {
            let p1 = -1_f64 * df.sqrt();
            let z1 = df / 2_f64;
            let z2 = 0.5_f64;
            let x = 2_f64 * area;
            let p2 = Beta::invregincbeta(z1, z2, x);
            let p3 = 1_f64 / p2;
            let p4 = p3 - 1_f64;
            let p5 = p4.sqrt();
            p1 * p5
        } else if area == 0.5_f64 {
            0_f64
        } else if (0.5_f64..1_f64).contains(&area) {
            let p1 = df.sqrt();
            let z1 = df / 2_f64;
            let z2 = 0.5_f64;
            let x = 2_f64 * (1_f64 - area);
            let p2 = Beta::invregincbeta(z1, z2, x);
            let p3 = 1_f64 / p2;
            let p4 = p3 - 1_f64;
            let p5 = p4.sqrt();
            p1 * p5
        } else if area < 0_f64 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    }
}