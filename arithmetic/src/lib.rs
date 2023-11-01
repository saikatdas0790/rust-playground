#[cfg(test)]
mod tests {
    #[test]
    fn compute_1() {
        // let hypotenuse: f64 = 2.0;
        let base: f64 = 6.0;
        let height: f64 = 15.0;

        let hypotenuse = (base.powi(2) + height.powi(2)).sqrt();

        println!("Hypotenuse: {}", hypotenuse);
    }

    #[test]
    fn compute_2() {
        let side_a: f64 = 2.0;
        let side_b: f64 = 3.0;
        let side_c: f64 = 6.0;

        let diagonal = (side_a.powi(2) + side_b.powi(2) + side_c.powi(2)).sqrt();

        println!("Diagonal: {}", diagonal);
    }

    #[test]
    fn compute_3() {}
}
