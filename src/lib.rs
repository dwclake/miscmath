

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector2::add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod mathlib
{

}

pub mod vector2
{
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}
